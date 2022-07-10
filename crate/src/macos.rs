//! Functionality specific to macOS.

use cocoa::appkit::NSWindow;
use tauri::Window;

pub trait WindowExt {
    fn set_titlebar_transparent(&self);
}

impl WindowExt for Window {
    fn set_titlebar_transparent(&self) {
        let window = self.ns_window().unwrap() as cocoa::base::id;
        unsafe {
            window.setTitlebarAppearsTransparent_(cocoa::base::YES);
        }
    }
}
