// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Setting;
#[cfg(any(feature = "v1_34", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_34")))]
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::fmt;
#[cfg(any(feature = "v1_34", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_34")))]
use std::{boxed::Box as Box_, mem::transmute};

glib::wrapper! {
    /// Bond Port Settings
    ///
    /// ## Properties
    ///
    ///
    /// #### `queue-id`
    ///  The queue ID of this bond port. The maximum value of queue ID is
    /// the number of TX queues currently active in device.
    ///
    /// Readable | Writeable
    /// <details><summary><h4>Setting</h4></summary>
    ///
    ///
    /// #### `name`
    ///  The setting's name, which uniquely identifies the setting within the
    /// connection. Each setting type has a name unique to that type, for
    /// example "ppp" or "802-11-wireless" or "802-3-ethernet".
    ///
    /// Readable
    /// </details>
    ///
    /// # Implements
    ///
    /// [`SettingExt`][trait@crate::prelude::SettingExt], [`trait@glib::ObjectExt`]
    #[doc(alias = "NMSettingBondPort")]
    pub struct SettingBondPort(Object<ffi::NMSettingBondPort, ffi::NMSettingBondPortClass>) @extends Setting;

    match fn {
        type_ => || ffi::nm_setting_bond_port_get_type(),
    }
}

impl SettingBondPort {
    /// Creates a new [`SettingBondPort`][crate::SettingBondPort] object with default values.
    ///
    /// # Returns
    ///
    /// the new empty [`SettingBondPort`][crate::SettingBondPort] object
    #[cfg(any(feature = "v1_34", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_34")))]
    #[doc(alias = "nm_setting_bond_port_new")]
    pub fn new() -> SettingBondPort {
        unsafe { Setting::from_glib_full(ffi::nm_setting_bond_port_new()).unsafe_cast() }
    }

    ///
    /// # Returns
    ///
    /// the [`queue_id`][struct@crate::SettingBondPort#queue_id] property of the setting
    #[cfg(any(feature = "v1_34", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_34")))]
    #[doc(alias = "nm_setting_bond_port_get_queue_id")]
    #[doc(alias = "get_queue_id")]
    pub fn queue_id(&self) -> u32 {
        unsafe { ffi::nm_setting_bond_port_get_queue_id(self.to_glib_none().0) }
    }

    /// The queue ID of this bond port. The maximum value of queue ID is
    /// the number of TX queues currently active in device.
    #[cfg(any(feature = "v1_34", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_34")))]
    #[doc(alias = "queue-id")]
    pub fn set_queue_id(&self, queue_id: u32) {
        glib::ObjectExt::set_property(self, "queue-id", &queue_id)
    }

    #[cfg(any(feature = "v1_34", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_34")))]
    #[doc(alias = "queue-id")]
    pub fn connect_queue_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_queue_id_trampoline<F: Fn(&SettingBondPort) + 'static>(
            this: *mut ffi::NMSettingBondPort,
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
                b"notify::queue-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_queue_id_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v1_34", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_34")))]
impl Default for SettingBondPort {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingBondPort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SettingBondPort")
    }
}