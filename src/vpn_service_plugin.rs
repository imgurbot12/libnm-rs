// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib::StaticType;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib::Value;
use glib_sys;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use gobject_sys;
use libc;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use crate::VpnPluginFailure;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use crate::VpnServiceState;

glib_wrapper! {
    pub struct VpnServicePlugin(Object<nm_sys::NMVpnServicePlugin, nm_sys::NMVpnServicePluginClass, VpnServicePluginClass>);

    match fn {
        get_type => || nm_sys::nm_vpn_service_plugin_get_type(),
    }
}

impl VpnServicePlugin {
    //#[cfg(any(feature = "v1_2", feature = "dox"))]
    //pub fn get_secret_flags(data: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 25 }/TypeId { ns_id: 0, id: 25 }, secret_name: &str) -> Option<SettingSecretFlags> {
    //    unsafe { TODO: call nm_sys:nm_vpn_service_plugin_get_secret_flags() }
    //}

    //#[cfg(any(feature = "v1_2", feature = "dox"))]
    //pub fn read_vpn_details(fd: i32, out_data: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 25 }/TypeId { ns_id: 0, id: 25 }, out_secrets: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 25 }/TypeId { ns_id: 0, id: 25 }) -> bool {
    //    unsafe { TODO: call nm_sys:nm_vpn_service_plugin_read_vpn_details() }
    //}
}

pub const NONE_VPN_SERVICE_PLUGIN: Option<&VpnServicePlugin> = None;

