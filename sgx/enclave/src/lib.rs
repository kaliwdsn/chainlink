#![crate_name = "enclave"]
#![crate_type = "staticlib"]

#![cfg_attr(all(not(test), not(target_env = "sgx")), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]
#![feature(alloc)]

extern crate sgx_types;
#[cfg(not(test))]
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

    println!("Performing MULTIPLY from within enclave with {:?} * {:?} = {:?}",
             multiplicand, multiplier, multiply(&multiplicand, &multiplier));
    sgx_status_t::SGX_SUCCESS
}

fn from_sgx_string(ptr: *const u8, len: usize) -> String {
    let slice = unsafe { slice::from_raw_parts(ptr, len) };
    String::from_utf8(slice.to_vec()).unwrap()
}

fn multiply(multiplicand_str: &str, multiplier_str: &str) -> Result<String, std::num::ParseIntError> {
    let multiplicand = match i128::from_str_radix(multiplicand_str, 10) {
        Ok(a) => a,
        Err(err) => return Err(err)
    };
    let multiplier = match i128::from_str_radix(multiplier_str, 10) {
        Ok(a) => a,
        Err(err) => return Err(err)
    };

    Ok(format!("{:?}", multiplicand * multiplier))
}

#[cfg(test)]
mod test {
    use std::ffi::CString;
    use *;

    #[test]
    fn test_from_sgx_string() {
      assert_eq!(from_sgx_string(CString::new("hello world!").unwrap().as_ptr() as *const u8, 12), "hello world!");
    }

    #[test]
    fn test_multiply() {
      assert_eq!(multiply("1", "1"), Ok("1".to_string()));
      assert_eq!(multiply("13", "19"), Ok("247".to_string()));

      assert!(multiply("", "19").is_err());
    }
}
