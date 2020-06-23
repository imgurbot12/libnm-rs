// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib::translate::*;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use glib::GString;
use nm_sys;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use std::ptr;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use crate::SettingCompareFlags;
#[cfg(any(feature = "v1_16", feature = "dox"))]
use crate::SettingSecretFlags;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct WireGuardPeer(Shared<nm_sys::NMWireGuardPeer>);

    match fn {
        ref => |ptr| nm_sys::nm_wireguard_peer_ref(ptr),
        unref => |ptr| nm_sys::nm_wireguard_peer_unref(ptr),
        get_type => || nm_sys::nm_wireguard_peer_get_type(),
    }
}

impl WireGuardPeer {
    ///
    /// Feature: `v1_16`
    ///
    ///
    /// # Returns
    ///
    /// a new, default, unsealed `WireGuardPeer` instance.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn new() -> WireGuardPeer {
        unsafe { from_glib_full(nm_sys::nm_wireguard_peer_new()) }
    }

    /// Appends `allowed_ip` setting to the list. This does not check
    /// for duplicates and always appends `allowed_ip` to the end of the
    /// list. If `allowed_ip` is valid, it will be normalized and a modified
    /// for might be appended. If `allowed_ip` is invalid, it will still be
    /// appended, but later verification will fail.
    ///
    /// It is a bug trying to modify a sealed `WireGuardPeer` instance.
    ///
    /// Feature: `v1_16`
    ///
    /// ## `allowed_ip`
    /// the allowed-ip entry to set.
    /// ## `accept_invalid`
    /// if `true`, also invalid `allowed_ip` value
    ///  will be appended. Otherwise, the function does nothing
    ///  in face of invalid values and returns `false`.
    ///
    /// # Returns
    ///
    /// `true` if the value is a valid allowed-ips value, `false` otherwise.
    ///  Depending on `accept_invalid`, also invalid values are added.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn append_allowed_ip(&self, allowed_ip: &str, accept_invalid: bool) -> bool {
        unsafe {
            from_glib(nm_sys::nm_wireguard_peer_append_allowed_ip(
                self.to_glib_none().0,
                allowed_ip.to_glib_none().0,
                accept_invalid.to_glib(),
            ))
        }
    }

    /// Removes all allowed-ip entries.
    ///
    /// It is a bug trying to modify a sealed `WireGuardPeer` instance.
    ///
    /// Feature: `v1_16`
    ///
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn clear_allowed_ips(&self) {
        unsafe {
            nm_sys::nm_wireguard_peer_clear_allowed_ips(self.to_glib_none().0);
        }
    }

    ///
    /// Feature: `v1_16`
    ///
    /// ## `b`
    /// the other `WireGuardPeer` to compare.
    /// ## `compare_flags`
    /// `SettingCompareFlags` to affect the comparison.
    ///
    /// # Returns
    ///
    /// zero of the two instances are equivalent or
    ///  a non-zero integer otherwise. This defines a total ordering
    ///  over the peers. Whether a peer is sealed or not, does not
    ///  affect the comparison.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn cmp(&self, b: Option<&WireGuardPeer>, compare_flags: SettingCompareFlags) -> i32 {
        unsafe {
            nm_sys::nm_wireguard_peer_cmp(
                self.to_glib_none().0,
                b.to_glib_none().0,
                compare_flags.to_glib(),
            )
        }
    }

    ///
    /// Feature: `v1_16`
    ///
    ///
    /// # Returns
    ///
    /// the number of allowed-ips entries.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_allowed_ips_len(&self) -> u32 {
        unsafe { nm_sys::nm_wireguard_peer_get_allowed_ips_len(self.to_glib_none().0) }
    }

    ///
    /// Feature: `v1_16`
    ///
    ///
    /// # Returns
    ///
    /// the endpoint or `None` if none was set.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_endpoint(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_wireguard_peer_get_endpoint(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// Feature: `v1_16`
    ///
    ///
    /// # Returns
    ///
    /// get the persistent-keepalive setting in seconds. Set to zero to disable
    ///  keep-alive.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_persistent_keepalive(&self) -> u16 {
        unsafe { nm_sys::nm_wireguard_peer_get_persistent_keepalive(self.to_glib_none().0) }
    }

    ///
    /// Feature: `v1_16`
    ///
    ///
    /// # Returns
    ///
    /// the preshared key or `None` if unset.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_preshared_key(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_wireguard_peer_get_preshared_key(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// Feature: `v1_16`
    ///
    ///
    /// # Returns
    ///
    /// get the secret flags for the preshared-key.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_preshared_key_flags(&self) -> SettingSecretFlags {
        unsafe {
            from_glib(nm_sys::nm_wireguard_peer_get_preshared_key_flags(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// Feature: `v1_16`
    ///
    ///
    /// # Returns
    ///
    /// the public key or `None` if unset.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn get_public_key(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_wireguard_peer_get_public_key(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// Feature: `v1_16`
    ///
    ///
    /// # Returns
    ///
    /// whether `self` is sealed or not.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn is_sealed(&self) -> bool {
        unsafe { from_glib(nm_sys::nm_wireguard_peer_is_sealed(self.to_glib_none().0)) }
    }

    ///
    /// Feature: `v1_16`
    ///
    /// ## `check_non_secrets`
    /// if `true`, secret properties are validated.
    ///  Otherwise they are ignored for this purpose.
    /// ## `check_secrets`
    /// if `true`, non-secret properties are validated.
    ///  Otherwise they are ignored for this purpose.
    ///
    /// # Returns
    ///
    /// `true` if the peer is valid or fails with an error
    ///  reason.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn is_valid(
        &self,
        check_non_secrets: bool,
        check_secrets: bool,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = nm_sys::nm_wireguard_peer_is_valid(
                self.to_glib_none().0,
                check_non_secrets.to_glib(),
                check_secrets.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    ///
    /// Feature: `v1_16`
    ///
    /// ## `with_secrets`
    /// if `true`, the preshared-key secrets are copied
    ///  as well. Otherwise, they will be removed.
    ///
    /// # Returns
    ///
    /// a clone of `self`. This instance
    ///  is always unsealed.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn new_clone(&self, with_secrets: bool) -> Option<WireGuardPeer> {
        unsafe {
            from_glib_full(nm_sys::nm_wireguard_peer_new_clone(
                self.to_glib_none().0,
                with_secrets.to_glib(),
            ))
        }
    }

    /// Removes the allowed-ip at the given `idx`. This shifts all
    /// following entries one index down.
    ///
    /// It is a bug trying to modify a sealed `WireGuardPeer` instance.
    ///
    /// Feature: `v1_16`
    ///
    /// ## `idx`
    /// the index from zero to (allowed-ips-len - 1) to
    ///  retrieve. If the index is out of range, `false` is returned
    ///  and nothing is done.
    ///
    /// # Returns
    ///
    /// `true` if `idx` was valid and the allowed-ip was removed.
    ///  `false` otherwise, and the peer will not be changed.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn remove_allowed_ip(&self, idx: u32) -> bool {
        unsafe {
            from_glib(nm_sys::nm_wireguard_peer_remove_allowed_ip(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    /// Seal the `WireGuardPeer` instance. Afterwards, it is a bug
    /// to call all functions that modify the instance (except ref/unref).
    /// A sealed instance cannot be unsealed again, but you can create
    /// an unsealed copy with `WireGuardPeer::new_clone`.
    ///
    /// Feature: `v1_16`
    ///
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn seal(&self) {
        unsafe {
            nm_sys::nm_wireguard_peer_seal(self.to_glib_none().0);
        }
    }

    /// Sets or clears the endpoint of `self`.
    ///
    /// It is a bug trying to modify a sealed `WireGuardPeer` instance.
    ///
    /// Feature: `v1_16`
    ///
    /// ## `endpoint`
    /// the socket address endpoint to set or `None`.
    /// ## `allow_invalid`
    /// if `true`, also invalid values are set.
    ///  If `false`, the function does nothing for invalid `endpoint`
    ///  arguments.
    ///
    /// # Returns
    ///
    /// `true` if the endpoint is `None` or valid. For an
    ///  invalid `endpoint` argument, `false` is returned. Depending
    ///  on `allow_invalid`, the instance will be modified.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_endpoint(&self, endpoint: &str, allow_invalid: bool) -> bool {
        unsafe {
            from_glib(nm_sys::nm_wireguard_peer_set_endpoint(
                self.to_glib_none().0,
                endpoint.to_glib_none().0,
                allow_invalid.to_glib(),
            ))
        }
    }

    /// It is a bug trying to modify a sealed `WireGuardPeer` instance.
    ///
    /// Feature: `v1_16`
    ///
    /// ## `persistent_keepalive`
    /// the keep-alive value to set.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_persistent_keepalive(&self, persistent_keepalive: u16) {
        unsafe {
            nm_sys::nm_wireguard_peer_set_persistent_keepalive(
                self.to_glib_none().0,
                persistent_keepalive,
            );
        }
    }

    /// Reset the preshared key. Note that if the preshared key is valid, it
    /// will be normalized (which may or may not modify the set value).
    ///
    /// Note that the preshared-key is a secret and consequently has corresponding
    /// preshared-key-flags property. This is so that secrets can be optional
    /// and requested on demand from a secret-agent. Also, an invalid preshared-key
    /// may optionally cause `WireGuardPeer::is_valid` to fail or it may
    /// be accepted.
    ///
    /// It is a bug trying to modify a sealed `WireGuardPeer` instance.
    ///
    /// Feature: `v1_16`
    ///
    /// ## `preshared_key`
    /// the new preshared
    ///  key or `None` to clear the preshared key.
    /// ## `accept_invalid`
    /// whether to allow setting the key to an invalid
    ///  value. If `false`, `self` is unchanged if the key is invalid
    ///  and if `false` is returned.
    ///
    /// # Returns
    ///
    /// `true` if the preshared-key is valid, otherwise `false`.
    ///  `None` is considered a valid value.
    ///  If the key is invalid, it depends on `accept_invalid` whether the
    ///  previous value was reset.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_preshared_key(&self, preshared_key: Option<&str>, accept_invalid: bool) -> bool {
        unsafe {
            from_glib(nm_sys::nm_wireguard_peer_set_preshared_key(
                self.to_glib_none().0,
                preshared_key.to_glib_none().0,
                accept_invalid.to_glib(),
            ))
        }
    }

    /// It is a bug trying to modify a sealed `WireGuardPeer` instance.
    ///
    /// Feature: `v1_16`
    ///
    /// ## `preshared_key_flags`
    /// the secret flags to set.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_preshared_key_flags(&self, preshared_key_flags: SettingSecretFlags) {
        unsafe {
            nm_sys::nm_wireguard_peer_set_preshared_key_flags(
                self.to_glib_none().0,
                preshared_key_flags.to_glib(),
            );
        }
    }

    /// Reset the public key. Note that if the public key is valid, it
    /// will be normalized (which may or may not modify the set value).
    ///
    /// It is a bug trying to modify a sealed `WireGuardPeer` instance.
    ///
    /// Feature: `v1_16`
    ///
    /// ## `public_key`
    /// the new public
    ///  key or `None` to clear the public key.
    /// ## `accept_invalid`
    /// if `true` and `public_key` is not `None` and
    ///  invalid, then do not modify the instance.
    ///
    /// # Returns
    ///
    /// `true` if the key was valid or `None`. Returns
    ///  `false` for invalid keys. Depending on `accept_invalid`
    ///  will an invalid key be set or not.
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    pub fn set_public_key(&self, public_key: Option<&str>, accept_invalid: bool) -> bool {
        unsafe {
            from_glib(nm_sys::nm_wireguard_peer_set_public_key(
                self.to_glib_none().0,
                public_key.to_glib_none().0,
                accept_invalid.to_glib(),
            ))
        }
    }
}

#[cfg(any(feature = "v1_16", feature = "dox"))]
impl Default for WireGuardPeer {
    fn default() -> Self {
        Self::new()
    }
}
