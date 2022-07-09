use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() -> io::Result<()> {
    //创建一个stream流，连接tcp server
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    //循环读取用户输入
    loop {
        //定义一个string类型的变量
        let mut input = String::new();
        //从标准输入读取一行，放到input里面
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");
        //把input的内容转换成byte后写到stream流
        stream
            .write(input.as_bytes())
            .expect("failed to write input");
        //从stream流创建一个读
        let mut reader = BufReader::new(&stream);
        //创建一个buffer来保存server返回的数据
        let mut buffer: Vec<u8> = Vec::new();
        //读取server的数据，读到换行为止，将数据保存到buffer
        reader.read_until(b'\n', &mut buffer)?;
        //打印buffer中的数据
        println!("read from server:{}", str::from_utf8(&buffer).unwrap());
    }
}
