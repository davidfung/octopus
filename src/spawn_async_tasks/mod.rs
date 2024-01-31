use async_std::io::prelude::*;
use async_std::net;

use crate::menu::MenuItem;

pub fn menu() -> MenuItem {
    MenuItem{task: entry, desc: "Spawn Async Tasks"}
}

pub fn entry() {
    let requests = vec![
        ("example.com".to_string(), 80, "/".to_string()),
        ("www.red-bean.com".to_string(), 80, "/".to_string()),
        ("en.wikipedia.org".to_string(), 80, "/".to_string()),
    ];

    let results = async_std::task::block_on(many_requests(requests));
    for result in results {
        match result {
            Ok(response) => println!("{}", &response[..20]),
            Err(err) => eprintln!("error: {}", err),
        }
    }
}

async fn many_requests(requests: Vec<(String, u16, String)>) -> Vec<std::io::Result<String>> {
    use async_std::task;

    let mut handles = vec![];
    for (host, port, path) in requests {
        handles.push(task::spawn_local(async move {cheapo_request(&host, port, &path).await}));
    }
    
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }

    results
}

async fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    eprintln!("cheapo_request starts...");
    let mut socket = net::TcpStream::connect((host, port)).await?;
    eprintln!("{} connected", host);

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);

    socket.write_all(request.as_bytes()).await?;
    eprintln!("{} write_all", host);
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response).await?;
    eprintln!("{} read_to_string", host);

    Ok(response)
}