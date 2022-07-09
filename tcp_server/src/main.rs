use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;
use std::time;

//处理客户端连接函数
fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    //用来保存客户端数据流
    let mut buf = [0; 128];
    //循环读取
    loop {
        // 读取客户端数据流
        let len = stream.read(&mut buf)?;
        //如果读到的长度为0则返回
        if len == 0 {
            return Ok(());
        }
        //把客户端的数据再写回去
        stream.write(&buf[..len])?;
        //打印客户端的数据
        println!("read from client:{}", str::from_utf8(&buf).unwrap());
        //让线程睡眠1秒
        thread::sleep(time::Duration::from_secs(1));
    }
}

fn main() -> io::Result<()> {
    //定义tcp的linser,监听本地8080端口,同时捕获异常
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    //创建一个容器，用来保存线程句柄
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    // 对每一个连接开启一个线程进行处理
    for stream in listener.incoming() {
        //转换stream流
        let stream = stream.expect("failed");
        let handle = thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error))
        });
        //对每一个流都创建一个线程来处理
        thread_vec.push(handle);
    }
    //此循环为了等待线程处理结束
    for handle in thread_vec {
        handle.join().unwrap();
    }
    Ok(())
}
