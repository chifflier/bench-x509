use std;
use std::os::raw::c_void;
use x509_parser::{parse_x509_der, X509Certificate};

extern "C" {
    fn DecodeDer(buffer: *const u8, size: u32, errcode: *mut u32) -> *mut c_void;
    fn DerFree(cert: *mut c_void);
}

pub fn decode_x509_c(input: &[u8]) -> *mut c_void {
    let mut errcode: u32 = 0;
    let size = input.len() as u32;
    let buffer = input.as_ptr();

    let ret = unsafe { DecodeDer(buffer, size, &mut errcode) };
    if errcode != 0 {
        std::ptr::null_mut()
    } else {
        ret
    }
}

pub fn free_x509_c(cert: *mut c_void) {
    unsafe {
        DerFree(cert);
    }
}

pub fn decode_x509_rs(input: &[u8]) -> Option<X509Certificate> {
    let res = parse_x509_der(input);
    res.map(|(_, cert)| cert).ok()
}
