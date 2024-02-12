use futures::future::{BoxFuture, FutureExt};
use futures::task::{waker_ref, ArcWake};
use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

struct Task {
    // 実行するコルーチン
    future: Mutex<BoxFuture<'static, ()>>,
    // Executor へスケジューリングするためのチャネル
    sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // 自身をスケジューリング
        let self0 = arc_self.clone();
        arc_self.sender.send(self0).unwrap();
    }
}

pub struct Executor {
    // ❶
    // 実行キュー
    sender: SyncSender<Arc<Task>>,
    receiver: Receiver<Arc<Task>>,
}
impl Executor {
    pub fn new() -> Self {
        // チャネルを生成。キューのサイズは最大1024 個
        let (sender, receiver) = sync_channel(1024);
        Executor {
            sender: sender.clone(),
            receiver,
        }
    }
    // 新たにTask を生成するためのSpawner を作成 ❷
    pub fn get_spawner(&self) -> Spawner {
        Spawner {
            sender: self.sender.clone(),
        }
    }
    pub fn run(&self) {
        // ❸
        // チャネルからTask を受信して順に実行
        while let Ok(task) = self.receiver.recv() {
            // コンテキストを生成
            let mut future = task.future.lock().unwrap();
            let waker = waker_ref(&task);
            let mut ctx = Context::from_waker(&waker);
            // poll を呼び出し実行
            let _ = future.as_mut().poll(&mut ctx);
        }
    }
}
pub struct Spawner {
    sender: SyncSender<Arc<Task>>,
}
impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed(); // Future をBox 化
        let task = Arc::new(Task {
            // Task 生成
            future: Mutex::new(future),
            sender: self.sender.clone(),
        });
        // 実行キューにエンキュー
        self.sender.send(task).unwrap();
    }
}

pub struct Hello {
    state: StateHello,
}

enum StateHello {
    HELLO,
    WORLD,
    END,
}

impl Hello {
    pub fn new() -> Self {
        Hello {
            state: StateHello::HELLO, // 初期状態
        }
    }
}

impl Future for Hello {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        match (*self).state {
            StateHello::HELLO => {
                print!("Hello, ");
                (*self).state = StateHello::WORLD;
                cx.waker().wake_by_ref(); // 自身を実行キューにエンキュー
                return Poll::Pending;
            }
            StateHello::WORLD => {
                println!("World!");
                (*self).state = StateHello::END;
                cx.waker().wake_by_ref(); // 自身を実行キューにエンキュー
                return Poll::Pending;
            }
            StateHello::END => {
                return Poll::Ready(());
            }
        }
    }
}
