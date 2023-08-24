use crate::allocator::Allocator;
use core::fmt;
use ncnn_bind::*;
use std::os::raw::c_void;

const PIXEL_CONVERT_SHIFT: u32 = 16;

pub enum PixelType {
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

impl PixelType {
    fn to_int(&self) -> i32 {
        match self {
            PixelType::Bgr => NCNN_MAT_PIXEL_BGR as _,
            PixelType::Bgra => NCNN_MAT_PIXEL_BGRA as _,
            PixelType::Gray => NCNN_MAT_PIXEL_GRAY as _,
            PixelType::Rgb => NCNN_MAT_PIXEL_RGB as _,
            PixelType::Rgba => NCNN_MAT_PIXEL_RGBA as _,
            PixelType::RgbToBgr => {
                (NCNN_MAT_PIXEL_RGB | (NCNN_MAT_PIXEL_BGR << PIXEL_CONVERT_SHIFT)) as _
            }
            PixelType::RgbToGray => {
                (NCNN_MAT_PIXEL_RGB | (NCNN_MAT_PIXEL_GRAY << PIXEL_CONVERT_SHIFT)) as _
            }
            PixelType::RgbToRgba => {
                (NCNN_MAT_PIXEL_RGB | (NCNN_MAT_PIXEL_RGBA << PIXEL_CONVERT_SHIFT)) as _
            }
            PixelType::RgbToBgra => {
                (NCNN_MAT_PIXEL_RGB | (NCNN_MAT_PIXEL_BGRA << PIXEL_CONVERT_SHIFT)) as _
            }
            PixelType::BgrToRgb => {
                (NCNN_MAT_PIXEL_BGR | (NCNN_MAT_PIXEL_RGB << PIXEL_CONVERT_SHIFT)) as _
            }
            PixelType::BgrToGray => {
                (NCNN_MAT_PIXEL_BGR | (NCNN_MAT_PIXEL_GRAY << PIXEL_CONVERT_SHIFT)) as _
            }
            PixelType::BgrToRgba => {
                (NCNN_MAT_PIXEL_BGR | (NCNN_MAT_PIXEL_RGBA << PIXEL_CONVERT_SHIFT)) as _
            }
            PixelType::BgrToBgra => {
                (NCNN_MAT_PIXEL_BGR | (NCNN_MAT_PIXEL_BGRA << PIXEL_CONVERT_SHIFT)) as _
            }
            PixelType::GrayToRgb => {
                (NCNN_MAT_PIXEL_GRAY | (NCNN_MAT_PIXEL_RGB << PIXEL_CONVERT_SHIFT)) as _
            }
            PixelType::GrayToBgr => {
                (NCNN_MAT_PIXEL_GRAY | (NCNN_MAT_PIXEL_BGR << PIXEL_CONVERT_SHIFT)) as _
            }
            PixelType::GrayToRgba => {
                (NCNN_MAT_PIXEL_GRAY | (NCNN_MAT_PIXEL_RGBA << PIXEL_CONVERT_SHIFT)) as _
            }
            PixelType::GrayToBgra => {
                (NCNN_MAT_PIXEL_GRAY | (NCNN_MAT_PIXEL_BGRA << PIXEL_CONVERT_SHIFT)) as _
            }
            PixelType::RgbaToRgb => {
                (NCNN_MAT_PIXEL_RGBA | (NCNN_MAT_PIXEL_RGB << PIXEL_CONVERT_SHIFT)) as _
            }
            PixelType::RgbaToBgr => {
                (NCNN_MAT_PIXEL_RGBA | (NCNN_MAT_PIXEL_BGR << PIXEL_CONVERT_SHIFT)) as _
            }
            PixelType::RgbaToGray => {
                (NCNN_MAT_PIXEL_RGBA | (NCNN_MAT_PIXEL_GRAY << PIXEL_CONVERT_SHIFT)) as _
            }
            PixelType::RgbaToBgra => {
                (NCNN_MAT_PIXEL_RGBA | (NCNN_MAT_PIXEL_BGRA << PIXEL_CONVERT_SHIFT)) as _
            }
            PixelType::BgraToRgb => {
                (NCNN_MAT_PIXEL_BGRA | (NCNN_MAT_PIXEL_RGB << PIXEL_CONVERT_SHIFT)) as _
            }
            PixelType::BgraToBgr => {
                (NCNN_MAT_PIXEL_BGRA | (NCNN_MAT_PIXEL_BGR << PIXEL_CONVERT_SHIFT)) as _
            }
            PixelType::BgraToGray => {
                (NCNN_MAT_PIXEL_BGRA | (NCNN_MAT_PIXEL_GRAY << PIXEL_CONVERT_SHIFT)) as _
            }
            PixelType::BgraToRgba => {
                (NCNN_MAT_PIXEL_BGRA | (NCNN_MAT_PIXEL_RGBA << PIXEL_CONVERT_SHIFT)) as _
            }
        }
    }

