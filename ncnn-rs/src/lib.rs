mod allocator;
mod datareader;
mod extractor;
mod mat;
mod net;
mod option;
mod layer;

pub use allocator::*;
pub use datareader::*;
pub use extractor::*;
pub use mat::*;
pub use net::*;
pub use option::*;
pub use layer::*;

pub use ncnn_bind as ncnn;

use std::ffi::CStr;

pub fn version_tag() -> u64 {
    ncnn::NCNN_TAG
}

pub fn version_string() -> &'static str {
    let c_buf = unsafe { ncnn::ncnn_version() };
    let c_str = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    str_slice
}
