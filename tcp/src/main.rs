use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // 断开连接的命令
    let quit_string = "quit".to_string();

    // 用一个死循环处理客户端发来的消息
    loop {
        // 准备一个消息缓存，用来缓存客户端发来的消息
        let mut buffer = [0; 512];
        // 读取客户端消息(模式匹配)
        match stream.read(&mut buffer) {
            // 当读取到客户端消息后
            Ok(_) => {
                // 客户端发来的消息转成string
                let msg = String::from_utf8_lossy(&buffer[..]).to_string();
                // 处理客户端消息

                if msg.contains(&quit_string) {
                    break;
                }
                // 打印客户端消息
                println!("Request: {}", msg);

                // 准备响应
                let response = "Response: ".to_string() + &msg;
                // 把客户端的消息再发回客户端
                stream.write(response.as_bytes()).unwrap();
                // 刷新流
                stream.flush().unwrap();
            }
            // 当读取出错，打印错误信息
            Err(e) => println!("read err {}", e),
        }
    }
}

fn main() {
    // 监听服务器的6666端口
    match TcpListener::bind("127.0.0.1:6666") {
        Ok(listener) => {
            // 当监听的端口收到连接后
            for stream in listener.incoming() {
                //这里也可以用?来简化match
                match stream {
                    Ok(stream) => {
                        handle_client(stream);
                    }
                    // 当连接出错，打印错误信息
                    Err(e) => println!("incoming err {}", e),
                }
                // 用一个函数处理客户端的连接
            }
        }
        // 当监听出错，打印错误信息
        Err(e) => println!("listen err {}", e),
    }
}
