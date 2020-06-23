// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use nm_sys;
use std::fmt;
use crate::Device;
use crate::Object;

glib_wrapper! {
    pub struct DeviceDummy(Object<nm_sys::NMDeviceDummy, nm_sys::NMDeviceDummyClass, DeviceDummyClass>) @extends Device, Object;

    match fn {
        get_type => || nm_sys::nm_device_dummy_get_type(),
    }
}

impl DeviceDummy {}

impl fmt::Display for DeviceDummy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceDummy")
    }
}
