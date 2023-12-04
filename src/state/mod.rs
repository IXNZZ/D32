pub mod map;
pub mod sprite;

use std::path::PathBuf;
use std::thread;
use crossbeam::channel::{Receiver, Sender};
use futures_util::SinkExt;
use ggez::Context;
use tokio::runtime::Runtime;
use tracing::info;
use crate::cache::ImageCache;
use crate::net::command::Command;
use crate::net::{NetNotice, run};
use crate::net::command::frame::Ack;
use crate::state::map::MapState;
use crate::state::sprite::SpriteState;


pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub build: u8,
}
pub struct State {
    pub(crate) base_dir: PathBuf,
    pub(crate) scale_factor: f32,
    pub(crate) screen_width: f32,
    pub(crate) screen_height: f32,
    pub(crate) window_width: f32,
    pub(crate) window_height: f32,
    pub(crate) center_x: f32,
    pub(crate) center_y: f32,
    pub(crate) initialled: bool,
    pub(crate) version: Version,
    pub(crate) cache: ImageCache,
    pub(crate) map: MapState,
    pub(crate) sprite: SpriteState,
    pub(crate) net: NetState,
}

impl State {

    pub fn new(base_dir: PathBuf, addr: &str, ctx: &mut Context) -> Self {
        let scale_factor = ctx.gfx.window().scale_factor();
        let (window_width, window_height) = ctx.gfx.drawable_size();
        let monitor_size = ctx.gfx.window().current_monitor().unwrap().size();
        let net = NetState::new(addr);
        // 这里的runtime 对象无法drop
        // let rt = tokio::runtime::Runtime::new().unwrap();
        Self {
            base_dir: base_dir.clone(),
            scale_factor: scale_factor as f32,
            screen_width: monitor_size.width as f32,
            screen_height: monitor_size.height as f32,
            window_width,
            window_height,
            center_x: window_width / 2.,
            center_y: window_height / 2.,
            initialled: false,
            version: Version {major: 0, minor: 1, build: 0 },
            cache: ImageCache::new(base_dir.join("data")),
            map: MapState::new(&base_dir),
            sprite: SpriteState::default(),
            net
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum NetStatus {
    None, // 未连接
    Connecting, //连接中
    Connected, // 连接成功
    ConnectFailed, //连接失败
    Loading, // 同步数据
    Ready, // 网络就绪
}

pub struct NetState {
    sender: Sender<NetNotice>,
    receiver: Receiver<NetNotice>,
    net_sender: Sender<NetNotice>,
    net_receiver: Receiver<NetNotice>,
    pub addr: String,
    pub status: NetStatus,
    pub retry: u32,
}

impl NetState {

    pub fn new(addr: &str) -> Self {
        let (sender, net_receiver) = crossbeam::channel::unbounded::<NetNotice>();
        let (net_sender, receiver) = crossbeam::channel::unbounded::<NetNotice>();
        Self {
            sender,
            receiver,
            net_sender,
            net_receiver,
            addr: String::from(addr),
            status: NetStatus::None,
            retry: 0,
        }
    }

    pub fn connect(&mut self) {
        let addr = self.addr.clone();
        self.status = NetStatus::Connecting;
        let sender = self.net_sender.clone();
        let receiver = self.net_receiver.clone();
        thread::spawn(move || {
            run(&addr, sender, receiver);
        });
    }

    pub fn init(&mut self) {
        if self.retry > 3 { return; }
        if self.status == NetStatus::None || self.status == NetStatus::ConnectFailed {
            self.retry += 1;
            self.connect();
        }
    }

    pub fn close(&mut self) {
        // if let Some(sender) = &self.sender {
            let _ = self.sender.send(NetNotice::Close);
        // }
    }

    pub fn send_command(&self, cmd: Command) {
        if self.status != NetStatus::Ready {
            return;
        }
        let _ = self.sender.send(NetNotice::Message(cmd));
    }

    pub fn recv_command(&mut self) -> Option<Command> {
        let receiver = &mut self.receiver;
        if let Ok(notice) = receiver.try_recv() {
            match notice {
                NetNotice::Connected => {
                    self.status = NetStatus::Connected;
                    // send init net
                    let _ = self.sender.send(NetNotice::Message(Command::Ack(Ack::ok(1))));
                }
                NetNotice::ConnectFailed => {
                    self.status = NetStatus::ConnectFailed;
                }
                NetNotice::Message(cmd) => {
                    match &cmd {
                        Command::Ack(ack) => {
                            if ack.ack_frame == 1 && ack.result == 0 {
                                info!("网络初始化成功!");
                                self.status = NetStatus::Ready;
                            }
                        }
                        _ => {}
                    }
                    return Some(cmd);
                }
                NetNotice::Close => {
                    self.status = NetStatus::None;
                }
            }
        }
        None
    }
}