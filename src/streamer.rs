use std::{ collections::HashSet, io::{self}, net::{SocketAddr, TcpListener, TcpStream}, path::{Path, PathBuf}};
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};

use crate::sender::{itfile, send_all, send_file};


pub fn real_addr() -> std::io::Result<std::net::IpAddr>{
    let interfaces = if_addrs::get_if_addrs()?;
    let mut is_local = HashSet::new();
    for interface in interfaces {
        match interface.ip() {
            std::net::IpAddr::V4(ipv4) => {
                if (ipv4.octets()[0] == 192 && ipv4.octets()[1] == 168)  
                || ipv4.octets()[0] == 10 
                || (ipv4.octets()[0] == 172 && (16..=31).contains(&ipv4.octets()[1])){
                    is_local.insert(interface.ip());
                }
            }
            std::net::IpAddr::V6(_) => {/*is_local.insert(interface.ip());*/}
        }
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

pub fn receiver(service_type: &str) -> Result<(TcpStream, SocketAddr), Box<dyn std::error::Error>> {
    let instance_name = "my-localTransfer-instance";
    let host_name = "my-localTransfer-host.local.";
    let local_ip = real_addr()?;

    let listener = create_stream().unwrap();      

    println!("Запуск TCP ");
    
    let stream = listener;
    let local_addr = stream.local_addr()?;

    println!("TCP Запущен ");

    let mdns = ServiceDaemon::new()?; 
    let servise_info = ServiceInfo::new(service_type, instance_name, host_name, local_ip, local_addr.port(), None)?;
    mdns.register(servise_info)?;

    let stream = stream.accept()?;
    Ok(stream)
}

pub fn send(service_type: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {

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

                    
                    let mut stream = TcpStream::connect(addr)?;
                    
                    let path = PathBuf::from(path);
                    if path.is_dir() {
                        println!("это директория");
                        itfile(&mut stream, false)?;
                        send_all(&path, &mut stream)?;
                    } else {
                        println!("это файл");
                        itfile(&mut stream, true)?;
                        send_file(&path.to_string_lossy(), &mut stream)?; 
                    }
                    
                }
                ServiceEvent::ServiceRemoved(serv_type, name ) => {
                    println!("Хост {} ({}) удален", name, serv_type)
                }
                _ => {}
            }
        }
        Ok(())
}


