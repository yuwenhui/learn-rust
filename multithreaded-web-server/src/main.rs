use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").expect("could not connect to the server");

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        // TODO 构造线程池，限制启动的线程数量
        thread::spawn(move || {
          handle_connection(stream);
        });
      },
      Err(_) => {
        println!("connection error");
      }
    }
  }

  drop(listener);
}

fn handle_connection(mut stream: TcpStream) {
  // 创建一个512字节的缓冲区，用来存放读取到的数据
  let mut buffer = [0; 512];

  // 从stream中读取字节并放入缓冲区中，并处理异常
  match stream.read(&mut buffer) {
    Ok(size) => {
      stream.write(&buffer[0..size]).unwrap();
    },
    Err(_) => {
      println!("an error occurred");
    }
  }

  // 将缓冲区中的字节转换为字符串，并打印出来
  let request = String::from_utf8_lossy(&buffer[..]);
  println!("request: {}", request);

  let response = "HTTP/1.1 200 OK\r\n\r\n";

  // 将响应结果写入stream中
  stream.write(response.as_bytes()).expect("unexpect stream write");
  // flush会等待并阻塞程序执行，直到所有字节都被写入连接中
  stream.flush().expect("unexpect flush stream");
}
