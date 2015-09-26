extern crate xiapi_sys;
extern crate libc;
extern crate num;

use xiapi_sys::xiapi::*;
use super::constants::xiReturn;
//use super::types::*;
use num::FromPrimitive;

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

/*
    u32xiapi_sys::xiapi::xiGetDeviceInfo(DevId: DWORD, prm: *const ::libc::c_char,
                           val: *mut ::libc::c_void, size: *mut DWORD,
                           _type: *mut XI_PRM_TYPE) -> XI_RETURN;
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
    u32xiapi_sys::xiapi::xiGetDeviceInfoInt(DevId: DWORD, prm: *const ::libc::c_char,
                              value: *mut ::libc::c_int) -> XI_RETURN;
    u32xiapi_sys::xiapi::xiGetDeviceInfoString(DevId: DWORD, prm: *const ::libc::c_char,
                                 value: *mut ::libc::c_char,
                                 value_size: DWORD) -> XI_RETURN;
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
