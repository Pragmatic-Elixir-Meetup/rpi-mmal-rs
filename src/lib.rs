#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::os::raw::c_uint;

#[macro_export]
macro_rules! mmal_fourcc {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        {
            (($a as c_uint) | (($b as c_uint) << 8) | (($c as c_uint) << 16) | (($d as c_uint) << 24)) as c_uint
        }
    }
}

pub const MMAL_ENCODING_I420: c_uint = mmal_fourcc!('I','4','2','0');
pub const MMAL_ENCODING_OPAQUE: c_uint = mmal_fourcc!('O','P','Q','V');
