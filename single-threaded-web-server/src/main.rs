use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;


fn main() {
  // 初始化一个TcpListener实例
  let listener = TcpListener::bind("127.0.0.1:9000").expect("could't connect to the server");

  // TcpListener的incoming方法返回一个迭代器，是一系列TcpStream类型的流
  for stream in listener.incoming() {
    // 通过match模式匹配处理错误
    match stream {
      Ok(stream) => {
        // 调用handle_connection处理接收到的流
        handle_connection(stream);
        println!("connection established");
      },
      Err(_) => {
        println!("connection failed");
      }
    };
  }
}

fn handle_connection(mut stream: TcpStream) {
  // 创建一个512字节的缓冲区，用来存放读取到的数据
  let mut buffer = [0; 512];

  // 从stream中读取字节并放入缓冲区中
  stream.read(&mut buffer).expect("unexpect stream read");

  // 将缓冲区中的字节转换为字符串，并打印出来
  let request = String::from_utf8_lossy(&buffer[..]);
  println!("request: {}", request);

  let response = "HTTP/1.1 200 OK\r\n\r\n";

  // 将响应结果写入stream中
  stream.write(response.as_bytes()).expect("unexpect stream write");
  // flush会等待并阻塞程序执行，直到所有字节都被写入连接中
  stream.flush().expect("unexpect flush stream");
}
