use std::thread;

// JoinHandle 来源参考 thread 的 spawn 函数
pub struct ThreadPool {
    // 这里与 spawn 函数不同，这里不需要返回值，所以写 ()
    //threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
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
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            /*
                这里我们系统线程创建后处于等待状态，然后有代码传给他们时再执行
                所以我们使用一个叫 Worker 的数据结构
                Worker 是线程池里常用的术语
                然后让 Worker 来管理上述行为
             */
            workers.push(Worker::new(id));

        }
        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static, {

    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        // 传入 id 创建一个线程
        let thread = thread::spawn(|| {
        });
        Worker { id, thread }
    }
}
