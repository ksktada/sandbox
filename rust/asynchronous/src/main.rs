// https://github.com/ProgrammingRust/examples/blob/master/cheapo-request/src/main.rs

use async_std::io::prelude::*;
use async_std::net;
use asynchronous::{Executor, Hello};

async fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    let mut socket = net::TcpStream::connect((host, port)).await?;

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response).await?;

    Ok(response)
}

fn main() {
    // use async_std::task;

    // let response = task::block_on(cheapo_request("example.com", 80, "/"))?;
    // println!("{}", response);

    // Asynchronous Task Execution
    let executor = Executor::new();
    executor.get_spawner().spawn(Hello::new());
    executor.run();

    // Ok(())
}