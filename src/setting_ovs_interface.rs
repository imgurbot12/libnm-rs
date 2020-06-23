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
use glib::StaticType;
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
    pub struct SettingOvsInterface(Object<nm_sys::NMSettingOvsInterface, nm_sys::NMSettingOvsInterfaceClass, SettingOvsInterfaceClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_ovs_interface_get_type(),
    }
}

impl SettingOvsInterface {
    /// Creates a new `SettingOvsInterface` object with default values.
    ///
    /// Feature: `v1_10`
    ///
    ///
    /// # Returns
    ///
    /// the new empty `SettingOvsInterface` object
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn new() -> SettingOvsInterface {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_ovs_interface_new()).unsafe_cast() }
    }

    ///
    /// Feature: `v1_10`
    ///
    ///
    /// # Returns
    ///
    /// the `SettingOvsInterface:type` property of the setting
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_interface_type(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_ovs_interface_get_interface_type(
                self.to_glib_none().0,
            ))
        }
    }

    /// The interface type. Either "internal", "system", "patch", "dpdk", or empty.
    ///
    /// Feature: `v1_10`
    ///
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_property_type(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `type` getter")
        }
    }

    /// The interface type. Either "internal", "system", "patch", "dpdk", or empty.
    ///
    /// Feature: `v1_10`
    ///
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn set_property_type(&self, type_: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"type\0".as_ptr() as *const _,
                Value::from(type_).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn connect_property_type_notify<F: Fn(&SettingOvsInterface) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<F: Fn(&SettingOvsInterface) + 'static>(
            this: *mut nm_sys::NMSettingOvsInterface,
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
                b"notify::type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
impl Default for SettingOvsInterface {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingOvsInterface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingOvsInterface")
    }
}
