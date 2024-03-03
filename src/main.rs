use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
use std::fs;//文件系统
use std::io::prelude::*;


fn main() {
    //bind会监听传进去的地址
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming()//incoming方法产生流序列迭代，也就是Tcp流
    {
     let stream = stream.unwrap();
        handle_connection(stream);
        println!("connection extablished!");
    }
}
fn handle_connection(mut stream:TcpStream)
{
    let mut buffer = [0;1024];//1024个字节的缓冲区,开大点显示html

    stream.read(&mut buffer).unwrap();//从TcpStream中读取数据放入buffer

    //请求
    //Method Request-URI HTTP-Version CRLF
    //headers CRLF
    //message-body

    //响应
    //HTTP-Version Status-Code Reason-Phrase CRLF
    //headers CRLF
    //message-body

    let get = b"GET / HTTP/1.1\r\n";//原始字节字符串
    if buffer.starts_with(get){
        let contents = fs::read_to_string("hello.html").unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}",contents);

          stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();//等待并阻塞程序运行，直到所有字节流写入

    }
    else {
        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}",contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();//等待并阻塞程序运行，直到所有字节流写入

    }
}
