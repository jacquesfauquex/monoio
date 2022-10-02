//! rpc_50002_client
const ADDRESS: &str = "127.0.0.1:50002";

use monoio::{
    io::{AsyncReadRent, AsyncWriteRentExt},
    net::TcpStream,
    Buildable, Driver,
};

//diferent driver, depending on the platform
#[cfg(target_os = "linux")]
fn main() {
    println!("Will run with IoUringDriver");
    run::<monoio::IoUringDriver>();
}
#[cfg(not(target_os = "linux"))]
fn main() {
    println!("Will run with LegacyDriver");
    //you must enable legacy feature
    run::<monoio::LegacyDriver>();
}

fn run<D>()
where
    D: Buildable + Driver,
{
    let client_thread = std::thread::spawn(|| {
        monoio::start::<D, _>(async move {

            //connect
            let call = TcpStream::connect(ADDRESS)
                .await;
            match call {
                Ok(mut tcp_stream) => {
                      
                    //write
                    let write_buf: Vec<u8> = vec![97; 11];
                    let (bytes_written, _) = tcp_stream.write_all(write_buf).await;// _ = buffer passed
                    println!("> {} Bytes", bytes_written.unwrap());

                    //read
                    let read_buf = Vec::with_capacity(1024);
                    let (res, read_buf) = tcp_stream.read(read_buf).await;
                    
                    let bytes_read=res.unwrap();
                    println!("< {:?} Bytes", bytes_read);

                    println!("{:?}", read_buf);

                }
                Err(e) => {
                    println!("failed: {}", e);
                    return;
                }
            }
        });
    });
    client_thread.join().unwrap();
}
