// https://github.com/oreilly-japan/conc_ytakano/blob/main/chap5/5.3/ch5_3_2_ioselect/src/main.rs

fn main() {
    let executor = Executor::new();
    let selector = IOSelector::new();
    let spawner = executor.get_spawner();

    let server = async move { // <1>
        // 非同期アクセプト用のリスナを生成 <2>
        let listener = AsyncListener::listen("127.0.0.1:10000",
                                             selector.clone());
        loop {
            // 非同期コネクションアクセプト <3>
            let (mut reader, mut writer, addr) =
                listener.accept().await;
            println!("accept: {}", addr);

            // コネクションごとにタスクを生成 <4>
            spawner.spawn(async move {
                // 1行非同期読み込み <5>
                while let Some(buf) = reader.read_line().await {
                    print!("read: {}, {}", addr, buf);
                    writer.write(buf.as_bytes()).unwrap();
                    writer.flush().unwrap();
                }
                println!("close: {}", addr);
            });
        }
    };

    // タスクを生成して実行
    executor.get_spawner().spawn(server);
    executor.run();
}