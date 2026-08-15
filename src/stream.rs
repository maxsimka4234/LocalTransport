use std::{ collections::HashSet, fs::{File}, io::{self, Read, Write}, net::{TcpListener, TcpStream}, path::{Path}};
use indicatif::ProgressBar;

pub fn real_addr() -> std::io::Result<std::net::IpAddr>{
    let interfaces = if_addrs::get_if_addrs()?;
    let mut is_local = HashSet::new();
    for interface in interfaces {
        match interface.ip() {
            std::net::IpAddr::V4(ipv4) => {
                if ipv4.octets()[0] == 192 && ipv4.octets()[1] == 168  || ipv4.octets()[0] == 10 || ipv4.octets()[0] == 172 && (16..=31).contains(&ipv4.octets()[1]){
                    is_local.insert(interface.ip());
                }
            }
            std::net::IpAddr::V6(_) => {/*is_local.insert(interface.ip());*/}
        }
       // if interface.ip().to_string().contains("192.168") {
       //     println!("interface{:?}", interface)
      //  }
    }
    let interfaces:Vec<std::net::IpAddr> = is_local.into_iter().collect();
    if interfaces.is_empty() {
        return Err(std::io::Error::new(io::ErrorKind::NotFound, "Не найден сетевой интерфейс (192.168.x.x / 10.x.x.x / 172.16-31.x.x)"));
    }
    Ok(interfaces[rand::random_range(..interfaces.len())])
}



pub fn has_path(path: &str) -> bool{
    let path = Path::new(path);
    path.is_absolute()
}

pub fn create_stream() -> std::io::Result<TcpListener>{
    let addr = "0.0.0.0:0";

    TcpListener::bind(addr)
}


pub fn send_file(path: &str, mut stream: TcpStream) -> std::io::Result<()>{
    let file = File::open(path)?;
    let file_name = std::path::Path::new(path).file_name().ok_or(io::Error::new(io::ErrorKind::NotFound,"Отсутсвует"))?.to_string_lossy();

    let name_bytes = file_name.as_bytes();
    stream.write_all(&(name_bytes.len() as u32).to_be_bytes())?;
    stream.write_all(name_bytes)?;

    let bytes = file.metadata()?.len();
    stream.write_all(&bytes.to_be_bytes())?;
    
    println!("Отправляю файл");

    let pb = ProgressBar::new(bytes);
    let mut file_reader = pb.wrap_read(file);
    let bytes_copy = io::copy(&mut file_reader, &mut stream)?;

    println!("Отправлено байт {}", bytes_copy);
    Ok(())
}

pub fn getting_file(stream: &mut TcpStream, path: &str) -> std::io::Result<()>{
    // TcpStream отдельно от TcpListener чтобы accept() не сбрасывался после окончания принятия
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf)?;
    let name_len = u32::from_be_bytes(len_buf) as usize;

    let mut name_buf = vec![0u8; name_len];
    stream.read_exact(&mut name_buf)?;
    let file_name = String::from_utf8_lossy(&name_buf);

    let mut buf = [0u8; 8];
    stream.read_exact(&mut buf)?;
    let file_len = u64::from_be_bytes(buf);

    let mut file = if path.ends_with("/") { File::create(format!("{}{}", path, file_name))? } else {
        File::create(format!("{}/{}", path, file_name))?
    };
    
    let pb = ProgressBar::new(file_len);
    let mut limited_reader = pb.wrap_read(stream.take(file_len));

    let bytes_written = std::io::copy(&mut limited_reader, &mut file)?;

    if bytes_written != file_len {
        return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Ошибка в получении файла"));
    }

    println!("Файл получен!");
    Ok(())
}