use crate::{Backend,Error,Features};
use objc::{msg_send, sel, sel_impl, class, runtime::Object};
use procspawn;
use ipc_channel::ipc::*;
use ::serde::{Serialize,Deserialize};
use log::trace;

#[derive(Debug,Serialize, Deserialize)]
pub enum Message{
    SupportedFeatures((),IpcSender<Features>),
    Speak((String,bool),IpcSender<Result<(),Error>>),
    Stop((),IpcSender<Result<(),Error>>),
    MinRate((),IpcSender<f32>),
    MaxRate((),IpcSender<f32>),
    NormalRate((),IpcSender<f32>),
    GetRate((),IpcSender<Result<f32,Error>>),
    SetRate((f32,),IpcSender<Result<(),Error>>),
    MinPitch((),IpcSender<f32>),
    MaxPitch((),IpcSender<f32>),
    NormalPitch((),IpcSender<f32>),
    GetPitch((),IpcSender<Result<f32,Error>>),
    SetPitch((f32,),IpcSender<Result<(),Error>>),
    MinVolume((),IpcSender<f32>),
    MaxVolume((),IpcSender<f32>),
    NormalVolume((),IpcSender<f32>),
    GetVolume((),IpcSender<Result<f32,Error>>),
    SetVolume((f32,),IpcSender<Result<(),Error>>),
    IsSpeaking((),IpcSender<Result<bool,Error>>),
}

pub struct ThreadWrapper(procspawn::JoinHandle<()>, IpcSender<Message>);

pub trait ThreadWrapped: Backend + Sized {
    fn new() -> Self;
    fn wrap() -> ThreadWrapper {
        let (tx, rx) = channel::<Message>().unwrap();
        procspawn::init();
        
        ThreadWrapper(
            procspawn::spawn(rx, |rx| {
                let mut back = Self::new();
                let runloop: *const Object = unsafe{ msg_send![class!(NSRunLoop), currentRunLoop] };
                let mut date: *mut Object = unsafe{ msg_send![class!(NSDate), alloc] };
                loop {
                    date = unsafe{ msg_send![date, initWithTimeIntervalSinceNow: 0.01] };
                    back.step(&rx);
                    let _: () = unsafe{ msg_send![runloop, runUntilDate:date] };
                }
            }),
            tx,
        )
    }

    fn step(&mut self, rx: &IpcReceiver<Message>) {
        match rx.try_recv() {
            Ok(msg) => {trace!("{:?}", msg); match msg {
                Message::SupportedFeatures((), ret)   => ret.send(self.supported_features()).unwrap(),
                Message::Speak((text,interrupt), ret) => ret.send(self.speak(text.as_str(),interrupt)).unwrap(),
                Message::Stop((), ret)                => ret.send(self.stop()).unwrap(),
                Message::MinRate((), ret)             => ret.send(self.min_rate()).unwrap(),
                Message::MaxRate((), ret)             => ret.send(self.max_rate()).unwrap(),
                Message::NormalRate((), ret)          => ret.send(self.normal_rate()).unwrap(),
                Message::GetRate((), ret)             => ret.send(self.get_rate()).unwrap(),
                Message::SetRate((rate,), ret)        => ret.send(self.set_rate(rate)).unwrap(),
                Message::MinPitch((), ret)            => ret.send(self.min_pitch()).unwrap(),
                Message::MaxPitch((), ret)            => ret.send(self.max_pitch()).unwrap(),
                Message::NormalPitch((), ret)         => ret.send(self.normal_pitch()).unwrap(),
                Message::GetPitch((), ret)            => ret.send(self.get_pitch()).unwrap(),
                Message::SetPitch((pitch,), ret)      => ret.send(self.set_pitch(pitch)).unwrap(),
                Message::MinVolume((), ret)           => ret.send(self.min_volume()).unwrap(),
                Message::MaxVolume((), ret)           => ret.send(self.max_volume()).unwrap(),
                Message::NormalVolume((), ret)        => ret.send(self.normal_volume()).unwrap(),
                Message::GetVolume((), ret)           => ret.send(self.get_volume()).unwrap(),
                Message::SetVolume((volume,), ret)    => ret.send(self.set_volume(volume)).unwrap(),
                Message::IsSpeaking((), ret)          => ret.send(self.is_speaking()).unwrap(),
            }},
            Err(_e) => ()
        }
    }
}

