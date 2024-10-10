use std::thread;
use std::sync::mpsc; // mpscはマルチスレッド間でメッセージを送受信するためのチャンネル
use std::sync::Arc; // Arcは複数のスレッドが同時に所有できるスマートポインタ
use std::sync::Mutex; // Mutexは複数のスレッドが同時にアクセスできないようにするための構造体

// Job型はFnOnceトレイト(引数を取らず、戻り値を返さないクロージャを表す)とSendトレイト(スレッド間で値を送信できる)を実装する
type Job = Box<dyn FnOnce() + Send + 'static>;

// ThreadPool構造体は、スレッドプールのメイン構造体
pub struct ThreadPool{
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>, // Job型の値を送信するためのチャンネル
}

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0); // sizeが0以下の場合はpanicを起こす

    let mut workers = Vec::with_capacity(size); // size分の要素を持つVecを作成

    let (sender, receiver) = mpsc::channel(); // チャンネルを作成
    let receiver = Arc::new(Mutex::new(receiver)); // ArcとMutexを使ってreceiverをスレッド間で共有可能にする

    for id in 0..size {
      // thread::spawn(|| {})で新しいスレッドを生成し、そのスレッドが実行するクロージャを引数に取る
      workers.push(Worker::new(id, Arc::clone(&receiver))); // Worker::new関数でWorker構造体を生成し、workersベクタに追加
    }
    ThreadPool { workers, sender }
  }

  pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
  {
    let job: Job = Box::new(f);

    self.sender.send(job).unwrap(); // スレッドプール内のスレッドにジョブを送信
  }
}

// Worker構造体は、スレッドプール内のスレッドがジョブを受信して実行するための構造体
struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || {
      loop {
        let job = receiver.lock().unwrap().recv().unwrap(); // ジョブが送信されるまでブロックし、ジョブが送信されたらそれを取り出す
        println!("Worker {} got a job; executing.", id);
        job();
      }
    });
    Worker { id, thread }
  }
}