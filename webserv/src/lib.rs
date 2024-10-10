use std::sync::Mutex;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;

type Job = Box<dyn FnOnce() + Send + 'static>; // Job型はFnOnceトレイト(FnOnceトレイトは引数を取らず、戻り値を返さないクロージャを表す)を実装するクロージャを表す

pub struct ThreadPool{
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0); // sizeが0以下の場合はpanicを起こす

    let mut workers = Vec::with_capacity(size); // size分の要素を持つVecを作成

    let (sender, receiver) = mpsc::channel(); // チャンネルを作成
    let receiver = Arc::new(Mutex::new(receiver));

    for id in 0..size {
      // thread::spawn(|| {})で新しいスレッドを生成し、そのスレッドが実行するクロージャを引数に取る
      // このクロージャは何もしない
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }
    ThreadPool { workers, sender }
  }

  pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
  {
    let job: Job = Box::new(f);

    self.sender.send(job).unwrap();
  }
}

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || {
      loop {
        let job = receiver.lock().unwrap().recv().unwrap();
        println!("Worker {} got a job; executing.", id);
        job();
      }
    });
    Worker { id, thread }
  }
}