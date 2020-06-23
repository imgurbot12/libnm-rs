// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use crate::Device;
use crate::DeviceWifi;
use crate::Object;

glib_wrapper! {
    pub struct DeviceOlpcMesh(Object<nm_sys::NMDeviceOlpcMesh, nm_sys::NMDeviceOlpcMeshClass, DeviceOlpcMeshClass>) @extends Device, Object;

    match fn {
        get_type => || nm_sys::nm_device_olpc_mesh_get_type(),
    }
}

impl DeviceOlpcMesh {
    /// Returns the active channel of the `DeviceOlpcMesh` device.
    ///
    /// # Returns
    ///
    /// active channel of the device
    pub fn get_active_channel(&self) -> u32 {
        unsafe { nm_sys::nm_device_olpc_mesh_get_active_channel(self.to_glib_none().0) }
    }

    /// Gets the companion device of the `DeviceOlpcMesh`.
    ///
    /// # Returns
    ///
    /// the companion of the device of `None`
    pub fn get_companion(&self) -> Option<DeviceWifi> {
        unsafe {
            from_glib_none(nm_sys::nm_device_olpc_mesh_get_companion(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn connect_property_active_channel_notify<F: Fn(&DeviceOlpcMesh) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_channel_trampoline<F: Fn(&DeviceOlpcMesh) + 'static>(
            this: *mut nm_sys::NMDeviceOlpcMesh,
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
                b"notify::active-channel\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_active_channel_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_companion_notify<F: Fn(&DeviceOlpcMesh) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_companion_trampoline<F: Fn(&DeviceOlpcMesh) + 'static>(
            this: *mut nm_sys::NMDeviceOlpcMesh,
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
                b"notify::companion\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_companion_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceOlpcMesh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceOlpcMesh")
    }
}
