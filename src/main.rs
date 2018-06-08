mod mp3;
mod player;
mod playlist;
mod toolbar;

extern crate crossbeam;
extern crate gdk_pixbuf;
extern crate gio;
extern crate gtk;
extern crate gtk_sys;
extern crate id3;
extern crate pulse_simple;
extern crate simplemad;

use std::env;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};

use gtk::{Application, ApplicationWindow, ContainerExt, GtkWindowExt, ToolButtonExt, WidgetExt};

use gtk::Orientation::{Horizontal, Vertical};
use gtk::{Adjustment, Image, ImageExt, Scale, ScaleExt};

use playlist::Playlist;
use toolbar::MusicToolbar;

struct App {
    playlist: Rc<Playlist>,
    state: Arc<Mutex<State>>,
    toolbar: MusicToolbar,
    adjustment: Adjustment,
    cover: Image,
    window: ApplicationWindow,
}

impl App {
    pub fn new(application: &Application) -> Self {
        let window = ApplicationWindow::new(&application);
        window.set_title("Music player");

        let music_toolbar = MusicToolbar::new();
        let vbox = gtk::Box::new(Vertical, 0);
        vbox.add(music_toolbar.toolbar());
        window.add(&vbox);

        let state = Arc::new(Mutex::new(State::new(true)));
        // add playlist
        let playlist = Rc::new(Playlist::new(state.clone()));
        vbox.add(playlist.view());

        let cover = Image::new();
        vbox.add(&cover);

        let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
        let scale = Scale::new(Horizontal, &adjustment);
        scale.set_draw_value(false);
        vbox.add(&scale);

        window.show_all();

        let app = App {
            playlist,
            state,
            toolbar: music_toolbar,
            adjustment,
            cover,
            window,
        };

        app.connect_events();
        app.connect_toolbar_events();

        app
    }

    fn connect_events(&self) {
        // do nothing now
    }
}

struct State {
    stopped: bool,
}

/// Converts `Duration` to milliseconds
fn to_millis(duration: Duration) -> u64 {
    duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1_000_000
}

fn main() {
    let application = Application::new("com.github.barricade", ApplicationFlags::empty())
        .expect("Unable to create an application");
    application.connect_startup(|application| {
        App::new(&application);
    });

    application.connect_activate(|_| {});

    application.run(&env::args().collect::<Vec<_>>());
}
