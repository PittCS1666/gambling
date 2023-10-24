use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, Error, ErrorKind};
use local_ip_address::local_ip;
use std::thread;

/*pub struct Server {
    pub ip: u32,
    pub listener: TcpListener,
}*/

fn handle_client(mut stream: TcpStream) -> Result<(), Error>
{
    //
    println!("Client is connecting! Client address: {}", stream.peer_addr()?);

    // Simple code from tutorial used to send buffers of chars back and forth
    let mut buffer = [0; 512];
    loop
    {
        let bytes_read = stream.read(&mut buffer).unwrap();
        if bytes_read == 0 { return Ok(()) }
        
        stream.write(&buffer[..bytes_read])?;
    }

}

pub fn server_init(/*mut in_server: Server*/)
{
    
}

pub fn server_tick(/*mut in_server: Server*/)
{
    // 0.0.0.0. means listens to all open ports
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");
    
    // Output local IP address so the client can connect using it
    let my_local_ip = local_ip().unwrap();
    println!("You created a server with IP {:?}", my_local_ip);

    for stream in listener.incoming()
    {
        match stream
        {
            // Error catching
            Err(e) if e.kind() == ErrorKind::WouldBlock =>
            {
                continue;
            }

            Err(e) =>
            {
                panic!("encountered IO error: {}", e);
            }

            // For each incoming connection (client), we create a thread to handle any interaction with it
            // (possibly not necesarry for each client to have its own, maybe just one that handles all of them?)
            Ok(stream) =>
            {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
