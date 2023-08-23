use crate::allocator::Allocator;
use core::fmt;
use ncnn_bind::*;
use std::os::raw::c_void;

const PIXEL_CONVERT_SHIFT: u32 = 16;

pub enum MatPixelType {
    Bgr,
    Bgra,
    Gray,
    Rgb,
    Rgba,

    RgbToBgr,
    RgbToGray,
    RgbToRgba,
    RgbToBgra,

    BgrToRgb,
    BgrToGray,
    BgrToRgba,
    BgrToBgra,

    GrayToRgb,
    GrayToBgr,
    GrayToRgba,
    GrayToBgra,

    RgbaToRgb,
    RgbaToBgr,
    RgbaToGray,
    RgbaToBgra,

    BgraToRgb,
    BgraToBgr,
    BgraToGray,
    BgraToRgba,
}

impl MatPixelType {
    fn to_int(&self) -> i32 {
        match self {
            MatPixelType::Bgr => NCNN_MAT_PIXEL_BGR as _,
            MatPixelType::Bgra => NCNN_MAT_PIXEL_BGRA as _,
            MatPixelType::Gray => NCNN_MAT_PIXEL_GRAY as _,
            MatPixelType::Rgb => NCNN_MAT_PIXEL_RGB as _,
            MatPixelType::Rgba => NCNN_MAT_PIXEL_RGBA as _,
            MatPixelType::RgbToBgr => {
                (NCNN_MAT_PIXEL_RGB | (NCNN_MAT_PIXEL_BGR << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::RgbToGray => {
                (NCNN_MAT_PIXEL_RGB | (NCNN_MAT_PIXEL_GRAY << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::RgbToRgba => {
                (NCNN_MAT_PIXEL_RGB | (NCNN_MAT_PIXEL_RGBA << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::RgbToBgra => {
                (NCNN_MAT_PIXEL_RGB | (NCNN_MAT_PIXEL_BGRA << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::BgrToRgb => {
                (NCNN_MAT_PIXEL_BGR | (NCNN_MAT_PIXEL_RGB << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::BgrToGray => {
                (NCNN_MAT_PIXEL_BGR | (NCNN_MAT_PIXEL_GRAY << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::BgrToRgba => {
                (NCNN_MAT_PIXEL_BGR | (NCNN_MAT_PIXEL_RGBA << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::BgrToBgra => {
                (NCNN_MAT_PIXEL_BGR | (NCNN_MAT_PIXEL_BGRA << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::GrayToRgb => {
                (NCNN_MAT_PIXEL_GRAY | (NCNN_MAT_PIXEL_RGB << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::GrayToBgr => {
                (NCNN_MAT_PIXEL_GRAY | (NCNN_MAT_PIXEL_BGR << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::GrayToRgba => {
                (NCNN_MAT_PIXEL_GRAY | (NCNN_MAT_PIXEL_RGBA << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::GrayToBgra => {
                (NCNN_MAT_PIXEL_GRAY | (NCNN_MAT_PIXEL_BGRA << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::RgbaToRgb => {
                (NCNN_MAT_PIXEL_RGBA | (NCNN_MAT_PIXEL_RGB << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::RgbaToBgr => {
                (NCNN_MAT_PIXEL_RGBA | (NCNN_MAT_PIXEL_BGR << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::RgbaToGray => {
                (NCNN_MAT_PIXEL_RGBA | (NCNN_MAT_PIXEL_GRAY << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::RgbaToBgra => {
                (NCNN_MAT_PIXEL_RGBA | (NCNN_MAT_PIXEL_BGRA << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::BgraToRgb => {
                (NCNN_MAT_PIXEL_BGRA | (NCNN_MAT_PIXEL_RGB << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::BgraToBgr => {
                (NCNN_MAT_PIXEL_BGRA | (NCNN_MAT_PIXEL_BGR << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::BgraToGray => {
                (NCNN_MAT_PIXEL_BGRA | (NCNN_MAT_PIXEL_GRAY << PIXEL_CONVERT_SHIFT)) as _
            }
            MatPixelType::BgraToRgba => {
                (NCNN_MAT_PIXEL_BGRA | (NCNN_MAT_PIXEL_RGBA << PIXEL_CONVERT_SHIFT)) as _
            }
        }
    }

    fn stride(&self) -> i32 {
        match self {
            MatPixelType::Bgr => 3,
            MatPixelType::Bgra => 4,
            MatPixelType::Gray => 1,
            MatPixelType::Rgb => 3,
            MatPixelType::Rgba => 4,
            MatPixelType::RgbToBgr => 3,
            MatPixelType::RgbToGray => 3,
            MatPixelType::RgbToRgba => 3,
            MatPixelType::RgbToBgra => 3,
            MatPixelType::BgrToRgb => 3,
            MatPixelType::BgrToGray => 3,
            MatPixelType::BgrToRgba => 3,
            MatPixelType::BgrToBgra => 3,
            MatPixelType::GrayToRgb => 1,
            MatPixelType::GrayToBgr => 1,
            MatPixelType::GrayToRgba => 1,
            MatPixelType::GrayToBgra => 1,
            MatPixelType::RgbaToRgb => 4,
            MatPixelType::RgbaToBgr => 4,
            MatPixelType::RgbaToGray => 4,
            MatPixelType::RgbaToBgra => 4,
            MatPixelType::BgraToRgb => 4,
            MatPixelType::BgraToBgr => 4,
            MatPixelType::BgraToGray => 4,
            MatPixelType::BgraToRgba => 4,
        }
    }
}

pub struct Mat {
    ptr: ncnn_mat_t,
}

// Mat is basically a glorified atomically refcounted matrix.
unsafe impl Send for Mat {}

impl Mat {
    /// Constructs an empty matrix.
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs an empty 1D matrix.
    pub fn new_1d(w: i32, alloc: Option<&Allocator>) -> Self {
        Self {
            ptr: unsafe {
                ncnn_mat_create_1d(
                    w,
                    alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
                )
            },
        }
    }

    /// Constructs an empty 2D matrix.
    pub fn new_2d(w: i32, h: i32, alloc: Option<&Allocator>) -> Self {
        Self {
            ptr: unsafe {
                ncnn_mat_create_2d(
                    w,
                    h,
                    alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
                )
            },
        }
    }

    /// Constructs an empty 3D matrix.
    pub fn new_3d(w: i32, h: i32, c: i32, alloc: Option<&Allocator>) -> Self {
        Self {
            ptr: unsafe {
                ncnn_mat_create_3d(
                    w,
                    h,
                    c,
                    alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
                )
            },
        }
    }

    /// Constructs an empty 4D matrix.
    pub fn new_4d(w: i32, h: i32, d: i32, c: i32, alloc: Option<&Allocator>) -> Self {
        Self {
            ptr: unsafe {
                ncnn_mat_create_4d(
                    w,
                    h,
                    d,
                    c,
                    alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
                )
            },
        }
    }

    /// Constructs 1D matrix with a given raw data.
    ///
    /// # Safety
    ///
    /// Data pointer must not be aliased, it must be valid for the entire lifetime of Mat and it must be of correct size.
    pub unsafe fn new_external_1d(w: i32, data: *mut c_void, alloc: Option<&Allocator>) -> Self {
        Self {
            ptr: ncnn_mat_create_external_1d(
                w,
                data,
                alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
            ),
        }
    }

    /// Constructs 2D matrix with a given raw data.
    ///
    /// # Safety
    ///
    /// Data pointer must not be aliased, it must be valid for the entire lifetime of Mat and it must be of correct size.
    pub unsafe fn new_external_2d(
        w: i32,
        h: i32,
        data: *mut c_void,
        alloc: Option<&Allocator>,
    ) -> Self {
        Self {
            ptr: ncnn_mat_create_external_2d(
                w,
                h,
                data,
                alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
            ),
        }
    }

    /// Constructs 3D matrix with a given raw data.
    ///
    /// # Safety
    ///
    /// Data pointer must not be aliased, it must be valid for the entire lifetime of Mat and it must be of correct size.
    pub unsafe fn new_external_3d(
        w: i32,
        h: i32,
        c: i32,
        data: *mut c_void,
        alloc: Option<&Allocator>,
    ) -> Self {
        Self {
            ptr: ncnn_mat_create_external_3d(
                w,
                h,
                c,
                data,
                alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
            ),
        }
    }

    /// Constructs 4D matrix with a given raw data.
    ///
    /// # Safety
    ///
    /// Data pointer must not be aliased, it must be valid for the entire lifetime of Mat and it must be of correct size.
    pub unsafe fn new_external_4d(
        w: i32,
        h: i32,
        d: i32,
        c: i32,
        data: *mut c_void,
        alloc: Option<&Allocator>,
    ) -> Self {
        Self {
            ptr: ncnn_mat_create_external_4d(
                w,
                h,
                d,
                c,
                data,
                alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
            ),
        }
    }

    /// Constructs matrix from a pixel byte array
    pub fn from_pixels(
        data: &[u8],
        pixel_type: MatPixelType,
        width: i32,
        height: i32,
        alloc: Option<&Allocator>,
    ) -> anyhow::Result<Self> {
        let len = width * height * pixel_type.stride();
        anyhow::ensure!(
            data.len() != len as _,
            "Expected data length {}, provided {}",
            len,
            data.len()
        );

        Ok(Self {
            ptr: unsafe {
                ncnn_mat_from_pixels(
                    data.as_ptr(),
                    pixel_type.to_int(),
                    width,
                    height,
                    width * pixel_type.stride(),
                    alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
                )
            },
        })
    }

    /// Constructs matrix from resizing a pixel byte array.
    pub fn from_pixels_resize(
        data: &[u8],
        pixel_type: MatPixelType,
        width: i32,
        height: i32,
        target_width: i32,
        target_height: i32,
        alloc: Option<&Allocator>,
    ) -> anyhow::Result<Self> {
        let len = width * height * pixel_type.stride();
        anyhow::ensure!(
            data.len() != len as _,
            "Expected data length {}, provided {}",
            len,
            data.len()
        );

        Ok(Self {
            ptr: unsafe {
                ncnn_mat_from_pixels_resize(
                    data.as_ptr(),
                    pixel_type.to_int(),
                    width,
                    height,
                    width * pixel_type.stride(),
                    target_width,
                    target_height,
                    alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
                )
            },
        })
    }

    pub fn subtract_mean_normalize(
        &mut self,
        mean_vals: Option<&[f32]>,
        norm_vals: Option<&[f32]>,
    ) -> anyhow::Result<()> {
        fn map_vals(vals: Option<&[f32]>, channels: usize) -> anyhow::Result<*const f32> {
            match vals {
                None => Ok(core::ptr::null_mut()),
                Some(vals) => {
                    if vals.len() < channels {
                        Err(anyhow::anyhow!(
                            "Expected data length {}, provided {}",
                            channels,
                            vals.len()
                        ))
                    } else {
                        Ok(vals.as_ptr())
                    }
                }
            }
        }

        let channels = self.channels() as usize;

        let mean_vals = map_vals(mean_vals, channels)?;
        let norm_vals = map_vals(norm_vals, channels)?;

        unsafe {
            ncnn_mat_substract_mean_normalize(self.ptr, mean_vals, norm_vals);
        }

        Ok(())
    }

    /// Fills matrix with a given value.
    pub fn fill(&mut self, value: f32) {
        unsafe { ncnn_mat_fill_float(self.ptr, value) };
    }

    /// Returns number of matrix dimensions.
    pub fn dimensions(&self) -> i32 {
        unsafe { ncnn_mat_get_dims(self.ptr) }
    }

    /// Returns matrix width.
    pub fn width(&self) -> i32 {
        unsafe { ncnn_mat_get_w(self.ptr) }
    }

    /// Returns matrix height.
    pub fn height(&self) -> i32 {
        unsafe { ncnn_mat_get_h(self.ptr) }
    }

    /// Returns matrix depth.
    pub fn depth(&self) -> i32 {
        unsafe { ncnn_mat_get_d(self.ptr) }
    }

    /// Returns matrix channels.
    pub fn channels(&self) -> i32 {
        unsafe { ncnn_mat_get_c(self.ptr) }
    }

    pub fn element_size(&self) -> u64 {
        (unsafe { ncnn_mat_get_elemsize(self.ptr) }) as u64
    }

    pub fn element_packing(&self) -> i32 {
        unsafe { ncnn_mat_get_elempack(self.ptr) }
    }

    pub fn channel_step(&self) -> u64 {
        unsafe { ncnn_mat_get_cstep(self.ptr) }
    }

    /// Pointer to raw matrix data
    pub fn data(&self) -> *mut ::std::os::raw::c_void {
        unsafe { ncnn_mat_get_data(self.ptr) }
    }

    pub(crate) fn ptr(&self) -> ncnn_mat_t {
        self.ptr
    }

    pub(crate) fn mut_ptr(&mut self) -> ncnn_mat_t {
        self.ptr
    }
}

impl Default for Mat {
    fn default() -> Self {
        Self {
            ptr: unsafe { ncnn_mat_create() },
        }
    }
}

impl fmt::Debug for Mat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mat")
            .field("dimensions", &self.dimensions())
            .field("channels", &self.channels())
            .field("height", &self.height())
            .field("width", &self.width())
            .field("element_size", &self.element_size())
            .field("element_packing", &self.element_packing())
            .field("channel_step", &self.channel_step())
            .finish()
    }
}

impl Drop for Mat {
    fn drop(&mut self) {
        unsafe {
            ncnn_mat_destroy(self.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Mat, MatPixelType};

    #[test]
    fn basic_getter_and_setter() {
        let m: Mat = Mat::new_3d(224, 224, 3, None);
        assert_eq!(224, m.height());
        assert_eq!(224, m.width());
        assert_eq!(3, m.channels());
    }

    #[test]
    fn stride() {
        assert_eq!(MatPixelType::Bgr.stride(), 3);
        assert_eq!(MatPixelType::Bgra.stride(), 4);
        assert_eq!(MatPixelType::Gray.stride(), 1);
        assert_eq!(MatPixelType::Rgb.stride(), 3);
        assert_eq!(MatPixelType::Rgba.stride(), 4);

        assert_eq!(MatPixelType::RgbToBgr.stride(), 3);
        assert_eq!(MatPixelType::RgbToGray.stride(), 3);
        assert_eq!(MatPixelType::RgbToRgba.stride(), 3);
        assert_eq!(MatPixelType::RgbToBgra.stride(), 3);

        assert_eq!(MatPixelType::BgrToRgb.stride(), 3);
        assert_eq!(MatPixelType::BgrToGray.stride(), 3);
        assert_eq!(MatPixelType::BgrToRgba.stride(), 3);
        assert_eq!(MatPixelType::BgrToBgra.stride(), 3);

        assert_eq!(MatPixelType::GrayToRgb.stride(), 1);
        assert_eq!(MatPixelType::GrayToBgr.stride(), 1);
        assert_eq!(MatPixelType::GrayToRgba.stride(), 1);
        assert_eq!(MatPixelType::GrayToBgra.stride(), 1);

        assert_eq!(MatPixelType::RgbaToRgb.stride(), 4);
        assert_eq!(MatPixelType::RgbaToBgr.stride(), 4);
        assert_eq!(MatPixelType::RgbaToGray.stride(), 4);
        assert_eq!(MatPixelType::RgbaToBgra.stride(), 4);

        assert_eq!(MatPixelType::BgraToRgb.stride(), 4);
        assert_eq!(MatPixelType::BgraToBgr.stride(), 4);
        assert_eq!(MatPixelType::BgraToGray.stride(), 4);
        assert_eq!(MatPixelType::BgraToRgba.stride(), 4);
    }
}
