// This file was generated by gir (e912ccf) from gir-files (469db10)
// DO NOT EDIT

use Application;
use Bin;
use Buildable;
use Container;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use ShortcutsWindow;
use Widget;
use Window;
use ffi;
use gio;
use gio_ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ApplicationWindow(Object<ffi::GtkApplicationWindow, ffi::GtkApplicationWindowClass>): [
        Window,
        Bin,
        Container,
        Widget,
        Buildable,
        gio::ActionGroup => gio_ffi::GActionGroup,
        gio::ActionMap => gio_ffi::GActionMap,
    ];

    match fn {
        get_type => || ffi::gtk_application_window_get_type(),
    }
}

impl ApplicationWindow {
    pub fn new(application: &Application) -> ApplicationWindow {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_application_window_new(application.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait ApplicationWindowExt {
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_help_overlay(&self) -> Option<ShortcutsWindow>;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_id(&self) -> u32;

    fn get_show_menubar(&self) -> bool;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_help_overlay<'a, P: Into<Option<&'a ShortcutsWindow>>>(&self, help_overlay: P);

    fn set_show_menubar(&self, show_menubar: bool);

    fn connect_property_show_menubar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ApplicationWindow> + IsA<glib::object::Object>> ApplicationWindowExt for O {
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_help_overlay(&self) -> Option<ShortcutsWindow> {
        unsafe {
            from_glib_none(ffi::gtk_application_window_get_help_overlay(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_id(&self) -> u32 {
        unsafe {
            ffi::gtk_application_window_get_id(self.to_glib_none().0)
        }
    }

    fn get_show_menubar(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_application_window_get_show_menubar(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_help_overlay<'a, P: Into<Option<&'a ShortcutsWindow>>>(&self, help_overlay: P) {
        let help_overlay = help_overlay.into();
        let help_overlay = help_overlay.to_glib_none();
        unsafe {
            ffi::gtk_application_window_set_help_overlay(self.to_glib_none().0, help_overlay.0);
        }
    }

    fn set_show_menubar(&self, show_menubar: bool) {
        unsafe {
            ffi::gtk_application_window_set_show_menubar(self.to_glib_none().0, show_menubar.to_glib());
        }
    }

    fn connect_property_show_menubar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::show-menubar",
                transmute(notify_show_menubar_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_show_menubar_trampoline<P>(this: *mut ffi::GtkApplicationWindow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ApplicationWindow> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ApplicationWindow::from_glib_borrow(this).downcast_unchecked())
}
