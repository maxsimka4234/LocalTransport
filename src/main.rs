use std::{error::Error, fs::File, io::{self, Read, Write}, net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream}, sync::mpsc, thread, time::Duration};
use local_ip_address::local_ip;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};

fn hello() -> Result<(), Box<dyn std::error::Error>> {
    println!("Здраствуй! 1 - принимает файл (путь к итогу), 2 - отправляет файл (путь к файлу) ");
   // let mut addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5656);
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не строка");
    let input = input.trim().parse::<u8>().expect("Не число");

    let service_type = "_localTransfer._tcp.local.";
    let instance_name = "my-localTransfer-instance";
    let host_name = "my-localTransfer-host.local.";
    let ip = local_ip()?;
    
    if input == 1 {
        println!("Понял, я принимаю.\n Закончим настройку");
        println!("Укажите путь принятия файла: ");

        let mut path =String::new();
        io::stdin().read_line(&mut path).expect("Не строка");
        if !has_path(&path){println!("Не путь!"); return Err(Box::new(std::io::Error::new(io::ErrorKind::IsADirectory, "Не путь!")));}
        let path = path.trim();
        let (tx, rx) = mpsc::channel();

        let tcp_handle = thread::spawn(move || {
            //C:\Users\макс4234\Desktop\brb2
            println!("открыл тсп1 ");
            let listener = create_stream().unwrap();
            
           // let listener = listener.accept().unwrap();
            println!("открыл тсп {:?}", &listener);
            
            tx.send(listener).expect("send failed");
        });


        println!("запускаю ");
        let stream = rx.iter().next().unwrap();
         println!("запускаю2 ");
         let local_addr = stream.local_addr().unwrap();
        
        
        

        
        let mdns = ServiceDaemon::new().unwrap();
        println!("мднс {:?} ", mdns.monitor());
        
        let servise_info = ServiceInfo::new(service_type, instance_name, host_name, local_addr.ip(), local_addr.port(), None).unwrap();
        println!("serv info {:?} ", servise_info);
        mdns.register(servise_info).unwrap();
        println!("мднс зареган ");

        let mut stream = stream.accept().unwrap();

        println!("беб {:?}", &stream.0);
        getting_file(&mut stream.0, &path).unwrap();
        
        

        tcp_handle.join().unwrap();
   
        
    } else {
        //println!("Понял, принимаю {}", addr);
        println!("Понял, я отправляю.\n Закончим настройку");
        println!("Укажите путь отправки файла: ");

        let mut path =String::new();
        io::stdin().read_line(&mut path).expect("Не строка");
        if !has_path(&path){println!("Не путь!"); return Err(Box::new(std::io::Error::new(io::ErrorKind::IsADirectory, "Не путь!")));}
        let path = path.trim();

        let mdns = ServiceDaemon::new()?;

        let receiver = mdns.browse(service_type)?;
        println!("Ищу хост... ");
        
        while let Ok(event) = receiver.recv() {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    println!("хост найден!");
                    let port = info.get_addresses_v4();
                    
                    let addr = port.iter().next().ok_or("не ок")?;
                    let mut addr = addr.to_string();

                    addr.push_str(&format!(":{}",&info.get_port().to_string()));

                    
                    let stream = TcpStream::connect(addr)?;
                    
                    send_file(&path, stream)?;
                }
                ServiceEvent::ServiceRemoved(serv_type, name ) => {
                    println!("Хост {} ({}) удален", name, serv_type)
                }
                _ => {}
            }
        }
    
    }
    Ok(())
}



fn has_path(path: &str) -> bool{
    path.contains(":\\")
}

fn create_stream() -> std::io::Result<TcpListener>{
    let addr = "127.0.0.1:0";

    TcpListener::bind(addr)
}


fn send_file(path: &str, mut stream: TcpStream) -> std::io::Result<()>{
    let mut file = File::open(path)?;
    let bytes = file.metadata()?.len();
    stream.write_all(&bytes.to_be_bytes())?;

    let bytes_copy = io::copy(&mut file, &mut stream)?;
    println!("Отправлено байт {}", bytes_copy);
    Ok(())
}

fn getting_file(stream: &mut TcpStream, path: &str) -> std::io::Result<()>{
    // TcpStream отдельно от TcpListener чтобы accept() не сбрасывался после окончания принятия
    let mut buf = [0u8; 8];
    stream.read_exact(&mut buf)?;
    let file_len = u64::from_be_bytes(buf);


    let mut file = File::create(path)?;
    let mut limited_reader = stream.take(file_len);

    let bytes_written = std::io::copy(&mut limited_reader, &mut file)?;

    if bytes_written != file_len {
        return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Ошибка в получении файла"));
    }

    println!("Файл получен");
    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>>{
   hello()?;
   Ok(())
}

#[cfg(test)]

#[test]
fn server_test()-> std::io::Result<()>  {
    use std::thread;
    use std::fs;

    let temp_file_get = tempfile::NamedTempFile::new()?;
    let mut temp_file_send = tempfile::NamedTempFile::new()?;

    writeln!(&mut temp_file_send, "Возможно тут типо большой да очень файл .exe ааа дааа оооо ууеее бббеееб да файл большой просто")?;
    let tfg2 = fs::read(&temp_file_send)?;

    let stream = TcpListener::bind("127.0.0.1:5656")?;

    let server = thread::spawn(move || {
        let mut stream = TcpListener::accept(&stream).expect("мсг");
        let pathi= &temp_file_get.path().to_str().expect("ее").to_string();
        //temp_file_get.close();
        getting_file(&mut stream.0, pathi).unwrap();


        let tfg1 = fs::read(pathi).expect("иеше");
        println!("Получено! {:?} {:?}", String::from_utf8(tfg1.clone()), stream.0);
        assert_eq!(tfg1, tfg2);
    });

    thread::sleep(std::time::Duration::from_millis(50));
    let mut client = TcpStream::connect("127.0.0.1:5656")?;
    send_file(temp_file_send.path().to_str().expect("Чета"), client)?;
    println!("[КЛИЕНТ] отправил: {:?}", temp_file_send);
    server.join().unwrap();

    Ok(())
}
