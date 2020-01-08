use std::thread;

pub trait Message: 'static + Send {
    fn process(&self);
}

#[derive(Clone, Debug)]
pub struct FirstM();

impl Message for FirstM {
    fn process(&self) {
        println!("{:#?}", self);
    }
}

#[derive(Clone, Debug)]
pub struct SecondM();

impl Message for SecondM {
    fn process(&self) {
        println!("{:#?}", self);
    }
}

pub fn new() -> (
    thread::JoinHandle<()>,
    crossbeam_channel::Sender<Box<dyn Message>>,
) {
    let (sender, receiver) = crossbeam_channel::unbounded();
    let thread_handle = thread::Builder::new().spawn(move || run(receiver)).unwrap();

    (thread_handle, sender)
}

fn run(receiver: crossbeam_channel::Receiver<Box<dyn Message>>) {
    loop {
        crossbeam_channel::select! {
            recv(receiver) -> result => { result.as_ref().unwrap().process(); }
            default(std::time::Duration::from_millis(100)) => (),
        };
    }
}
