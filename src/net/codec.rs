use std::io;
use std::io::Read;
use anyhow::anyhow;
use bytes::{BufMut, Bytes, BytesMut};
use tokio_util::codec::{Decoder, Encoder};
use tracing::{debug, warn};
use crate::net::command::Command;

pub struct CommandCodec;

impl CommandCodec {
    const MAX_SIZE: usize = 0xFFFF - 6;
    const MARK: u16 = 0x6D32;

    fn encode_command(&self, item: &Command) -> anyhow::Result<Bytes> {
        return Ok(item.bytes());
        // Err(anyhow::anyhow!("数据解析错误"))
    }

    fn decode_command(&self, data: &[u8]) -> anyhow::Result<Command> {
        if let Some(cmd) = Command::from(data) {
            return Ok(cmd);
        }
        Err(anyhow::anyhow!("数据解析错误"))
    }
}

impl Encoder<Command> for CommandCodec {
    type Error = anyhow::Error;

    fn encode(&mut self, item: Command, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let data = self.encode_command(&item);
        if data.is_err() {
            return Err(data.err().unwrap());
        }
        let data = data.unwrap();
        let len = data.len();
        if len > Self::MAX_SIZE {
            return Err(anyhow::anyhow!("数据长度超出限制: {}", len))
        }

        dst.reserve(len + 6);
        dst.put_u16_le(Self::MARK);
        dst.put_u16_le(len as u16 + 2);
        dst.extend_from_slice(&data);
        dst.put_u16_le(0xAACC);
        debug!("encode net message: {:X?}", &dst[..]);
        Ok(())
    }
}

impl Decoder for CommandCodec {
    type Item = Command;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let buf_len = src.len();

        // 如果buf中的数据量连长度声明的大小都不足，则先跳过等待后面更多数据的到来
        if buf_len < 8 { return Ok(None); }

        // let src_bytes = src.bytes();
        let mut mark_bytes = [0; 2];
        mark_bytes.copy_from_slice(&src[..2]);
        let mark = u16::from_le_bytes(mark_bytes);
        if mark != Self::MARK {
            return Err(anyhow!("网络标识错误: {}.", mark));
        }

        // 先读取帧首，获得声明的帧中实际数据大小
        let mut length_bytes = [0u8; 2];
        length_bytes.copy_from_slice(&src[2..4]);
        let data_len = u16::from_le_bytes(length_bytes) as usize;
        if data_len > Self::MAX_SIZE {
            return Err(anyhow!("Frame of length {} is too large.", data_len));
        }

        // 帧的总长度为 4 + frame_len
        let frame_len = data_len + 4;
        // debug!("decode net message1: {}, buf: {}", frame_len, buf_len);
        // buf中数据量不够，跳过，并预先申请足够的空闲空间来存放该帧后续到来的数据
        if buf_len < frame_len {
            src.reserve(frame_len - buf_len);
            return Ok(None);
        }

        // 数据量足够了，从buf中取出数据转编成帧，并转换为指定类型后返回
        // 需同时将buf截断(split_to会截断)
        let frame_bytes = src.split_to(frame_len);
        debug!("decode net message2: {:X?}", &frame_bytes[..]);
        let data = self.decode_command(&frame_bytes[4..]);
        if let Ok(data) = data {
            return Ok(Some(data))
        }
        if data.is_err() {
            warn!("网络解码错误: {:?}", data.err());
        }
        Ok(None)
    }
}