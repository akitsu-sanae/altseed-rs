/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/altseed-rs
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/


#[link(name = "user32")]
extern {}

use std::os::raw::c_char;
use std::os::raw::c_int;

pub enum AsdTextureObject2dImpl {}
pub enum AsdTexture2dImpl {}

#[link(name = "asd-c")]
extern {
    pub fn asd_initialize(title: *const c_char, width: c_int, height: c_int) -> bool;
    pub fn asd_do_events() -> bool;
    pub fn asd_update();
    pub fn asd_terminate();

    pub fn asd_texture_object2d_create() -> *mut AsdTextureObject2dImpl;
    pub fn asd_texture_object2d_free(obj: *mut AsdTextureObject2dImpl);
    pub fn asd_texture_object2d_set_texture(obj: *mut AsdTextureObject2dImpl, tex: *mut AsdTexture2dImpl);

    pub fn asd_texture2d_create(path: *const c_char) -> *mut AsdTexture2dImpl;
    pub fn asd_texture2d_free(tex: *mut AsdTexture2dImpl);

    pub fn asd_add_object2d(obj: *mut AsdTextureObject2dImpl);
}




