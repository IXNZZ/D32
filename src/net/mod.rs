use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use crate::net::command::Command;

pub mod command;

pub fn create_network() -> (Sender<Command>, Receiver<Command>) {
    let (sender, net_receiver) = mpsc::channel::<Command>();
    let (net_sender, receiver) = mpsc::channel::<Command>();
    (sender, receiver)
}