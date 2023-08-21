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
        (match self {
            MatPixelType::Bgr => NCNN_MAT_PIXEL_BGR,
            MatPixelType::Bgra => NCNN_MAT_PIXEL_BGRA,
            MatPixelType::Gray => NCNN_MAT_PIXEL_GRAY,
            MatPixelType::Rgb => NCNN_MAT_PIXEL_RGB,
            MatPixelType::Rgba => NCNN_MAT_PIXEL_RGBA,
            MatPixelType::RgbToBgr => {
                NCNN_MAT_PIXEL_RGB | (NCNN_MAT_PIXEL_BGR << PIXEL_CONVERT_SHIFT)
            }
            MatPixelType::RgbToGray => {
                NCNN_MAT_PIXEL_RGB | (NCNN_MAT_PIXEL_GRAY << PIXEL_CONVERT_SHIFT)
            }
            MatPixelType::RgbToRgba => {
                NCNN_MAT_PIXEL_RGB | (NCNN_MAT_PIXEL_RGBA << PIXEL_CONVERT_SHIFT)
            }
            MatPixelType::RgbToBgra => {
                NCNN_MAT_PIXEL_RGB | (NCNN_MAT_PIXEL_BGRA << PIXEL_CONVERT_SHIFT)
            }
            MatPixelType::BgrToRgb => {
                NCNN_MAT_PIXEL_BGR | (NCNN_MAT_PIXEL_RGB << PIXEL_CONVERT_SHIFT)
            }
            MatPixelType::BgrToGray => {
                NCNN_MAT_PIXEL_BGR | (NCNN_MAT_PIXEL_GRAY << PIXEL_CONVERT_SHIFT)
            }
            MatPixelType::BgrToRgba => {
                NCNN_MAT_PIXEL_BGR | (NCNN_MAT_PIXEL_RGBA << PIXEL_CONVERT_SHIFT)
            }
            MatPixelType::BgrToBgra => {
                NCNN_MAT_PIXEL_BGR | (NCNN_MAT_PIXEL_BGRA << PIXEL_CONVERT_SHIFT)
            }
            MatPixelType::GrayToRgb => {
                NCNN_MAT_PIXEL_GRAY | (NCNN_MAT_PIXEL_RGB << PIXEL_CONVERT_SHIFT)
            }
            MatPixelType::GrayToBgr => {
                NCNN_MAT_PIXEL_GRAY | (NCNN_MAT_PIXEL_BGR << PIXEL_CONVERT_SHIFT)
            }
            MatPixelType::GrayToRgba => {
                NCNN_MAT_PIXEL_GRAY | (NCNN_MAT_PIXEL_RGBA << PIXEL_CONVERT_SHIFT)
            }
            MatPixelType::GrayToBgra => {
                NCNN_MAT_PIXEL_GRAY | (NCNN_MAT_PIXEL_BGRA << PIXEL_CONVERT_SHIFT)
            }
            MatPixelType::RgbaToRgb => {
                NCNN_MAT_PIXEL_RGBA | (NCNN_MAT_PIXEL_RGB << PIXEL_CONVERT_SHIFT)
            }
            MatPixelType::RgbaToBgr => {
                NCNN_MAT_PIXEL_RGBA | (NCNN_MAT_PIXEL_BGR << PIXEL_CONVERT_SHIFT)
            }
            MatPixelType::RgbaToGray => {
                NCNN_MAT_PIXEL_RGBA | (NCNN_MAT_PIXEL_GRAY << PIXEL_CONVERT_SHIFT)
            }
            MatPixelType::RgbaToBgra => {
                NCNN_MAT_PIXEL_RGBA | (NCNN_MAT_PIXEL_BGRA << PIXEL_CONVERT_SHIFT)
            }
            MatPixelType::BgraToRgb => {
                NCNN_MAT_PIXEL_BGRA | (NCNN_MAT_PIXEL_RGB << PIXEL_CONVERT_SHIFT)
            }
            MatPixelType::BgraToBgr => {
                NCNN_MAT_PIXEL_BGRA | (NCNN_MAT_PIXEL_BGR << PIXEL_CONVERT_SHIFT)
            }
            MatPixelType::BgraToGray => {
                NCNN_MAT_PIXEL_BGRA | (NCNN_MAT_PIXEL_GRAY << PIXEL_CONVERT_SHIFT)
            }
            MatPixelType::BgraToRgba => {
                NCNN_MAT_PIXEL_BGRA | (NCNN_MAT_PIXEL_RGBA << PIXEL_CONVERT_SHIFT)
            }
        }) as _
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

pub enum MatBorderType {
    Constant,
    Replicate,
    Reflect,
    Transparent,
}