impl Backend for ThreadWrapper {
    fn supported_features(&self) -> Features {
        let (tx,rx) = channel().unwrap();
        self.1.send(Message::SupportedFeatures((),tx)).unwrap();
        rx.recv().unwrap()
    }

    fn speak(&mut self, text: &str, interrupt: bool) -> Result<(), Error> {
        let (tx,rx) = channel().unwrap();
        self.1.send(Message::Speak((text.to_string(),interrupt),tx)).unwrap();
        rx.recv().unwrap()
    }

    fn stop(&mut self) -> Result<(), Error> {
        let (tx,rx) = channel().unwrap();
        self.1.send(Message::Stop((),tx)).unwrap();
        rx.recv().unwrap()
    }

    fn min_rate(&self) -> f32 {
        let (tx,rx) = channel().unwrap();
        self.1.send(Message::MinRate((),tx)).unwrap();
        rx.recv().unwrap()
    }

    fn max_rate(&self) -> f32 {
        let (tx,rx) = channel().unwrap();
        self.1.send(Message::MaxRate((),tx)).unwrap();
        rx.recv().unwrap()
    }

    fn normal_rate(&self) -> f32 {
        let (tx,rx) = channel().unwrap();
        self.1.send(Message::NormalRate((),tx)).unwrap();
        rx.recv().unwrap()
    }

    fn get_rate(&self) -> Result<f32, Error> {
        let (tx,rx) = channel().unwrap();
        self.1.send(Message::GetRate((),tx)).unwrap();
        rx.recv().unwrap()
    }

    fn set_rate(&mut self, rate: f32) -> Result<(), Error> {
        let (tx,rx) = channel().unwrap();
        self.1.send(Message::SetRate((rate,),tx)).unwrap();
        rx.recv().unwrap()
    }

    fn min_pitch(&self) -> f32 {
        let (tx,rx) = channel().unwrap();
        self.1.send(Message::MinPitch((),tx)).unwrap();
        rx.recv().unwrap()
    }

    fn max_pitch(&self) -> f32 {
        let (tx,rx) = channel().unwrap();
        self.1.send(Message::MaxPitch((),tx)).unwrap();
        rx.recv().unwrap()
    }

    fn normal_pitch(&self) -> f32 {
        let (tx,rx) = channel().unwrap();
        self.1.send(Message::NormalPitch((),tx)).unwrap();
        rx.recv().unwrap()
    }

    fn get_pitch(&self) -> Result<f32, Error> {
        let (tx,rx) = channel().unwrap();
        self.1.send(Message::GetPitch((),tx)).unwrap();
        rx.recv().unwrap()
    }

    fn set_pitch(&mut self, pitch: f32) -> Result<(), Error> {
        let (tx,rx) = channel().unwrap();
        self.1.send(Message::SetPitch((pitch,),tx)).unwrap();
        rx.recv().unwrap()
    }

    fn min_volume(&self) -> f32 {
        let (tx,rx) = channel().unwrap();
        self.1.send(Message::MinVolume((),tx)).unwrap();
        rx.recv().unwrap()
    }

    fn max_volume(&self) -> f32 {
        let (tx,rx) = channel().unwrap();
        self.1.send(Message::MaxVolume((),tx)).unwrap();
        rx.recv().unwrap()
    }

    fn normal_volume(&self) -> f32 {
        let (tx,rx) = channel().unwrap();
        self.1.send(Message::NormalVolume((),tx)).unwrap();
        rx.recv().unwrap()
    }

    fn get_volume(&self) -> Result<f32, Error> {
        let (tx,rx) = channel().unwrap();
        self.1.send(Message::GetVolume((),tx)).unwrap();
        rx.recv().unwrap()
    }

    fn set_volume(&mut self, volume: f32) -> Result<(), Error> {
        let (tx,rx) = channel().unwrap();
        self.1.send(Message::SetVolume((volume,),tx)).unwrap();
        rx.recv().unwrap()
    }

    fn is_speaking(&self) -> Result<bool, Error> {
        let (tx,rx) = channel().unwrap();
        self.1.send(Message::IsSpeaking((),tx)).unwrap();
        rx.recv().unwrap()
    }

}
