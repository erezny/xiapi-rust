#![allow(non_snake_case)]
extern crate xiapi_sys;
extern crate libc;

use super::constants::*;
use std::vec::Vec;

#[derive(Copy, PartialEq)]
pub struct ImgDesc {
    pub Area0Left: u32,
    pub Area1Left: u32,
    pub Area2Left: u32,
    pub Area3Left: u32,
    pub Area4Left: u32,
    pub Area5Left: u32,
    pub ActiveAreaWidth: u32,
    pub Area5Right: u32,
    pub Area4Right: u32,
    pub Area3Right: u32,
    pub Area2Right: u32,
    pub Area1Right: u32,
    pub Area0Right: u32,
    pub Area0Top: u32,
    pub Area1Top: u32,
    pub Area2Top: u32,
    pub Area3Top: u32,
    pub Area4Top: u32,
    pub Area5Top: u32,
    pub ActiveAreaHeight: u32,
    pub Area5Bottom: u32,
    pub Area4Bottom: u32,
    pub Area3Bottom: u32,
    pub Area2Bottom: u32,
    pub Area1Bottom: u32,
    pub Area0Bottom: u32,
    pub format: u32,
    pub flags: u32,
}
impl ::std::clone::Clone for ImgDesc {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for ImgDesc {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

pub struct Img {
    pub bp: Vec<u8>,
    pub frm: ImgFormat,
    pub width: u32,
    pub height: u32,
    pub nframe: u32,
    pub tsSec: u32,
    pub tsUSec: u32,
    pub GPI_level: u32,
    pub black_level: u32,
    pub padding_x: u32,
    pub AbsoluteOffsetX: u32,
    pub AbsoluteOffsetY: u32,
    pub transport_frm: GenTLImageFormat,
    pub img_desc: ImgDesc,
}
// impl ::std::clone::Clone for Img {
//     fn clone(&self) -> Self {  }
// }
// impl ::std::default::Default for Img {
//     fn default() -> Self { unsafe { ::std::mem::zeroed() } }
// }
