use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn test_tcp_2046_local_integration_lifecycle() {
    let bind_addr = "127.0.0.1:12046"; // Menggunakan port uji loopback lokal untuk TCP 2046
    let listener = TcpListener::bind(bind_addr).expect("Failed to bind TCP test server");
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");

    let server_running = Arc::new(AtomicBool::new(true));
    let server_flag = server_running.clone();

    // Server worker thread
    let server_handle = thread::spawn(move || {
        let mut client_count = 0;
        while server_flag.load(Ordering::SeqCst) {
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    client_count += 1;
                    stream
                        .set_read_timeout(Some(Duration::from_millis(500)))
                        .unwrap();
                    let mut buf = [0u8; 128];
                    if let Ok(n) = stream.read(&mut buf) {
                        if n > 0 {
                            // Echo response
                            let _ = stream.write_all(&buf[..n]);
                        }
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(e) => {
                    eprintln!("Accept error: {}", e);
                    break;
                }
            }
        }
        client_count
    });

    // 1. Skenario: Client connect normal
    let mut client = TcpStream::connect(bind_addr).expect("Client 1 must connect");
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();

    // 2. Skenario: Pertukaran data dua arah (Echo test)
    let test_cmd = b"\x10\x01\x00\x00PING_TEST";
    client
        .write_all(test_cmd)
        .expect("Write command must succeed");

    let mut response_buf = [0u8; 64];
    let bytes_read = client
        .read(&mut response_buf)
        .expect("Read response must succeed");
    assert_eq!(&response_buf[..bytes_read], test_cmd);

    // 3. Skenario: Client graceful disconnect
    drop(client);

    // 4. Skenario: Client reconnect
    let mut client2 = TcpStream::connect(bind_addr).expect("Client 2 must reconnect");
    let test_cmd2 = b"HELLO_RECONNECT";
    client2.write_all(test_cmd2).expect("Write 2 must succeed");
    let bytes_read2 = client2
        .read(&mut response_buf)
        .expect("Read 2 must succeed");
    assert_eq!(&response_buf[..bytes_read2], test_cmd2);
    drop(client2);

    // Shutdown server
    server_running.store(false, Ordering::SeqCst);
    let total_clients = server_handle
        .join()
        .expect("Server thread must terminate cleanly");
    assert_eq!(total_clients, 2);
}
