// This file was generated by gir (https://github.com/gtk-rs/gir @ 464833e)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use glib::StaticType;
use glib::Type;
use gobject_ffi;

bitflags! {
    pub struct _80211ApFlags: u32 {
        const NONE = 0;
        const PRIVACY = 1;
        const WPS = 2;
        const WPS_PBC = 4;
        const WPS_PIN = 8;
    }
}

#[doc(hidden)]
impl ToGlib for _80211ApFlags {
    type GlibType = ffi::NM80211ApFlags;

    fn to_glib(&self) -> ffi::NM80211ApFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NM80211ApFlags> for _80211ApFlags {
    fn from_glib(value: ffi::NM80211ApFlags) -> _80211ApFlags {
        _80211ApFlags::from_bits_truncate(value)
    }
}

impl StaticType for _80211ApFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nm_802_11_ap_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for _80211ApFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for _80211ApFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for _80211ApFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct _80211ApSecurityFlags: u32 {
        const NONE = 0;
        const PAIR_WEP40 = 1;
        const PAIR_WEP104 = 2;
        const PAIR_TKIP = 4;
        const PAIR_CCMP = 8;
        const GROUP_WEP40 = 16;
        const GROUP_WEP104 = 32;
        const GROUP_TKIP = 64;
        const GROUP_CCMP = 128;
        const KEY_MGMT_PSK = 256;
        const KEY_MGMT_802_1X = 512;
    }
}

#[doc(hidden)]
impl ToGlib for _80211ApSecurityFlags {
    type GlibType = ffi::NM80211ApSecurityFlags;

