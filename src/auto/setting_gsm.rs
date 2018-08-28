// This file was generated by gir (https://github.com/gtk-rs/gir @ 464833e)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::Value;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;
use Setting;
use SettingSecretFlags;

glib_wrapper! {
    pub struct SettingGsm(Object<ffi::NMSettingGsm, ffi::NMSettingGsmClass>): Setting;

    match fn {
        get_type => || ffi::nm_setting_gsm_get_type(),
    }
}

impl SettingGsm {
    pub fn new() -> SettingGsm {
        unsafe { Setting::from_glib_full(ffi::nm_setting_gsm_new()).downcast_unchecked() }
    }
}

impl Default for SettingGsm {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SettingGsmExt {
    fn get_apn(&self) -> Option<String>;

    fn get_device_id(&self) -> Option<String>;

    fn get_home_only(&self) -> bool;

    #[cfg(any(feature = "v1_8", feature = "dox"))]
    fn get_mtu(&self) -> u32;

    fn get_network_id(&self) -> Option<String>;

    fn get_number(&self) -> Option<String>;

    fn get_password(&self) -> Option<String>;

    fn get_password_flags(&self) -> SettingSecretFlags;

    fn get_pin(&self) -> Option<String>;

    fn get_pin_flags(&self) -> SettingSecretFlags;

    fn get_sim_id(&self) -> Option<String>;

    fn get_sim_operator_id(&self) -> Option<String>;

    fn get_username(&self) -> Option<String>;

    fn set_property_apn(&self, apn: Option<&str>);

    fn set_property_device_id(&self, device_id: Option<&str>);

    fn set_property_home_only(&self, home_only: bool);

    #[cfg(any(feature = "v1_8", feature = "dox"))]
    fn set_property_mtu(&self, mtu: u32);

    fn set_property_network_id(&self, network_id: Option<&str>);

    fn set_property_number(&self, number: Option<&str>);

    fn set_property_password(&self, password: Option<&str>);

    fn set_property_password_flags(&self, password_flags: SettingSecretFlags);

    fn set_property_pin(&self, pin: Option<&str>);

    fn set_property_pin_flags(&self, pin_flags: SettingSecretFlags);

    fn set_property_sim_id(&self, sim_id: Option<&str>);

    fn set_property_sim_operator_id(&self, sim_operator_id: Option<&str>);

    fn set_property_username(&self, username: Option<&str>);

    fn connect_property_apn_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_device_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_home_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_8", feature = "dox"))]
    fn connect_property_mtu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_network_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_number_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_password_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_password_flags_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_pin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pin_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_sim_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_sim_operator_id_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_username_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SettingGsm> + IsA<glib::object::Object>> SettingGsmExt for O {
    fn get_apn(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_gsm_get_apn(self.to_glib_none().0)) }
    }

