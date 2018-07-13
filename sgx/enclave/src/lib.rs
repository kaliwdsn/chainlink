#![crate_name = "enclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[macro_use]
extern crate sgx_tstd as std;

use sgx_types::*;
use std::string::String;
use std::slice;

#[no_mangle]
pub extern "C" fn sgx_http_get(url_ptr: *const u8, url_len: usize) -> sgx_status_t {
    let url = from_sgx_string(url_ptr, url_len);

    println!("Performing HTTP GET from within enclave with {:?}", url);
    sgx_status_t::SGX_SUCCESS
}

#[no_mangle]
pub extern "C" fn sgx_http_post(url_ptr: *const u8, url_len: usize, body_ptr: *const u8, body_len: usize) -> sgx_status_t {
    let url = from_sgx_string(url_ptr, url_len);
    let body = from_sgx_string(body_ptr, body_len);

    println!("Performing HTTP POST from within enclave with {:?}: {:?}", url, body);
    sgx_status_t::SGX_SUCCESS
}

#[no_mangle]
pub extern "C" fn sgx_multiply(multiplicand_ptr: *const u8, multiplicand_len: usize, multiplier_ptr: *const u8, multiplier_len: usize) -> sgx_status_t {
    let multiplicand = from_sgx_string(multiplicand_ptr, multiplicand_len);
    let multiplier = from_sgx_string(multiplier_ptr, multiplier_len);

    println!("Performing MULTIPLY from within enclave with {:?} {:?}", multiplicand, multiplier);
    sgx_status_t::SGX_SUCCESS
}

fn from_sgx_string(ptr: *const u8, len: usize) -> String {
    let slice = unsafe { slice::from_raw_parts(ptr, len) };
    String::from_utf8(slice.to_vec()).unwrap()
}
