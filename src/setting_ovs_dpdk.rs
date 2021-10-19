// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Setting;
#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
use glib::object::Cast;
#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
use glib::translate::*;
#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
use glib::ToValue;
#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMSettingOvsDpdk")]
    pub struct SettingOvsDpdk(Object<ffi::NMSettingOvsDpdk, ffi::NMSettingOvsDpdkClass>) @extends Setting;

    match fn {
        type_ => || ffi::nm_setting_ovs_dpdk_get_type(),
    }
}

impl SettingOvsDpdk {
    /// Creates a new [`SettingOvsDpdk`][crate::SettingOvsDpdk] object with default values.
    ///
    /// # Returns
    ///
    /// the new empty [`SettingOvsDpdk`][crate::SettingOvsDpdk] object
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "nm_setting_ovs_dpdk_new")]
    pub fn new() -> SettingOvsDpdk {
        unsafe { Setting::from_glib_full(ffi::nm_setting_ovs_dpdk_new()).unsafe_cast() }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingOvsDpdk::devargs` property of the setting
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "nm_setting_ovs_dpdk_get_devargs")]
    #[doc(alias = "get_devargs")]
    pub fn devargs(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_ovs_dpdk_get_devargs(self.to_glib_none().0)) }
    }

    /// Open vSwitch DPDK device arguments.
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    pub fn set_devargs(&self, devargs: Option<&str>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"devargs\0".as_ptr() as *const _,
                devargs.to_value().to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "devargs")]
    pub fn connect_devargs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_devargs_trampoline<F: Fn(&SettingOvsDpdk) + 'static>(
            this: *mut ffi::NMSettingOvsDpdk,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::devargs\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_devargs_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl Default for SettingOvsDpdk {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingOvsDpdk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SettingOvsDpdk")
    }
}
