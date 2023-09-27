// 监听 TCP 需要用到
use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;
use std::time::Duration;
use std::thread;
use hello_web_20::ThreadPool;

fn main() {
    /*
        bind 函数会监听传入的地址，这里是 127.0.0.1:7878，返回值是 Result<T, E>

     */
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);


    /*
        incoming 产生一个 TCP Stream 的流序列的迭代器，
        而单个流就表示客户端和服务器之间打开了一个连接
        而 for 循环就会依次处理每一个连接，并生成一系列的流进行处理

        take(2) 只能处理 2 次请求，第三次就停止了
     */
    
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });

        /*
            为每一个连接都创建一个新的线程
            但这样来一个连接就创建一个线程，容易被 DDoS 攻击
         */
        // thread::spawn(|| {
        //     handle_connection(stream);
        // });
    }

    println!("Shutting down!");
}
/*
    参数为可变的
    因为 TcpStream 实例内部记录了返回给我们的数据
    它可能会返回多于我们请求的数据，并将这些数据保存下来
    以备下次请求使用，因为 TcpStream 的内部状态可能会改变，所以标记为 mut

 */
fn handle_connection(mut stream: TcpStream) {
    // 缓存 512 字节
    let mut buffer = [0; 512];
    /*
        read 方法会从 TcpStream 读取数据
        并把放到 buffer 中

     */
    stream.read(&mut buffer).unwrap();

    // 请求(CRLF 是换行回车符号)
    // Method Request-URI HTTP-Version CRLF
    // headers CRLF
    // message-body

    // 响应
    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body

    //let response = "HTTP/1.1 200 OK\r\n\r\n";

    /*
        b"" 是字节字符串的语法
        将 get 里的文本转化为字节字符串，后边就可以进行比较了

     */
    let get = b"GET / HTTP/1.1\r\n";

    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // 看看 buffer 是否以 GET 开头
    // 这里返回一个元组
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        // 如果以 http://127.0.0.1/sleep 开头就休眠 5s
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();


    // 将 response 写回去
    //stream.write(response.as_bytes()).unwrap();
    // flush 会等待并阻止程序的运行, 直到所有的字节都被写入到连接中
    //stream.flush().unwrap();
    
    // 然后把 buffer 中的字节转为字符串打印出来
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
