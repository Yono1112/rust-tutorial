use std::thread;

pub struct ThreadPool{
  workers: Vec<Worker>,
}

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0); // sizeが0以下の場合はpanicを起こす

    let mut workers = Vec::with_capacity(size); // size分の要素を持つVecを作成

    for id in 0..size {
      // thread::spawn(|| {})で新しいスレッドを生成し、そのスレッドが実行するクロージャを引数に取る
      // このクロージャは何もしない
      workers.push(Worker::new(id));
    }
    ThreadPool { workers }
  }

  pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
  {
    f();
  }
}

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize) -> Worker {
    let thread = thread::spawn(|| {});
    Worker { id, thread }
  }
}