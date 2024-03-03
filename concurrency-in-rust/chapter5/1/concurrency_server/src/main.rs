use nix::sys::epoll::{
    epoll_create1, epoll_ctl, epoll_wait, EpollCreateFlags, EpollEvent, EpollFlags, EpollOp,
};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpListener;
use std::os::unix::io::{AsRawFd, RawFd};

fn main() {
    let epoll_in = EpollFlags::EPOLLIN;
    let epoll_add = EpollOp::EpollCtlAdd;
    let epoll_del = EpollOp::EpollCtlDel;

    let listner = TcpListener::bind("0.0.0.0:10001").unwrap();

    // 監視対象ソケット（ファイルディスクリプタ）を epoll オブジェクトに登録する
    // 「監視対象にイベント発生 → 検知 → 登録しておいた対応処理実行」
    let epfd = epoll_create1(EpollCreateFlags::empty()).unwrap();

    let listen_fd = listner.as_raw_fd();
    let mut ev = EpollEvent::new(epoll_in, listen_fd as u64);
    // epoll オブジェクトにリッスン用ソケットを監視対象として登録
    epoll_ctl(epfd, epoll_add, listen_fd, &mut ev).unwrap();

    // 監視対象のファイルディスクリプタの HashMap
    let mut fd2buf = HashMap::new();
    let mut events = vec![EpollEvent::empty(); 1024];

    // イベント発生の監視（epoll_wait）
    // 第二引数: イベント発生したファイルディスクリプタの数
    // 第三引数: タイムアウトミリ秒（-1だとタイムアウトしない）
    while let Ok(nfds) = epoll_wait(epfd, &mut events, -1) {
        // イベント発生したファイルディスクリプタに順番に処理（epoll_wait の第二引数でイベント発生した数が返ってきている）
        for n in 0..nfds {
            if events[n].data() == listen_fd as u64 {
                if let Ok((stream, _)) = listner.accept() {

                    // 読み込み・書き込みオブジェクト生成
                    let fd = stream.as_raw_fd();
                    let stream0 = stream.try_clone().unwrap();
                    let reader = BufReader::new(stream0);
                    let writer = BufWriter::new(stream);

                    // ファイルディスクリプタに reader, writer を紐づけ
                    fd2buf.insert(fd, (reader, writer));

                    println!("accept: fd = {}", fd);

                    // ファイルディスクリプタを監視対象に登録
                    let mut ev = EpollEvent::new(epoll_in, fd as u64);
                    epoll_ctl(epfd, epoll_add, fd, &mut ev).unwrap();
                } else {
                    // クライアントからデータ受信
                    let fd = events[n].data() as RawFd;
                    let (reader, writer) = fd2buf.get_mut(&fd).unwrap();

                    // 1行読み込む
                    let mut buf = String::new();
                    let n = reader.read_line(&mut buf).unwrap();

                    // コネクションクローズした場合は epoll の監視対象から外す
                    if n == 0 {
                        let mut ev = EpollEvent::new(epoll_in, fd as u64);
                        epoll_ctl(epfd, epoll_del, fd, &mut ev).unwrap();

                        fd2buf.remove(&fd);
                        println!("closed: fd = {}", fd);
                        continue;
                    }
                    print!("read: fd = {}, buf = {}", fd, buf);

                    // 読み込んだデータをそのまま書き込み
                    writer.write(buf.as_bytes()).unwrap();
                    writer.flush().unwrap();
                }
            }
        }
    }
}
