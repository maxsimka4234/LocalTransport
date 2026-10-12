use std::{fs::{self, File}, io::{self, Read, Write}, net::TcpStream, path::{Path, PathBuf}};
use indicatif::ProgressBar;

pub fn itfile(stream: &mut TcpStream, is_file: bool ) -> std::io::Result<()>{
    match is_file {
        true => {   
            let buf = [1u8];
            stream.write_all(&buf)?;
            Ok(())
        }
        false => {
            let buf = [0u8];
            stream.write_all(&buf)?;
            Ok(())
        }
    }  
}

pub fn send_file(path: &str, relative: &str, stream: &mut TcpStream) -> std::io::Result<()>{
    
    let file = File::open(path)?;
   // let file_name = Path::new(path).file_name().ok_or(io::Error::new(io::ErrorKind::NotFound,"Отсутствует"))?.to_string_lossy();
    let name_bytes = relative.as_bytes();
    stream.write_all(&(name_bytes.len() as u32).to_be_bytes())?;
    stream.write_all(name_bytes)?;

    let bytes = file.metadata()?.len();
    stream.write_all(&bytes.to_be_bytes())?;
    
    println!("Отправляю файл {}", relative);

    let pb = ProgressBar::new(bytes);
    let mut file_reader = pb.wrap_read(file);
    let bytes_copy = io::copy(&mut file_reader, stream)?;
    tracing::debug!(target: "all", "Отправка файла: {}, bytes: {}", path, bytes_copy);
    println!("Отправлено байт {}", bytes_copy);
    Ok(())
}

/// берет путь и добавляет к нему имя 
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

    let full_path = Path::new(path).join(&*file_name);
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = File::create(full_path)?;
    
    let pb = ProgressBar::new(file_len);
    let mut limited_reader = pb.wrap_read(stream.take(file_len));

    let bytes_written = std::io::copy(&mut limited_reader, &mut file)?;

    if bytes_written != file_len {
        return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Ошибка в получении файла"));
    }
    tracing::debug!(target: "reciever", "Получение файла: {}", file_name);
    println!("Файл получен!");
    Ok(())
}

fn send_quantity(quantity: u64, stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>>{
    // Отправлять количество файлов, после принимать getting_file столько раз, сколько это нужно 
    let path_quantity = quantity.to_be_bytes();
    println!("Отправляю количество файлов: {} ",quantity);
    tracing::debug!(target: "sender", "Отправка файлов в кол-ве {}",quantity );

    stream.write_all(&path_quantity)?;
    Ok(())
}



fn get_quantity(stream: &mut TcpStream) -> Result<u64, Box<dyn std::error::Error>>{
    let mut buf = [0u8];
    stream.read_exact(&mut buf)?;

    if buf[0] != 1 {
        let mut buf = [0u8; 8];
        stream.read_exact(&mut buf)?;
        let file_quantity = u64::from_be_bytes(buf);
        println!("Принял количество файлов: {}", file_quantity);
        tracing::debug!(target: "reciever", "Принял {} файлов", file_quantity);
        Ok(file_quantity)
    } else {
        Ok(0)
    }
}

//fn send_directory(path: &Path, root: &Path, stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
//    for entry in fs::read_dir(path)? {
//        let entry = entry?;
//        let path = entry.path();  
//        if path.is_symlink() {
//            continue;
//        }
//        if path.is_dir() {
//            send_directory(&path, root, stream)?;
//        } else {
//            let relative = &path.strip_prefix(root)?.to_string_lossy().replace("\\", "/");
//            send_file(&path.to_string_lossy(), relative, stream)?; //просто отправлять вместе с 0
//        }
//    }   
//    Ok(())
//}


pub fn send_all(path: &PathBuf, stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {

    fn files_quantity(path: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>>{
    let mut pathes = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        // считает количество файлов
        if path.is_symlink() {
            continue;
        }
        if path.is_dir() {
            pathes.extend(files_quantity(&path)?);
        } else {
            pathes.push(path);
        }
    }   
    Ok(pathes)
    }

    let files = files_quantity(path)?;
    send_quantity(files.len() as u64, stream)?;

    for file_path in files {
        tracing::debug!(target: "sender", "(MANY) Отправка файла: {:?}", file_path);
        let relative = &file_path.strip_prefix(path)?.to_string_lossy().replace("\\", "/");
        send_file(&file_path.to_string_lossy(), relative, stream)?;
        //send_file(&relative, relative, stream)?;
    }
    
    Ok(())
   // send_directory(path, path, stream)
}

pub fn read_files(path: &Path, stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>>{
    let mut quantity = get_quantity(stream)?;
    if quantity != 0 {
    while quantity != 0 {
        getting_file(stream, &path.to_string_lossy())?;
        quantity -= 1;
        println!("Отправлено! ")
    } } else {
        getting_file(stream, &path.to_string_lossy())?;
    }
    Ok(())
}
