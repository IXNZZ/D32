use crossbeam::channel::{Receiver, Sender};
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::{SplitSink, SplitStream};
use serde::Serialize;
use tokio::net::TcpStream;
use tokio_util::codec::{Encoder, Framed};
use tracing::{debug, error, info, warn};

use crate::net::codec::CommandCodec;
use crate::net::command::Command;

pub mod command;
mod codec;

pub enum NetNotice {
    Connected,
    ConnectFailed,
    Message(Command),
    Close,
}

#[tokio::main]
pub async fn run(addr: &str, sender: Sender<NetNotice>, receiver: Receiver<NetNotice>) -> anyhow::Result<()> {
    let net = start_connection(addr, sender, receiver).await;
    if net.is_err() {
        return Err(net.unwrap_err())
    }
    Ok(())
}

// #[tokio::main]
// pub async fn create_network(addr: &str, sender: Sender<NetNotice>, receiver: Receiver<NetNotice>) -> anyhow::Result<()> {
//     let net = start_connection(addr, sender, receiver).await;
//     if net.is_err() {
//         return Err(net.unwrap_err())
//     }
//     Ok(())
// }

async fn start_connection(addr: &str, sender: Sender<NetNotice>, receiver: Receiver<NetNotice>) -> anyhow::Result<()> {

    if let Ok(stream) = TcpStream::connect(addr).await {
        debug!("连接网络成功: {}", addr);
        sender.send(NetNotice::Connected)?;
        tokio::spawn(async move {
            let _ = process_stream(stream, sender, receiver).await;
        }).await?;
        return Ok(());
    }
    sender.send(NetNotice::ConnectFailed)?;
    debug!("网络连接失败: {}", addr);
    // let err = anyhow::anyhow!("网络连接失败: {}", addr);
    Err(anyhow::anyhow!("网络连接失败: {}", addr))
    // Ok(())
}

async fn process_stream(stream: TcpStream, sender: Sender<NetNotice>, receiver: Receiver<NetNotice>) -> anyhow::Result<()> {
    let framed = Framed::new(stream, CommandCodec);
    let (frame_writer, frame_reader) = framed.split::<Command>();
    let mut read_task = tokio::spawn(async move {
        read_from_client(frame_reader, sender).await;
    });

    // 负责向客户端写行数据的异步子任务
    let mut write_task = tokio::spawn(async move {
        write_to_client(frame_writer, receiver).await;
    });

    // 无论是读任务还是写任务的终止，另一个任务都将没有继续存在的意义，因此都将另一个任务也终止
    if tokio::try_join!(&mut read_task, &mut write_task).is_err() {
        warn!("网络连接监听结束!");
        read_task.abort();
        write_task.abort();
    };
    Ok(())
}

type FramedStream = SplitStream<Framed<TcpStream, CommandCodec>>;
type FramedSink = SplitSink<Framed<TcpStream, CommandCodec>, Command>;
async fn read_from_client(mut reader: FramedStream, sender: Sender<NetNotice>) {
    loop {
        match reader.next().await {
            None => {
                info!("server closed");
                break;
            }
            Some(Err(e)) => {
                error!("read from server error: {}", e);
                break;
            }
            Some(Ok(cmd)) => {
                debug!("read from server. content: {:?}", cmd);
                // 将内容发送给writer，让writer响应给客户端，
                // 如果无法发送给writer，继续从客户端读取内容将没有意义，因此break退出
                if sender.send(NetNotice::Message(cmd)).is_err() {
                    error!("receiver closed");
                }
            }
        }
    }
}

async fn write_to_client(mut writer: FramedSink, mut receiver: Receiver<NetNotice>) {
    while let Ok(notice) = receiver.recv() {
        match notice {
            NetNotice::Message(cmd) => {
                // println!("发送消息: {:?}", cmd);
                if writer.send(cmd).await.is_err() {
                    error!("write to client failed");
                    break;
                }
            }
            NetNotice::Close => {
                return;
            }
            _ => {}
        }

    }
}




