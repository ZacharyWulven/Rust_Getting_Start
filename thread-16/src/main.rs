use std::thread;
use std::time::Duration;
use std::sync::mpsc;


fn main() {

    //mpsc();

    //wait();

    clone_tx();


    //join();

    //moved();



}

fn join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            // 让当前线程 sleep 
            thread::sleep(Duration::from_millis(1));
        }
    }); 
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);  
        thread::sleep(Duration::from_millis(1));
    }
    // 主线程结束，上边的异步线程也结束了, 可以使用 joinHandle 解决

    /*
        join(）会阻塞当前线程的执行，直到 handle 对应的线程结束
        类似 swift semaphore 的 wait
     */
    handle.join().unwrap();
    println!("thread done");
}

fn moved() {
    let v = vec![1, 2, 3];
    /*
        未加 move 时报错，因为 v 的在线程内的生命周期可能长于外部
        强制异步线程闭包获得 v 的所有权可以加上 move 关键字
     */
    let handle = thread::spawn(move || {
       println!("Here's a vector: {:?}", v); 
    });

    handle.join().unwrap();
}

fn mpsc() {
    // tx 是 sender，rx 是 receiver
    let (tx, rx) = mpsc::channel();

    // 创建一个线程，并使用 move 让线程有 tx 的所有权
    // 新的线程必须拥有发送端的所有权才能往通道里发消息
    thread::spawn(move || { // 加 move，获得 tx 的所有权
        let val = String::from("hi");
        // 发送消息，如果接收到已经被丢弃，这时会产生一个错误，这里用 unwrap 简单处理
        tx.send(val).unwrap();
        // 这里报错因为 val 已经 move 了
        //println!("val is {}", val);
    });

    /*
        recv 会阻塞当前线程，直到有消息被传入
        有消息就返回 Ok，否则返回 Err
     */
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn wait() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    /*
        把接收端当成迭代器，这样就不需要调用 recv 函数了
        每收到一个值就将其打印
        当 Channel 关闭时就会退出这个循环
        这也是一种常用的用法
     */
    for received in rx {
        println!("Got: {}", received);
    }
}

fn clone_tx() {
    let (tx, rx) = mpsc::channel();

    // 这里通过 clone 创建一个新的发送端 tx1
    let tx1 = mpsc::Sender::clone(&tx);

    // 用 tx1 进行 send
    thread::spawn(move || {
        let vals = vec![
            String::from("1: hi"),
            String::from("1: from"),
            String::from("1: the"),
            String::from("1: thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // 用 tx 进行 send
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });
    
    for received in rx {
        println!("Got: {}", received);
    }
}