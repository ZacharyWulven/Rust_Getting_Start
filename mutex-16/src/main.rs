use std::sync::{ Mutex, Arc };
use std::thread;
fn main() {
    //test_mu();
    //mutil_mu();

    mutil_mu_arc();
}

fn test_mu() {
    /*
        创建 Mutex，这个 5 就是要保护的数据
     */
    let m = Mutex::new(5);

    {   // 内部作用域
        /*
            通过 lock 获取锁，这个方法会阻塞当前线程
            直到获取到锁，它有可能发生错误，这里用 unwrap 进行处理
            返回 MutexGuard，它实现了 Deref 所以可以指向它内部
            的数据，从而我们可以获得其内部数据的一个引用


         */
        let mut num = m.lock().unwrap();

        // 这个可以修改其值是因为 MutexGuard，它实现了 Deref trait
        *num = 6;
        
    } // MutexGuard，它实现了 Drop trait，所以这里会释放，也就解锁了
    println!("m = {:?}", m);
}
/*
fn mutil_mu() {
    let counter = Mutex::new(0);

    let mut handles = vec![];

    for _ in 0..10 {
        // 这里 move 报错，因为第一次开线程已经获取了 counter 的所有权
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle); 
    } 

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
*/

fn mutil_mu_arc() {
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        // 这里 move 报错，因为第一次开线程已经获取了 counter 的所有权
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle); 
    } 

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}