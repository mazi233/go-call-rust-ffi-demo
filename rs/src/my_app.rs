use std::mem::ManuallyDrop;

pub fn my_app_simple_rust_func_called_from_go(arg1: u8, arg2: u16, arg3: u32) -> usize {
  arg1 as usize + arg2 as usize + arg3 as usize
}

pub fn my_app_receive_string_and_return_string(s: String) -> String {
  if s.len() > 15 {
    s[0..15].to_string()
  } else {
    s
  }
}

pub fn my_app_receive_str_and_return_string(s: &str) -> String {
  if s.len() > 15 {
    s[0..15].to_string()
  } else {
    s.to_string()
  }
}

pub fn my_app_receive_str_and_return_str(s: &str) -> &str {
  if s.len() > 15 {
    &s[0..15]
  } else {
    s
  }
}

pub fn my_app_receive_string_and_return_str<'a>(s: String) -> (&'a str, *const u8, usize, usize) {
  let my_slice = unsafe {if s.len() > 15 {
    &*(&s[0..15] as &str as *const str)
  } else {
    &*(&s as &str as *const str)
  }};

  let s = ManuallyDrop::new(s);
  (my_slice, s.as_ptr(), s.len(), s.capacity())
}
