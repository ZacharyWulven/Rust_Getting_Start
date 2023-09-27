use std::thread;
use std::sync::{Arc, mpsc::{self, Receiver}, Mutex};

type Job = Box<dyn FnBox + Send + 'static>;


enum Message {
    NewJob(Job),
    Terminate,
}

// JoinHandle 来源参考 thread 的 spawn 函数
pub struct ThreadPool {
    // 这里与 spawn 函数不同，这里不需要返回值，所以写 ()
    //threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            /*
                这里我们系统线程创建后处于等待状态，然后有代码传给他们时再执行
                所以我们使用一个叫 Worker 的数据结构
                Worker 是线程池里常用的术语
                然后让 Worker 来管理上述行为
             */
            workers.push(Worker::new(id, Arc::clone(&receiver)));

        }
        // 线程池持有通道的发送端
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static, {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all works");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            // worker.thread.join() 这里不能直接这样
            // 因为 join 要获取所有权，所以将 Worker 的 thread 变为 Option<> 类型解决
            if let Some(thread) = worker.thread.take() {
                // 这里依然不会释放 因为 join 还在等待线程，并且获取接收的内容
                // 需要通过其他信号解决
                thread.join().unwrap();
            }
        }
    }
}


// 使得 self 可以在 Box 上调用
trait FnBox {
    fn call_box(self: Box<Self>);
}
// 所有实现 FnOnce 的类型都给他实现 FnBox 上调用

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)();
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // 传入 id 创建一个线程
        // let thread = thread::spawn(move || {
        //     // 这块的问题是：慢的请求还是会阻塞其他线程
        //     while let Ok(job) = receiver.lock().unwrap().recv() {
        //         println!("Worker {} got a job: executing.", id);

        //         // (*job)(); 无法直接调用，换成 job.call_box()
        //         job.call_box();
        //     }
        // });
        
        // 从上边 while let 换成 loop
        let thread = thread::spawn(move || loop {
            // 这块的问题是：慢的请求还是会阻塞其他线程
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job: executing.", id);
                    job.call_box();
                },
                Message::Terminate => {
                    break;
                },
            }
        });


        Worker { id, thread: Some(thread) }
    }
}
