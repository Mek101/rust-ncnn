use ncnn_bind::*;
use std::ffi::{c_char, CStr, CString};

pub enum LayerError {
    BadTypeName,
    UnknownType,
}

#[derive(Clone, Copy)]
pub struct LayerId(i32);

#[derive(Clone, Copy, Default)]
pub struct LayerShape {
    dims: i32,
    width: i32,
    height: i32,
    channels: i32,
}

pub struct Layer {
    ptr: ncnn_layer_t,
}

impl Layer {
    fn to_static_str(c_buf: *const c_char) -> &'static str {
        let c_str = unsafe { CStr::from_ptr(c_buf) };
        let str_slice: &str = c_str.to_str().unwrap();
        str_slice
    }

    pub fn create() -> Self {
        Self {
            ptr: unsafe { ncnn_layer_create() },
        }
    }

    pub fn create_by_type_name<S: AsRef<str>>(type_name: S) -> Result<Self, LayerError> {
        let c_str = CString::new(type_name.as_ref()).map_err(|_| LayerError::BadTypeName)?;

        let ptr = unsafe { ncnn_layer_create_by_type(c_str.as_ptr()) };

        if ptr.is_null() {
            Err(LayerError::UnknownType)
        } else {
            Ok(Self { ptr })
        }
    }

    pub fn create_by_type_id(id: LayerId) -> Result<Self, LayerError> {
        let ptr = unsafe { ncnn_layer_create_by_typeindex(id.0) };

        if ptr.is_null() {
            Err(LayerError::UnknownType)
        } else {
            Ok(Self { ptr })
        }
    }

    pub fn name(&self) -> &'static str {
        Self::to_static_str(unsafe { ncnn_layer_get_name(self.ptr) })
    }

    pub fn id(&self) -> LayerId {
        LayerId(unsafe { ncnn_layer_get_typeindex(self.ptr) })
    }

    pub fn type_name(&self) -> &'static str {
        Self::to_static_str(unsafe { ncnn_layer_get_type(self.ptr) })
    }

    pub fn one_blob_only(&self) -> bool {
        unsafe { ncnn_layer_get_one_blob_only(self.ptr) != 0 }
    }

    pub fn support_inplace(&self) -> bool {
        unsafe { ncnn_layer_get_support_inplace(self.ptr) != 0 }
    }

    pub fn support_vulkan(&self) -> bool {
        unsafe { ncnn_layer_get_support_vulkan(self.ptr) != 0 }
    }

    pub fn support_packing(&self) -> bool {
        unsafe { ncnn_layer_get_support_packing(self.ptr) != 0 }
    }

    pub fn support_bf16_storage(&self) -> bool {
        unsafe { ncnn_layer_get_support_bf16_storage(self.ptr) != 0 }
    }

    pub fn support_fp16_storage(&self) -> bool {
        unsafe { ncnn_layer_get_support_fp16_storage(self.ptr) != 0 }
    }

    pub fn support_image_storage(&self) -> bool {
        unsafe { ncnn_layer_get_support_image_storage(self.ptr) != 0 }
    }

    pub fn set_one_blob_only(&mut self, enable: bool) {
        unsafe { ncnn_layer_set_one_blob_only(self.ptr, enable as i32) }
    }

    pub fn set_support_inplace(&mut self, enable: bool) {
        unsafe { ncnn_layer_set_support_inplace(self.ptr, enable as i32) }
    }

    pub fn set_support_vulkan(&mut self, enable: bool) {
        unsafe { ncnn_layer_set_support_vulkan(self.ptr, enable as i32) }
    }

    pub fn set_support_packing(&mut self, enable: bool) {
        unsafe { ncnn_layer_set_support_packing(self.ptr, enable as i32) }
    }

    pub fn set_support_bf16_storage(&mut self, enable: bool) {
        unsafe { ncnn_layer_set_support_bf16_storage(self.ptr, enable as i32) }
    }

    pub fn set_support_fp16_storage(&mut self, enable: bool) {
        unsafe { ncnn_layer_set_support_fp16_storage(self.ptr, enable as i32) }
    }

    pub fn set_support_image_storage(&mut self, enable: bool) {
        unsafe { ncnn_layer_set_support_image_storage(self.ptr, enable as i32) }
    }

    pub fn bottom_len(&self) -> i32 {
        unsafe { ncnn_layer_get_bottom_count(self.ptr) }
    }

    pub fn top_len(&self) -> i32 {
        unsafe { ncnn_layer_get_top_count(self.ptr) }
    }

    pub fn bottom(&self, index: i32) -> Option<i32> {
        unsafe {
            let size = ncnn_layer_get_bottom_count(self.ptr);
            if index >= 0 && index < size {
                Some(ncnn_layer_get_bottom(self.ptr, index))
            } else {
                None
            }
        }
    }

    pub fn top(&self, index: i32) -> Option<i32> {
        unsafe {
            let size = ncnn_layer_get_top_count(self.ptr);
            if index >= 0 && index < size {
                Some(ncnn_layer_get_top(self.ptr, index))
            } else {
                None
            }
        }
    }

    pub unsafe fn bottom_unchecked(&self, index: i32) -> i32 {
        ncnn_layer_get_bottom(self.ptr, index)
    }

    pub unsafe fn top_unchecked(&self, index: i32) -> i32 {
        ncnn_layer_get_top(self.ptr, index)
    }

    pub fn blob_bottom_shape(&self, index: i32) -> Option<LayerShape> {
        unsafe {
            let size = ncnn_layer_get_bottom_count(self.ptr);
            if index >= 0 && index < size {
                Some(self.blob_bottom_shape_unchecked(index))
            } else {
                None
            }
        }
    }

    pub fn blob_top_shape(&self, index: i32) -> Option<LayerShape> {
        unsafe {
            let size = ncnn_layer_get_top_count(self.ptr);
            if index >= 0 && index < size {
                Some(self.blob_top_shape_unchecked(index))
            } else {
                None
            }
        }
    }

    pub unsafe fn blob_bottom_shape_unchecked(&self, index: i32) -> LayerShape {
        let mut shape = LayerShape::default();
        ncnn_blob_get_bottom_shape(
            self.ptr,
            index,
            &mut shape.dims as *mut _,
            &mut shape.width as *mut _,
            &mut shape.height as *mut _,
            &mut shape.channels as *mut _,
        );

        shape
    }

    pub unsafe fn blob_top_shape_unchecked(&self, index: i32) -> LayerShape {
        let mut shape = LayerShape::default();
        ncnn_blob_get_top_shape(
            self.ptr,
            index,
            &mut shape.dims as *mut _,
            &mut shape.width as *mut _,
            &mut shape.height as *mut _,
            &mut shape.channels as *mut _,
        );

        shape
    }
}

impl Drop for Layer {
    fn drop(&mut self) {
        unsafe {
            ncnn_layer_destroy(self.ptr);
        }
    }
}
