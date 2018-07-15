#![crate_name = "enclave"]
#![crate_type = "staticlib"]

#![cfg_attr(all(not(test), not(target_env = "sgx")), no_std)]
//#![cfg_attr(not(target_env = "sgx"), no_std)]
//#![cfg_attr(target_env = "sgx", feature(rustc_private))]
//#![feature(alloc)]
//
extern crate sgx_types;
//// #[cfg(not(test))]
//#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
//extern crate serde;
////#[cfg(test)]
//#[macro_use]
extern crate serde_json;
//
//#[macro_use]
//extern crate serde_derive;
//
use sgx_types::*;
use std::string::String;
use std::slice;
//
//#[no_mangle]
//pub extern "C" fn sgx_http_get(url_ptr: *const u8, url_len: usize) -> sgx_status_t {
//    let url = from_sgx_string(url_ptr, url_len);
//
//    println!("Performing HTTP GET from within enclave with {:?}", url);
//    sgx_status_t::SGX_SUCCESS
//}
//
//#[no_mangle]
//pub extern "C" fn sgx_http_post(url_ptr: *const u8, url_len: usize, body_ptr: *const u8, body_len: usize) -> sgx_status_t {
//    let url = from_sgx_string(url_ptr, url_len);
//    let body = from_sgx_string(body_ptr, body_len);
//
//    println!("Performing HTTP POST from within enclave with {:?}: {:?}", url, body);
//    sgx_status_t::SGX_SUCCESS
//}
//
#[no_mangle]
pub extern "C" fn sgx_multiply(adapter_str_ptr: *const u8, adapter_str_len: usize, input_str_ptr: *const u8, input_str_len: usize) -> sgx_status_t {
    let adapter_str = from_sgx_string(adapter_str_ptr, adapter_str_len);
    let adapter: serde_json::Value = match serde_json::from_str(&adapter_str) {
        Ok(result) => result,
        Err(_err) => return sgx_status_t::SGX_ERROR_INVALID_PARAMETER,
    };
//
//    let input_str = from_sgx_string(input_str_ptr, input_str_len);
//    let input = match parse_run_result_json(&input_str) {
//        Ok(result) => result,
//        Err(_err) => return sgx_status_t::SGX_ERROR_INVALID_PARAMETER,
//    };
//
//    if let serde_json::Value::String(multiplier) = &adapter["times"] {
//        if let serde_json::Value::String(multiplicand) = &input.data["values"] {
//            println!("Performing MULTIPLY from within enclave with {:?} * {:?} = {:?}",
//                     multiplicand, multiplier, multiply(&multiplicand, &multiplier));
//        }
//    }
//
    sgx_status_t::SGX_SUCCESS
}
//
fn from_sgx_string(ptr: *const u8, len: usize) -> String {
    let slice = unsafe { slice::from_raw_parts(ptr, len) };
    String::from_utf8(slice.to_vec()).unwrap()
}
//
//fn multiply(multiplicand_str: &str, multiplier_str: &str) -> Result<String, std::num::ParseIntError> {
//    let multiplicand = match i128::from_str_radix(multiplicand_str, 10) {
//        Ok(result) => result,
//        Err(err) => return Err(err),
//    };
//    let multiplier = match i128::from_str_radix(multiplier_str, 10) {
//        Ok(result) => result,
//        Err(err) => return Err(err),
//    };
//
//    Ok(format!("{:?}", multiplicand * multiplier))
//}
//
//fn parse_run_result_json(input: &str) -> Result<RunResult, serde_json::Error> {
//    let result: RunResult = serde_json::from_str(input)?;
//    Ok(result)
//}
//
//#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
//#[serde(rename_all = "camelCase")]
//struct RunResult {
//    job_run_id: String,
//    data: serde_json::Value,
//    status: Option<String>,
//    error_message: Option<String>,
//    amount: Option<u64>,
//}
//
//#[cfg(test)]
//mod test {
//    use std::ffi::CString;
//    use std::default::Default;
//    use *;
//
//    #[test]
//    fn test_from_sgx_string() {
//      assert_eq!(from_sgx_string(CString::new("hello world!").unwrap().as_ptr() as *const u8, 12), "hello world!");
//    }
//
//    #[test]
//    fn test_multiply() {
//      assert_eq!(multiply("1", "1"), Ok("1".to_string()));
//      assert_eq!(multiply("13", "19"), Ok("247".to_string()));
//
//      assert!(multiply("", "19").is_err());
//    }
//
//    #[test]
//    fn test_parse_run_result_json() {
//      assert_eq!(parse_run_result_json(r#"{"jobRunId": "abcd", "data":{}}"#).unwrap(), RunResult{
//          job_run_id: "abcd".to_string(),
//          data: json!({}),
//          ..Default::default()
//      });
//
//      assert_eq!(parse_run_result_json(r#"{"jobRunId": "abcd", "data": {"a": "b"}}"#).unwrap(), RunResult{
//          job_run_id: "abcd".to_string(),
//          data: json!({"a": "b"}),
//          ..Default::default()
//      });
//
//      // invalid JSON
//      assert!(parse_run_result_json(r#"{"jobRunId": "invalid", "data":{}"#).is_err());
//      // no job run ID
//      assert!(parse_run_result_json(r#"{}, "data":{}"#).is_err());
//      // wrong type
//      assert!(parse_run_result_json(r#"{"jobRunId": 2, "data":{}}"#).is_err());
//    }
//}
