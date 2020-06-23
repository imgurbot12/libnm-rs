// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib::object::Cast;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib::GString;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib::Value;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use gobject_sys;
use nm_sys;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use std::mem::transmute;
use crate::Setting;

glib_wrapper! {
    pub struct SettingOvsPatch(Object<nm_sys::NMSettingOvsPatch, nm_sys::NMSettingOvsPatchClass, SettingOvsPatchClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_ovs_patch_get_type(),
    }
}

impl SettingOvsPatch {
    /// Creates a new `SettingOvsPatch` object with default values.
    ///
    /// Feature: `v1_10`
    ///
    ///
    /// # Returns
    ///
    /// the new empty `SettingOvsPatch` object
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn new() -> SettingOvsPatch {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_ovs_patch_new()).unsafe_cast() }
    }

    ///
    /// Feature: `v1_10`
    ///
    ///
    /// # Returns
    ///
    /// the `SettingOvsPatch:peer` property of the setting
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_peer(&self) -> Option<GString> {
        unsafe { from_glib_none(nm_sys::nm_setting_ovs_patch_get_peer(self.to_glib_none().0)) }
    }

    /// Specifies the unicast destination IP address of a remote Open vSwitch
    /// bridge port to connect to.
    ///
    /// Feature: `v1_10`
    ///
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn set_property_peer(&self, peer: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"peer\0".as_ptr() as *const _,
                Value::from(peer).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn connect_property_peer_notify<F: Fn(&SettingOvsPatch) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_peer_trampoline<F: Fn(&SettingOvsPatch) + 'static>(
            this: *mut nm_sys::NMSettingOvsPatch,
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
                b"notify::peer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_peer_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
impl Default for SettingOvsPatch {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingOvsPatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingOvsPatch")
    }
}
