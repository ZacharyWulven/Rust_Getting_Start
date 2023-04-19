use std::thread;
use std::time::Duration;
use std::sync::mpsc;


fn main() {
    join();

    moved();

    mpsc();
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

    thread::spawn(move || { // 加 move，获得 tx 的所有权
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    /*
        recv 会阻塞当前线程，直到有消息被传入
        有消息就返回 Ok，否则返回 Err
     */
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}