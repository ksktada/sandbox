// https://github.com/oreilly-japan/conc_ytakano/blob/main/chap5/5.2/ch5_2_2_sched/src/main.rs
use asynchronous::{Executor, Hello};

fn main() {
    let executor = Executor::new();
    executor.get_spawner().spawn(async {
        let h = Hello::new();
        h.await;
    });
    executor.run();
}
