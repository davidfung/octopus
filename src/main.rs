use std::io::{self, Write};

const MENU: &str = "
MENU
1. Double Through Channel
2. Spawning Async Tasks
> ";

fn main() {
    type Task = fn();
    let id = menu();
    let g: Task;
    match id {
        1 => g = double_thru_channel,
        2 => g = spawn_async_tasks,
        _ => g = bye,
    }
    g();
}

fn menu() -> i32 {
    print!("{MENU}");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Input Error");
    println!("");
    match s.trim().parse::<i32>() {
        Err(e) => {
            // println!("{}", e);
            return 0;
        }
        Ok(choice) => return choice,
    }
}

fn bye() {
    println!("Bye");
}

fn double_thru_channel() {
    use std::sync::mpsc::{self, Receiver, Sender};
    use std::thread;
    use std::time;

    println!("Double a number through channel");
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    thread::spawn(move || {
        let i = rx.recv().unwrap();
        println!("{} double is {}", i, i * 2);
    });

    tx.send(2).unwrap();
    thread::sleep(time::Duration::from_millis(5));
}

use async_std::io::prelude::*;
use async_std::net;

fn spawn_async_tasks() {
//    async_main(); 
    let requests = vec![
        ("example.com".to_string(), 80, "/".to_string()),
        ("www.red-bean.com".to_string(), 80, "/".to_string()),
        ("en.wikipedia.org".to_string(), 80, "/".to_string()),
    ];

    let results = async_std::task::block_on(many_requests(requests));
    for result in results {
        match result {
            Ok(response) => println!("{}", response),
            Err(err) => eprintln!("error: {}", err),
        }
    }
}

// fn async_main() -> std::io::Result<()> {
//     use async_std::task;
//     Ok(())
// }

pub async fn many_requests(requests: Vec<(String, u16, String)>) -> Vec<std::io::Result<String>> {
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
    let mut socket = net::TcpStream::connect((host, port)).await?;

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);

    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response).await?;

    Ok(response)
}

