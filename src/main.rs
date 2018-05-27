mod toolbar;
mod playlist;


extern crate gio;
extern crate gtk;
extern crate gdk_pixbuf;
extern crate id3;
extern crate gtk_sys;

use std::env;
use std::rc::Rc;
use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};

use gtk::{

    ToolButtonExt,
    Application,
    ApplicationWindow,
    WidgetExt,
    GtkWindowExt,
    ContainerExt,
};

use gtk::{
    Adjustment,
    Image,
    ImageExt,
    Scale,
    ScaleExt
};
use gtk::Orientation::{Horizontal, Vertical};

use toolbar::MusicToolbar;
use playlist::Playlist;


struct App {
    playlist: Rc<Playlist>,
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

        // add playlist
        let playlist = Rc::new(Playlist::new());
        vbox.add(playlist.view());

        let cover = Image::new();
        cover.set_from_file("assets/cover.jpg");
        vbox.add(&cover);

        let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
        let scale = Scale::new(Horizontal, &adjustment);
        scale.set_draw_value(false);
        vbox.add(&scale);

        //window.add(music_toolbar.toolbar());

        window.show_all();
        
        let app = App {
            playlist,
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

fn main() {
    let application = Application::new("com.github.barricade", ApplicationFlags::empty())
        .expect("Unable to create an application");
    application.connect_startup(|application| {

        App::new(&application);
    });

    application.connect_activate(|_|{});

    application.run(&env::args().collect::<Vec<_>>());


}
