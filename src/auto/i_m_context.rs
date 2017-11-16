// This file was generated by gir (e912ccf) from gir-files (469db10)
// DO NOT EDIT

#[cfg(any(feature = "v3_6", feature = "dox"))]
use InputHints;
#[cfg(any(feature = "v3_6", feature = "dox"))]
use InputPurpose;
use ffi;
use gdk;
use glib;
#[cfg(any(feature = "v3_6", feature = "dox"))]
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use pango;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct IMContext(Object<ffi::GtkIMContext, ffi::GtkIMContextClass>);

    match fn {
        get_type => || ffi::gtk_im_context_get_type(),
    }
}

pub trait IMContextExt {
    fn delete_surrounding(&self, offset: i32, n_chars: i32) -> bool;

    fn filter_keypress(&self, event: &gdk::EventKey) -> bool;

    fn focus_in(&self);

    fn focus_out(&self);

    fn get_preedit_string(&self) -> (String, pango::AttrList, i32);

    fn get_surrounding(&self) -> Option<(String, i32)>;

    fn reset(&self);

    fn set_client_window<'a, P: Into<Option<&'a gdk::Window>>>(&self, window: P);

    fn set_cursor_location(&self, area: &gdk::Rectangle);

    fn set_surrounding(&self, text: &str, cursor_index: i32);

    fn set_use_preedit(&self, use_preedit: bool);

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_property_input_hints(&self) -> InputHints;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_property_input_hints(&self, input_hints: InputHints);

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_property_input_purpose(&self) -> InputPurpose;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_property_input_purpose(&self, input_purpose: InputPurpose);