impl MatBorderType {
    fn to_int(&self) -> i32 {
        match self {
            MatBorderType::Constant => 0,
            MatBorderType::Replicate => 1,
            MatBorderType::Reflect => 2,
            MatBorderType::Transparent => -233,
        }
    }
}

fn cast_into_i32(val: usize, name: &str) -> anyhow::Result<i32> {
    val.try_into()
        .map_err(|_| anyhow::format_err!("Invalid {} size", name))
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
    pub fn new_1d(width: usize, alloc: Option<&Allocator>) -> anyhow::Result<Self> {
        let w = cast_into_i32(width, "width")?;

        Ok(Self {
            ptr: unsafe {
                ncnn_mat_create_1d(
                    w,
                    alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
                )
            },
        })
    }

    /// Constructs an empty 2D matrix.
    pub fn new_2d(width: usize, height: usize, alloc: Option<&Allocator>) -> anyhow::Result<Self> {
        let w = cast_into_i32(width, "width")?;
        let h = cast_into_i32(height, "height")?;

        Ok(Self {
            ptr: unsafe {
                ncnn_mat_create_2d(
                    w,
                    h,
                    alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
                )
            },
        })
    }

    /// Constructs an empty 3D matrix.
    pub fn new_3d(
        width: usize,
        height: usize,
        channels: usize,
        alloc: Option<&Allocator>,
    ) -> anyhow::Result<Self> {
        let w = cast_into_i32(width, "width")?;
        let h = cast_into_i32(height, "height")?;
        let c = cast_into_i32(channels, "channels")?;

        Ok(Self {
            ptr: unsafe {
                ncnn_mat_create_3d(
                    w,
                    h,
                    c,
                    alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
                )
            },
        })
    }

    /// Constructs an empty 4D matrix.
    pub fn new_4d(
        width: usize,
        height: usize,
        channels: usize,
        depth: usize,
        alloc: Option<&Allocator>,
    ) -> anyhow::Result<Self> {
        let w = cast_into_i32(width, "width")?;
        let h = cast_into_i32(height, "height")?;
        let c = cast_into_i32(channels, "channels")?;
        let d = cast_into_i32(depth, "depth")?;

        Ok(Self {
            ptr: unsafe {
                ncnn_mat_create_4d(
                    w,
                    h,
                    d,
                    c,
                    alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
                )
            },
        })
    }

    /// Constructs 1D matrix with a given raw data.
    ///
    /// # Safety
    ///
    /// Data pointer must not be aliased, it must be valid for the entire lifetime of Mat and it must be of correct size.
    pub unsafe fn new_external_1d(
        width: usize,
        data: *mut c_void,
        alloc: Option<&Allocator>,
    ) -> anyhow::Result<Self> {
        let w = cast_into_i32(width, "width")?;

        Ok(Self {
            ptr: ncnn_mat_create_external_1d(
                w,
                data,
                alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
            ),
        })
    }

    /// Constructs 2D matrix with a given raw data.
    ///
    /// # Safety
    ///
    /// Data pointer must not be aliased, it must be valid for the entire lifetime of Mat and it must be of correct size.
    pub unsafe fn new_external_2d(
        width: usize,
        height: usize,
        data: *mut c_void,
        alloc: Option<&Allocator>,
    ) -> anyhow::Result<Self> {
        let w = cast_into_i32(width, "width")?;
        let h = cast_into_i32(height, "height")?;

        Ok(Self {
            ptr: ncnn_mat_create_external_2d(
                w,
                h,
                data,
                alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
            ),
        })
    }

    /// Constructs 3D matrix with a given raw data.
    ///
    /// # Safety
    ///
    /// Data pointer must not be aliased, it must be valid for the entire lifetime of Mat and it must be of correct size.
    pub unsafe fn new_external_3d(
        width: usize,
        height: usize,
        channels: usize,
        data: *mut c_void,
        alloc: Option<&Allocator>,
    ) -> anyhow::Result<Self> {
        let w = cast_into_i32(width, "width")?;
        let h = cast_into_i32(height, "height")?;
        let c = cast_into_i32(channels, "channels")?;

        Ok(Self {
            ptr: ncnn_mat_create_external_3d(
                w,
                h,
                c,
                data,
                alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
            ),
        })
    }

    /// Constructs 4D matrix with a given raw data.
    ///
    /// # Safety
    ///
    /// Data pointer must not be aliased, it must be valid for the entire lifetime of Mat and it must be of correct size.
    pub unsafe fn new_external_4d(
        width: usize,
        height: usize,
        depth: usize,
        channels: usize,
        data: *mut c_void,
        alloc: Option<&Allocator>,
    ) -> anyhow::Result<Self> {
        let w = cast_into_i32(width, "width")?;
        let h = cast_into_i32(height, "height")?;
        let c = cast_into_i32(channels, "channels")?;
        let d = cast_into_i32(depth, "depth")?;

        Ok(Self {
            ptr: ncnn_mat_create_external_4d(
                w,
                h,
                d,
                c,
                data,
                alloc.map(Allocator::ptr).unwrap_or(core::ptr::null_mut()),
            ),
        })
    }

    /// Constructs matrix from a pixel byte array
    pub fn from_pixels(
        data: &[u8],
        pixel_type: MatPixelType,
        width: usize,
        height: usize,
        alloc: Option<&Allocator>,
    ) -> anyhow::Result<Self> {
        let width = cast_into_i32(width, "width")?;
        let height = cast_into_i32(height, "height")?;

        let len = width * height * pixel_type.stride();
        anyhow::ensure!(
            data.len() >= len as _,
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
        width: usize,
        height: usize,
        target_width: usize,
        target_height: usize,
        alloc: Option<&Allocator>,
    ) -> anyhow::Result<Self> {
        let width = cast_into_i32(width, "width")?;
        let height = cast_into_i32(height, "height")?;
        let target_width = cast_into_i32(target_width, "target_width")?;
        let target_height = cast_into_i32(target_height, "target_height")?;

        let len = width * height * pixel_type.stride();
        anyhow::ensure!(
            data.len() >= len as _,
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

    /// Add a padding border. Convenience method for copy_make_border.
    pub fn add_border(
        &mut self,
        top: usize,
        bottom: usize,
        left: usize,
        right: usize,
        border_type: MatBorderType,
        value: f32,
        opt: &crate::option::Option,
    ) -> anyhow::Result<()> {
        unsafe {
            let src = self.ptr;
            let dst = ncnn_mat_create();
            let top = cast_into_i32(top, "top")?;
            let bottom = cast_into_i32(bottom, "bottom")?;
            let left = cast_into_i32(left, "left")?;
            let right = cast_into_i32(right, "right")?;
            let border_type = border_type.to_int();
            let opt = opt.ptr();
            ncnn_copy_make_border(src, dst, top, bottom, left, right, border_type, value, opt);

            self.ptr = dst;

            ncnn_mat_destroy(src);
        }

        Ok(())
    }

    /// Add a padding border. Convenience method for copy_make_border_3d.
    pub fn add_border_3d(
        &mut self,
        top: usize,
        bottom: usize,
        left: usize,
        right: usize,
        front: usize,
        behind: usize,
        border_type: MatBorderType,
        value: f32,
        opt: &crate::option::Option,
    ) -> anyhow::Result<()> {
        unsafe {
            let src = self.ptr();
            let dst = ncnn_mat_create();
            let top = cast_into_i32(top, "top")?;
            let bottom = cast_into_i32(bottom, "bottom")?;
            let left = cast_into_i32(left, "left")?;
            let right = cast_into_i32(right, "right")?;
            let front = cast_into_i32(front, "front")?;
            let behind = cast_into_i32(behind, "behind")?;
            let border_type = border_type.to_int();
            let opt = opt.ptr();

            ncnn_copy_make_border_3d(
                src,
                dst,
                top,
                bottom,
                left,
                right,
                front,
                behind,
                border_type,
                value,
                opt,
            );

            self.ptr = dst;

            ncnn_mat_destroy(src);
        }

        Ok(())
    }

    pub fn cut_border(
        &mut self,
        top: usize,
        bottom: usize,
        left: usize,
        right: usize,
        opt: &crate::option::Option,
    ) -> anyhow::Result<()> {
        unsafe {
            let src = self.ptr;
            let dst = ncnn_mat_create();
            let top = cast_into_i32(top, "top")?;
            let bottom = cast_into_i32(bottom, "bottom")?;
            let left = cast_into_i32(left, "left")?;
            let right = cast_into_i32(right, "right")?;

            let opt = opt.ptr();

            ncnn_copy_cut_border(src, dst, top, bottom, left, right, opt);

            self.ptr = dst;

            ncnn_mat_destroy(src);
        }

        Ok(())
    }

    pub fn cut_border_3d(
        &mut self,
        top: usize,
        bottom: usize,
        left: usize,
        right: usize,
        front: usize,
        behind: usize,
        opt: &crate::option::Option,
    ) -> anyhow::Result<()> {
        unsafe {
            let src = self.ptr;
            let dst = ncnn_mat_create();
            let top = cast_into_i32(top, "top")?;
            let bottom = cast_into_i32(bottom, "bottom")?;
            let left = cast_into_i32(left, "left")?;
            let right = cast_into_i32(right, "right")?;
            let front = cast_into_i32(front, "front")?;
            let behind = cast_into_i32(behind, "behind")?;
            let opt = opt.ptr();

            ncnn_copy_cut_border_3d(src, dst, top, bottom, left, right, front, behind, opt);

            self.ptr = dst;

            ncnn_mat_destroy(src);
        }

        Ok(())
    }

    /// Fills matrix with a given value.
    pub fn fill(&mut self, value: f32) {
        unsafe { ncnn_mat_fill_float(self.ptr, value) };
    }

    /// Returns number of matrix dimensions.
    pub fn dimensions(&self) -> usize {
        unsafe { ncnn_mat_get_dims(self.ptr) as _ }
    }

    /// Returns matrix width.
    pub fn width(&self) -> usize {
        unsafe { ncnn_mat_get_w(self.ptr) as _ }
    }

    /// Returns matrix height.
    pub fn height(&self) -> usize {
        unsafe { ncnn_mat_get_h(self.ptr) as _ }
    }

    /// Returns matrix depth.
    pub fn depth(&self) -> usize {
        unsafe { ncnn_mat_get_d(self.ptr) as _ }
    }

    /// Returns matrix channels.
    pub fn channels(&self) -> usize {
        unsafe { ncnn_mat_get_c(self.ptr) as _ }
    }

    pub fn element_size(&self) -> usize {
        unsafe { ncnn_mat_get_elemsize(self.ptr) as _ }
    }

    pub fn element_packing(&self) -> usize {
        unsafe { ncnn_mat_get_elempack(self.ptr) as _ }
    }

    pub fn channel_step(&self) -> usize {
        unsafe { ncnn_mat_get_cstep(self.ptr) as _ }
    }

    /// Pointer to raw matrix data
    pub fn data(&self) -> *mut c_void {
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

/// Add a padding border to src's content, copying it in dst.
pub fn copy_make_border(
    src: &Mat,
    dst: &mut Mat,
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
    border_type: MatBorderType,
    value: f32,
    opt: &crate::option::Option,
) -> anyhow::Result<()> {
    unsafe {
        let src = src.ptr();
        let dst = dst.mut_ptr();
        let top = cast_into_i32(top, "top")?;
        let bottom = cast_into_i32(bottom, "bottom")?;
        let left = cast_into_i32(left, "left")?;
        let right = cast_into_i32(right, "right")?;
        let border_type = border_type.to_int();
        let opt = opt.ptr();

        ncnn_copy_make_border(src, dst, top, bottom, left, right, border_type, value, opt);
    }

    Ok(())
}

/// Add a 3d padding border to src's content, copying it in dst.
pub fn copy_make_border_3d(
    src: &Mat,
    dst: &mut Mat,
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
    front: usize,
    behind: usize,
    border_type: MatBorderType,
    value: f32,
    opt: &crate::option::Option,
) -> anyhow::Result<()> {
    unsafe {
        let src = src.ptr();
        let dst = dst.mut_ptr();
        let top = cast_into_i32(top, "top")?;
        let bottom = cast_into_i32(bottom, "bottom")?;
        let left = cast_into_i32(left, "left")?;
        let right = cast_into_i32(right, "right")?;
        let front = cast_into_i32(front, "front")?;
        let behind = cast_into_i32(behind, "behind")?;
        let border_type = border_type.to_int();
        let opt = opt.ptr();

        ncnn_copy_make_border_3d(
            src,
            dst,
            top,
            bottom,
            left,
            right,
            front,
            behind,
            border_type,
            value,
            opt,
        );
    }

    Ok(())
}

pub fn copy_cut_border(
    src: &Mat,
    dst: &mut Mat,
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
    opt: &crate::option::Option,
) -> anyhow::Result<()> {
    unsafe {
        let src = src.ptr();
        let dst = dst.mut_ptr();
        let top = cast_into_i32(top, "top")?;
        let bottom = cast_into_i32(bottom, "bottom")?;
        let left = cast_into_i32(left, "left")?;
        let right = cast_into_i32(right, "right")?;
        let opt = opt.ptr();

        ncnn_copy_cut_border(src, dst, top, bottom, left, right, opt);
    }

    Ok(())
}

pub fn copy_cut_border_3d(
    src: &Mat,
    dst: &mut Mat,
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
    front: usize,
    behind: usize,
    opt: &crate::option::Option,
) -> anyhow::Result<()> {
    unsafe {
        let src = src.ptr();
        let dst = dst.mut_ptr();
        let top = cast_into_i32(top, "top")?;
        let bottom = cast_into_i32(bottom, "bottom")?;
        let left = cast_into_i32(left, "left")?;
        let right = cast_into_i32(right, "right")?;
        let front = cast_into_i32(front, "front")?;
        let behind = cast_into_i32(behind, "behind")?;
        let opt = opt.ptr();

        ncnn_copy_cut_border_3d(src, dst, top, bottom, left, right, front, behind, opt);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Mat, MatPixelType};

    #[test]
    fn basic_getter_and_setter() {
        let m = Mat::new_3d(224, 224, 3, None).unwrap();
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
