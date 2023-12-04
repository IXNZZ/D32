use std::thread;
use std::time::Duration;
use ggez::{Context, GameError};
use tracing::{debug, error, info};
use tracing::field::debug;
use crate::event::AppEventHandler;
use crate::net;
use crate::net::command::Command;
use crate::net::command::frame::Ack;
use crate::net::{NetNotice, run};
use crate::state::{NetStatus, State};





pub struct LoadingScene {
    loaded: bool,
    net_loading_count: u8,
}

impl LoadingScene {

    pub fn new(_ctx: &mut Context, state: &mut State) -> Self {
        // LoadingScene::load_network(state);
        Self {
            loaded: false,
            net_loading_count: 0,
        }
    }

    // pub fn load_network(&mut self, state: &mut State) {
    //     state.net.status = NetStatus::Connecting;
    //     let (sender, net_receiver) = crossbeam::channel::unbounded::<NetNotice>();
    //     let (net_sender, receiver) = crossbeam::channel::unbounded::<NetNotice>();
    //     let addr = String::from(&state.net.addr);
    //     thread::spawn(move || {
    //         run(&addr, net_sender, net_receiver);
    //     });
    // }
}

impl AppEventHandler<GameError> for LoadingScene {
    fn select(&mut self, _ctx: &mut Context, _state: &mut State) -> Option<Box<dyn AppEventHandler>> {

        None
    }

    fn net(&mut self, _ctx: &mut Context, _state: &mut State, cmd: Command) -> Result<bool, GameError> {

        Ok(false)
    }



    fn update(&mut self, _ctx: &mut Context, state: &mut State) -> Result<(), GameError> {
        // if state.net.status == NetStatus::None {
        //     self.load_network(state);
        //     return Ok(());
        // }
        // // println!("net status: {:?}, eq: {}", state.net.status, state.net.status == NetStatus::Connected);
        // if state.net.status == NetStatus::Connected {
        //     // debug!("test send ack");
        //     state.net.send_command(Command::Ack(Ack::ok(1)));
        //     state.net.status = NetStatus::Loading;
        // }
        state.net.init();
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context, _state: &mut State) -> Result<(), GameError> {
        Ok(())
    }
}