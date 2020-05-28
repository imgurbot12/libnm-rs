// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use _80211ApFlags;
use _80211ApSecurityFlags;
use _80211Mode;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Setting;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use SettingMacRandomization;
use SettingWirelessSecurity;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use SettingWirelessWakeOnWLan;

glib_wrapper! {
    pub struct SettingWireless(Object<nm_sys::NMSettingWireless, nm_sys::NMSettingWirelessClass, SettingWirelessClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_wireless_get_type(),
    }
}

impl SettingWireless {
    pub fn new() -> SettingWireless {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_wireless_new()).unsafe_cast() }
    }
}

impl Default for SettingWireless {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SETTING_WIRELESS: Option<&SettingWireless> = None;

pub trait SettingWirelessExt: 'static {
    fn add_mac_blacklist_item(&self, mac: &str) -> bool;

    fn add_seen_bssid(&self, bssid: &str) -> bool;

    fn ap_security_compatible<P: IsA<SettingWirelessSecurity>>(
        &self,
        s_wireless_sec: &P,
        ap_flags: _80211ApFlags,
        ap_wpa: _80211ApSecurityFlags,
        ap_rsn: _80211ApSecurityFlags,
        ap_mode: _80211Mode,
    ) -> bool;

    fn clear_mac_blacklist_items(&self);

    fn get_band(&self) -> Option<GString>;

    fn get_bssid(&self) -> Option<GString>;

    fn get_channel(&self) -> u32;

    fn get_cloned_mac_address(&self) -> Option<GString>;

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_generate_mac_address_mask(&self) -> Option<GString>;

    fn get_hidden(&self) -> bool;

    fn get_mac_address(&self) -> Option<GString>;

    fn get_mac_address_blacklist(&self) -> Vec<GString>;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_mac_address_randomization(&self) -> SettingMacRandomization;

    fn get_mac_blacklist_item(&self, idx: u32) -> Option<GString>;

    fn get_mode(&self) -> Option<GString>;

    fn get_mtu(&self) -> u32;

    fn get_num_mac_blacklist_items(&self) -> u32;

    fn get_num_seen_bssids(&self) -> u32;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_powersave(&self) -> u32;

    fn get_rate(&self) -> u32;

    fn get_seen_bssid(&self, i: u32) -> Option<GString>;

    fn get_ssid(&self) -> Option<glib::Bytes>;

    fn get_tx_power(&self) -> u32;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_wake_on_wlan(&self) -> SettingWirelessWakeOnWLan;

    fn remove_mac_blacklist_item(&self, idx: u32);

    fn remove_mac_blacklist_item_by_value(&self, mac: &str) -> bool;

    fn set_property_band(&self, band: Option<&str>);

    fn set_property_bssid(&self, bssid: Option<&str>);

    fn set_property_channel(&self, channel: u32);

    fn set_property_cloned_mac_address(&self, cloned_mac_address: Option<&str>);

    fn get_property_generate_mac_address_mask(&self) -> Option<GString>;

    fn set_property_generate_mac_address_mask(&self, generate_mac_address_mask: Option<&str>);

    fn set_property_hidden(&self, hidden: bool);

    fn set_property_mac_address(&self, mac_address: Option<&str>);

    fn set_property_mac_address_blacklist(&self, mac_address_blacklist: &[&str]);

    #[cfg_attr(feature = "v1_4", deprecated)]
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_mac_address_randomization(&self, mac_address_randomization: u32);

    fn set_property_mode(&self, mode: Option<&str>);

    fn set_property_mtu(&self, mtu: u32);

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_powersave(&self, powersave: u32);

    fn set_property_rate(&self, rate: u32);

    fn get_property_seen_bssids(&self) -> Vec<GString>;

    fn set_property_seen_bssids(&self, seen_bssids: &[&str]);

    fn set_property_ssid(&self, ssid: Option<&glib::Bytes>);

    fn set_property_tx_power(&self, tx_power: u32);

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn set_property_wake_on_wlan(&self, wake_on_wlan: u32);