    fn get_device_id(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_gsm_get_device_id(self.to_glib_none().0)) }
    }

    fn get_home_only(&self) -> bool {
        unsafe { from_glib(ffi::nm_setting_gsm_get_home_only(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_8", feature = "dox"))]
    fn get_mtu(&self) -> u32 {
        unsafe { ffi::nm_setting_gsm_get_mtu(self.to_glib_none().0) }
    }

    fn get_network_id(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_gsm_get_network_id(self.to_glib_none().0)) }
    }

    fn get_number(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_gsm_get_number(self.to_glib_none().0)) }
    }

    fn get_password(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_gsm_get_password(self.to_glib_none().0)) }
    }

    fn get_password_flags(&self) -> SettingSecretFlags {
        unsafe {
            from_glib(ffi::nm_setting_gsm_get_password_flags(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_pin(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_gsm_get_pin(self.to_glib_none().0)) }
    }

    fn get_pin_flags(&self) -> SettingSecretFlags {
        unsafe { from_glib(ffi::nm_setting_gsm_get_pin_flags(self.to_glib_none().0)) }
    }

    fn get_sim_id(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_gsm_get_sim_id(self.to_glib_none().0)) }
    }

    fn get_sim_operator_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_setting_gsm_get_sim_operator_id(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_username(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_gsm_get_username(self.to_glib_none().0)) }
    }

    fn set_property_apn(&self, apn: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "apn".to_glib_none().0,
                Value::from(apn).to_glib_none().0,
            );
        }
    }

    fn set_property_device_id(&self, device_id: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "device-id".to_glib_none().0,
                Value::from(device_id).to_glib_none().0,
            );
        }
    }

    fn set_property_home_only(&self, home_only: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "home-only".to_glib_none().0,
                Value::from(&home_only).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_8", feature = "dox"))]
    fn set_property_mtu(&self, mtu: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "mtu".to_glib_none().0,
                Value::from(&mtu).to_glib_none().0,
            );
        }
    }

    fn set_property_network_id(&self, network_id: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "network-id".to_glib_none().0,
                Value::from(network_id).to_glib_none().0,
            );
        }
    }

    fn set_property_number(&self, number: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "number".to_glib_none().0,
                Value::from(number).to_glib_none().0,
            );
        }
    }

    fn set_property_password(&self, password: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "password".to_glib_none().0,
                Value::from(password).to_glib_none().0,
            );
        }
    }

    fn set_property_password_flags(&self, password_flags: SettingSecretFlags) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "password-flags".to_glib_none().0,
                Value::from(&password_flags).to_glib_none().0,
            );
        }
    }

    fn set_property_pin(&self, pin: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "pin".to_glib_none().0,
                Value::from(pin).to_glib_none().0,
            );
        }
    }

    fn set_property_pin_flags(&self, pin_flags: SettingSecretFlags) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "pin-flags".to_glib_none().0,
                Value::from(&pin_flags).to_glib_none().0,
            );
        }
    }

    fn set_property_sim_id(&self, sim_id: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "sim-id".to_glib_none().0,
                Value::from(sim_id).to_glib_none().0,
            );
        }
    }

    fn set_property_sim_operator_id(&self, sim_operator_id: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "sim-operator-id".to_glib_none().0,
                Value::from(sim_operator_id).to_glib_none().0,
            );
        }
    }

    fn set_property_username(&self, username: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "username".to_glib_none().0,
                Value::from(username).to_glib_none().0,
            );
        }
    }

    fn connect_property_apn_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::apn",
                transmute(notify_apn_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_device_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::device-id",
                transmute(notify_device_id_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_home_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::home-only",
                transmute(notify_home_only_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_8", feature = "dox"))]
    fn connect_property_mtu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::mtu",
                transmute(notify_mtu_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_network_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::network-id",
                transmute(notify_network_id_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_number_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::number",
                transmute(notify_number_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_password_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::password",
                transmute(notify_password_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_password_flags_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::password-flags",
                transmute(notify_password_flags_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_pin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::pin",
                transmute(notify_pin_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_pin_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::pin-flags",
                transmute(notify_pin_flags_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_sim_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::sim-id",
                transmute(notify_sim_id_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_sim_operator_id_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::sim-operator-id",
                transmute(notify_sim_operator_id_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_username_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::username",
                transmute(notify_username_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

unsafe extern "C" fn notify_apn_trampoline<P>(
    this: *mut ffi::NMSettingGsm,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingGsm>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingGsm::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_device_id_trampoline<P>(
    this: *mut ffi::NMSettingGsm,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingGsm>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingGsm::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_home_only_trampoline<P>(
    this: *mut ffi::NMSettingGsm,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingGsm>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingGsm::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_8", feature = "dox"))]
unsafe extern "C" fn notify_mtu_trampoline<P>(
    this: *mut ffi::NMSettingGsm,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingGsm>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingGsm::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_network_id_trampoline<P>(
    this: *mut ffi::NMSettingGsm,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingGsm>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingGsm::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_number_trampoline<P>(
    this: *mut ffi::NMSettingGsm,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingGsm>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingGsm::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_password_trampoline<P>(
    this: *mut ffi::NMSettingGsm,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingGsm>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingGsm::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_password_flags_trampoline<P>(
    this: *mut ffi::NMSettingGsm,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingGsm>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingGsm::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pin_trampoline<P>(
    this: *mut ffi::NMSettingGsm,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingGsm>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingGsm::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pin_flags_trampoline<P>(
    this: *mut ffi::NMSettingGsm,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingGsm>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingGsm::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_sim_id_trampoline<P>(
    this: *mut ffi::NMSettingGsm,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingGsm>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingGsm::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_sim_operator_id_trampoline<P>(
    this: *mut ffi::NMSettingGsm,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingGsm>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingGsm::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_username_trampoline<P>(
    this: *mut ffi::NMSettingGsm,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingGsm>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingGsm::from_glib_borrow(this).downcast_unchecked())
}
