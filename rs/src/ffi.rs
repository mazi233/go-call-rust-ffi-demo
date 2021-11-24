use std::{ffi::{CStr, CString}, os::raw::c_char};

use crate::my_app;

#[no_mangle]
pub fn simple_rust_func_called_from_go(arg1: u8, arg2: u16, arg3: u32) -> usize {
  my_app::my_app_simple_rust_func_called_from_go(arg1, arg2, arg3) as usize
}

#[no_mangle]
pub fn receive_str_and_return_string(s: *const c_char) -> *const c_char {
  let cstr = {
    assert!(!s.is_null());
    unsafe{CStr::from_ptr(s)}
  };

  let rstr = cstr.to_str().expect("not valid utf-8 string");

  let ret = my_app::my_app_receive_str_and_return_string(rstr);

  let c_ret = CString::new(ret).expect("null byte in the middle");

  c_ret.into_raw()
}

#[no_mangle]
pub fn receive_string_and_return_string(s: *const c_char) -> *const c_char {
  let cstr = {
    assert!(!s.is_null());
    unsafe{CStr::from_ptr(s)}
  };

  let r_string = cstr.to_str().expect("not valid utf-8 string").to_string();

  let ret = my_app::my_app_receive_string_and_return_string(r_string);

  let c_ret = CString::new(ret).expect("null byte in the middle");

  c_ret.into_raw()
}

#[no_mangle]
pub fn receive_str_and_return_str(s: *const c_char) -> *const c_char {
  let cstr = {
    assert!(!s.is_null());
    unsafe{CStr::from_ptr(s)}
  };

  let rstr = cstr.to_str().expect("not valid utf-8 string");

  let ret = my_app::my_app_receive_str_and_return_str(rstr);

  let c_ret = CString::new(ret).expect("null byte in the middle");

  c_ret.into_raw()
}

#[no_mangle]
pub fn receive_string_and_return_str(s: *const c_char, new_ptr: *mut *const c_char, c_origin_ptr: *mut *const c_char, len: *mut usize, cap: *mut usize) {
  let cstr = {
    assert!(!s.is_null());
    unsafe{CStr::from_ptr(s)}
  };

  let r_string = cstr.to_str().expect("not valid utf-8 string").to_string();
  
  let (ret, t_c_origin_ptr, t_len, t_cap) = my_app::my_app_receive_string_and_return_str(r_string);

  let c_ret = CString::new(ret).expect("null byte in the middle");
  unsafe {
    *new_ptr = c_ret.into_raw();
    *c_origin_ptr = t_c_origin_ptr as *const i8;
    *len = t_len;
    *cap = t_cap;
  }
}

#[no_mangle]
pub unsafe fn free_string_alloc_by_rust_by_raw_parts(s: *const c_char, len: usize, cap: usize) {
  String::from_raw_parts(s as *mut u8, len, cap);
}

#[no_mangle]
pub unsafe fn free_cstring_alloc_by_rust(s: *mut c_char) {
  CString::from_raw(s);
}

#[no_mangle]
pub fn receive_str_and_return_str_no_copy(s: *const c_char, new_ptr: *mut *const c_char, len: *mut usize) {
  let cstr = {
    assert!(!s.is_null());
    unsafe{CStr::from_ptr(s)}
  };

  let rstr = cstr.to_str().expect("not valid utf-8 string");
  let ret = my_app::my_app_receive_str_and_return_str(rstr);
  let c_ret = ret.as_ptr();

  unsafe {
    *new_ptr = c_ret as *const i8;
    *len = ret.len();
  }
}
