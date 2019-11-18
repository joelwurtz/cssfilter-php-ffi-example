extern crate libc;
extern crate scraper;

use std::intrinsics::transmute;

#[no_mangle]
pub extern "C" fn cssfilter(html_cstr: *const libc::c_char, filter_cstr: *const libc::c_char) -> *const libc::c_char {
    let html = cstr_to_str(html_cstr);
    let filter = cstr_to_str(filter_cstr);

    let document = scraper::Html::parse_document(html);
    let selector = scraper::Selector::parse(filter).unwrap();

    str_to_cstr(document.select(&selector).next().unwrap().html())
}

fn cstr_to_str(cstr: *const libc::c_char) -> &'static str {
    unsafe {
        let cstring = std::ffi::CStr::from_ptr(cstr);
        let result = cstring.to_str();

        if result.is_err() {
            panic!(
                "Unable to create string for '{}': {}",
                String::from_utf8_lossy(cstring.to_bytes()),
                result.err().unwrap()
            );
        }

        result.unwrap()
    }
}

fn str_to_cstr(str: String) -> *const libc::c_char {
    unsafe {
        let string_result = std::ffi::CString::new(str.as_bytes());

        if string_result.is_err() {
            panic!(
                "Cannot create c string {}: {}",
                str,
                string_result.err().unwrap()
            );
        }

        let data: *const std::ffi::CString = transmute(Box::new(string_result.unwrap()));

        (&*data).as_ptr()
    }
}