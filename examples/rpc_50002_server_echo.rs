//! rpc_50002_server_echo.rs
//!
//! rpc prototype
//! one input will be echoed out and the tcp stream closed inmediately after.
//! 
//! testing: 
//! - Run the example and `nc 127.0.0.1 50002` in another shell.
//! - execute rpc_50002_client.rs

const ENDPOINT:&str="127.0.0.1:50002";
const BUFFER_CAPACITY:usize =1024;//echoes no more than 1024 bytes in one write operation

use monoio::{
    io::{AsyncReadRent, AsyncWriteRentExt, AsyncWriteRent},
    net::{TcpListener, TcpStream},
};


#
[monoio::main(driver = "fusion")]
async fn main() {
    
    // tracing_subscriber::fmt().with_max_level(tracing::Level::TRACE).init();
    let listener = TcpListener::bind(ENDPOINT).unwrap();
    println!("listening");
    loop {
        let incoming = listener.accept().await;
        match incoming {
            Ok((stream, addr)) => {
                println!("accepted a connection from {}", addr);
                monoio::spawn(echo(stream));
            }
            Err(e) => {
                println!("accepted connection failed: {}", e);
                return;
            }
        }
    }
}

//handler which can be customized for other purposes
async fn echo(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf: Vec<u8> = Vec::with_capacity(BUFFER_CAPACITY);
    loop {

        // read
        let (res, _buf) = stream.read(buf).await;
        buf = _buf;
        let res: usize = res?;
        if res == 0 {
            return Ok(());
        }

        // write one buffer contents
        let (res, _buf) = stream.write_all(buf).await;
        buf = _buf;
        res?;
        buf.clear();

        //write 0 bytes (END OF STREAM)
        let _ = stream.shutdown();
    }
}
