// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Connection;
use crate::Object;
use crate::_80211ApFlags;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMWifiP2PPeer")]
    pub struct WifiP2PPeer(Object<ffi::NMWifiP2PPeer, ffi::NMWifiP2PPeerClass>) @extends Object;

    match fn {
        type_ => || ffi::nm_wifi_p2p_peer_get_type(),
    }
}

impl WifiP2PPeer {
    /// Validates a given connection against a given Wi-Fi P2P peer to ensure that
    /// the connection may be activated with that peer. The connection must match the
    /// `self`'s address and in the future possibly other attributes.
    /// ## `connection`
    /// an [`Connection`][crate::Connection] to validate against `self`
    ///
    /// # Returns
    ///
    /// [`true`] if the connection may be activated with this Wi-Fi P2P Peer,
    /// [`false`] if it cannot be.
    #[doc(alias = "nm_wifi_p2p_peer_connection_valid")]
    pub fn connection_valid(&self, connection: &impl IsA<Connection>) -> bool {
        unsafe {
            from_glib(ffi::nm_wifi_p2p_peer_connection_valid(
                self.to_glib_none().0,
                connection.as_ref().to_glib_none().0,
            ))
        }
    }

    /// Filters a given array of connections for a given [`WifiP2PPeer`][crate::WifiP2PPeer] object and
    /// returns connections which may be activated with the P2P peer. Any
    /// returned connections will match the `peers`'s HW address and in the future
    /// possibly other attributes.
    ///
    /// To obtain the list of connections that are compatible with this P2P peer,
    /// use [`Client::connections()`][crate::Client::connections()] and then filter the returned list for a given
    /// [`Device`][crate::Device] using [`DeviceExt::filter_connections()`][crate::prelude::DeviceExt::filter_connections()] and finally filter that list
    /// with this function.
    /// ## `connections`
    /// an array of `NMConnections` to
    /// filter
    ///
    /// # Returns
    ///
    /// an array of
    /// `NMConnections` that could be activated with the given `self`. The array should
    /// be freed with `g_ptr_array_unref()` when it is no longer required.
    #[doc(alias = "nm_wifi_p2p_peer_filter_connections")]
    pub fn filter_connections(&self, connections: &[Connection]) -> Vec<Connection> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::nm_wifi_p2p_peer_filter_connections(
                self.to_glib_none().0,
                connections.to_glib_none().0,
            ))
        }
    }

    /// Gets the flags of the P2P peer.
    ///
    /// # Returns
    ///
    /// the flags
    #[doc(alias = "nm_wifi_p2p_peer_get_flags")]
    #[doc(alias = "get_flags")]
    pub fn flags(&self) -> _80211ApFlags {
        unsafe { from_glib(ffi::nm_wifi_p2p_peer_get_flags(self.to_glib_none().0)) }
    }

    /// Gets the hardware address of the P2P peer.
    ///
    /// # Returns
    ///
    /// the hardware address
    #[doc(alias = "nm_wifi_p2p_peer_get_hw_address")]
    #[doc(alias = "get_hw_address")]
    pub fn hw_address(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_wifi_p2p_peer_get_hw_address(self.to_glib_none().0)) }
    }

    /// Returns the timestamp (in CLOCK_BOOTTIME seconds) for the last time the
    /// P2P peer was seen. A value of -1 means the P2P peer has never been seen.
    ///
    /// # Returns
    ///
    /// the last seen time in seconds
    #[doc(alias = "nm_wifi_p2p_peer_get_last_seen")]
    #[doc(alias = "get_last_seen")]
    pub fn last_seen(&self) -> i32 {
        unsafe { ffi::nm_wifi_p2p_peer_get_last_seen(self.to_glib_none().0) }
    }

    /// Gets the manufacturer of the P2P peer.
    ///
    /// # Returns
    ///
    /// the manufacturer
    #[doc(alias = "nm_wifi_p2p_peer_get_manufacturer")]
    #[doc(alias = "get_manufacturer")]
    pub fn manufacturer(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_wifi_p2p_peer_get_manufacturer(
                self.to_glib_none().0,
            ))
        }
    }

    /// Gets the model of the P2P peer.
    ///
    /// # Returns
    ///
    /// the model
    #[doc(alias = "nm_wifi_p2p_peer_get_model")]
    #[doc(alias = "get_model")]
    pub fn model(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_wifi_p2p_peer_get_model(self.to_glib_none().0)) }
    }

    /// Gets the model number of the P2P peer.
    ///
    /// # Returns
    ///
    /// the model number
    #[doc(alias = "nm_wifi_p2p_peer_get_model_number")]
    #[doc(alias = "get_model_number")]
    pub fn model_number(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_wifi_p2p_peer_get_model_number(
                self.to_glib_none().0,
            ))
        }
    }

    /// Gets the name of the P2P peer.
    ///
    /// # Returns
    ///
    /// the name
    #[doc(alias = "nm_wifi_p2p_peer_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_wifi_p2p_peer_get_name(self.to_glib_none().0)) }
    }

    /// Gets the serial number of the P2P peer.
    ///
    /// # Returns
    ///
    /// the serial number
    #[doc(alias = "nm_wifi_p2p_peer_get_serial")]
    #[doc(alias = "get_serial")]
    pub fn serial(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_wifi_p2p_peer_get_serial(self.to_glib_none().0)) }
    }

    /// Gets the current signal strength of the P2P peer as a percentage.
    ///
    /// # Returns
    ///
    /// the signal strength (0 to 100)
    #[doc(alias = "nm_wifi_p2p_peer_get_strength")]
    #[doc(alias = "get_strength")]
    pub fn strength(&self) -> u8 {
        unsafe { ffi::nm_wifi_p2p_peer_get_strength(self.to_glib_none().0) }
    }

    /// Gets the WFD information elements of the P2P peer.
    ///
    /// # Returns
    ///
    /// the [`glib::Bytes`][crate::glib::Bytes] containing the WFD IEs, or [`None`].
    #[doc(alias = "nm_wifi_p2p_peer_get_wfd_ies")]
    #[doc(alias = "get_wfd_ies")]
    pub fn wfd_ies(&self) -> Option<glib::Bytes> {
        unsafe { from_glib_none(ffi::nm_wifi_p2p_peer_get_wfd_ies(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "flags")]
    pub fn connect_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<F: Fn(&WifiP2PPeer) + 'static>(
            this: *mut ffi::NMWifiP2PPeer,
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
                b"notify::flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_flags_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "hw-address")]
    pub fn connect_hw_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_hw_address_trampoline<F: Fn(&WifiP2PPeer) + 'static>(
            this: *mut ffi::NMWifiP2PPeer,
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
                b"notify::hw-address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_hw_address_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "last-seen")]
    pub fn connect_last_seen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_last_seen_trampoline<F: Fn(&WifiP2PPeer) + 'static>(
            this: *mut ffi::NMWifiP2PPeer,
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
                b"notify::last-seen\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_last_seen_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "manufacturer")]
    pub fn connect_manufacturer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_manufacturer_trampoline<F: Fn(&WifiP2PPeer) + 'static>(
            this: *mut ffi::NMWifiP2PPeer,
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
                b"notify::manufacturer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_manufacturer_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "model")]
    pub fn connect_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&WifiP2PPeer) + 'static>(
            this: *mut ffi::NMWifiP2PPeer,
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
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "model-number")]
    pub fn connect_model_number_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_number_trampoline<F: Fn(&WifiP2PPeer) + 'static>(
            this: *mut ffi::NMWifiP2PPeer,
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
                b"notify::model-number\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_number_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "name")]
    pub fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<F: Fn(&WifiP2PPeer) + 'static>(
            this: *mut ffi::NMWifiP2PPeer,
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
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "serial")]
    pub fn connect_serial_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_serial_trampoline<F: Fn(&WifiP2PPeer) + 'static>(
            this: *mut ffi::NMWifiP2PPeer,
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
                b"notify::serial\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_serial_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "strength")]
    pub fn connect_strength_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_strength_trampoline<F: Fn(&WifiP2PPeer) + 'static>(
            this: *mut ffi::NMWifiP2PPeer,
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
                b"notify::strength\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_strength_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    #[doc(alias = "wfd-ies")]
    pub fn connect_wfd_ies_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_wfd_ies_trampoline<F: Fn(&WifiP2PPeer) + 'static>(
            this: *mut ffi::NMWifiP2PPeer,
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
                b"notify::wfd-ies\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_wfd_ies_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for WifiP2PPeer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WifiP2PPeer")
    }
}
