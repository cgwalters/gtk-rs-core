// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{SocketControlMessage, UnixFDList};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GUnixFDMessage")]
    pub struct UnixFDMessage(Object<ffi::GUnixFDMessage, ffi::GUnixFDMessageClass>) @extends SocketControlMessage;

    match fn {
        type_ => || ffi::g_unix_fd_message_get_type(),
    }
}

impl UnixFDMessage {
    pub const NONE: Option<&'static UnixFDMessage> = None;

    #[doc(alias = "g_unix_fd_message_new")]
    pub fn new() -> UnixFDMessage {
        unsafe { SocketControlMessage::from_glib_full(ffi::g_unix_fd_message_new()).unsafe_cast() }
    }

    #[doc(alias = "g_unix_fd_message_new_with_fd_list")]
    #[doc(alias = "new_with_fd_list")]
    pub fn with_fd_list(fd_list: &impl IsA<UnixFDList>) -> UnixFDMessage {
        unsafe {
            SocketControlMessage::from_glib_full(ffi::g_unix_fd_message_new_with_fd_list(
                fd_list.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

impl Default for UnixFDMessage {
    fn default() -> Self {
        Self::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::UnixFDMessage>> Sealed for T {}
}

pub trait UnixFDMessageExt: IsA<UnixFDMessage> + sealed::Sealed + 'static {
    #[doc(alias = "g_unix_fd_message_get_fd_list")]
    #[doc(alias = "get_fd_list")]
    fn fd_list(&self) -> UnixFDList {
        unsafe {
            from_glib_none(ffi::g_unix_fd_message_get_fd_list(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<UnixFDMessage>> UnixFDMessageExt for O {}

impl fmt::Display for UnixFDMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("UnixFDMessage")
    }
}
