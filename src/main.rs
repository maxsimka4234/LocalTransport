use std::io::{self};
use std::path::PathBuf;

use crate::streamer::{has_path, receiver, send};
use crate::sender::{get_all};
mod streamer; mod sender;

fn hello() -> Result<(), Box<dyn std::error::Error>> {
    println!("Здравствуй! 1 - принимает файл (путь к итогу), 2 - отправляет файл (путь к файлу) ");

    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не строка");
    let input = input.trim().parse::<u8>().expect("Не число");

    let service_type = "_localTransfer._tcp.local.";
 

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
#[ignore = "Нужно изменить"]
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
    let send = temp_file_send.path().to_str().expect("Чета"); 
    send_file(send, send, &mut client)?;
    println!("[КЛИЕНТ] отправил: {:?}", temp_file_send);
    server.join().unwrap();

    Ok(())
}

#[test]
fn sender_test() -> Result<(), Box<dyn std::error::Error>>  {
    use std::process::exit;
    use std::fs;
    use std::io::{self, Write};
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::thread;

    fn to_send(e: Box<dyn std::error::Error>) -> Box<dyn std::error::Error + Send + Sync> {
        Box::new(std::io::Error::new(io::ErrorKind::Other, e.to_string()))
    }
    

    let service_type = "_localTrafnsfer._tcp.local.";

    let temp_file_get = tempfile::NamedTempFile::new()?;

    let mut temp_file_send = tempfile::NamedTempFile::new()?;
    temp_file_send.write_all(b"Hello world!!!!!!!!!!!!")?;
    let tfg2 = fs::read(&temp_file_send.path())?;
    let temp_file_send = Arc::new(temp_file_send);

    let tfg_clone = Arc::clone(&temp_file_send);
   

    let _sender = thread::spawn(move || -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
         println!("Отправляю");
        send(service_type, &tfg_clone.path().to_string_lossy()).map_err(to_send)?;
        println!("Отправил");
        Ok(())
        //чтобы отправлял в другом потоке а не принимал
    }); 

    let mut stream = receiver(service_type)?; // .accept
    println!("Запустил поток! {:?}", &stream.0);

    let path = PathBuf::from(&temp_file_get.path());
    temp_file_get.close()?;
    println!("get_all запуск! {:?}", path);
    get_all(&path, &mut stream.0)?;
    println!("get_all great! {:?}", path);
    let file_name = temp_file_send.path().file_name().expect("Файл не найден").to_string_lossy();
    let path = format!("{}/{}", path.to_string_lossy(), file_name);
   
    let tfg1 = match fs::read(&path) {
        Ok(data) => Ok(data),
        Err(e) => {
            eprintln!("kind: {:?}, raw_os_error: {:?}", e.kind(), e.raw_os_error());
           // eprintln!("is_dir: {:?}", path.is_dir());
            eprintln!("metadata: {:?}", fs::metadata(&path));
            Err(e)
        }
    };
    let tfg1 = tfg1?;
    
    println!("бЕБ");
    
    assert_eq!(tfg1, tfg2);
    exit(0);
    
}
