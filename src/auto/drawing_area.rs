// This file was generated by gir (e912ccf) from gir-files (469db10)
// DO NOT EDIT

use Buildable;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DrawingArea(Object<ffi::GtkDrawingArea, ffi::GtkDrawingAreaClass>): Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_drawing_area_get_type(),
    }
}

impl DrawingArea {
    pub fn new() -> DrawingArea {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_drawing_area_new()).downcast_unchecked()
        }
    }
}

impl Default for DrawingArea {
    fn default() -> Self {
        Self::new()
    }
}
