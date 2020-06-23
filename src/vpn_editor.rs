// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use crate::Connection;

glib_wrapper! {
    pub struct VpnEditor(Interface<nm_sys::NMVpnEditor>);

    match fn {
        get_type => || nm_sys::nm_vpn_editor_get_type(),
    }
}

pub const NONE_VPN_EDITOR: Option<&VpnEditor> = None;

/// Trait containing all `VpnEditor` methods.
///
/// # Implementors
///
/// [`VpnEditor`](struct.VpnEditor.html)
pub trait VpnEditorExt: 'static {
    fn get_widget(&self) -> Option<glib::Object>;

    fn update_connection<P: IsA<Connection>>(&self, connection: &P) -> Result<(), glib::Error>;

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<VpnEditor>> VpnEditorExt for O {
    fn get_widget(&self) -> Option<glib::Object> {
        unsafe {
            from_glib_none(nm_sys::nm_vpn_editor_get_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn update_connection<P: IsA<Connection>>(&self, connection: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = nm_sys::nm_vpn_editor_update_connection(
                self.as_ref().to_glib_none().0,
                connection.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMVpnEditor,
            f: glib_sys::gpointer,
        ) where
            P: IsA<VpnEditor>,
        {
            let f: &F = &*(f as *const F);
            f(&VpnEditor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for VpnEditor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VpnEditor")
    }
}