    fn connect_commit<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_delete_surrounding<F: Fn(&Self, i32, i32) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_preedit_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_preedit_end<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_preedit_start<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_retrieve_surrounding<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_input_hints_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_input_purpose_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<IMContext> + IsA<glib::object::Object>> IMContextExt for O {
    fn delete_surrounding(&self, offset: i32, n_chars: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_im_context_delete_surrounding(self.to_glib_none().0, offset, n_chars))
        }
    }

    fn filter_keypress(&self, event: &gdk::EventKey) -> bool {
        unsafe {
            from_glib(ffi::gtk_im_context_filter_keypress(self.to_glib_none().0, mut_override(event.to_glib_none().0)))
        }
    }

    fn focus_in(&self) {
        unsafe {
            ffi::gtk_im_context_focus_in(self.to_glib_none().0);
        }
    }

    fn focus_out(&self) {
        unsafe {
            ffi::gtk_im_context_focus_out(self.to_glib_none().0);
        }
    }

    fn get_preedit_string(&self) -> (String, pango::AttrList, i32) {
        unsafe {
            let mut str = ptr::null_mut();
            let mut attrs = ptr::null_mut();
            let mut cursor_pos = mem::uninitialized();
            ffi::gtk_im_context_get_preedit_string(self.to_glib_none().0, &mut str, &mut attrs, &mut cursor_pos);
            (from_glib_full(str), from_glib_full(attrs), cursor_pos)
        }
    }

    fn get_surrounding(&self) -> Option<(String, i32)> {
        unsafe {
            let mut text = ptr::null_mut();
            let mut cursor_index = mem::uninitialized();
            let ret = from_glib(ffi::gtk_im_context_get_surrounding(self.to_glib_none().0, &mut text, &mut cursor_index));
            if ret { Some((from_glib_full(text), cursor_index)) } else { None }
        }
    }

    fn reset(&self) {
        unsafe {
            ffi::gtk_im_context_reset(self.to_glib_none().0);
        }
    }

    fn set_client_window<'a, P: Into<Option<&'a gdk::Window>>>(&self, window: P) {
        let window = window.into();
        let window = window.to_glib_none();
        unsafe {
            ffi::gtk_im_context_set_client_window(self.to_glib_none().0, window.0);
        }
    }

    fn set_cursor_location(&self, area: &gdk::Rectangle) {
        unsafe {
            ffi::gtk_im_context_set_cursor_location(self.to_glib_none().0, area.to_glib_none().0);
        }
    }

    fn set_surrounding(&self, text: &str, cursor_index: i32) {
        let len = text.len() as i32;
        unsafe {
            ffi::gtk_im_context_set_surrounding(self.to_glib_none().0, text.to_glib_none().0, len, cursor_index);
        }
    }

    fn set_use_preedit(&self, use_preedit: bool) {
        unsafe {
            ffi::gtk_im_context_set_use_preedit(self.to_glib_none().0, use_preedit.to_glib());
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_property_input_hints(&self) -> InputHints {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "input-hints".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<u32>().unwrap()))
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_property_input_hints(&self, input_hints: InputHints) {
        let input_hints = input_hints.to_glib().bits() as u32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "input-hints".to_glib_none().0, Value::from(&input_hints).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_property_input_purpose(&self) -> InputPurpose {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "input-purpose".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn set_property_input_purpose(&self, input_purpose: InputPurpose) {
        let input_purpose = input_purpose.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "input-purpose".to_glib_none().0, Value::from(&input_purpose).to_glib_none().0);
        }
    }

    fn connect_commit<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "commit",
                transmute(commit_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_delete_surrounding<F: Fn(&Self, i32, i32) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32, i32) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "delete-surrounding",
                transmute(delete_surrounding_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_preedit_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "preedit-changed",
                transmute(preedit_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_preedit_end<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "preedit-end",
                transmute(preedit_end_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_preedit_start<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "preedit-start",
                transmute(preedit_start_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_retrieve_surrounding<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "retrieve-surrounding",
                transmute(retrieve_surrounding_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_input_hints_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::input-hints",
                transmute(notify_input_hints_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn connect_property_input_purpose_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::input-purpose",
                transmute(notify_input_purpose_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn commit_trampoline<P>(this: *mut ffi::GtkIMContext, str: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<IMContext> {
    callback_guard!();
    let f: &&(Fn(&P, &str) + 'static) = transmute(f);
    f(&IMContext::from_glib_borrow(this).downcast_unchecked(), &String::from_glib_none(str))
}

unsafe extern "C" fn delete_surrounding_trampoline<P>(this: *mut ffi::GtkIMContext, offset: libc::c_int, n_chars: libc::c_int, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<IMContext> {
    callback_guard!();
    let f: &&(Fn(&P, i32, i32) -> bool + 'static) = transmute(f);
    f(&IMContext::from_glib_borrow(this).downcast_unchecked(), offset, n_chars).to_glib()
}

unsafe extern "C" fn preedit_changed_trampoline<P>(this: *mut ffi::GtkIMContext, f: glib_ffi::gpointer)
where P: IsA<IMContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IMContext::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn preedit_end_trampoline<P>(this: *mut ffi::GtkIMContext, f: glib_ffi::gpointer)
where P: IsA<IMContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IMContext::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn preedit_start_trampoline<P>(this: *mut ffi::GtkIMContext, f: glib_ffi::gpointer)
where P: IsA<IMContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IMContext::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn retrieve_surrounding_trampoline<P>(this: *mut ffi::GtkIMContext, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<IMContext> {
    callback_guard!();
    let f: &&(Fn(&P) -> bool + 'static) = transmute(f);
    f(&IMContext::from_glib_borrow(this).downcast_unchecked()).to_glib()
}

#[cfg(any(feature = "v3_6", feature = "dox"))]
unsafe extern "C" fn notify_input_hints_trampoline<P>(this: *mut ffi::GtkIMContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IMContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IMContext::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_6", feature = "dox"))]
unsafe extern "C" fn notify_input_purpose_trampoline<P>(this: *mut ffi::GtkIMContext, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<IMContext> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&IMContext::from_glib_borrow(this).downcast_unchecked())
}
