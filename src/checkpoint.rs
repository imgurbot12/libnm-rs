// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_12", feature = "dox"))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use glib_sys;
use nm_sys;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use std::mem::transmute;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use crate::Device;
use crate::Object;

glib_wrapper! {
    pub struct Checkpoint(Object<nm_sys::NMCheckpoint, nm_sys::NMCheckpointClass, CheckpointClass>) @extends Object;

    match fn {
        get_type => || nm_sys::nm_checkpoint_get_type(),
    }
}

impl Checkpoint {
    /// Gets the timestamp (in CLOCK_BOOTTIME milliseconds) of checkpoint creation.
    ///
    /// Use `nm_utils_get_timestamp_msec` to obtain current time value suitable for
    /// comparing to this value.
    ///
    /// Feature: `v1_12`
    ///
    ///
    /// # Returns
    ///
    /// the timestamp of checkpoint creation.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_created(&self) -> i64 {
        unsafe { nm_sys::nm_checkpoint_get_created(self.to_glib_none().0) }
    }

    /// The devices that are part of this checkpoint.
    ///
    /// Feature: `v1_12`
    ///
    ///
    /// # Returns
    ///
    /// the devices list.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_devices(&self) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(nm_sys::nm_checkpoint_get_devices(
                self.to_glib_none().0,
            ))
        }
    }

    /// Gets the timeout in seconds for automatic rollback.
    ///
    /// Feature: `v1_12`
    ///
    ///
    /// # Returns
    ///
    /// the rollback timeout.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn get_rollback_timeout(&self) -> u32 {
        unsafe { nm_sys::nm_checkpoint_get_rollback_timeout(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn connect_property_created_notify<F: Fn(&Checkpoint) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_created_trampoline<F: Fn(&Checkpoint) + 'static>(
            this: *mut nm_sys::NMCheckpoint,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::created\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_created_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn connect_property_devices_notify<F: Fn(&Checkpoint) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_devices_trampoline<F: Fn(&Checkpoint) + 'static>(
            this: *mut nm_sys::NMCheckpoint,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::devices\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_devices_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn connect_property_rollback_timeout_notify<F: Fn(&Checkpoint) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_rollback_timeout_trampoline<F: Fn(&Checkpoint) + 'static>(
            this: *mut nm_sys::NMCheckpoint,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::rollback-timeout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_rollback_timeout_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Checkpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Checkpoint")
    }
}