    fn connect_property_band_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_bssid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_channel_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cloned_mac_address_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_generate_mac_address_mask_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_hidden_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mac_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mac_address_blacklist_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg_attr(feature = "v1_4", deprecated)]
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_mac_address_randomization_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mtu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_powersave_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_seen_bssids_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ssid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tx_power_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_property_wake_on_wlan_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: IsA<SettingWireless>> SettingWirelessExt for O {
    fn add_mac_blacklist_item(&self, mac: &str) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_wireless_add_mac_blacklist_item(
                self.as_ref().to_glib_none().0,
                mac.to_glib_none().0,
            ))
        }
    }

    fn add_seen_bssid(&self, bssid: &str) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_wireless_add_seen_bssid(
                self.as_ref().to_glib_none().0,
                bssid.to_glib_none().0,
            ))
        }
    }

    fn ap_security_compatible<P: IsA<SettingWirelessSecurity>>(
        &self,
        s_wireless_sec: &P,
        ap_flags: _80211ApFlags,
        ap_wpa: _80211ApSecurityFlags,
        ap_rsn: _80211ApSecurityFlags,
        ap_mode: _80211Mode,
    ) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_wireless_ap_security_compatible(
                self.as_ref().to_glib_none().0,
                s_wireless_sec.as_ref().to_glib_none().0,
                ap_flags.to_glib(),
                ap_wpa.to_glib(),
                ap_rsn.to_glib(),
                ap_mode.to_glib(),
            ))
        }
    }

    fn clear_mac_blacklist_items(&self) {
        unsafe {
            nm_sys::nm_setting_wireless_clear_mac_blacklist_items(self.as_ref().to_glib_none().0);
        }
    }

    fn get_band(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_wireless_get_band(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_bssid(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_wireless_get_bssid(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_channel(&self) -> u32 {
        unsafe { nm_sys::nm_setting_wireless_get_channel(self.as_ref().to_glib_none().0) }
    }

    fn get_cloned_mac_address(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_wireless_get_cloned_mac_address(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_generate_mac_address_mask(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_wireless_get_generate_mac_address_mask(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_hidden(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_wireless_get_hidden(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_mac_address(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_wireless_get_mac_address(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_mac_address_blacklist(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(
                nm_sys::nm_setting_wireless_get_mac_address_blacklist(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_mac_address_randomization(&self) -> SettingMacRandomization {
        unsafe {
            from_glib(nm_sys::nm_setting_wireless_get_mac_address_randomization(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_mac_blacklist_item(&self, idx: u32) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_wireless_get_mac_blacklist_item(
                self.as_ref().to_glib_none().0,
                idx,
            ))
        }
    }

    fn get_mode(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_wireless_get_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_mtu(&self) -> u32 {
        unsafe { nm_sys::nm_setting_wireless_get_mtu(self.as_ref().to_glib_none().0) }
    }

    fn get_num_mac_blacklist_items(&self) -> u32 {
        unsafe {
            nm_sys::nm_setting_wireless_get_num_mac_blacklist_items(self.as_ref().to_glib_none().0)
        }
    }

    fn get_num_seen_bssids(&self) -> u32 {
        unsafe { nm_sys::nm_setting_wireless_get_num_seen_bssids(self.as_ref().to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_powersave(&self) -> u32 {
        unsafe { nm_sys::nm_setting_wireless_get_powersave(self.as_ref().to_glib_none().0) }
    }

    fn get_rate(&self) -> u32 {
        unsafe { nm_sys::nm_setting_wireless_get_rate(self.as_ref().to_glib_none().0) }
    }

    fn get_seen_bssid(&self, i: u32) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_wireless_get_seen_bssid(
                self.as_ref().to_glib_none().0,
                i,
            ))
        }
    }

    fn get_ssid(&self) -> Option<glib::Bytes> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_wireless_get_ssid(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_tx_power(&self) -> u32 {
        unsafe { nm_sys::nm_setting_wireless_get_tx_power(self.as_ref().to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_wake_on_wlan(&self) -> SettingWirelessWakeOnWLan {
        unsafe {
            from_glib(nm_sys::nm_setting_wireless_get_wake_on_wlan(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_mac_blacklist_item(&self, idx: u32) {
        unsafe {
            nm_sys::nm_setting_wireless_remove_mac_blacklist_item(
                self.as_ref().to_glib_none().0,
                idx,
            );
        }
    }

    fn remove_mac_blacklist_item_by_value(&self, mac: &str) -> bool {
        unsafe {
            from_glib(
                nm_sys::nm_setting_wireless_remove_mac_blacklist_item_by_value(
                    self.as_ref().to_glib_none().0,
                    mac.to_glib_none().0,
                ),
            )
        }
    }

    fn set_property_band(&self, band: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"band\0".as_ptr() as *const _,
                Value::from(band).to_glib_none().0,
            );
        }
    }

    fn set_property_bssid(&self, bssid: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"bssid\0".as_ptr() as *const _,
                Value::from(bssid).to_glib_none().0,
            );
        }
    }

    fn set_property_channel(&self, channel: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"channel\0".as_ptr() as *const _,
                Value::from(&channel).to_glib_none().0,
            );
        }
    }

    fn set_property_cloned_mac_address(&self, cloned_mac_address: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"cloned-mac-address\0".as_ptr() as *const _,
                Value::from(cloned_mac_address).to_glib_none().0,
            );
        }
    }

    fn get_property_generate_mac_address_mask(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"generate-mac-address-mask\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `generate-mac-address-mask` getter")
        }
    }

    fn set_property_generate_mac_address_mask(&self, generate_mac_address_mask: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"generate-mac-address-mask\0".as_ptr() as *const _,
                Value::from(generate_mac_address_mask).to_glib_none().0,
            );
        }
    }

    fn set_property_hidden(&self, hidden: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"hidden\0".as_ptr() as *const _,
                Value::from(&hidden).to_glib_none().0,
            );
        }
    }

    fn set_property_mac_address(&self, mac_address: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"mac-address\0".as_ptr() as *const _,
                Value::from(mac_address).to_glib_none().0,
            );
        }
    }

    fn set_property_mac_address_blacklist(&self, mac_address_blacklist: &[&str]) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"mac-address-blacklist\0".as_ptr() as *const _,
                Value::from(mac_address_blacklist).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_mac_address_randomization(&self, mac_address_randomization: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"mac-address-randomization\0".as_ptr() as *const _,
                Value::from(&mac_address_randomization).to_glib_none().0,
            );
        }
    }

    fn set_property_mode(&self, mode: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"mode\0".as_ptr() as *const _,
                Value::from(mode).to_glib_none().0,
            );
        }
    }

    fn set_property_mtu(&self, mtu: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"mtu\0".as_ptr() as *const _,
                Value::from(&mtu).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_powersave(&self, powersave: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"powersave\0".as_ptr() as *const _,
                Value::from(&powersave).to_glib_none().0,
            );
        }
    }

    fn set_property_rate(&self, rate: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"rate\0".as_ptr() as *const _,
                Value::from(&rate).to_glib_none().0,
            );
        }
    }

    fn get_property_seen_bssids(&self) -> Vec<GString> {
        unsafe {
            let mut value = Value::from_type(<Vec<GString> as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"seen-bssids\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `seen-bssids` getter")
                .unwrap()
        }
    }

    fn set_property_seen_bssids(&self, seen_bssids: &[&str]) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"seen-bssids\0".as_ptr() as *const _,
                Value::from(seen_bssids).to_glib_none().0,
            );
        }
    }

    fn set_property_ssid(&self, ssid: Option<&glib::Bytes>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"ssid\0".as_ptr() as *const _,
                Value::from(ssid).to_glib_none().0,
            );
        }
    }

    fn set_property_tx_power(&self, tx_power: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"tx-power\0".as_ptr() as *const _,
                Value::from(&tx_power).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn set_property_wake_on_wlan(&self, wake_on_wlan: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"wake-on-wlan\0".as_ptr() as *const _,
                Value::from(&wake_on_wlan).to_glib_none().0,
            );
        }
    }

    fn connect_property_band_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_band_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingWireless,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingWireless>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingWireless::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::band\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_band_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_bssid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_bssid_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingWireless,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingWireless>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingWireless::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::bssid\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_bssid_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_channel_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_channel_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingWireless,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingWireless>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingWireless::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::channel\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_channel_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_cloned_mac_address_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_cloned_mac_address_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingWireless,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingWireless>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingWireless::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::cloned-mac-address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_cloned_mac_address_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_generate_mac_address_mask_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_generate_mac_address_mask_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingWireless,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingWireless>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingWireless::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::generate-mac-address-mask\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_generate_mac_address_mask_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_hidden_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_hidden_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingWireless,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingWireless>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingWireless::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::hidden\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_hidden_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_mac_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mac_address_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingWireless,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingWireless>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingWireless::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mac-address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mac_address_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_mac_address_blacklist_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mac_address_blacklist_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingWireless,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingWireless>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingWireless::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mac-address-blacklist\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mac_address_blacklist_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_mac_address_randomization_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mac_address_randomization_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingWireless,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingWireless>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingWireless::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mac-address-randomization\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mac_address_randomization_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingWireless,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingWireless>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingWireless::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_mtu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mtu_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingWireless,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingWireless>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingWireless::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mtu\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mtu_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_powersave_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_powersave_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingWireless,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingWireless>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingWireless::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::powersave\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_powersave_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rate_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingWireless,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingWireless>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingWireless::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::rate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_rate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_seen_bssids_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_seen_bssids_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingWireless,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingWireless>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingWireless::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::seen-bssids\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_seen_bssids_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_ssid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ssid_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingWireless,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingWireless>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingWireless::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ssid\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ssid_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_tx_power_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tx_power_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingWireless,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingWireless>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingWireless::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tx-power\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tx_power_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_property_wake_on_wlan_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_wake_on_wlan_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingWireless,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingWireless>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingWireless::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::wake-on-wlan\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_wake_on_wlan_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SettingWireless {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingWireless")
    }
}