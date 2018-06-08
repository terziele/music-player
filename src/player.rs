use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use crossbeam::sync::SegQueue;
use pulse_simple::Playback;

use self::Action::*;
use mp3::Mp3Decoder;
use State;

const BUFFER_SIZE: usize = 1000;
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

impl State {
    pub fn new(initial_state: bool) -> Self {
        State {
            stopped: initial_state,
        }
    }
}

pub struct Player {
    app_state: Arc<Mutex<State>>,
    event_loop: EventLoop,
}

impl Player {
    pub(crate) fn new(app_state: Arc<Mutex<State>>) -> Self {
        let event_loop = EventLoop::new();

        {
            let app_state = app_state.clone();
            let event_loop = event_loop.clone();
            thread::spawn(move || {
                let mut buffer = [[0; 2]; BUFFER_SIZE];
                let mut playback = Playback::new("MP3", "MP3 Playback", None, DEFAULT_RATE as u32);
                let mut source = None;

                loop {
                    if let Some(action) = event_loop.queue.try_pop() {
                        match action {
                            Load(path) => {
                                let file = File::open(path).unwrap();
                                source = Some(Mp3Decoder::new(BufReader::new(file)).unwrap());

                                let rate = source
                                    .as_ref()
                                    .map(|source| source.sample_rate())
                                    .unwrap_or(DEFAULT_RATE as u32);
                                playback = Playback::new("MP3", "MP3 Playback", None, rate);
                                app_state.lock().unwrap().stopped = false;
                            }
                            Stop => {}
                        }
                    }
                }
            });
        }

        Player {
            app_state,
            event_loop,
        }
    }
}
