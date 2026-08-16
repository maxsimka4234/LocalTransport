use std::io::{self};
use std::path::PathBuf;

use crate::streamer::{has_path, receiver, send};
use crate::sender::{get_all};
mod streamer; mod sender;

fn hello() -> Result<(), Box<dyn std::error::Error>> {
    println!("Здравствуй! 1 - принимает файл (путь к итогу), 2 - отправляет файл (путь к файлу) ");
   // let mut addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5656);
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не строка");
    let input = input.trim().parse::<u8>().expect("Не число");

    let service_type = "_localTransfer._tcp.local.";
   
    //local_ip_address::local_ip()?; 

    match input {
        1 => {
        println!("Понял, я принимаю.\n Закончим настройку");
        println!("Укажите путь принятия файла: ");

        let mut path =String::new();
        io::stdin().read_line(&mut path).expect("Не строка");
        if !has_path(&path){println!("Не путь!"); return Err(Box::new(std::io::Error::new(io::ErrorKind::IsADirectory, "Не путь!")));}
        let path = path.trim();
        
        let mut stream = receiver(service_type)?;

        println!("Запустил поток! {:?}", &stream.0);
        let path = PathBuf::from(path);

        get_all(&path, &mut stream.0)?;
        
        }

        _ => {
            //println!("Понял, принимаю {}", addr);
        println!("Понял, я отправляю.\n Закончим настройку");
        println!("Укажите путь отправки файла: ");

        let mut path =String::new();
        io::stdin().read_line(&mut path).expect("Не строка");
        if !has_path(&path){println!("Не путь!"); return Err(Box::new(std::io::Error::new(io::ErrorKind::IsADirectory, "Не путь!")));}

       
        let path = path.trim();
            
        send(service_type, path)?;
    
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

use crate::sender::send_file;

    let temp_file_get = tempfile::NamedTempFile::new()?;
    let mut temp_file_send = tempfile::NamedTempFile::new()?;

    writeln!(&mut temp_file_send, "Возможно тут большой очень файл .exeооооооооооооооооо")?;
    let tfg2 = fs::read(&temp_file_send)?;

    let stream = TcpListener::bind("127.0.0.1:5656")?;

    let server = thread::spawn(move || {
        use std::net::TcpListener;

use crate::sender::getting_file;

        let mut stream = TcpListener::accept(&stream).expect("мсг");
        let pathi= &temp_file_get.path().to_str().expect("ее").to_string();
        //temp_file_get.close();
        getting_file(&mut stream.0, pathi).unwrap();


        let tfg1 = fs::read(pathi).expect("иеше");
        println!("Получено! {:?} {:?}", String::from_utf8(tfg1.clone()), stream.0);
        assert_eq!(tfg1, tfg2);
    });

    thread::sleep(std::time::Duration::from_millis(50));
    let mut client = std::net::TcpStream::connect("127.0.0.1:5656")?;
    send_file(temp_file_send.path().to_str().expect("Чета"), &mut client)?;
    println!("[КЛИЕНТ] отправил: {:?}", temp_file_send);
    server.join().unwrap();

    Ok(())
}
