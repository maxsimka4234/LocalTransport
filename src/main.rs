use std::{io::{self}, net::{TcpStream}, sync::mpsc, thread};

use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use crate::stream::{real_addr, has_path, create_stream, getting_file, send_file};
mod stream;

fn hello() -> Result<(), Box<dyn std::error::Error>> {
    println!("Здраствуй! 1 - принимает файл (путь к итогу), 2 - отправляет файл (путь к файлу) ");
   // let mut addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5656);
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не строка");
    let input = input.trim().parse::<u8>().expect("Не число");

    let service_type = "_localTransfer._tcp.local.";
    let instance_name = "my-localTransfer-instance";
    let host_name = "my-localTransfer-host.local.";
    let local_ip = real_addr()?;
    //local_ip_address::local_ip()?; 

    match input {
        1 => {
        println!("Понял, я принимаю.\n Закончим настройку");
        println!("Укажите путь принятия файла: ");

        let mut path =String::new();
        io::stdin().read_line(&mut path).expect("Не строка");
        if !has_path(&path){println!("Не путь!"); return Err(Box::new(std::io::Error::new(io::ErrorKind::IsADirectory, "Не путь!")));}
        let path = path.trim();
        let (tx, rx) = mpsc::channel();

        let tcp_handle = thread::spawn(move || {
            println!("открыл тсп1 ");
            let listener = create_stream().unwrap();
            
            println!("открыл принятие {:?}", &listener);
            
            tx.send(listener).expect("send failed");
        });


        println!("запускаю ");
        let stream = rx.iter().next().ok_or("ерр")?;
        println!("запускаю2 ");
        let local_addr = stream.local_addr()?;
        


        
        let mdns = ServiceDaemon::new()?; 
        let servise_info = ServiceInfo::new(service_type, instance_name, host_name, local_ip, local_addr.port(), None)?;
        mdns.register(servise_info)?;
        
        
        
        let mut stream = stream.accept()?;

        println!("Запустил поток! {:?}", &stream.0);

        getting_file(&mut stream.0, &path)?;

        let _ = tcp_handle.join().map_err(|e| println!("не удалось обработать поток: {:?}", e));

        }
        _ => {
            //println!("Понял, принимаю {}", addr);
        println!("Понял, я отправляю.\n Закончим настройку");
        println!("Укажите путь отправки файла: ");

        let mut path =String::new();
        io::stdin().read_line(&mut path).expect("Не строка");
        if !has_path(&path){println!("Не путь!"); return Err(Box::new(std::io::Error::new(io::ErrorKind::IsADirectory, "Не путь!")));}

       // if !fs::metadata(&path).is_ok() {println!("Файл не найден ");  return Err(Box::new(std::io::Error::new(io::ErrorKind::InvalidFilename, "Файл не найден!")));};
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
    }
    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>>{
   hello()?;
   Ok(())
}

#[cfg(test)]

#[test]
fn server_test()-> std::io::Result<()>  {
    use std::net::TcpListener;
    use std::thread;
    use std::fs;
    use io::Write;

    let temp_file_get = tempfile::NamedTempFile::new()?;
    let mut temp_file_send = tempfile::NamedTempFile::new()?;

    writeln!(&mut temp_file_send, "Возможно тут большой очень файл .exeооооооооооооооооо")?;
    let tfg2 = fs::read(&temp_file_send)?;

    let stream = TcpListener::bind("127.0.0.1:5656")?;

    let server = thread::spawn(move || {
        use std::net::TcpListener;

        let mut stream = TcpListener::accept(&stream).expect("мсг");
        let pathi= &temp_file_get.path().to_str().expect("ее").to_string();
        //temp_file_get.close();
        getting_file(&mut stream.0, pathi).unwrap();


        let tfg1 = fs::read(pathi).expect("иеше");
        println!("Получено! {:?} {:?}", String::from_utf8(tfg1.clone()), stream.0);
        assert_eq!(tfg1, tfg2);
    });

    thread::sleep(std::time::Duration::from_millis(50));
    let client = TcpStream::connect("127.0.0.1:5656")?;
    send_file(temp_file_send.path().to_str().expect("Чета"), client)?;
    println!("[КЛИЕНТ] отправил: {:?}", temp_file_send);
    server.join().unwrap();

    Ok(())
}
