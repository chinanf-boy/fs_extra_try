use std::path::Path;
use std::{thread, time};
use std::sync::mpsc::{self, TryRecvError};

extern crate fs_extra;
use fs_extra::dir::*;
use fs_extra::error::*;

fn example_copy() -> Result<()> {

    let path_from = Path::new("./temp"); // temp 路径
    let path_to = path_from.join("out"); // temp + out 路径
    let test_folder = path_from.join("test_folder"); // temp + test_folder 路径
    let dir = test_folder.join("dir"); // test_folder + dir 路径
    let sub = dir.join("sub");
    let file1 = dir.join("file1.txt");
    let file2 = sub.join("file2.txt");

    create_all(&sub, true)?; // 创建
    create_all(&path_to, true)?;
    fs_extra::file::write_all(&file1, "content1")?; // 写入
    fs_extra::file::write_all(&file2, "content2")?;

    assert!(dir.exists()); // 参数如果是错误, 会 抛出错误❌
    assert!(sub.exists());
    assert!(file1.exists());
    assert!(file2.exists());


    let mut options = CopyOptions::new(); // 可用于配置文件将如何复制或移动的选项和标志。
    options.buffer_size = 1;
    let (tx, rx) = mpsc::channel(); // 通道
    thread::spawn(move || { // 线程
        let handler = |process_info: TransitProcess| {
            tx.send(process_info).unwrap(); // 通道发送
            thread::sleep(time::Duration::from_millis(2)); // 线程等待
            fs_extra::dir::TransitProcessResult::ContinueOrAbort //如果进程没有错误，则继续执行进程，如果进程内容错误则继续进行。
            //https://docs.rs/fs_extra/*/fs_extra/dir/enum.TransitProcessResult.html
        };
        // 分进程-复制, 从 test_folder 到 out 目录 全复制
        copy_with_progress(&test_folder, &path_to, &options, handler).unwrap();
        // https://docs.rs/fs_extra/*/fs_extra/dir/fn.copy_with_progress.html
    });

    loop {
        match rx.try_recv() { // 通道接收
            Ok(process_info) => {
                println!("{} of {} bytes",
                         process_info.copied_bytes,
                         process_info.total_bytes);
            }
            Err(TryRecvError::Disconnected) => {
                println!("finished");
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
    }
    Ok(())

}
fn main() {
    example_copy().expect("Error handle filess");
}