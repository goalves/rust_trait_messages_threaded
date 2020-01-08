use crate::module_a::{FirstM, Message, SecondM};
use std::thread;

pub fn new(a_sender: crossbeam_channel::Sender<Box<dyn Message>>) -> thread::JoinHandle<()> {
    let thread_handle = thread::Builder::new().spawn(move || run(a_sender)).unwrap();

    thread_handle
}

fn run(sender: crossbeam_channel::Sender<Box<dyn Message>>) {
    let periodic_ticks = crossbeam_channel::tick(std::time::Duration::from_secs(5));
    let periodic_ticks_2 = crossbeam_channel::tick(std::time::Duration::from_secs(2));

    loop {
        crossbeam_channel::select! {
            recv(periodic_ticks) -> _ => { sender.send(Box::new(FirstM {})).unwrap(); }
            recv(periodic_ticks_2) -> _ => { sender.send(Box::new(SecondM {})).unwrap(); }
            default(std::time::Duration::from_millis(100)) => (),
        };
    }
}