    fn to_glib(&self) -> ffi::NM80211ApSecurityFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NM80211ApSecurityFlags> for _80211ApSecurityFlags {
    fn from_glib(value: ffi::NM80211ApSecurityFlags) -> _80211ApSecurityFlags {
        _80211ApSecurityFlags::from_bits_truncate(value)
    }
}

impl StaticType for _80211ApSecurityFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nm_802_11_ap_security_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for _80211ApSecurityFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for _80211ApSecurityFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for _80211ApSecurityFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
bitflags! {
    pub struct ActivationStateFlags: u32 {
        const NONE = 0;
        const IS_MASTER = 1;
        const IS_SLAVE = 2;
        const LAYER2_READY = 4;
        const IP4_READY = 8;
        const IP6_READY = 16;
        const MASTER_HAS_SLAVES = 32;
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for ActivationStateFlags {
    type GlibType = ffi::NMActivationStateFlags;

    fn to_glib(&self) -> ffi::NMActivationStateFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<ffi::NMActivationStateFlags> for ActivationStateFlags {
    fn from_glib(value: ffi::NMActivationStateFlags) -> ActivationStateFlags {
        ActivationStateFlags::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
impl StaticType for ActivationStateFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nm_activation_state_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
impl<'a> FromValueOptional<'a> for ActivationStateFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
impl<'a> FromValue<'a> for ActivationStateFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
impl SetValue for ActivationStateFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v1_4", feature = "dox"))]
bitflags! {
    pub struct CheckpointCreateFlags: u32 {
        const NONE = 0;
        const DESTROY_ALL = 1;
        const DELETE_NEW_CONNECTIONS = 2;
        const DISCONNECT_NEW_DEVICES = 4;
        const ALLOW_OVERLAPPING = 8;
    }
}

#[cfg(any(feature = "v1_4", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for CheckpointCreateFlags {
    type GlibType = ffi::NMCheckpointCreateFlags;

    fn to_glib(&self) -> ffi::NMCheckpointCreateFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v1_4", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<ffi::NMCheckpointCreateFlags> for CheckpointCreateFlags {
    fn from_glib(value: ffi::NMCheckpointCreateFlags) -> CheckpointCreateFlags {
        CheckpointCreateFlags::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_4", feature = "dox"))]
impl StaticType for CheckpointCreateFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nm_checkpoint_create_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_4", feature = "dox"))]
impl<'a> FromValueOptional<'a> for CheckpointCreateFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v1_4", feature = "dox"))]
impl<'a> FromValue<'a> for CheckpointCreateFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_4", feature = "dox"))]
impl SetValue for CheckpointCreateFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct ConnectionSerializationFlags: u32 {
        const ALL = 0;
        const NO_SECRETS = 1;
        const ONLY_SECRETS = 2;
    }
}

#[doc(hidden)]
impl ToGlib for ConnectionSerializationFlags {
    type GlibType = ffi::NMConnectionSerializationFlags;

    fn to_glib(&self) -> ffi::NMConnectionSerializationFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NMConnectionSerializationFlags> for ConnectionSerializationFlags {
    fn from_glib(value: ffi::NMConnectionSerializationFlags) -> ConnectionSerializationFlags {
        ConnectionSerializationFlags::from_bits_truncate(value)
    }
}

impl StaticType for ConnectionSerializationFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nm_connection_serialization_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ConnectionSerializationFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ConnectionSerializationFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for ConnectionSerializationFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct DeviceCapabilities: u32 {
        const NONE = 0;
        const NM_SUPPORTED = 1;
        const CARRIER_DETECT = 2;
        const IS_SOFTWARE = 4;
        const SRIOV = 8;
    }
}

#[doc(hidden)]
impl ToGlib for DeviceCapabilities {
    type GlibType = ffi::NMDeviceCapabilities;

    fn to_glib(&self) -> ffi::NMDeviceCapabilities {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NMDeviceCapabilities> for DeviceCapabilities {
    fn from_glib(value: ffi::NMDeviceCapabilities) -> DeviceCapabilities {
        DeviceCapabilities::from_bits_truncate(value)
    }
}

impl StaticType for DeviceCapabilities {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nm_device_capabilities_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for DeviceCapabilities {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for DeviceCapabilities {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for DeviceCapabilities {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct DeviceWifiCapabilities: u32 {
        const NONE = 0;
        const CIPHER_WEP40 = 1;
        const CIPHER_WEP104 = 2;
        const CIPHER_TKIP = 4;
        const CIPHER_CCMP = 8;
        const WPA = 16;
        const RSN = 32;
        const AP = 64;
        const ADHOC = 128;
        const FREQ_VALID = 256;
        const FREQ_2GHZ = 512;
        const FREQ_5GHZ = 1024;
    }
}

#[doc(hidden)]
impl ToGlib for DeviceWifiCapabilities {
    type GlibType = ffi::NMDeviceWifiCapabilities;

    fn to_glib(&self) -> ffi::NMDeviceWifiCapabilities {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NMDeviceWifiCapabilities> for DeviceWifiCapabilities {
    fn from_glib(value: ffi::NMDeviceWifiCapabilities) -> DeviceWifiCapabilities {
        DeviceWifiCapabilities::from_bits_truncate(value)
    }
}

impl StaticType for DeviceWifiCapabilities {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nm_device_wifi_capabilities_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for DeviceWifiCapabilities {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for DeviceWifiCapabilities {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for DeviceWifiCapabilities {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct IPTunnelFlags: u32 {
        const NONE = 0;
        const IP6_IGN_ENCAP_LIMIT = 1;
        const IP6_USE_ORIG_TCLASS = 2;
        const IP6_USE_ORIG_FLOWLABEL = 4;
        const IP6_MIP6_DEV = 8;
        const IP6_RCV_DSCP_COPY = 16;
        const IP6_USE_ORIG_FWMARK = 32;
    }
}

#[doc(hidden)]
impl ToGlib for IPTunnelFlags {
    type GlibType = ffi::NMIPTunnelFlags;

    fn to_glib(&self) -> ffi::NMIPTunnelFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NMIPTunnelFlags> for IPTunnelFlags {
    fn from_glib(value: ffi::NMIPTunnelFlags) -> IPTunnelFlags {
        IPTunnelFlags::from_bits_truncate(value)
    }
}

impl StaticType for IPTunnelFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nm_ip_tunnel_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for IPTunnelFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for IPTunnelFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for IPTunnelFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v1_8", feature = "dox"))]
bitflags! {
    pub struct Setting8021xAuthFlags: u32 {
        const NONE = 0;
        const TLS_1_0_DISABLE = 1;
        const TLS_1_1_DISABLE = 2;
        const TLS_1_2_DISABLE = 4;
        const ALL = 7;
    }
}

#[cfg(any(feature = "v1_8", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for Setting8021xAuthFlags {
    type GlibType = ffi::NMSetting8021xAuthFlags;

    fn to_glib(&self) -> ffi::NMSetting8021xAuthFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v1_8", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<ffi::NMSetting8021xAuthFlags> for Setting8021xAuthFlags {
    fn from_glib(value: ffi::NMSetting8021xAuthFlags) -> Setting8021xAuthFlags {
        Setting8021xAuthFlags::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_8", feature = "dox"))]
impl StaticType for Setting8021xAuthFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nm_setting_802_1x_auth_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_8", feature = "dox"))]
impl<'a> FromValueOptional<'a> for Setting8021xAuthFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v1_8", feature = "dox"))]
impl<'a> FromValue<'a> for Setting8021xAuthFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_8", feature = "dox"))]
impl SetValue for Setting8021xAuthFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct SettingDcbFlags: u32 {
        const NONE = 0;
        const ENABLE = 1;
        const ADVERTISE = 2;
        const WILLING = 4;
    }
}

#[doc(hidden)]
impl ToGlib for SettingDcbFlags {
    type GlibType = ffi::NMSettingDcbFlags;

    fn to_glib(&self) -> ffi::NMSettingDcbFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NMSettingDcbFlags> for SettingDcbFlags {
    fn from_glib(value: ffi::NMSettingDcbFlags) -> SettingDcbFlags {
        SettingDcbFlags::from_bits_truncate(value)
    }
}

impl StaticType for SettingDcbFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nm_setting_dcb_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SettingDcbFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SettingDcbFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for SettingDcbFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct SettingSecretFlags: u32 {
        const NONE = 0;
        const AGENT_OWNED = 1;
        const NOT_SAVED = 2;
        const NOT_REQUIRED = 4;
    }
}

#[doc(hidden)]
impl ToGlib for SettingSecretFlags {
    type GlibType = ffi::NMSettingSecretFlags;

    fn to_glib(&self) -> ffi::NMSettingSecretFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NMSettingSecretFlags> for SettingSecretFlags {
    fn from_glib(value: ffi::NMSettingSecretFlags) -> SettingSecretFlags {
        SettingSecretFlags::from_bits_truncate(value)
    }
}

impl StaticType for SettingSecretFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nm_setting_secret_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SettingSecretFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SettingSecretFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for SettingSecretFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct SettingWiredWakeOnLan: u32 {
        const PHY = 2;
        const UNICAST = 4;
        const MULTICAST = 8;
        const BROADCAST = 16;
        const ARP = 32;
        const MAGIC = 64;
        const DEFAULT = 1;
        const IGNORE = 32768;
    }
}

#[doc(hidden)]
impl ToGlib for SettingWiredWakeOnLan {
    type GlibType = ffi::NMSettingWiredWakeOnLan;

    fn to_glib(&self) -> ffi::NMSettingWiredWakeOnLan {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NMSettingWiredWakeOnLan> for SettingWiredWakeOnLan {
    fn from_glib(value: ffi::NMSettingWiredWakeOnLan) -> SettingWiredWakeOnLan {
        SettingWiredWakeOnLan::from_bits_truncate(value)
    }
}

impl StaticType for SettingWiredWakeOnLan {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nm_setting_wired_wake_on_lan_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SettingWiredWakeOnLan {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SettingWiredWakeOnLan {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for SettingWiredWakeOnLan {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
bitflags! {
    pub struct SettingWirelessWakeOnWLan: u32 {
        const ANY = 2;
        const DISCONNECT = 4;
        const MAGIC = 8;
        const GTK_REKEY_FAILURE = 16;
        const EAP_IDENTITY_REQUEST = 32;
        const 4WAY_HANDSHAKE = 64;
        const RFKILL_RELEASE = 128;
        const TCP = 256;
        const DEFAULT = 1;
        const IGNORE = 32768;
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for SettingWirelessWakeOnWLan {
    type GlibType = ffi::NMSettingWirelessWakeOnWLan;

    fn to_glib(&self) -> ffi::NMSettingWirelessWakeOnWLan {
        self.bits()
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<ffi::NMSettingWirelessWakeOnWLan> for SettingWirelessWakeOnWLan {
    fn from_glib(value: ffi::NMSettingWirelessWakeOnWLan) -> SettingWirelessWakeOnWLan {
        SettingWirelessWakeOnWLan::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl StaticType for SettingWirelessWakeOnWLan {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nm_setting_wireless_wake_on_wlan_get_type()) }
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl<'a> FromValueOptional<'a> for SettingWirelessWakeOnWLan {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl<'a> FromValue<'a> for SettingWirelessWakeOnWLan {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl SetValue for SettingWirelessWakeOnWLan {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
bitflags! {
    pub struct SettingsConnectionFlags: u32 {
        const NONE = 0;
        const UNSAVED = 1;
        const NM_GENERATED = 2;
        const VOLATILE = 4;
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for SettingsConnectionFlags {
    type GlibType = ffi::NMSettingsConnectionFlags;

    fn to_glib(&self) -> ffi::NMSettingsConnectionFlags {
        self.bits()
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<ffi::NMSettingsConnectionFlags> for SettingsConnectionFlags {
    fn from_glib(value: ffi::NMSettingsConnectionFlags) -> SettingsConnectionFlags {
        SettingsConnectionFlags::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl StaticType for SettingsConnectionFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nm_settings_connection_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl<'a> FromValueOptional<'a> for SettingsConnectionFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl<'a> FromValue<'a> for SettingsConnectionFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl SetValue for SettingsConnectionFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
bitflags! {
    pub struct SettingsUpdate2Flags: u32 {
        const NONE = 0;
        const TO_DISK = 1;
        const IN_MEMORY = 2;
        const IN_MEMORY_DETACHED = 4;
        const IN_MEMORY_ONLY = 8;
        const VOLATILE = 16;
        const BLOCK_AUTOCONNECT = 32;
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for SettingsUpdate2Flags {
    type GlibType = ffi::NMSettingsUpdate2Flags;

    fn to_glib(&self) -> ffi::NMSettingsUpdate2Flags {
        self.bits()
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<ffi::NMSettingsUpdate2Flags> for SettingsUpdate2Flags {
    fn from_glib(value: ffi::NMSettingsUpdate2Flags) -> SettingsUpdate2Flags {
        SettingsUpdate2Flags::from_bits_truncate(value)
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl StaticType for SettingsUpdate2Flags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nm_settings_update2_flags_get_type()) }
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl<'a> FromValueOptional<'a> for SettingsUpdate2Flags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl<'a> FromValue<'a> for SettingsUpdate2Flags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl SetValue for SettingsUpdate2Flags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct TeamLinkWatcherArpPingFlags: u32 {
        const VALIDATE_ACTIVE = 2;
        const VALIDATE_INACTIVE = 4;
        const SEND_ALWAYS = 8;
    }
}

#[doc(hidden)]
impl ToGlib for TeamLinkWatcherArpPingFlags {
    type GlibType = ffi::NMTeamLinkWatcherArpPingFlags;

    fn to_glib(&self) -> ffi::NMTeamLinkWatcherArpPingFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NMTeamLinkWatcherArpPingFlags> for TeamLinkWatcherArpPingFlags {
    fn from_glib(value: ffi::NMTeamLinkWatcherArpPingFlags) -> TeamLinkWatcherArpPingFlags {
        TeamLinkWatcherArpPingFlags::from_bits_truncate(value)
    }
}

impl StaticType for TeamLinkWatcherArpPingFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nm_team_link_watcher_arp_ping_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for TeamLinkWatcherArpPingFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for TeamLinkWatcherArpPingFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for TeamLinkWatcherArpPingFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct VlanFlags: u32 {
        const REORDER_HEADERS = 1;
        const GVRP = 2;
        const LOOSE_BINDING = 4;
        const MVRP = 8;
    }
}

#[doc(hidden)]
impl ToGlib for VlanFlags {
    type GlibType = ffi::NMVlanFlags;

    fn to_glib(&self) -> ffi::NMVlanFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::NMVlanFlags> for VlanFlags {
    fn from_glib(value: ffi::NMVlanFlags) -> VlanFlags {
        VlanFlags::from_bits_truncate(value)
    }
}

impl StaticType for VlanFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::nm_vlan_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for VlanFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for VlanFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for VlanFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}
