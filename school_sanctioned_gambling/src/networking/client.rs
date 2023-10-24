use std::net::TcpStream;
use std::str;
use std::io::{self, BufRead, BufReader, Write};

/*pub struct Client {
    pub ip: u32,
    pub stream: TcpStream,
}*/

//use bevy::reflect::serde::*;
pub fn client_init(/*mut in_client: Client*/)
{

}

pub fn client_tick(server_ip_address: &String)
{
    //192.168.1.171
    let ip = format!("{}:8888", server_ip_address);
    let mut stream = TcpStream::connect(ip).expect("Failed trying to connect to server");

    loop
    {

        // Tutorial code used to send and then recieve packs of chars to and from the server
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();

        io::stdin().read_line(&mut input).expect("Failed to read from stdin");
        stream.write(input.as_bytes()).expect("Failed to write to server");

        let mut reader = BufReader::new(&stream);

        reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
        
        print!("{}", str::from_utf8(&buffer).expect("Could not write buffer as string"));
    }
}