/// Trait containing all `VpnServicePlugin` methods.
///
/// # Implementors
///
/// [`VpnServicePlugin`](struct.VpnServicePlugin.html)
pub trait VpnServicePluginExt: 'static {
    fn disconnect(&self) -> Result<(), glib::Error>;

    fn failure(&self, reason: VpnPluginFailure);

    fn set_config(&self, config: &glib::Variant);

    fn set_ip4_config(&self, ip4_config: &glib::Variant);

    fn set_ip6_config(&self, ip6_config: &glib::Variant);

    fn set_login_banner(&self, banner: &str);

    /// Shutdown the `self` and disconnect from D-Bus. After this,
    /// the plugin instance is dead and should no longer be used.
    /// It ensures to get no more requests from D-Bus. In principle,
    /// you don't need to shutdown the plugin, disposing the instance
    /// has the same effect. However, this gives a way to deactivate
    /// the plugin before giving up the last reference.
    ///
    /// Feature: `v1_12`
    ///
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn shutdown(&self);

    /// The D-Bus service name of this plugin.
    ///
    /// Feature: `v1_2`
    ///
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_property_service_name(&self) -> Option<GString>;

    /// The state of the plugin.
    ///
    /// Feature: `v1_2`
    ///
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_property_state(&self) -> VpnServiceState;

    /// The state of the plugin.
    ///
    /// Feature: `v1_2`
    ///
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_state(&self, state: VpnServiceState);

    /// Whether to watch for D-Bus peer's changes.
    ///
    /// Feature: `v1_2`
    ///
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_property_watch_peer(&self) -> bool;

    fn connect_config<F: Fn(&Self, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_failure<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_ip4_config<F: Fn(&Self, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_ip6_config<F: Fn(&Self, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_login_banner<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_quit<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    //fn connect_secrets_required<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_state_changed<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<VpnServicePlugin>> VpnServicePluginExt for O {
    fn disconnect(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = nm_sys::nm_vpn_service_plugin_disconnect(
                self.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn failure(&self, reason: VpnPluginFailure) {
        unsafe {
            nm_sys::nm_vpn_service_plugin_failure(self.as_ref().to_glib_none().0, reason.to_glib());
        }
    }

    fn set_config(&self, config: &glib::Variant) {
        unsafe {
            nm_sys::nm_vpn_service_plugin_set_config(
                self.as_ref().to_glib_none().0,
                config.to_glib_none().0,
            );
        }
    }

    fn set_ip4_config(&self, ip4_config: &glib::Variant) {
        unsafe {
            nm_sys::nm_vpn_service_plugin_set_ip4_config(
                self.as_ref().to_glib_none().0,
                ip4_config.to_glib_none().0,
            );
        }
    }

    fn set_ip6_config(&self, ip6_config: &glib::Variant) {
        unsafe {
            nm_sys::nm_vpn_service_plugin_set_ip6_config(
                self.as_ref().to_glib_none().0,
                ip6_config.to_glib_none().0,
            );
        }
    }

    fn set_login_banner(&self, banner: &str) {
        unsafe {
            nm_sys::nm_vpn_service_plugin_set_login_banner(
                self.as_ref().to_glib_none().0,
                banner.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn shutdown(&self) {
        unsafe {
            nm_sys::nm_vpn_service_plugin_shutdown(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_property_service_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"service-name\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `service-name` getter")
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_property_state(&self) -> VpnServiceState {
        unsafe {
            let mut value = Value::from_type(<VpnServiceState as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"state\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `state` getter")
                .unwrap()
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_state(&self, state: VpnServiceState) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"state\0".as_ptr() as *const _,
                Value::from(&state).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_property_watch_peer(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"watch-peer\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `watch-peer` getter")
                .unwrap()
        }
    }

    fn connect_config<F: Fn(&Self, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn config_trampoline<P, F: Fn(&P, &glib::Variant) + 'static>(
            this: *mut nm_sys::NMVpnServicePlugin,
            object: *mut glib_sys::GVariant,
            f: glib_sys::gpointer,
        ) where
            P: IsA<VpnServicePlugin>,
        {
            let f: &F = &*(f as *const F);
            f(
                &VpnServicePlugin::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"config\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    config_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_failure<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn failure_trampoline<P, F: Fn(&P, u32) + 'static>(
            this: *mut nm_sys::NMVpnServicePlugin,
            object: libc::c_uint,
            f: glib_sys::gpointer,
        ) where
            P: IsA<VpnServicePlugin>,
        {
            let f: &F = &*(f as *const F);
            f(
                &VpnServicePlugin::from_glib_borrow(this).unsafe_cast_ref(),
                object,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"failure\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    failure_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_ip4_config<F: Fn(&Self, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn ip4_config_trampoline<P, F: Fn(&P, &glib::Variant) + 'static>(
            this: *mut nm_sys::NMVpnServicePlugin,
            object: *mut glib_sys::GVariant,
            f: glib_sys::gpointer,
        ) where
            P: IsA<VpnServicePlugin>,
        {
            let f: &F = &*(f as *const F);
            f(
                &VpnServicePlugin::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"ip4-config\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    ip4_config_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_ip6_config<F: Fn(&Self, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn ip6_config_trampoline<P, F: Fn(&P, &glib::Variant) + 'static>(
            this: *mut nm_sys::NMVpnServicePlugin,
            object: *mut glib_sys::GVariant,
            f: glib_sys::gpointer,
        ) where
            P: IsA<VpnServicePlugin>,
        {
            let f: &F = &*(f as *const F);
            f(
                &VpnServicePlugin::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"ip6-config\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    ip6_config_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_login_banner<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn login_banner_trampoline<P, F: Fn(&P, &str) + 'static>(
            this: *mut nm_sys::NMVpnServicePlugin,
            object: *mut libc::c_char,
            f: glib_sys::gpointer,
        ) where
            P: IsA<VpnServicePlugin>,
        {
            let f: &F = &*(f as *const F);
            f(
                &VpnServicePlugin::from_glib_borrow(this).unsafe_cast_ref(),
                &GString::from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"login-banner\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    login_banner_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_quit<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn quit_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMVpnServicePlugin,
            f: glib_sys::gpointer,
        ) where
            P: IsA<VpnServicePlugin>,
        {
            let f: &F = &*(f as *const F);
            f(&VpnServicePlugin::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"quit\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    quit_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    //fn connect_secrets_required<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Empty ctype p0: *.CArray TypeId { ns_id: 0, id: 28 }
    //}

    fn connect_state_changed<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn state_changed_trampoline<P, F: Fn(&P, u32) + 'static>(
            this: *mut nm_sys::NMVpnServicePlugin,
            object: libc::c_uint,
            f: glib_sys::gpointer,
        ) where
            P: IsA<VpnServicePlugin>,
        {
            let f: &F = &*(f as *const F);
            f(
                &VpnServicePlugin::from_glib_borrow(this).unsafe_cast_ref(),
                object,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"state-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    state_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMVpnServicePlugin,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<VpnServicePlugin>,
        {
            let f: &F = &*(f as *const F);
            f(&VpnServicePlugin::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::state\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_state_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for VpnServicePlugin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VpnServicePlugin")
    }
}
