use std::process;

#[allow(unused_imports)]
use tokio::io::AsyncWriteExt;
use tokio::{net::{TcpListener, TcpStream}, io::{BufReader, AsyncReadExt}};

#[tokio::main]
async fn main() {
    // Create "NewStyle" NBD Socket
    let listener = TcpListener::bind("127.0.0.1:10809").await.unwrap_or_else(|err| {
        eprintln!("got error while bind socket: {err}");
        process::exit(1)
    });

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("accept connection from {addr}");
                tokio::spawn(async move {
                    process(socket).await
                });
            },
            Err(err) => {
                eprintln!("got error while accept: {err}")
            }
        }
    }
}

/// Handshake flag for Newstyle
static NBD_FLAG_FIXED_NEWSTYLE: i16 = 1;
#[allow(dead_code)]
/// Handshake Flag from Client
/// Accepts "NewStyle" Connection
static NBD_FLAG_FIXED_NEWSTYLE_CLIENT: i32 = 1;

async fn process(mut socket: TcpStream) {
    let (r, mut w) = socket.split();
    let mut r = BufReader::new(r);
    if let Err(err) = w.write_all("NBDMAGICIHAVEOPT".as_bytes()).await {
        eprintln!("got error while write: {err}");
        return
    }
    if let Err(err) = w.write_all(NBD_FLAG_FIXED_NEWSTYLE.to_ne_bytes().as_slice()).await {
        eprintln!("got error while write: {err}");
        return
    }
    let client_flags = match r.read_i32().await {
        Ok(val) => val,
        Err(err) => {
            eprintln!("got error while read: {err}");
            return
        }
    };
    eprintln!("got: {client_flags}");
    return
}
