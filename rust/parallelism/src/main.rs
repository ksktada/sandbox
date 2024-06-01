use std::io;
use std::thread;
use std::vec;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello, Thread World!");
    });

    let _ = handle.join();
    println!("Hello, Main World!");
    fork_and_join();
}

fn fork_and_join() {
    let filenames = vec![
        "file1.txt".to_string(),
        "file2.txt".to_string(),
        "file3.txt".to_string(),
    ];

    let filenames2 = filenames.clone();

    match process_files(filenames) {
        Ok(_) => println!("Hello, Ok World!"),
        Err(_) => println!("Hello, Err World!"),
    }

    match process_files_in_parallel(filenames2) {
        Ok(_) => println!("Hello, Parallel Ok World!"),
        Err(_) => println!("Hello, Parallel Err World!"),
    }
}

// parallel
fn process_files_in_parallel(filenames: Vec<String>) -> io::Result<()> {
    // Divide the work into several chunks. 仕事をいくつかのチャンクに分割
    const NTHREADS: usize = 3;
    let worklists = split_vec_into_chunks(filenames, NTHREADS);

    // Fork: Spawn a thread to handle each chunk. フォーク：それぞれのチャンクを処理するスレッドを起動
    let mut thread_handles = vec![];
    for worklist in worklists {
        thread_handles.push(thread::spawn(move || process_files(worklist)));
    }

    // Join: Wait for all threads to finish. ジョイン：すべてのスレッドが終了するのを待つ
    for handle in thread_handles {
        handle.join().unwrap()?;
    }
    Ok(())
}

fn split_vec_into_chunks(_filenames: Vec<String>, _n_threads: usize) -> Vec<Vec<String>> {
    vec![
        vec!["file1.txt".to_string()],
        vec!["file2.txt".to_string()],
        vec!["file3.txt".to_string()],
    ]
}

// not parallel
fn process_files(filenames: Vec<String>) -> io::Result<()> {
    for document in filenames {
        let text = load(&document)?; // read source file ソースファイルを読み込む
        let results = process(text); // compute statistics 統計値を計算
        save(&document, results)?; // write output file 出力ファイルに書き出す
    }
    Ok(())
}

fn load(document: &String) -> io::Result<String> {
    Ok(document.clone())
}

fn process(text: String) -> String {
    text.to_string()
}

fn save(_document: &String, _results: String) -> io::Result<()> {
    Ok(())
}
