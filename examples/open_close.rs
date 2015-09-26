extern crate xiapi;

fn run() -> Result<(),String> {

    let num_devices = xiapi::get_num_devices();

    println!("{} devices found", num_devices);

    // let mut numDevices = 0u32;
    // unsafe {
    //
    //     let result = xiapi::xiGetNumberDevices(&mut numDevices);
    //     println!("xiGetNumberDevices:\treturn: {}\tvalue: {}", result, numDevices);
    //     if(numDevices == 0){
    //         return Err(String::from("no devices listed"));
    //     }

    let device_index = 0;

    let mut vid = xiapi::start_capture(device_index);

    //     let wIndex = 0;
    //
    //     let handle: xiapi::PHANDLE = libc::malloc(mem::size_of::<xiapi::PHANDLE>() as libc::size_t) as xiapi::PHANDLE;
    //     let open_result = xiapi::xiOpenDevice( wIndex, handle);
    //
    //     if (open_result != 0) {
    //         return Err(String::from("Open XI_DEVICE failed"));
    //     }
    //
    //     let mut width: i32 = 0;
    //     let mut height: i32 = 0;
    //     let mut data_format: i32 = 0;
    //
    //     // always use auto exposure/gain
    //     let mut mvret = xiapi::xiSetParamInt( *handle, CString::new(xiapi::XI_PRM_AEAG).unwrap().as_ptr(), 1);
    //     println!("set param result {}", mvret);
    //
    //     // always use auto white balance for color cameras
    //     mvret = xiapi::xiSetParamInt( *handle, CString::new(xiapi::XI_PRM_AUTO_WB).unwrap().as_ptr(), 1);
    //     println!("set param result {}", mvret);
    //
    //     mvret = xiapi::xiGetParamInt( *handle, CString::new(xiapi::XI_PRM_WIDTH).unwrap().as_ptr(), &mut width);
    //     println!("get param result {}", mvret);
    //
    //     mvret = xiapi::xiGetParamInt( *handle, CString::new(xiapi::XI_PRM_HEIGHT).unwrap().as_ptr(), &mut height);
    //     println!("get param result {}", mvret);
    //
    //     mvret = xiapi::xiGetParamInt(*handle, CString::new(xiapi::XI_PRM_IMAGE_DATA_FORMAT).unwrap().as_ptr(), &mut
    //     data_format);
    //     println!("get param result {}", mvret);
    //
    //     mvret = xiapi::xiSetParamInt( *handle, CString::new(xiapi::XI_PRM_BUFFER_POLICY).unwrap().as_ptr(), 1);
    //     println!("set param result {}", mvret);
    //
    //     //mvret = xiSetParamInt( handle, XI_PRM_ACQ_TRANSPORT_BUFFER_SIZE, 96560128);
    //     //HandleXiResult(mvret);
    //
    //     //mvret = xiSetParamInt( handle, XI_PRM_RECENT_FRAME, 0);
    //     //HandleXiResult(mvret);
    //
    //     //default capture timeout 10s
    //     let timeout = 10000;

    //
    // loop {
    //     let mut frame = core::mat();
    //     try!(cam.read(&mut frame));
    //     if try!(frame.size()).width > 0 {
    //         try!(highgui::imshow(window, &mut frame));
    //     }
    //     if try!(highgui::waitKey(10)) > 0 {
    //         break;
    //     }
    // }
    Ok(())
}

fn main() {
    run().unwrap()
}
