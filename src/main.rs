use std::net::{TcpStream};
use std::str;
use std::io::{self ,BufReader, BufRead, Write};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("No se pudo conectar");

    loop{
        let mut message = String::new();
        let mut buf : Vec<u8> = Vec::new();
        io::stdin().read_line(&mut message).expect("No se pudo leer el mesaje");
        stream.write(message.as_bytes()).expect("No se pudo enviar el mensaje");

        let mut reader = BufReader::new(&stream);
        reader.read_until(b'\n', &mut buf).expect("No se pudo leer la respuesta");
        println!("{}", str::from_utf8(&buf).expect("No se pudieron castear los bytes"));
    
    }
    
}
