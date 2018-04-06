mod toolbar;


extern crate gio;
extern crate gtk;

use std::env;
use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};

use gtk::{

    ToolButtonExt,
    Application,
    ApplicationWindow,
    WidgetExt,
    GtkWindowExt,
};

use toolbar::MusicToolbar;


struct App {
    toolbar: MusicToolbar,
    window: ApplicationWindow,
}

impl App {
    pub fn new(application: &Application) -> Self {

        let window = ApplicationWindow::new(&application);
        window.set_title("Music player");


        let music_toolbar = MusicToolbar::new();
        window.add(&music_toolbar.toolbar());

        window.show_all();
        
        let app = App {
            toolbar: music_toolbar,
            window,
        };

        app.connect_events();


        app
    }

    fn connect_events(&self) {
        // do nothing now
    }
}

fn main() {
    let application = Application::new("com.github.barricade", ApplicationFlags::empty())
        .expect("Unable to create an application");
    application.connect_startup(|application| {

        App::new(&application)
    });

    application.connect_activate(|_|{});

    application.run(&env::args().collect::<Vec<_>>());


}
