use std::ffi::CString;

mod sys;

pub fn initialize(title: &str, width: i32, height: i32) -> bool {
    let title = CString::new(title).unwrap();
    unsafe {
        sys::asd_initialize(title.as_ptr(), width, height)
    }
}

pub fn do_events() -> bool {
    unsafe {
        sys::asd_do_events()
    }
}

pub fn update() {
    unsafe {
        sys::asd_update();
    }
}

pub fn terminate() {
    unsafe {
        sys::asd_terminate();
    }
}

pub struct TextureObject2D {
    raw: *mut sys::AsdTextureObject2dImpl
}

impl TextureObject2D {
    pub fn new() -> Self {
        unsafe {
            TextureObject2D {
                raw: sys::asd_texture_object2d_create()
            }
        }
    }

    pub fn texture(&mut self, tex: &mut Texture2D) {
        unsafe {
            sys::asd_texture_object2d_set_texture(self.raw, tex.raw);
        }
    }
}

impl Drop for TextureObject2D {
    fn drop(&mut self) {
        unsafe {
            sys::asd_texture_object2d_free(self.raw);
        }
    }
}

pub struct Texture2D {
    raw: *mut sys::AsdTexture2dImpl
}

impl Texture2D {
    pub fn new(path: &str) -> Self {
        unsafe {
            Texture2D {
                raw: sys::asd_texture2d_create(CString::new(path).unwrap().as_ptr())
            }
        }
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe {
            sys::asd_texture2d_free(self.raw);
        }
    }
}

pub fn add_object2d(obj: &mut TextureObject2D) {
    unsafe {
        sys::asd_add_object2d(obj.raw);
    }
}


