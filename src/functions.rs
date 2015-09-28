extern crate xiapi_sys;
extern crate libc;
extern crate num;

use xiapi_sys::xiapi::*;
use super::constants::{
    xiReturn,
    Parameter,
};
//use super::types::*;
use num::FromPrimitive;
use std::mem;
use libc::c_char;
use std::ffi::{
    CStr,
    CString,
};
use std::convert::Into;

pub fn get_num_devices() -> u32 {
    let mut num_devices = 0u32;
    unsafe {
        match xiReturn::from_i32(xiapi_sys::xiapi::xiGetNumberDevices(&mut num_devices)).unwrap()
        {
            xiReturn::OK => {
                return num_devices;
            }
            _ => {
                panic!("getNumberDevices failed");
            }
        }
    }

}

//This function queries the device without opening it.
// DeviceName, // Return device name
// DeviceType, // Return device type
// DeviceSN, // Return device serial number
// DeviceInstancePath, // Return device system instance path.
// Returns None on incorrect parameter
pub fn get_device_info_string(device_id: u32, param: Parameter) -> Option<String> {
    match param {
        Parameter::DeviceName |
        Parameter::DeviceType |
        Parameter::DeviceSN |
        Parameter::DeviceInstancePath => {
            //continue
        }
        _ => { //all other params invalid
            return None;
        }
    }
    unsafe {
        let mut buf: [c_char; 82] = mem::zeroed();
        match xiReturn::from_i32(xiapi_sys::xiapi::xiGetDeviceInfoString(device_id as DWORD,
                param.into(), buf.as_mut_ptr(), 80)).unwrap()
        {
            xiReturn::OK => {
                //turn c_char into String
                //likely a better way from [i8;80] rather than through a string
                let mut new_buf = Vec::<u8>::from_raw_parts(mem::transmute(buf.as_mut_ptr()),82, 82);
                //trim {
                for i in 0..new_buf.len() {
                    if new_buf[i] == 0 {
                        new_buf.truncate(i);
                        break;
                    }
                }
                //}

                match CString::new(new_buf){
                    Ok(my_cstring) => {
                        return Some(String::from_utf8_lossy(my_cstring.to_bytes()).into_owned());
                    },
                    Err(e) => {
                        println!("Error: {}", e);
                        return None;
                        },
                }

            }
            _ => {
                panic!("getDeviceInfo failed");
            }
        }
    }

}

/*
    u32xiapi_sys::xiapi::xiOpenDevice(DevId: DWORD, hDevice: PHANDLE) -> XI_RETURN;
    u32xiapi_sys::xiapi::xiOpenDeviceBy(sel: XI_OPEN_BY, prm: *const ::libc::c_char,
                          hDevice: PHANDLE) -> XI_RETURN;
    u32xiapi_sys::xiapi::xiCloseDevice(hDevice: HANDLE) -> XI_RETURN;
    u32xiapi_sys::xiapi::xiStartAcquisition(hDevice: HANDLE) -> XI_RETURN;
    u32xiapi_sys::xiapi::xiStopAcquisition(hDevice: HANDLE) -> XI_RETURN;
    u32xiapi_sys::xiapi::xiGetImage(hDevice: HANDLE, timeout: DWORD, img: LPXI_IMG)
     -> XI_RETURN;
    u32xiapi_sys::xiapi::xiSetParam(hDevice: HANDLE, prm: *const ::libc::c_char,
                      val: *mut ::libc::c_void, size: DWORD,
                      _type: XI_PRM_TYPE) -> XI_RETURN;
    u32xiapi_sys::xiapi::xiGetParam(hDevice: HANDLE, prm: *const ::libc::c_char,
                      val: *mut ::libc::c_void, size: *mut DWORD,
                      _type: *mut XI_PRM_TYPE) -> XI_RETURN;
    u32xiapi_sys::xiapi::xiSetParamInt(hDevice: HANDLE, prm: *const ::libc::c_char,
                         val: ::libc::c_int) -> XI_RETURN;
    u32xiapi_sys::xiapi::xiSetParamFloat(hDevice: HANDLE, prm: *const ::libc::c_char,
                           val: ::libc::c_float) -> XI_RETURN;
    u32xiapi_sys::xiapi::xiSetParamString(hDevice: HANDLE, prm: *const ::libc::c_char,
                            val: *mut ::libc::c_void, size: DWORD)
     -> XI_RETURN;
    u32xiapi_sys::xiapi::xiGetParamInt(hDevice: HANDLE, prm: *const ::libc::c_char,
                         val: *mut ::libc::c_int) -> XI_RETURN;
    u32xiapi_sys::xiapi::xiGetParamFloat(hDevice: HANDLE, prm: *const ::libc::c_char,
                           val: *mut ::libc::c_float) -> XI_RETURN;
    u32xiapi_sys::xiapi::xiGetParamString(hDevice: HANDLE, prm: *const ::libc::c_char,
                            val: *mut ::libc::c_void, size: DWORD)
     -> XI_RETURN;
    u32xiapi_sys::xiapi::xiProcOpen(processing_handle: *mut xiProcessingHandle_t)
     -> XI_RETURN;
    u32xiapi_sys::xiapi::xiProcSetParam(processing_handle: xiProcessingHandle_t,
                          prm: *const ::libc::c_char,
                          val: *mut ::libc::c_void, size: DWORD,
                          _type: XI_PRM_TYPE) -> XI_RETURN;
    u32xiapi_sys::xiapi::xiProcPushImage(processing_handle: xiProcessingHandle_t,
                           first_pixel: *mut ::libc::c_uchar) -> XI_RETURN;
    u32xiapi_sys::xiapi::xiProcPushXiImg(processing_handle: xiProcessingHandle_t,
                           image: *mut XI_IMG) -> XI_RETURN;
    u32xiapi_sys::xiapi::xiProcPullImage(processing_handle: xiProcessingHandle_t,
                           timeout_ms: ::libc::c_int, new_image: *mut XI_IMG)
     -> XI_RETURN;
    u32xiapi_sys::xiapi::xiProcClose(processing_handle: xiProcessingHandle_t) -> XI_RETURN;
*/
