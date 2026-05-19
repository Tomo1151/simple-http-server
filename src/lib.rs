use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

enum Message {
  NewJob(Job),
  Terminate,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
  fn call_box(self: Box<F>) {
      (*self)()
  }
}

// Job構造体の定義
type Job = Box<dyn FnBox + Send + 'static>;


// ThreadPool構造体の定義
pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>,
}

// ThreadPool構造体の実装
impl ThreadPool {
  // コンストラクタの実装
  pub fn new(size: usize) -> ThreadPool {
    // サイズが0の時にパニックを起こす
    assert!(size > 0);

    // 双方向チャンネルの生成
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    // スレッドの生成
    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool{
      workers,
      sender,
    }
  }

  // 実行メソッドの実装
  pub fn execute<F>(&self, f: F)
    where
      F: FnOnce() + Send + 'static
  {
    let job = Box::new(f);
    self.sender.send(Message::NewJob(job)).unwrap();
  }
}

// Dropトレイトの実装
impl Drop for ThreadPool {
  fn drop(&mut self) {
    for _ in &mut self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }

    for worker in &mut self.workers {
      println!("Shutting down worker {}", worker.id);
      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}

// Worker構造体の定義
struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

// Worker構造体の実装
impl Worker {
  // コンストラクタの実装
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || {
      loop {
          let message = receiver.lock().unwrap().recv().unwrap();

          match message {
              Message::NewJob(job) => {
                println!("Worker {} got a job; executing.", id);
                job.call_box();
              },
              Message::Terminate => {
                println!("Worker {} was told to terminate.", id);
                break;
              }
          }
      }
    });

    Worker {
      id,
      thread: Some(thread),
    }
  }
}

