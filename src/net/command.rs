use bytes::{Buf, Bytes};
use serde::{Deserialize, Serialize};
use crate::net::command::frame::Ack;

#[derive(Debug)]
pub enum Command {
    // 通用应答
    Ack(Ack),
    // 连接握手
    Shake {
        major: u8,
        minor: u8,
        build: u8,
    },
    // 信息同步
    Sync,
}

impl Command {

    pub fn from(src: &[u8]) -> Option<Self> {
        let mut src = Bytes::copy_from_slice(src);
        let frame = src.get_u16_le();
        if frame == Ack::frame() {
            return Some(Ack::from_bytes(&mut src));
        }

        None
    }

    pub fn bytes(&self) -> Bytes {
        match self {
            Command::Ack(ack) => {ack.bytes()}
            Command::Shake { .. } => {Bytes::new()}
            Command::Sync => {Bytes::new()}
            _ => {Bytes::new()}
        }
    }


}

trait CommandFrame {

    fn frame() -> u16;

    fn bytes(&self) -> Bytes;

    fn from_bytes(src: &mut Bytes) -> Command;
}

pub mod frame {
    use bytes::{Buf, BufMut, Bytes, BytesMut};
    use crate::net::command::{Command, CommandFrame};

    #[derive(Debug)]
    pub struct Ack {
        pub ack_frame: u16,
        pub result: u8,
    }

    impl Ack {
        pub fn ok(frame: u16) -> Self {
            Ack {
                ack_frame: frame,
                result: 0
            }
        }

        pub fn fail(frame: u16, result: u8) -> Self {
            Ack {
                ack_frame: frame,
                result,
            }
        }
    }

    impl CommandFrame for Ack {
        fn frame() -> u16 {
            0x0000
        }

        fn bytes(&self) -> Bytes {
            let mut dst = BytesMut::new();
            dst.put_u16_le(Ack::frame());
            dst.put_u16_le(self.ack_frame);
            dst.put_u8(self.result);
            dst.freeze()
        }

        fn from_bytes(src: &mut Bytes) -> Command {
            let mut src = src;
            Command::Ack(Ack { ack_frame: src.get_u16_le(), result: src.get_u8() })
        }
    }
}



