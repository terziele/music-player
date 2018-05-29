

use gtk::{
    ContainerExt,
    SeparatorToolItem,
    Toolbar,
    ToolButton
};

use App;

use gtk:: {
    ToolButtonExt,
    WidgetExt,
    ApplicationWindow
};

use gtk::{
    FileChooserAction,
    FileChooserDialog, 
    FileFilter,
    FileFilterExt,
};


use std::path::PathBuf;

use gtk_sys::{GTK_RESPONSE_ACCEPT, GTK_RESPONSE_CANCEL};

const RESPONSE_ACCEPT: i32 = GTK_RESPONSE_ACCEPT as i32;
const RESPONSE_CANCEL: i32 = GTK_RESPONSE_CANCEL as i32;


const PLAY_STOCK: &str = "gtk-media-play";
const PAUSE_STOCK: &str = "gtk-media-pause";


pub struct MusicToolbar {
    open_button: ToolButton,
    next_button: ToolButton,
    play_button: ToolButton,
    previous_button: ToolButton,
    quit_button: ToolButton,
    remove_button: ToolButton,
    stop_button: ToolButton,
    toolbar: Toolbar,
}

impl MusicToolbar {
    pub fn new() -> Self {

        let toolbar = Toolbar::new();
        let open_button = ToolButton::new_from_stock("gtk-open");
        toolbar.add(&open_button);


        toolbar.add(&SeparatorToolItem::new());

        let previous_button = ToolButton::new_from_stock("gtk-media-previous");
        toolbar.add(&previous_button);

        let play_button = ToolButton::new_from_stock(PLAY_STOCK);
        toolbar.add(&play_button);

        let stop_button = ToolButton::new_from_stock("gtk-media-stop");
        toolbar.add(&stop_button);

        let next_button = ToolButton::new_from_stock("gtk-media-next");
        toolbar.add(&next_button);

        let remove_button = ToolButton::new_from_stock("gtk-remove");
        toolbar.add(&remove_button);

        toolbar.add(&SeparatorToolItem::new());

        let quit_button = ToolButton::new_from_stock("gtk-quit");
        toolbar.add(&quit_button);
        
        MusicToolbar {
            open_button,
            next_button,
            play_button,
            previous_button,
            quit_button,
            remove_button,
            stop_button,
            toolbar,
        }

    }

    /// Getter for a toolbar
    pub fn toolbar(&self) -> &Toolbar {
        &self.toolbar
    }

    /// Show dialog window for user to choose audio file to play
    fn show_open_dialog(parent: &ApplicationWindow) -> Option<PathBuf> {
        let mut file = None;

        let dialog = FileChooserDialog::new(Some("Select an MP3 file to play"), Some(parent), FileChooserAction::Open);
        let filter = FileFilter::new();
        filter.add_mime_type("audio/mp3");
        filter.set_name("MP3 audio file");
        dialog.add_filter(&filter);

        dialog.add_button("Cancel", RESPONSE_CANCEL);
        dialog.add_button("Accept", RESPONSE_ACCEPT);

        let result = dialog.run();
        if result == RESPONSE_ACCEPT {
            file = dialog.get_filename();
        }
        dialog.destroy();

        file
    }


}


impl App {
    pub fn connect_toolbar_events(&self) {
        let window = self.window.clone();
        self.toolbar.quit_button.connect_clicked(move |_| {
            window.destroy();
        });
    }
}