    fn stride(&self) -> i32 {
        match self {
            PixelType::Bgr
            | PixelType::BgrToBgra
            | PixelType::BgrToGray
            | PixelType::BgrToRgb
            | PixelType::BgrToRgba => 3,
            PixelType::Bgra
            | PixelType::BgraToBgr
            | PixelType::BgraToGray
            | PixelType::BgraToRgb
            | PixelType::BgraToRgba => 4,
            PixelType::Gray
            | PixelType::GrayToBgr
            | PixelType::GrayToBgra
            | PixelType::GrayToRgb
            | PixelType::GrayToRgba => 1,
            PixelType::Rgb
            | PixelType::RgbToBgr
            | PixelType::RgbToBgra
            | PixelType::RgbToGray
            | PixelType::RgbToRgba => 3,
            PixelType::Rgba
            | PixelType::RgbaToBgr
            | PixelType::RgbaToBgra
            | PixelType::RgbaToGray
            | PixelType::RgbaToRgb => 4,
        }
    }
}

pub enum BorderType {
    Constant,
    Replicate,
    Reflect,
    Transparent,
}

impl BorderType {
    fn to_int(&self) -> i32 {
        match self {
            BorderType::Constant => 0,
            BorderType::Replicate => 1,
            BorderType::Reflect => 2,
            BorderType::Transparent => -233,
        }
    }
}

fn cast_into_i32(val: u32, name: &str) -> anyhow::Result<i32> {
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
    pub fn new_1d(width: u32, alloc: Option<&Allocator>) -> anyhow::Result<Self> {
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
    pub fn new_2d(width: u32, height: u32, alloc: Option<&Allocator>) -> anyhow::Result<Self> {
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
        width: u32,
        height: u32,
        channels: u32,
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
        width: u32,
        height: u32,
        channels: u32,
        depth: u32,
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
        width: u32,
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
        width: u32,
        height: u32,
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
        width: u32,
        height: u32,
        channels: u32,
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
        width: u32,
        height: u32,
        depth: u32,
        channels: u32,
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
        pixel_type: PixelType,
        width: u32,
        height: u32,
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
        pixel_type: PixelType,
        width: u32,
        height: u32,
        target_width: u32,
        target_height: u32,
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
        top: u32,
        bottom: u32,
        left: u32,
        right: u32,
        border_type: BorderType,
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
        top: u32,
        bottom: u32,
        left: u32,
        right: u32,
        front: u32,
        behind: u32,
        border_type: BorderType,
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
        top: u32,
        bottom: u32,
        left: u32,
        right: u32,
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
        top: u32,
        bottom: u32,
        left: u32,
        right: u32,
        front: u32,
        behind: u32,
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
    pub fn dimensions(&self) -> u32 {
        unsafe { ncnn_mat_get_dims(self.ptr) as _ }
    }

    /// Returns matrix width.
    pub fn width(&self) -> u32 {
        unsafe { ncnn_mat_get_w(self.ptr) as _ }
    }

    /// Returns matrix height.
    pub fn height(&self) -> u32 {
        unsafe { ncnn_mat_get_h(self.ptr) as _ }
    }

    /// Returns matrix depth.
    pub fn depth(&self) -> u32 {
        unsafe { ncnn_mat_get_d(self.ptr) as _ }
    }

    /// Returns matrix channels.
    pub fn channels(&self) -> u32 {
        unsafe { ncnn_mat_get_c(self.ptr) as _ }
    }

    pub fn element_size(&self) -> u32 {
        unsafe { ncnn_mat_get_elemsize(self.ptr) as _ }
    }

    pub fn element_packing(&self) -> u32 {
        unsafe { ncnn_mat_get_elempack(self.ptr) as _ }
    }

    pub fn channel_step(&self) -> u32 {
        unsafe { ncnn_mat_get_cstep(self.ptr) as _ }
    }

    /// Pointer to raw matrix data
    pub fn data(&self) -> *mut c_void {
        unsafe { ncnn_mat_get_data(self.ptr) }
    }

    pub(crate) fn ptr(&self) -> ncnn_mat_t {
        self.ptr
    }

    pub(crate) fn mut_ptr(&mut self) -> *mut ncnn_mat_t {
        &mut self.ptr
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
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
    border_type: BorderType,
    value: f32,
    opt: &crate::option::Option,
) -> anyhow::Result<()> {
    unsafe {
        let src = src.ptr();
        let dst = dst.ptr();
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
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
    front: u32,
    behind: u32,
    border_type: BorderType,
    value: f32,
    opt: &crate::option::Option,
) -> anyhow::Result<()> {
    unsafe {
        let src = src.ptr();
        let dst = dst.ptr();
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
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
    opt: &crate::option::Option,
) -> anyhow::Result<()> {
    unsafe {
        let src = src.ptr();
        let dst = dst.ptr();
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
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
    front: u32,
    behind: u32,
    opt: &crate::option::Option,
) -> anyhow::Result<()> {
    unsafe {
        let src = src.ptr();
        let dst = dst.ptr();
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
    use super::*;

    #[test]
    fn basic_getter_and_setter() {
        let m = Mat::new_3d(224, 224, 3, None).unwrap();
        assert_eq!(224, m.height());
        assert_eq!(224, m.width());
        assert_eq!(3, m.channels());
    }

    #[test]
    fn stride() {
        assert_eq!(PixelType::Bgr.stride(), 3);
        assert_eq!(PixelType::Bgra.stride(), 4);
        assert_eq!(PixelType::Gray.stride(), 1);
        assert_eq!(PixelType::Rgb.stride(), 3);
        assert_eq!(PixelType::Rgba.stride(), 4);

        assert_eq!(PixelType::RgbToBgr.stride(), 3);
        assert_eq!(PixelType::RgbToGray.stride(), 3);
        assert_eq!(PixelType::RgbToRgba.stride(), 3);
        assert_eq!(PixelType::RgbToBgra.stride(), 3);

        assert_eq!(PixelType::BgrToRgb.stride(), 3);
        assert_eq!(PixelType::BgrToGray.stride(), 3);
        assert_eq!(PixelType::BgrToRgba.stride(), 3);
        assert_eq!(PixelType::BgrToBgra.stride(), 3);

        assert_eq!(PixelType::GrayToRgb.stride(), 1);
        assert_eq!(PixelType::GrayToBgr.stride(), 1);
        assert_eq!(PixelType::GrayToRgba.stride(), 1);
        assert_eq!(PixelType::GrayToBgra.stride(), 1);

        assert_eq!(PixelType::RgbaToRgb.stride(), 4);
        assert_eq!(PixelType::RgbaToBgr.stride(), 4);
        assert_eq!(PixelType::RgbaToGray.stride(), 4);
        assert_eq!(PixelType::RgbaToBgra.stride(), 4);

        assert_eq!(PixelType::BgraToRgb.stride(), 4);
        assert_eq!(PixelType::BgraToBgr.stride(), 4);
        assert_eq!(PixelType::BgraToGray.stride(), 4);
        assert_eq!(PixelType::BgraToRgba.stride(), 4);
    }
}
