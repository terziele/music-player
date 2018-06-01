use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use crossbeam::sync::SegQueue;
use pulse_simple::Playback;

use mp3::Mp3Decoder;
use self::Action::*;

const BUFFER_SIZE: usize =  1000;
const DEFAULT_RATE: usize = 44100;

enum Action {
    Load(PathBuf),
    Stop,
}

#[derive(Clone)]
struct EventLoop {
    queue: Arc<SegQueue<Action>>,
    playing: Arc<Mutex<bool>>,
}


impl EventLoop {
    /// Creates new instance of `EventLoop`
    fn new() -> Self {
        EventLoop {
            queue: Arc::new(SegQueue::new()),
            playing: Arc::new(Mutex::new(false)),
        }
    }
}


pub struct State {
    stopped: bool,
}

impl State {
    pub fn new(initial_state: bool) -> Self {
        State {
            stopped: initial_state,
        }
    }
}


