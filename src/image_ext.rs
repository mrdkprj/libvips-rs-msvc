#![allow(clippy::too_many_arguments)]
use crate::bindings;
use crate::error::*;
use crate::ops::*;
use crate::utils;
use crate::voption::{call, VOption, VipsValue};
use crate::Result;
use crate::VipsBlob;
use crate::VipsImage;
use crate::VipsSource;
use crate::VipsTarget;
use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_int;
use std::ptr::null_mut;

const NULL: *const std::ffi::c_void = null_mut();

impl VipsImage {
    pub fn as_mut_ptr(&self) -> *mut bindings::VipsImage {
        self.ctx
    }

    pub fn get_typeof(&self, type_: &[u8]) -> u64 {
        unsafe {
            bindings::vips_image_get_typeof(
                self.ctx as _,
                type_.as_ptr() as _,
            )
        }
    }

    pub fn get_int(&self, name: &[u8]) -> Result<i32> {
        unsafe {
            let mut out = 0;
            let res = bindings::vips_image_get_int(
                self.ctx as _,
                name.as_ptr() as _,
                &mut out,
            );
            utils::result(
                res,
                out,
                Error::IOError("Cannot get int"),
            )
        }
    }

    pub fn set_int(&self, name: &[u8], value: i32) {
        unsafe {
            bindings::vips_image_set_int(
                self.ctx,
                name.as_ptr() as _,
                value,
            );
        }
    }

    pub fn get_string(&self, name: &[u8]) -> Result<String> {
        unsafe {
            let mut out: *const c_char = std::ptr::null();
            let res = bindings::vips_image_get_string(
                self.ctx,
                name.as_ptr() as _,
                &mut out,
            );

            utils::result(
                res,
                CStr::from_ptr(out)
                    .to_string_lossy()
                    .into_owned(),
                Error::IOError("Cannot get string"),
            )
        }
    }

    pub fn set_string(&self, name: &[u8], value: &str) {
        unsafe {
            bindings::vips_image_set_string(
                self.ctx,
                name.as_ptr() as _,
                value.as_ptr() as _,
            )
        };
    }

    pub fn get_blob(&self, name: &[u8]) -> Result<Vec<u8>> {
        unsafe {
            let mut out: *const c_void = std::ptr::null();
            let mut length = 0;
            let res = bindings::vips_image_get_blob(
                self.ctx,
                name.as_ptr() as _,
                &mut out,
                &mut length,
            );
            utils::result(
                res,
                std::slice::from_raw_parts(
                    out as *const u8,
                    length as _,
                )
                .to_vec(),
                Error::IOError("Cannot get blob"),
            )
        }
    }

    pub fn set_blob(&self, name: &[u8], blob: &[u8]) {
        unsafe {
            bindings::vips_image_set_blob(
                self.ctx,
                name.as_ptr() as _,
                Some(Self::vips_area_free_cb_wrapper),
                blob.as_ptr() as _,
                blob.len() as _,
            )
        };
    }

    unsafe extern "C" fn vips_area_free_cb_wrapper(mem: *mut c_void, area: *mut c_void) -> c_int {
        // Reinterpret the second argument as *mut VipsArea
        let area = area as *mut bindings::VipsArea;
        bindings::vips_area_free_cb(mem, area)
    }

    pub fn get_array_int(&self, name: &[u8]) -> Result<Vec<i32>> {
        unsafe {
            let mut out: *mut i32 = std::ptr::null_mut();
            let mut size = 0;
            let res = bindings::vips_image_get_array_int(
                self.ctx,
                name.as_ptr() as _,
                &mut out,
                &mut size,
            );
            utils::result(
                res,
                utils::new_int_array(
                    out,
                    size as _,
                ),
                Error::IOError("Cannot get array int"),
            )
        }
    }

    pub fn set_array_int(&self, name: &[u8], value: &[i32]) {
        unsafe {
            bindings::vips_image_set_array_int(
                self.ctx,
                name.as_ptr() as _,
                value.as_ptr(),
                value.len() as _,
            )
        };
    }

    pub fn get_array_double(&self, name: &[u8]) -> Result<Vec<f64>> {
        unsafe {
            let mut out: *mut f64 = std::ptr::null_mut();
            let mut size = 0;
            let res = bindings::vips_image_get_array_double(
                self.ctx,
                name.as_ptr() as _,
                &mut out,
                &mut size,
            );

            utils::result(
                res,
                utils::new_double_array(
                    out,
                    size as _,
                ),
                Error::IOError("Cannot get array double"),
            )
        }
    }

    pub fn set_array_double(&self, name: &[u8], value: &[f64]) {
        unsafe {
            bindings::vips_image_set_array_double(
                self.ctx,
                name.as_ptr() as _,
                value.as_ptr(),
                value.len() as _,
            )
        };
    }

    pub fn remove(&self, name: &[u8]) -> bool {
        unsafe {
            bindings::vips_image_remove(
                self.ctx,
                name.as_ptr() as _,
            ) == 1
        }
    }
}

impl VipsImage {
    /// VipsCMC2LCh (CMC2LCh), transform LCh to CMC
    /// returns `VipsImage` - Output image
    pub fn CMC2LCh(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_CMC2LCh(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Cmc2LCh failed"),
            )
        }
    }

    /// VipsCMYK2XYZ (CMYK2XYZ), transform CMYK to XYZ
    /// returns `VipsImage` - Output image
    pub fn CMYK2XYZ(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_CMYK2XYZ(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Cmyk2Xyz failed"),
            )
        }
    }

    /// VipsHSV2sRGB (HSV2sRGB), transform HSV to sRGB
    /// returns `VipsImage` - Output image
    pub fn HSV2sRGB(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_HSV2sRGB(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Hsv2SRgb failed"),
            )
        }
    }

    /// VipsLCh2CMC (LCh2CMC), transform LCh to CMC
    /// returns `VipsImage` - Output image
    pub fn LCh2CMC(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_LCh2CMC(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("LCh2Cmc failed"),
            )
        }
    }

    /// VipsLCh2Lab (LCh2Lab), transform LCh to Lab
    /// returns `VipsImage` - Output image
    pub fn LCh2Lab(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_LCh2Lab(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("LCh2Lab failed"),
            )
        }
    }

    /// VipsLab2LCh (Lab2LCh), transform Lab to LCh
    /// returns `VipsImage` - Output image
    pub fn Lab2LCh(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_Lab2LCh(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Lab2LCh failed"),
            )
        }
    }

    /// VipsLab2LabQ (Lab2LabQ), transform float Lab to LabQ coding
    /// returns `VipsImage` - Output image
    pub fn Lab2LabQ(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_Lab2LabQ(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Lab2LabQ failed"),
            )
        }
    }

    /// VipsLab2LabS (Lab2LabS), transform float Lab to signed short
    /// returns `VipsImage` - Output image
    pub fn Lab2LabS(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_Lab2LabS(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Lab2LabS failed"),
            )
        }
    }

    /// VipsLab2XYZ (Lab2XYZ), transform CIELAB to XYZ
    /// returns `VipsImage` - Output image
    pub fn Lab2XYZ(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_Lab2XYZ(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Lab2Xyz failed"),
            )
        }
    }

    /// VipsLab2XYZ (Lab2XYZ), transform CIELAB to XYZ
    /// lab_2xyz_options: `&Lab2XyzOptions` -> optional arguments
    /// returns `VipsImage` - Output image
    pub fn Lab2XYZ_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "Lab2XYZ",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("LabQ2SRgb failed"),
        )
    }

    /// VipsLabQ2Lab (LabQ2Lab), unpack a LabQ image to float Lab
    pub fn LabQ2Lab(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_LabQ2Lab(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("LabQ2Lab failed"),
            )
        }
    }

    /// VipsLabQ2LabS (LabQ2LabS), unpack a LabQ image to short Lab
    pub fn LabQ2LabS(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_LabQ2LabS(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("LabQ2LabS failed"),
            )
        }
    }

    /// VipsLabQ2sRGB (LabQ2sRGB), convert a LabQ image to sRGB
    pub fn LabQ2sRGB(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_LabQ2sRGB(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("LabQ2SRgb failed"),
            )
        }
    }

    /// VipsLabS2Lab (LabS2Lab), transform signed short Lab to float
    pub fn LabS2Lab(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_LabS2Lab(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("LabS2Lab failed"),
            )
        }
    }

    /// VipsLabS2LabQ (LabS2LabQ), transform short Lab to LabQ coding
    pub fn LabS2LabQ(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_LabS2LabQ(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("LabS2LabQ failed"),
            )
        }
    }

    /// VipsXYZ2CMYK (XYZ2CMYK), transform XYZ to CMYK
    pub fn XYZ2CMYK(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_XYZ2CMYK(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Xyz2Cmyk failed"),
            )
        }
    }

    /// VipsXYZ2Lab (XYZ2Lab), transform XYZ to Lab
    /// returns `VipsImage` - Output image
    pub fn XYZ2Lab(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_XYZ2Lab(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Xyz2Lab failed"),
            )
        }
    }

    /// VipsXYZ2Lab (XYZ2Lab), transform XYZ to Lab
    /// xyz2_lab_options: `&Xyz2LabOptions` -> optional arguments
    /// returns `VipsImage` - Output image
    pub fn XYZ2Lab_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "XYZ2Yxy",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Xyz2Yxy failed"),
        )
    }

    /// VipsXYZ2Yxy (XYZ2Yxy), transform XYZ to Yxy
    /// returns `VipsImage` - Output image
    pub fn XYZ2Yxy(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_XYZ2Yxy(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Xyz2Yxy failed"),
            )
        }
    }

    /// VipsXYZ2scRGB (XYZ2scRGB), transform XYZ to scRGB
    /// returns `VipsImage` - Output image
    pub fn XYZ2scRGB(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_XYZ2scRGB(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Xyz2ScRgb failed"),
            )
        }
    }

    /// VipsYxy2XYZ (Yxy2XYZ), transform Yxy to XYZ
    /// returns `VipsImage` - Output image
    pub fn Yxy2XYZ(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_Yxy2XYZ(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Yxy2Xyz failed"),
            )
        }
    }

    /// VipsAbs (abs), absolute value of an image
    /// returns `VipsImage` - Output image
    pub fn abs(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "abs",
            VOption::new()
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Ab failed"),
        )
    }

    /// VipsAdd (add), add two images
    /// right: `&VipsImage` -> Right-hand image argument
    /// returns `VipsImage` - Output image
    pub fn add(&self, right: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "add",
            VOption::new()
                .with(
                    "left",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "right",
                    VipsValue::Image(right),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Add failed"),
        )
    }

    /// VipsAddAlpha (addalpha), append an alpha channel
    /// returns `VipsImage` - Output image
    pub fn addalpha(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_addalpha(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Addalpha failed"),
            )
        }
    }

    /// VipsAffine (affine), affine transform of an image
    /// matrix: `[f64]` -> Transformation Matrix coefficient
    /// returns `VipsImage` - Output image
    pub fn affine(&self, matrix: &[f64]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "affine",
            VOption::new()
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "matrix",
                    VipsValue::DoubleArray(matrix),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Affine failed"),
        )
    }

    /// VipsAffine (affine), affine transform of an image
    /// interpolate: VipsInterpolate, interpolate pixels with this
    /// oarea: VipsArrayInt, output rectangle
    /// idx: gdouble, input horizontal offset
    /// idy: gdouble, input vertical offset
    /// odx: gdouble, output horizontal offset
    /// ody: gdouble, output vertical offset
    /// extend: VipsExtend, how to generate new pixels
    /// background: VipsArrayDouble colour for new pixels
    /// premultiplied: gboolean, images are already premultiplied
    pub fn affine_with_opts(&self, matrix: &[f64], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "affine",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "matrix",
                    VipsValue::DoubleArray(matrix),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Affine failed"),
        )
    }

    /// VipsForeignLoadAnalyze (analyzeload), load an Analyze6 image (.img, .hdr), priority=-50, untrusted, is_a, get_flags, get_flags_filename, header, load
    /// filename: `&str` -> Filename to load from
    /// returns `VipsImage` - Output image
    pub fn analyzeload(filename: &str) -> Result<VipsImage> {
        unsafe {
            let filename_in: CString = utils::new_c_string(filename)?;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_analyzeload(
                filename_in.as_ptr(),
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Analyzeload failed"),
            )
        }
    }

    pub fn analyzeload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "analyzeload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Analyzeload failed"),
        )
    }

    /// VipsArrayjoin (arrayjoin), join an array of images
    /// inp: `&mut [VipsImage]` -> Array of input images
    /// returns `VipsImage` - Output image
    pub fn arrayjoin(inp: &[VipsImage]) -> Result<VipsImage> {
        unsafe {
            let (inp_len, mut inp_in) = {
                let len = inp.len();
                let mut input = Vec::new();
                for img in inp {
                    input.push(img.ctx)
                }
                (
                    len as i32,
                    input,
                )
            };
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_arrayjoin(
                inp_in.as_mut_ptr(),
                &mut out_out,
                inp_len,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Arrayjoin failed"),
            )
        }
    }

    pub fn arrayjoin_with_opts(inps: &[VipsImage], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "arrayjoin",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "in",
                    VipsValue::ImageArray(inps),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Arrayjoin failed"),
        )
    }

    /// VipsAutorot (autorot), autorotate image by exif tag
    /// returns `VipsImage` - Output image
    pub fn autorot(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_autorot(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Autorot failed"),
            )
        }
    }

    pub fn autorot_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "autorot",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Autorot failed"),
        )
    }

    /// VipsAvg (avg), find image average
    /// returns `f64` - Output value
    pub fn avg(&self) -> Result<f64> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: f64 = f64::from(0);

            let vips_op_response = bindings::vips_avg(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                out_out,
                Error::OperationError("Avg failed"),
            )
        }
    }

    /// VipsBandbool (bandbool), boolean operation across image bands
    /// boolean: `OperationBoolean` -> Boolean to perform
    ///  `And` -> VIPS_OPERATION_BOOLEAN_AND = 0 [DEFAULT]
    ///  `Or` -> VIPS_OPERATION_BOOLEAN_OR = 1
    ///  `Eor` -> VIPS_OPERATION_BOOLEAN_EOR = 2
    ///  `Lshift` -> VIPS_OPERATION_BOOLEAN_LSHIFT = 3
    ///  `Rshift` -> VIPS_OPERATION_BOOLEAN_RSHIFT = 4
    ///  `Last` -> VIPS_OPERATION_BOOLEAN_LAST = 5
    /// returns `VipsImage` - Output image
    pub fn bandbool(&self, boolean: OperationBoolean) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let boolean_in: i32 = boolean as i32;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_bandbool(
                inp_in,
                &mut out_out,
                boolean_in
                    .try_into()
                    .unwrap(),
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Bandbool failed"),
            )
        }
    }

    /// VipsBandfold (bandfold), fold up x axis into bands
    /// returns `VipsImage` - Output image
    pub fn bandfold(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_bandfold(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Bandfold failed"),
            )
        }
    }

    pub fn bandfold_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "bandfold",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Bandfold failed"),
        )
    }

    /// VipsBandjoin (bandjoin), bandwise join a set of images
    /// inp: `&[VipsImage]` -> Array of input images
    /// returns `VipsImage` - Output image
    pub fn bandjoin(inp: &[VipsImage]) -> Result<VipsImage> {
        unsafe {
            let (inp_len, mut inp_in) = {
                let len = inp.len();
                let mut input = Vec::new();
                for img in inp {
                    input.push(img.ctx)
                }
                (
                    len as i32,
                    input,
                )
            };
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_bandjoin(
                inp_in.as_mut_ptr(),
                &mut out_out,
                inp_len,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Bandjoin failed"),
            )
        }
    }

    /// VipsBandjoinConst (bandjoin_const), append a constant band to an image
    /// c: `&[f64]` -> Array of constants to add
    /// returns `VipsImage` - Output image
    pub fn bandjoin_const(&self, c: &[f64]) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let c_in: *const f64 = c.as_ptr();
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_bandjoin_const(
                inp_in,
                &mut out_out,
                c_in as *mut _,
                c.len() as i32,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("BandjoinConst failed"),
            )
        }
    }

    /// VipsBandmean (bandmean), band-wise average
    /// returns `VipsImage` - Output image
    pub fn bandmean(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_bandmean(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Bandmean failed"),
            )
        }
    }

    pub fn bandmean_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "bandmean",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Bandmean failed"),
        )
    }

    /// VipsBandrank (bandrank), band-wise rank of a set of images
    /// inp: `&mut [VipsImage]` -> Array of input images
    /// returns `VipsImage` - Output image
    pub fn bandrank(inp: &[VipsImage]) -> Result<VipsImage> {
        unsafe {
            let (inp_len, mut inp_in) = {
                let len = inp.len();
                let mut input = Vec::new();
                for img in inp {
                    input.push(img.ctx)
                }
                (
                    len as i32,
                    input,
                )
            };
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_bandrank(
                inp_in.as_mut_ptr(),
                &mut out_out,
                inp_len,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Bandrank failed"),
            )
        }
    }

    pub fn bandrank_with_opts(inps: &[VipsImage], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "bandrank",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "in",
                    VipsValue::ImageArray(inps),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Bandrank failed"),
        )
    }

    /// VipsBandunfold (bandunfold), unfold image bands into x axis
    /// returns `VipsImage` - Output image
    pub fn bandunfold(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_bandunfold(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Bandunfold failed"),
            )
        }
    }

    pub fn bandunfold_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "bandunfold",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Bandunfold failed"),
        )
    }

    /// VipsBlack (black), make a black image
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn black(width: i32, height: i32) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_black(
                &mut out_out,
                width_in,
                height_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Black failed"),
            )
        }
    }

    pub fn black_with_opts(width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "black",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Black failed"),
        )
    }

    /// VipsBoolean (boolean), boolean operation on two images
    /// right: `&VipsImage` -> Right-hand image argument
    /// boolean: `OperationBoolean` -> Boolean to perform
    ///  `And` -> VIPS_OPERATION_BOOLEAN_AND = 0 [DEFAULT]
    ///  `Or` -> VIPS_OPERATION_BOOLEAN_OR = 1
    ///  `Eor` -> VIPS_OPERATION_BOOLEAN_EOR = 2
    ///  `Lshift` -> VIPS_OPERATION_BOOLEAN_LSHIFT = 3
    ///  `Rshift` -> VIPS_OPERATION_BOOLEAN_RSHIFT = 4
    ///  `Last` -> VIPS_OPERATION_BOOLEAN_LAST = 5
    /// returns `VipsImage` - Output image
    pub fn boolean(&self, right: &VipsImage, boolean: OperationBoolean) -> Result<VipsImage> {
        unsafe {
            let left_in: *mut bindings::VipsImage = self.ctx;
            let right_in: *mut bindings::VipsImage = right.ctx;
            let boolean_in: i32 = boolean as i32;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_boolean(
                left_in,
                right_in,
                &mut out_out,
                boolean_in
                    .try_into()
                    .unwrap(),
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Boolean failed"),
            )
        }
    }

    /// VipsBooleanConst (boolean_const), boolean operations against a constant
    /// boolean: `OperationBoolean` -> Boolean to perform
    ///  `And` -> VIPS_OPERATION_BOOLEAN_AND = 0 [DEFAULT]
    ///  `Or` -> VIPS_OPERATION_BOOLEAN_OR = 1
    ///  `Eor` -> VIPS_OPERATION_BOOLEAN_EOR = 2
    ///  `Lshift` -> VIPS_OPERATION_BOOLEAN_LSHIFT = 3
    ///  `Rshift` -> VIPS_OPERATION_BOOLEAN_RSHIFT = 4
    ///  `Last` -> VIPS_OPERATION_BOOLEAN_LAST = 5
    /// c: `&mut [f64]` -> Array of constants
    /// returns `VipsImage` - Output image
    pub fn boolean_const(&self, boolean: OperationBoolean, c: &[f64]) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let boolean_in: i32 = boolean as i32;
            let c_in: *const f64 = c.as_ptr();
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_boolean_const(
                inp_in,
                &mut out_out,
                boolean_in
                    .try_into()
                    .unwrap(),
                c_in as *mut _,
                c.len() as i32,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("BooleanConst failed"),
            )
        }
    }

    /// VipsBuildlut (buildlut), build a look-up table
    /// inp: `&VipsImage` -> Matrix of XY coordinates
    /// returns `VipsImage` - Output image
    pub fn buildlut(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_buildlut(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Buildlut failed"),
            )
        }
    }

    /// VipsByteswap (byteswap), byteswap an image
    /// returns `VipsImage` - Output image
    pub fn byteswap(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_byteswap(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Byteswap failed"),
            )
        }
    }

    /// VipsCanny (canny), Canny edge detector
    /// returns `VipsImage` - Output image
    pub fn canny(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_canny(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Canny failed"),
            )
        }
    }

    pub fn canny_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "canny",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Canny failed"),
        )
    }

    pub fn case_image(&self, cases: &[VipsImage], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "case",
            option
                .with(
                    "index",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "cases",
                    VipsValue::ImageArray(cases),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Case failed"),
        )
    }

    /// VipsCast (cast), cast an image
    /// format: `BandFormat` -> Format to cast to
    ///  `Notset` -> VIPS_FORMAT_NOTSET = -1
    ///  `Uchar` -> VIPS_FORMAT_UCHAR = 0 [DEFAULT]
    ///  `Char` -> VIPS_FORMAT_CHAR = 1
    ///  `Ushort` -> VIPS_FORMAT_USHORT = 2
    ///  `Short` -> VIPS_FORMAT_SHORT = 3
    ///  `Uint` -> VIPS_FORMAT_UINT = 4
    ///  `Int` -> VIPS_FORMAT_INT = 5
    ///  `Float` -> VIPS_FORMAT_FLOAT = 6
    ///  `Complex` -> VIPS_FORMAT_COMPLEX = 7
    ///  `Double` -> VIPS_FORMAT_DOUBLE = 8
    ///  `Dpcomplex` -> VIPS_FORMAT_DPCOMPLEX = 9
    ///  `Last` -> VIPS_FORMAT_LAST = 10
    /// returns `VipsImage` - Output image
    pub fn cast(&self, format: BandFormat) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_cast(
                inp_in,
                &mut out_out,
                format as i32,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Cast failed"),
            )
        }
    }

    pub fn cast_with_opts(&self, format: BandFormat, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "cast",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "format",
                    VipsValue::Int(format as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Cast failed"),
        )
    }

    /// VipsClamp (clamp), clamp values of an image
    /// returns `VipsImage` - Output image
    pub fn clamp(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_clamp(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Clamp failed"),
            )
        }
    }

    pub fn clamp_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "clamp",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Clamp failed"),
        )
    }

    /// VipsColourspace (colourspace), convert to a new colorspace
    /// space: `Interpretation` -> Destination color space
    ///  `Error` -> VIPS_INTERPRETATION_ERROR = -1
    ///  `Multiband` -> VIPS_INTERPRETATION_MULTIBAND = 0
    ///  `BW` -> VIPS_INTERPRETATION_B_W = 1
    ///  `Histogram` -> VIPS_INTERPRETATION_HISTOGRAM = 10
    ///  `Xyz` -> VIPS_INTERPRETATION_XYZ = 12
    ///  `Lab` -> VIPS_INTERPRETATION_LAB = 13
    ///  `Cmyk` -> VIPS_INTERPRETATION_CMYK = 15
    ///  `Labq` -> VIPS_INTERPRETATION_LABQ = 16
    ///  `Rgb` -> VIPS_INTERPRETATION_RGB = 17
    ///  `Cmc` -> VIPS_INTERPRETATION_CMC = 18
    ///  `Lch` -> VIPS_INTERPRETATION_LCH = 19
    ///  `Lab` -> VIPS_INTERPRETATION_LABS = 21
    ///  `Srgb` -> VIPS_INTERPRETATION_sRGB = 22 [DEFAULT]
    ///  `Yxy` -> VIPS_INTERPRETATION_YXY = 23
    ///  `Fourier` -> VIPS_INTERPRETATION_FOURIER = 24
    ///  `Rgb16` -> VIPS_INTERPRETATION_RGB16 = 25
    ///  `Grey16` -> VIPS_INTERPRETATION_GREY16 = 26
    ///  `Matrix` -> VIPS_INTERPRETATION_MATRIX = 27
    ///  `Scrgb` -> VIPS_INTERPRETATION_scRGB = 28
    ///  `Hsv` -> VIPS_INTERPRETATION_HSV = 29
    ///  `Last` -> VIPS_INTERPRETATION_LAST = 30
    /// returns `VipsImage` - Output image
    pub fn colourspace(&self, space: Interpretation) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_colourspace(
                inp_in,
                &mut out_out,
                space as i32,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Colourspace failed"),
            )
        }
    }

    pub fn colourspace_with_opts(
        &self,
        space: Interpretation,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "colourspace",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "space",
                    VipsValue::Int(space as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Colourspace failed"),
        )
    }

    /// VipsCompass (compass), convolve with rotating mask
    /// mask: `&VipsImage` -> Input matrix image
    /// returns `VipsImage` - Output image
    pub fn compass(&self, mask: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mask_in: *mut bindings::VipsImage = mask.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_compass(
                inp_in,
                &mut out_out,
                mask_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Compass failed"),
            )
        }
    }

    pub fn compass_with_opts(&self, mask: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "compass",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "mask",
                    VipsValue::Image(mask),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Compass failed"),
        )
    }

    /// VipsComplex (complex), perform a complex operation on an image
    /// cmplx: `OperationComplex` -> Complex to perform
    ///  `Polar` -> VIPS_OPERATION_COMPLEX_POLAR = 0 [DEFAULT]
    ///  `Rect` -> VIPS_OPERATION_COMPLEX_RECT = 1
    ///  `Conj` -> VIPS_OPERATION_COMPLEX_CONJ = 2
    ///  `Last` -> VIPS_OPERATION_COMPLEX_LAST = 3
    /// returns `VipsImage` - Output image
    pub fn complex(&self, cmplx: OperationComplex) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let cmplx_in: i32 = cmplx as i32;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_complex(
                inp_in,
                &mut out_out,
                cmplx_in
                    .try_into()
                    .unwrap(),
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Complex failed"),
            )
        }
    }

    /// VipsComplex2 (complex2), complex binary operations on two images
    /// left: `&VipsImage` -> Left-hand image argument
    /// right: `&VipsImage` -> Right-hand image argument
    /// cmplx: `OperationComplex2` -> Binary complex operation to perform
    ///  `CrossPhase` -> VIPS_OPERATION_COMPLEX2_CROSS_PHASE = 0 [DEFAULT]
    ///  `Last` -> VIPS_OPERATION_COMPLEX2_LAST = 1
    /// returns `VipsImage` - Output image
    pub fn complex2(&self, right: &VipsImage, cmplx: OperationComplex2) -> Result<VipsImage> {
        unsafe {
            let left_in: *mut bindings::VipsImage = self.ctx;
            let right_in: *mut bindings::VipsImage = right.ctx;
            let cmplx_in: i32 = cmplx as i32;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_complex2(
                left_in,
                right_in,
                &mut out_out,
                cmplx_in
                    .try_into()
                    .unwrap(),
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Complex2 failed"),
            )
        }
    }

    /// VipsComplexform (complexform), form a complex image from two real images
    /// left: `&VipsImage` -> Left-hand image argument
    /// right: `&VipsImage` -> Right-hand image argument
    /// returns `VipsImage` - Output image
    pub fn complexform(&self, right: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let left_in: *mut bindings::VipsImage = self.ctx;
            let right_in: *mut bindings::VipsImage = right.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_complexform(
                left_in,
                right_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Complexform failed"),
            )
        }
    }
    /// VipsComplexget (complexget), get a component from a complex image
    /// get: `OperationComplexget` -> Complex to perform
    ///  `Real` -> VIPS_OPERATION_COMPLEXGET_REAL = 0 [DEFAULT]
    ///  `Imag` -> VIPS_OPERATION_COMPLEXGET_IMAG = 1
    ///  `Last` -> VIPS_OPERATION_COMPLEXGET_LAST = 2
    /// returns `VipsImage` - Output image
    pub fn complexget(&self, get: OperationComplexget) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let get_in: i32 = get as i32;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_complexget(
                inp_in,
                &mut out_out,
                get_in
                    .try_into()
                    .unwrap(),
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Complexget failed"),
            )
        }
    }

    /// VipsComposite (composite), blend an array of images with an array of blend modes
    /// inp: `&mut [VipsImage]` -> Array of input images
    /// mode: `&mut [i32]` -> Array of VipsBlendMode to join with
    /// returns `VipsImage` - Output image
    pub fn composite(&self, inp: &[VipsImage], mode: &[i32]) -> Result<VipsImage> {
        unsafe {
            let (inp_len, mut inp_in) = {
                let len = inp.len();
                let mut input = Vec::new();
                for img in inp {
                    input.push(img.ctx)
                }
                (
                    len as i32,
                    input,
                )
            };
            let mode_in: *const i32 = mode.as_ptr();
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_composite(
                inp_in.as_mut_ptr(),
                &mut out_out,
                inp_len,
                mode_in as *mut _,
                mode.len() as i32,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Composite failed"),
            )
        }
    }

    pub fn composite_with_opts(
        inps: &[VipsImage],
        mode: &[i32],
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "composite",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "in",
                    VipsValue::ImageArray(inps),
                )
                .with(
                    "mode",
                    VipsValue::IntArray(mode),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Composite failed"),
        )
    }

    /// VipsComposite2 (composite2), blend a pair of images with a blend mode
    /// base: `&VipsImage` -> Base image
    /// overlay: `&VipsImage` -> Overlay image
    /// mode: `BlendMode` -> VipsBlendMode to join with
    ///  `Clear` -> VIPS_BLEND_MODE_CLEAR = 0
    ///  `Source` -> VIPS_BLEND_MODE_SOURCE = 1
    ///  `Over` -> VIPS_BLEND_MODE_OVER = 2 [DEFAULT]
    ///  `In` -> VIPS_BLEND_MODE_IN = 3
    ///  `Out` -> VIPS_BLEND_MODE_OUT = 4
    ///  `Atop` -> VIPS_BLEND_MODE_ATOP = 5
    ///  `Dest` -> VIPS_BLEND_MODE_DEST = 6
    ///  `DestOver` -> VIPS_BLEND_MODE_DEST_OVER = 7
    ///  `DestIn` -> VIPS_BLEND_MODE_DEST_IN = 8
    ///  `DestOut` -> VIPS_BLEND_MODE_DEST_OUT = 9
    ///  `DestAtop` -> VIPS_BLEND_MODE_DEST_ATOP = 10
    ///  `Xor` -> VIPS_BLEND_MODE_XOR = 11
    ///  `Add` -> VIPS_BLEND_MODE_ADD = 12
    ///  `Saturate` -> VIPS_BLEND_MODE_SATURATE = 13
    ///  `Multiply` -> VIPS_BLEND_MODE_MULTIPLY = 14
    ///  `Screen` -> VIPS_BLEND_MODE_SCREEN = 15
    ///  `Overlay` -> VIPS_BLEND_MODE_OVERLAY = 16
    ///  `Darken` -> VIPS_BLEND_MODE_DARKEN = 17
    ///  `Lighten` -> VIPS_BLEND_MODE_LIGHTEN = 18
    ///  `ColourDodge` -> VIPS_BLEND_MODE_COLOUR_DODGE = 19
    ///  `ColourBurn` -> VIPS_BLEND_MODE_COLOUR_BURN = 20
    ///  `HardLight` -> VIPS_BLEND_MODE_HARD_LIGHT = 21
    ///  `SoftLight` -> VIPS_BLEND_MODE_SOFT_LIGHT = 22
    ///  `Difference` -> VIPS_BLEND_MODE_DIFFERENCE = 23
    ///  `Exclusion` -> VIPS_BLEND_MODE_EXCLUSION = 24
    ///  `Last` -> VIPS_BLEND_MODE_LAST = 25
    /// returns `VipsImage` - Output image
    pub fn composite2(&self, overlay: &VipsImage, mode: BlendMode) -> Result<VipsImage> {
        unsafe {
            let base_in: *mut bindings::VipsImage = self.ctx;
            let overlay_in: *mut bindings::VipsImage = overlay.ctx;
            let mode_in: i32 = mode as i32;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_composite2(
                base_in,
                overlay_in,
                &mut out_out,
                mode_in
                    .try_into()
                    .unwrap(),
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Composite2 failed"),
            )
        }
    }

    pub fn composite2_with_opts(
        &self,
        overlay: &VipsImage,
        mode: BlendMode,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "composite2",
            option
                .with(
                    "base",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "overlay",
                    VipsValue::Image(overlay),
                )
                .with(
                    "mode",
                    VipsValue::Int(mode as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Composite2 failed"),
        )
    }

    /// VipsConv (conv), convolution operation
    /// mask: `&VipsImage` -> Input matrix image
    /// returns `VipsImage` - Output image
    pub fn conv(&self, mask: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mask_in: *mut bindings::VipsImage = mask.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_conv(
                inp_in,
                &mut out_out,
                mask_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Conv failed"),
            )
        }
    }

    pub fn conv_with_opts(&self, mask: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "conv",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "mask",
                    VipsValue::Image(mask),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Conv failed"),
        )
    }

    /// VipsConva (conva), approximate integer convolution
    /// mask: `&VipsImage` -> Input matrix image
    /// returns `VipsImage` - Output image
    pub fn conva(&self, mask: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mask_in: *mut bindings::VipsImage = mask.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_conva(
                inp_in,
                &mut out_out,
                mask_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Conva failed"),
            )
        }
    }
    pub fn conva_with_opts(&self, mask: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "conva",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "mask",
                    VipsValue::Image(mask),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Conva failed"),
        )
    }

    /// VipsConvasep (convasep), approximate separable integer convolution
    /// mask: `&VipsImage` -> Input matrix image
    /// returns `VipsImage` - Output image
    pub fn convasep(&self, mask: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mask_in: *mut bindings::VipsImage = mask.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_convasep(
                inp_in,
                &mut out_out,
                mask_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Convasep failed"),
            )
        }
    }

    pub fn convasep_with_opts(&self, mask: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "convasep",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "mask",
                    VipsValue::Image(mask),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Convasep failed"),
        )
    }

    /// VipsConvf (convf), float convolution operation
    /// mask: `&VipsImage` -> Input matrix image
    /// returns `VipsImage` - Output image
    pub fn convf(&self, mask: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mask_in: *mut bindings::VipsImage = mask.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_convf(
                inp_in,
                &mut out_out,
                mask_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Convf failed"),
            )
        }
    }
    /// VipsConvi (convi), int convolution operation
    /// mask: `&VipsImage` -> Input matrix image
    /// returns `VipsImage` - Output image
    pub fn convi(&self, mask: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mask_in: *mut bindings::VipsImage = mask.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_convi(
                inp_in,
                &mut out_out,
                mask_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Convi failed"),
            )
        }
    }

    /// VipsConvsep (convsep), separable convolution operation
    /// mask: `&VipsImage` -> Input matrix image
    /// returns `VipsImage` - Output image
    pub fn convsep(&self, mask: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mask_in: *mut bindings::VipsImage = mask.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_convsep(
                inp_in,
                &mut out_out,
                mask_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Convsep failed"),
            )
        }
    }
    pub fn convsep_with_opts(&self, mask: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "convsep",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "mask",
                    VipsValue::Image(mask),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Convsep failed"),
        )
    }

    /// VipsCopy (copy), copy an image
    /// returns `VipsImage` - Output image
    pub fn copy(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_copy(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Copy failed"),
            )
        }
    }

    pub fn copy_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "copy",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Copy failed"),
        )
    }

    /// VipsCountlines (countlines), count lines in an image
    /// direction: `Direction` -> Countlines left-right or up-down
    ///  `Horizontal` -> VIPS_DIRECTION_HORIZONTAL = 0 [DEFAULT]
    ///  `Vertical` -> VIPS_DIRECTION_VERTICAL = 1
    ///  `Last` -> VIPS_DIRECTION_LAST = 2
    /// returns `f64` - Number of lines
    pub fn countlines(&self, direction: Direction) -> Result<f64> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let direction_in: i32 = direction as i32;
            let mut nolines_out: f64 = f64::from(0);

            let vips_op_response = bindings::vips_countlines(
                inp_in,
                &mut nolines_out,
                direction_in
                    .try_into()
                    .unwrap(),
                NULL,
            );
            utils::result(
                vips_op_response,
                nolines_out,
                Error::OperationError("Countline failed"),
            )
        }
    }

    pub fn crop(
        &self,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "crop",
            option
                .with(
                    "input",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "left",
                    VipsValue::Int(left),
                )
                .with(
                    "top",
                    VipsValue::Int(top),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Crop failed"),
        )
    }

    /// VipsForeignLoadCsvFile (csvload), load csv (.csv), priority=0, untrusted, get_flags, get_flags_filename, header, load
    /// filename: `&str` -> Filename to load from
    /// returns `VipsImage` - Output image
    pub fn csvload(filename: &str) -> Result<VipsImage> {
        unsafe {
            let filename_in: CString = utils::new_c_string(filename)?;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_csvload(
                filename_in.as_ptr(),
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Csvload failed"),
            )
        }
    }

    pub fn csvload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "csvload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Csvload failed"),
        )
    }

    /// VipsForeignLoadCsvSource (csvload_source), load csv, priority=0, untrusted, is_a_source, get_flags, header, load
    /// source: `&VipsSource` -> Source to load from
    /// returns `VipsImage` - Output image
    pub fn csvload_source(source: &VipsSource) -> Result<VipsImage> {
        unsafe {
            let source_in: *mut bindings::VipsSource = source.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_csvload_source(
                source_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("CsvloadSource failed"),
            )
        }
    }

    pub fn csvload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "csvload_source",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("CsvloadSource failed"),
        )
    }

    /// VipsForeignSaveCsvFile (csvsave), save image to csv (.csv), priority=0, mono
    /// inp: `&VipsImage` -> Image to save
    /// filename: `&str` -> Filename to save to
    pub fn csvsave(&self, filename: &str) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let filename_in: CString = utils::new_c_string(filename)?;

            let vips_op_response = bindings::vips_csvsave(
                inp_in,
                filename_in.as_ptr(),
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("Csvsave failed"),
            )
        }
    }

    pub fn csvsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "csvsave",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Csvsave failed"),
        )
    }

    /// VipsForeignSaveCsvTarget (csvsave_target), save image to csv (.csv), priority=0, mono
    /// inp: `&VipsImage` -> Image to save
    /// target: `&VipsTarget` -> Target to save to
    pub fn csvsave_target(&self, target: &VipsTarget) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let target_in: *mut bindings::VipsTarget = target.ctx;

            let vips_op_response = bindings::vips_csvsave_target(
                inp_in,
                target_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("CsvsaveTarget failed"),
            )
        }
    }

    pub fn csvsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "csvsave_target",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    VipsValue::Target(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("CsvsaveTarget failed"),
        )
    }

    /// VipsdE00 (dE00), calculate dE00
    /// left: `&VipsImage` -> Left-hand input image
    /// right: `&VipsImage` -> Right-hand input image
    /// returns `VipsImage` - Output image
    pub fn dE00(&self, right: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let left_in: *mut bindings::VipsImage = self.ctx;
            let right_in: *mut bindings::VipsImage = right.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_dE00(
                left_in,
                right_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("DE00 failed"),
            )
        }
    }

    /// VipsdE76 (dE76), calculate dE76
    /// left: `&VipsImage` -> Left-hand input image
    /// right: `&VipsImage` -> Right-hand input image
    /// returns `VipsImage` - Output image
    pub fn dE76(&self, right: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let left_in: *mut bindings::VipsImage = self.ctx;
            let right_in: *mut bindings::VipsImage = right.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_dE76(
                left_in,
                right_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("DE76 failed"),
            )
        }
    }

    /// VipsdECMC (dECMC), calculate dECMC
    /// left: `&VipsImage` -> Left-hand input image
    /// right: `&VipsImage` -> Right-hand input image
    /// returns `VipsImage` - Output image
    pub fn dECMC(&self, right: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let left_in: *mut bindings::VipsImage = self.ctx;
            let right_in: *mut bindings::VipsImage = right.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_dECMC(
                left_in,
                right_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("DEcmc failed"),
            )
        }
    }

    /// VipsDeviate (deviate), find image standard deviation
    /// returns `f64` - Output value
    pub fn deviate(&self) -> Result<f64> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: f64 = f64::from(0);

            let vips_op_response = bindings::vips_deviate(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                out_out,
                Error::OperationError("Deviate failed"),
            )
        }
    }

    /// VipsDivide (divide), divide two images
    /// left: `&VipsImage` -> Left-hand image argument
    /// right: `&VipsImage` -> Right-hand image argument
    /// returns `VipsImage` - Output image
    pub fn divide(&self, right: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let left_in: *mut bindings::VipsImage = self.ctx;
            let right_in: *mut bindings::VipsImage = right.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_divide(
                left_in,
                right_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Divide failed"),
            )
        }
    }

    /// VipsDrawCircle (draw_circle), draw a circle on an image
    /// image: `&VipsImage` -> Image to draw on
    /// ink: `&mut [f64]` -> Color for pixels
    /// cx: `i32` -> Centre of draw_circle
    /// min: -1000000000, max: 1000000000, default: 0
    /// cy: `i32` -> Centre of draw_circle
    /// min: -1000000000, max: 1000000000, default: 0
    /// radius: `i32` -> Radius in pixels
    /// min: 0, max: 1000000000, default: 0

    pub fn draw_circle(&self, ink: &[f64], cx: i32, cy: i32, radius: i32) -> Result<()> {
        unsafe {
            let image_in: *mut bindings::VipsImage = self.ctx;
            let ink_in: *const f64 = ink.as_ptr();
            let cx_in: i32 = cx;
            let cy_in: i32 = cy;
            let radius_in: i32 = radius;

            let vips_op_response = bindings::vips_draw_circle(
                image_in,
                ink_in as _,
                ink.len() as i32,
                cx_in,
                cy_in,
                radius_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("DrawCircle failed"),
            )
        }
    }

    pub fn draw_circle_with_opts(
        &self,
        ink: &[f64],
        cx: i32,
        cy: i32,
        radius: i32,
        option: VOption,
    ) -> Result<()> {
        let vips_op_response = call(
            "draw_circle",
            option
                .with(
                    "image",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "ink",
                    VipsValue::DoubleArray(ink),
                )
                .with(
                    "cx",
                    VipsValue::Int(cx),
                )
                .with(
                    "cy",
                    VipsValue::Int(cy),
                )
                .with(
                    "radius",
                    VipsValue::Int(radius),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("DrawCircle failed"),
        )
    }

    /// VipsDrawFlood (draw_flood), flood-fill an area
    /// image: `&VipsImage` -> Image to draw on
    /// ink: `&mut [f64]` -> Color for pixels
    /// x: `i32` -> DrawFlood start point
    /// min: 0, max: 1000000000, default: 0
    /// y: `i32` -> DrawFlood start point
    /// min: 0, max: 1000000000, default: 0
    pub fn draw_flood(&self, ink: &[f64], x: i32, y: i32) -> Result<()> {
        unsafe {
            let image_in: *mut bindings::VipsImage = self.ctx;
            let ink_in: *const f64 = ink.as_ptr();
            let x_in: i32 = x;
            let y_in: i32 = y;

            let vips_op_response = bindings::vips_draw_flood(
                image_in,
                ink_in as _,
                ink.len() as i32,
                x_in,
                y_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("DrawFlood failed"),
            )
        }
    }

    pub fn draw_flood_with_opts(&self, ink: &[f64], x: i32, y: i32, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "draw_flood",
            option
                .with(
                    "image",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "ink",
                    VipsValue::DoubleArray(ink),
                )
                .with(
                    "x",
                    VipsValue::Int(x),
                )
                .with(
                    "y",
                    VipsValue::Int(y),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("DrawFlood failed"),
        )
    }

    /// VipsDrawImage (draw_image), paint an image into another image
    /// image: `&VipsImage` -> Image to draw on
    /// sub: `&VipsImage` -> Sub-image to insert into main image
    /// x: `i32` -> Draw image here
    /// min: -1000000000, max: 1000000000, default: 0
    /// y: `i32` -> Draw image here
    /// min: -1000000000, max: 1000000000, default: 0
    pub fn draw_image(&self, sub: &VipsImage, x: i32, y: i32) -> Result<()> {
        unsafe {
            let image_in: *mut bindings::VipsImage = self.ctx;
            let sub_in: *mut bindings::VipsImage = sub.ctx;
            let x_in: i32 = x;
            let y_in: i32 = y;

            let vips_op_response = bindings::vips_draw_image(
                image_in,
                sub_in,
                x_in,
                y_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("DrawImage failed"),
            )
        }
    }

    pub fn draw_image_with_opts(
        &self,
        sub: &VipsImage,
        x: i32,
        y: i32,
        option: VOption,
    ) -> Result<()> {
        let vips_op_response = call(
            "draw_image",
            option
                .with(
                    "image",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "sub",
                    VipsValue::Image(sub),
                )
                .with(
                    "x",
                    VipsValue::Int(x),
                )
                .with(
                    "y",
                    VipsValue::Int(y),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("DrawImage failed"),
        )
    }

    /// VipsDrawLine (draw_line), draw a line on an image
    /// image: `&VipsImage` -> Image to draw on
    /// ink: `&mut [f64]` -> Color for pixels
    /// x1: `i32` -> Start of draw_line
    /// min: -1000000000, max: 1000000000, default: 0
    /// y1: `i32` -> Start of draw_line
    /// min: -1000000000, max: 1000000000, default: 0
    /// x2: `i32` -> End of draw_line
    /// min: -1000000000, max: 1000000000, default: 0
    /// y2: `i32` -> End of draw_line
    /// min: -1000000000, max: 1000000000, default: 0
    pub fn draw_line(&self, ink: &[f64], x1: i32, y1: i32, x2: i32, y2: i32) -> Result<()> {
        unsafe {
            let image_in: *mut bindings::VipsImage = self.ctx;
            let ink_in: *const f64 = ink.as_ptr();
            let x_1_in: i32 = x1;
            let y_1_in: i32 = y1;
            let x_2_in: i32 = x2;
            let y_2_in: i32 = y2;

            let vips_op_response = bindings::vips_draw_line(
                image_in,
                ink_in as _,
                ink.len() as i32,
                x_1_in,
                y_1_in,
                x_2_in,
                y_2_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("DrawLine failed"),
            )
        }
    }

    /// VipsDrawMask (draw_mask), draw a mask on an image
    /// image: `&VipsImage` -> Image to draw on
    /// ink: `&mut [f64]` -> Color for pixels
    /// mask: `&VipsImage` -> Mask of pixels to draw
    /// x: `i32` -> Draw mask here
    /// min: -1000000000, max: 1000000000, default: 0
    /// y: `i32` -> Draw mask here
    /// min: -1000000000, max: 1000000000, default: 0
    pub fn draw_mask(&self, ink: &[f64], mask: &VipsImage, x: i32, y: i32) -> Result<()> {
        unsafe {
            let image_in: *mut bindings::VipsImage = self.ctx;
            let ink_in: *const f64 = ink.as_ptr();
            let mask_in: *mut bindings::VipsImage = mask.ctx;
            let x_in: i32 = x;
            let y_in: i32 = y;

            let vips_op_response = bindings::vips_draw_mask(
                image_in,
                ink_in as _,
                ink.len() as i32,
                mask_in,
                x_in,
                y_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("DrawMask failed"),
            )
        }
    }

    /// VipsDrawRect (draw_rect), paint a rectangle on an image
    /// image: `&VipsImage` -> Image to draw on
    /// ink: `&mut [f64]` -> Color for pixels
    /// left: `i32` -> Rect to fill
    /// min: -1000000000, max: 1000000000, default: 0
    /// top: `i32` -> Rect to fill
    /// min: -1000000000, max: 1000000000, default: 0
    /// width: `i32` -> Rect to fill
    /// min: -1000000000, max: 1000000000, default: 0
    /// height: `i32` -> Rect to fill
    /// min: -1000000000, max: 1000000000, default: 0
    pub fn draw_rect(
        &self,
        ink: &[f64],
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> Result<()> {
        unsafe {
            let image_in: *mut bindings::VipsImage = self.ctx;
            let ink_in: *const f64 = ink.as_ptr();
            let left_in: i32 = left;
            let top_in: i32 = top;
            let width_in: i32 = width;
            let height_in: i32 = height;

            let vips_op_response = bindings::vips_draw_rect(
                image_in,
                ink_in as _,
                ink.len() as i32,
                left_in,
                top_in,
                width_in,
                height_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("DrawRect failed"),
            )
        }
    }
    pub fn draw_rect_with_opts(
        &self,
        ink: &[f64],
        left: i32,
        top: i32,
        width: i32,
        height: i32,
        option: VOption,
    ) -> Result<()> {
        let vips_op_response = call(
            "draw_rect",
            option
                .with(
                    "image",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "ink",
                    VipsValue::DoubleArray(ink),
                )
                .with(
                    "left",
                    VipsValue::Int(left),
                )
                .with(
                    "top",
                    VipsValue::Int(top),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("DrawRect failed"),
        )
    }
    /// VipsDrawSmudge (draw_smudge), blur a rectangle on an image
    /// image: `&VipsImage` -> Image to draw on
    /// left: `i32` -> Rect to fill
    /// min: -1000000000, max: 1000000000, default: 0
    /// top: `i32` -> Rect to fill
    /// min: -1000000000, max: 1000000000, default: 0
    /// width: `i32` -> Rect to fill
    /// min: -1000000000, max: 1000000000, default: 0
    /// height: `i32` -> Rect to fill
    /// min: -1000000000, max: 1000000000, default: 0
    pub fn draw_smudge(&self, left: i32, top: i32, width: i32, height: i32) -> Result<()> {
        unsafe {
            let image_in: *mut bindings::VipsImage = self.ctx;
            let left_in: i32 = left;
            let top_in: i32 = top;
            let width_in: i32 = width;
            let height_in: i32 = height;

            let vips_op_response = bindings::vips_draw_smudge(
                image_in,
                left_in,
                top_in,
                width_in,
                height_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("DrawSmudge failed"),
            )
        }
    }

    pub fn dzsave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "dzsave",
            VOption::new()
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("dzsave failed"),
        )
    }

    pub fn dzsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "dzsave",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("dzsave_with_opts failed"),
        )
    }

    pub fn dzsave_buffer(&self) -> Result<Vec<u8>> {
        let mut blob = VipsBlob::from(null_mut());

        let vips_op_response = call(
            "dzsave_buffer",
            VOption::new()
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    VipsValue::MutBlob(&mut blob),
                ),
        );
        utils::result(
            vips_op_response,
            blob.into(),
            Error::OperationError("dzsave_buffer failed"),
        )
    }

    pub fn dzsave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut blob = VipsBlob::from(null_mut());

        let vips_op_response = call(
            "dzsave_buffer",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    VipsValue::MutBlob(&mut blob),
                ),
        );
        utils::result(
            vips_op_response,
            blob.into(),
            Error::OperationError("dzsave_buffer_with_opts failed"),
        )
    }

    pub fn dzsave_target(&self, target: &VipsTarget) -> Result<()> {
        let vips_op_response = call(
            "dzsave_target",
            VOption::new()
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    VipsValue::Target(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("dzsave_target failed"),
        )
    }

    pub fn dzsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "dzsave_target",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    VipsValue::Target(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("dzsave_target_with_opts failed"),
        )
    }

    /// VipsEmbed (embed), embed an image in a larger image
    /// x: `i32` -> Left edge of input in output
    /// min: -1000000000, max: 1000000000, default: 0
    /// y: `i32` -> Top edge of input in output
    /// min: -1000000000, max: 1000000000, default: 0
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 1000000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 1000000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn embed(&self, x: i32, y: i32, width: i32, height: i32) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let x_in: i32 = x;
            let y_in: i32 = y;
            let width_in: i32 = width;
            let height_in: i32 = height;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_embed(
                inp_in,
                &mut out_out,
                x_in,
                y_in,
                width_in,
                height_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Embed failed"),
            )
        }
    }

    pub fn embed_with_opts(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "embed",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "x",
                    VipsValue::Int(x),
                )
                .with(
                    "y",
                    VipsValue::Int(y),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("embed_with_opts failed"),
        )
    }

    /// VipsExtractArea (extract_area), extract an area from an image
    /// input: `&VipsImage` -> Input image
    /// left: `i32` -> Left edge of extract area
    /// min: -100000000, max: 100000000, default: 0
    /// top: `i32` -> Top edge of extract area
    /// min: -100000000, max: 100000000, default: 0
    /// width: `i32` -> Width of extract area
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Height of extract area
    /// min: 1, max: 100000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn extract_area(&self, left: i32, top: i32, width: i32, height: i32) -> Result<VipsImage> {
        unsafe {
            let input_in: *mut bindings::VipsImage = self.ctx;
            let left_in: i32 = left;
            let top_in: i32 = top;
            let width_in: i32 = width;
            let height_in: i32 = height;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_extract_area(
                input_in,
                &mut out_out,
                left_in,
                top_in,
                width_in,
                height_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("ExtractArea failed"),
            )
        }
    }

    /// VipsExtractBand (extract_band), extract band from an image
    /// band: `i32` -> Band to extract
    /// min: 0, max: 100000000, default: 0
    /// returns `VipsImage` - Output image
    pub fn extract_band(&self, band: i32) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let band_in: i32 = band;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_extract_band(
                inp_in,
                &mut out_out,
                band_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("ExtractBand failed"),
            )
        }
    }

    pub fn extract_band_with_opts(&self, band: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "extract_band",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "band",
                    VipsValue::Int(band),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ExtractBand failed"),
        )
    }

    /// VipsEye (eye), make an image showing the eye's spatial response
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn eye(width: i32, height: i32) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_eye(
                &mut out_out,
                width_in,
                height_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Eye failed"),
            )
        }
    }

    pub fn eye_with_opts(width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "eye",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Eye failed"),
        )
    }

    /// VipsFalsecolour (falsecolour), false-color an image
    /// returns `VipsImage` - Output image
    pub fn falsecolour(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_falsecolour(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Falsecolour failed"),
            )
        }
    }

    /// VipsFastcor (fastcor), fast correlation
    /// refp: `&VipsImage` -> Input reference image
    /// returns `VipsImage` - Output image
    pub fn fastcor(&self, refp: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let refp_in: *mut bindings::VipsImage = refp.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_fastcor(
                inp_in,
                refp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Fastcor failed"),
            )
        }
    }

    /// VipsFillNearest (fill_nearest), fill image zeros with nearest non-zero pixel
    /// returns `VipsImage` - Value of nearest non-zero pixel
    pub fn fill_nearest(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_fill_nearest(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("FillNearest failed"),
            )
        }
    }

    pub fn fill_nearest_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "fill_nearest",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("FillNearest failed"),
        )
    }

    /// VipsFindTrim (find_trim), search an image for non-edge areas
    /// inp: `&VipsImage` -> Image to find_trim
    /// Tuple (
    /// i32 - Left edge of image
    /// i32 - Top edge of extract area
    /// i32 - Width of extract area
    /// i32 - Height of extract area
    ///)
    pub fn find_trim(
        &self,
    ) -> Result<(
        i32,
        i32,
        i32,
        i32,
    )> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut left_out: i32 = 1;
            let mut top_out: i32 = 0;
            let mut width_out: i32 = 1;
            let mut height_out: i32 = 1;

            let vips_op_response = bindings::vips_find_trim(
                inp_in,
                &mut left_out,
                &mut top_out,
                &mut width_out,
                &mut height_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                (
                    left_out,
                    top_out,
                    width_out,
                    height_out,
                ),
                Error::OperationError("FindTrim failed"),
            )
        }
    }

    pub fn find_trim_with_opts(
        &self,
        option: VOption,
    ) -> Result<(
        i32,
        i32,
        i32,
        i32,
    )> {
        let mut left: i32 = 1;
        let mut top: i32 = 0;
        let mut width: i32 = 1;
        let mut height: i32 = 1;

        let vips_op_response = call(
            "find_trim",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "left",
                    VipsValue::MutInt(&mut left),
                )
                .with(
                    "top",
                    VipsValue::MutInt(&mut top),
                )
                .with(
                    "width",
                    VipsValue::MutInt(&mut width),
                )
                .with(
                    "height",
                    VipsValue::MutInt(&mut height),
                ),
        );

        utils::result(
            vips_op_response,
            (
                left,
                top,
                width,
                height,
            ),
            Error::OperationError("FindTrim failed"),
        )
    }

    pub fn fitsload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "fitsload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("fitsloade failed"),
        )
    }

    pub fn fitsload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "fitsload_source",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("fitsload_source failed"),
        )
    }

    pub fn fitssave(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "fitssave",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("fitssave failed"),
        )
    }

    /// VipsFlatten (flatten), flatten alpha out of an image
    /// returns `VipsImage` - Output image
    pub fn flatten(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_flatten(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Flatten failed"),
            )
        }
    }

    pub fn flatten_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "flatten",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Flatten failed"),
        )
    }

    /// VipsFlip (flip), flip an image
    /// direction: `Direction` -> Direction to flip image
    ///  `Horizontal` -> VIPS_DIRECTION_HORIZONTAL = 0 [DEFAULT]
    ///  `Vertical` -> VIPS_DIRECTION_VERTICAL = 1
    ///  `Last` -> VIPS_DIRECTION_LAST = 2
    /// returns `VipsImage` - Output image
    pub fn flip(&self, direction: Direction) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let direction_in: i32 = direction as i32;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_flip(
                inp_in,
                &mut out_out,
                direction_in
                    .try_into()
                    .unwrap(),
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Flip failed"),
            )
        }
    }

    /// VipsFloat2rad (float2rad), transform float RGB to Radiance coding
    /// returns `VipsImage` - Output image
    pub fn float2rad(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_float2rad(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Float2Rad failed"),
            )
        }
    }

    /// VipsFractsurf (fractsurf), make a fractal surface
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 64
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 64
    /// fractal_dimension: `f64` -> Fractal dimension
    /// min: 2, max: 3, default: 2.5
    /// returns `VipsImage` - Output image
    pub fn fractsurf(width: i32, height: i32, fractal_dimension: f64) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let fractal_dimension_in: f64 = fractal_dimension;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_fractsurf(
                &mut out_out,
                width_in,
                height_in,
                fractal_dimension_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Fractsurf failed"),
            )
        }
    }

    /// VipsFreqmult (freqmult), frequency-domain filtering
    /// mask: `&VipsImage` -> Input mask image
    /// returns `VipsImage` - Output image
    pub fn freqmult(&self, mask: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mask_in: *mut bindings::VipsImage = mask.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_freqmult(
                inp_in,
                mask_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Freqmult failed"),
            )
        }
    }

    /// VipsFwfft (fwfft), forward FFT
    /// returns `VipsImage` - Output image
    pub fn fwfft(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_fwfft(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Fwfft failed"),
            )
        }
    }

    /// VipsGamma (gamma), gamma an image
    /// returns `VipsImage` - Output image
    pub fn gamma(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_gamma(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Gamma failed"),
            )
        }
    }

    pub fn gamma_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "gamma",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Gamma failed"),
        )
    }

    /// VipsGaussblur (gaussblur), gaussian blur
    /// sigma: `f64` -> Sigma of Gaussian
    /// min: 0, max: 1000, default: 1.5
    /// returns `VipsImage` - Output image
    pub fn gaussblur(&self, sigma: f64) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let sigma_in: f64 = sigma;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_gaussblur(
                inp_in,
                &mut out_out,
                sigma_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Gaussblur failed"),
            )
        }
    }

    pub fn gaussblur_with_opts(&self, sigma: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "gaussblur",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "sigma",
                    VipsValue::Double(sigma),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Gaussblur failed"),
        )
    }

    /// VipsGaussmat (gaussmat), make a gaussian image
    /// sigma: `f64` -> Sigma of Gaussian
    /// min: 0.000001, max: 10000, default: 1
    /// min_ampl: `f64` -> Minimum amplitude of Gaussian
    /// min: 0.000001, max: 10000, default: 0.1
    /// returns `VipsImage` - Output image
    pub fn gaussmat(sigma: f64, min_ampl: f64) -> Result<VipsImage> {
        unsafe {
            let sigma_in: f64 = sigma;
            let min_ampl_in: f64 = min_ampl;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_gaussmat(
                &mut out_out,
                sigma_in,
                min_ampl_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Gaussmat failed"),
            )
        }
    }

    pub fn gaussmat_with_opts(sigma: f64, min_ampl: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "gaussmat",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "sigma",
                    VipsValue::Double(sigma),
                )
                .with(
                    "min_ampl",
                    VipsValue::Double(min_ampl),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Gaussmat failed"),
        )
    }

    /// VipsGaussnoise (gaussnoise), make a gaussnoise image
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn gaussnoise(width: i32, height: i32) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_gaussnoise(
                &mut out_out,
                width_in,
                height_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Gaussnoise failed"),
            )
        }
    }

    pub fn gaussnoise_with_opts(width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "gaussnoise",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Gaussnoise failed"),
        )
    }

    // pub fn getpoint(&self, x: i32, y: i32) -> Result<Vec<f64>> {
    //     unsafe {
    //         let inp_in: *mut bindings::VipsImage = self.ctx;
    //         let mut out_array_size: i32 = 0;
    //         let mut out_array: *mut f64 = null_mut();

    //         let vips_op_response = bindings::vips_getpoint(
    //             inp_in,
    //             &mut out_array,
    //             &mut out_array_size,
    //             x,
    //             y,
    //             NULL,
    //         );
    //         utils::result(
    //             vips_op_response,
    //             utils::new_double_array(
    //                 out_array,
    //                 out_array_size
    //                     .try_into()
    //                     .unwrap(),
    //             ),
    //             Error::OperationError("Gaussnoise failed"),
    //         )
    //     }
    // }
    pub fn getpoint(&self, x: i32, y: i32) -> Result<Vec<f64>> {
        let mut out: Vec<f64> = Vec::new();
        let vips_op_response = call(
            "getpoint",
            VOption::new()
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out_array",
                    VipsValue::MutDoubleArray(&mut out),
                )
                .with(
                    "x",
                    VipsValue::Int(x),
                )
                .with(
                    "y",
                    VipsValue::Int(y),
                ),
        );
        utils::result(
            vips_op_response,
            out,
            Error::OperationError("Getpoint failed"),
        )
    }

    /// VipsForeignLoadNsgifFile (gifload), load GIF with libnsgif (.gif), priority=50, is_a, get_flags, get_flags_filename, header, load
    /// filename: `&str` -> Filename to load from
    /// returns `VipsImage` - Output image
    pub fn gifload(filename: &str) -> Result<VipsImage> {
        unsafe {
            let filename_in: CString = utils::new_c_string(filename)?;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_gifload(
                filename_in.as_ptr(),
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Gifload failed"),
            )
        }
    }

    pub fn gifload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "gifload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Gifload failed"),
        )
    }

    /// VipsForeignLoadNsgifBuffer (gifload_buffer), load GIF with libnsgif, priority=50, is_a_buffer, get_flags, get_flags_filename, header, load
    /// buffer: `&[u8]` -> Buffer to load from
    /// returns `VipsImage` - Output image
    pub fn gifload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        unsafe {
            let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_gifload_buffer(
                buffer_in,
                buffer.len() as u64,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("GifloadBuffer failed"),
            )
        }
    }

    pub fn gifload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "gifload_buffer",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "buffer",
                    VipsValue::Buffer(buffer),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("GifloadBuffer failed"),
        )
    }

    /// VipsForeignLoadNsgifSource (gifload_source), load gif from source, priority=50, is_a_source, get_flags, get_flags_filename, header, load
    /// source: `&VipsSource` -> Source to load from
    /// returns `VipsImage` - Output image
    pub fn gifload_source(source: &VipsSource) -> Result<VipsImage> {
        unsafe {
            let source_in: *mut bindings::VipsSource = source.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_gifload_source(
                source_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("GifloadSource failed"),
            )
        }
    }

    pub fn gifload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "gifload_source",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("GifloadSource failed"),
        )
    }

    /// VipsForeignSaveCgifFile (gifsave), save as gif (.gif), priority=0, rgba-only
    /// inp: `&VipsImage` -> Image to save
    /// filename: `&str` -> Filename to save to
    pub fn gifsave(&self, filename: &str) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let filename_in: CString = utils::new_c_string(filename)?;

            let vips_op_response = bindings::vips_gifsave(
                inp_in,
                filename_in.as_ptr(),
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("Gifsave failed"),
            )
        }
    }

    pub fn gifsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "gifsave",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Gifsave failed"),
        )
    }

    /// VipsForeignSaveCgifBuffer (gifsave_buffer), save as gif (.gif), priority=0, rgba-only
    /// inp: `&VipsImage` -> Image to save
    /// returns `Vec<u8>` - Buffer to save to
    pub fn gifsave_buffer(&self) -> Result<Vec<u8>> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut buffer_buf_size: u64 = 0;
            let mut buffer_out: *mut c_void = null_mut();

            let vips_op_response = bindings::vips_gifsave_buffer(
                inp_in,
                &mut buffer_out,
                &mut buffer_buf_size,
                NULL,
            );
            utils::result(
                vips_op_response,
                utils::new_byte_array(
                    buffer_out,
                    buffer_buf_size,
                ),
                Error::OperationError("GifsaveBuffer failed"),
            )
        }
    }

    pub fn gifsave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut blob = VipsBlob::from(null_mut());

        let vips_op_response = call(
            "gifsave_buffer",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    VipsValue::MutBlob(&mut blob),
                ),
        );
        utils::result(
            vips_op_response,
            blob.into(),
            Error::OperationError("GifsaveBuffer failed"),
        )
    }

    /// VipsForeignSaveCgifTarget (gifsave_target), save as gif (.gif), priority=0, rgba-only
    /// inp: `&VipsImage` -> Image to save
    /// target: `&VipsTarget` -> Target to save to
    pub fn gifsave_target(&self, target: &VipsTarget) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let target_in: *mut bindings::VipsTarget = target.ctx;

            let vips_op_response = bindings::vips_gifsave_target(
                inp_in,
                target_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("GifsaveTarget failed"),
            )
        }
    }

    pub fn gifsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "gifsave_target",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    VipsValue::Target(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("GifsaveTarget failed"),
        )
    }

    /// VipsGlobalbalance (globalbalance), global balance an image mosaic
    /// returns `VipsImage` - Output image
    pub fn globalbalance(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_globalbalance(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Globalbalance failed"),
            )
        }
    }

    pub fn globalbalance_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "globalbalance",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Globalbalance failed"),
        )
    }

    /// VipsGravity (gravity), place an image within a larger image with a certain gravity
    /// direction: `CompassDirection` -> Direction to place image within width/height
    ///  `Centre` -> VIPS_COMPASS_DIRECTION_CENTRE = 0 [DEFAULT]
    ///  `North` -> VIPS_COMPASS_DIRECTION_NORTH = 1
    ///  `East` -> VIPS_COMPASS_DIRECTION_EAST = 2
    ///  `South` -> VIPS_COMPASS_DIRECTION_SOUTH = 3
    ///  `West` -> VIPS_COMPASS_DIRECTION_WEST = 4
    ///  `NorthEast` -> VIPS_COMPASS_DIRECTION_NORTH_EAST = 5
    ///  `SouthEast` -> VIPS_COMPASS_DIRECTION_SOUTH_EAST = 6
    ///  `SouthWest` -> VIPS_COMPASS_DIRECTION_SOUTH_WEST = 7
    ///  `NorthWest` -> VIPS_COMPASS_DIRECTION_NORTH_WEST = 8
    ///  `Last` -> VIPS_COMPASS_DIRECTION_LAST = 9
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 1000000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 1000000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn gravity(
        &self,
        direction: CompassDirection,
        width: i32,
        height: i32,
    ) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let direction_in: i32 = direction as i32;
            let width_in: i32 = width;
            let height_in: i32 = height;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_gravity(
                inp_in,
                &mut out_out,
                direction_in
                    .try_into()
                    .unwrap(),
                width_in,
                height_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Gravity failed"),
            )
        }
    }

    pub fn gravity_with_opts(
        &self,
        direction: CompassDirection,
        width: i32,
        height: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "gravity",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "direction",
                    VipsValue::Int(direction as i32),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Gravity failed"),
        )
    }

    /// VipsGrey (grey), make a grey ramp image
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn grey(width: i32, height: i32) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_grey(
                &mut out_out,
                width_in,
                height_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Grey failed"),
            )
        }
    }

    pub fn grey_with_opts(width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "grey",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Grey failed"),
        )
    }

    /// VipsGrid (grid), grid an image
    /// tile_height: `i32` -> Chop into tiles this high
    /// min: 1, max: 10000000, default: 128
    /// across: `i32` -> Number of tiles across
    /// min: 1, max: 10000000, default: 1
    /// down: `i32` -> Number of tiles down
    /// min: 1, max: 10000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn grid(&self, tile_height: i32, across: i32, down: i32) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let tile_height_in: i32 = tile_height;
            let across_in: i32 = across;
            let down_in: i32 = down;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_grid(
                inp_in,
                &mut out_out,
                tile_height_in,
                across_in,
                down_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Grid failed"),
            )
        }
    }

    /// VipsForeignLoadHeifFile (heifload), load a HEIF image (.heic, .heif, .avif), priority=0, is_a, get_flags, header, load
    /// filename: `&str` -> Filename to load from
    /// returns `VipsImage` - Output image
    pub fn heifload(filename: &str) -> Result<VipsImage> {
        unsafe {
            let filename_in: CString = utils::new_c_string(filename)?;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_heifload(
                filename_in.as_ptr(),
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Heifload failed"),
            )
        }
    }
    pub fn heifload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "heifload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Heifload failed"),
        )
    }

    /// VipsForeignLoadHeifBuffer (heifload_buffer), load a HEIF image, priority=0, is_a_buffer, get_flags, header, load
    /// buffer: `&[u8]` -> Buffer to load from
    /// returns `VipsImage` - Output image
    pub fn heifload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        unsafe {
            let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_heifload_buffer(
                buffer_in,
                buffer.len() as u64,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("HeifloadBuffer failed"),
            )
        }
    }

    pub fn heifload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "heifload_buffer",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "buffer",
                    VipsValue::Buffer(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HeifloadBuffer failed"),
        )
    }

    /// VipsForeignLoadHeifSource (heifload_source), load a HEIF image, priority=0, is_a_source, get_flags, header, load
    /// source: `&VipsSource` -> Source to load from
    /// returns `VipsImage` - Output image
    pub fn heifload_source(source: &VipsSource) -> Result<VipsImage> {
        unsafe {
            let source_in: *mut bindings::VipsSource = source.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_heifload_source(
                source_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("HeifloadSource failed"),
            )
        }
    }

    pub fn heifload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "heifload_source",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HeifloadSource failed"),
        )
    }

    /// VipsForeignSaveHeifFile (heifsave), save image in HEIF format (.heic, .heif, .avif), priority=0, rgba-only
    /// inp: `&VipsImage` -> Image to save
    /// filename: `&str` -> Filename to save to
    pub fn heifsave(&self, filename: &str) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let filename_in: CString = utils::new_c_string(filename)?;

            let vips_op_response = bindings::vips_heifsave(
                inp_in,
                filename_in.as_ptr(),
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("Heifsave failed"),
            )
        }
    }

    pub fn heifsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "heifsave",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Heifsave failed"),
        )
    }

    /// VipsForeignSaveHeifBuffer (heifsave_buffer), save image in HEIF format (.heic, .heif), priority=0, rgba-only
    /// inp: `&VipsImage` -> Image to save
    /// returns `Vec<u8>` - Buffer to save to
    pub fn heifsave_buffer(&self) -> Result<Vec<u8>> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut buffer_buf_size: u64 = 0;
            let mut buffer_out: *mut c_void = null_mut();

            let vips_op_response = bindings::vips_heifsave_buffer(
                inp_in,
                &mut buffer_out,
                &mut buffer_buf_size,
                NULL,
            );
            utils::result(
                vips_op_response,
                utils::new_byte_array(
                    buffer_out,
                    buffer_buf_size,
                ),
                Error::OperationError("HeifsaveBuffer failed"),
            )
        }
    }

    pub fn heifsave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut blob = VipsBlob::from(null_mut());

        let vips_op_response = call(
            "heifsave_buffer",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    VipsValue::MutBlob(&mut blob),
                ),
        );

        utils::result(
            vips_op_response,
            blob.into(),
            Error::OperationError("HeifsaveBuffer failed"),
        )
    }

    /// VipsForeignSaveHeifTarget (heifsave_target), save image in HEIF format (.heic, .heif), priority=0, rgba-only
    /// inp: `&VipsImage` -> Image to save
    /// target: `&VipsTarget` -> Target to save to
    pub fn heifsave_target(&self, target: &VipsTarget) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let target_in: *mut bindings::VipsTarget = target.ctx;

            let vips_op_response = bindings::vips_heifsave_target(
                inp_in,
                target_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("HeifsaveTarget failed"),
            )
        }
    }

    pub fn heifsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "heifsave_target",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    VipsValue::Target(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("HeifsaveTarget failed"),
        )
    }

    /// VipsHistCum (hist_cum), form cumulative histogram
    /// returns `VipsImage` - Output image
    pub fn hist_cum(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_hist_cum(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("HistCum failed"),
            )
        }
    }

    /// VipsHistEntropy (hist_entropy), estimate image entropy
    /// inp: `&VipsImage` -> Input histogram image
    /// returns `f64` - Output value
    pub fn hist_entropy(&self) -> Result<f64> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: f64 = f64::from(0);

            let vips_op_response = bindings::vips_hist_entropy(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                out_out,
                Error::OperationError("HistEntropy failed"),
            )
        }
    }

    /// VipsHistEqual (hist_equal), histogram equalisation
    /// returns `VipsImage` - Output image
    pub fn hist_equal(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_hist_equal(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("HistEqual failed"),
            )
        }
    }

    pub fn hist_equal_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "hist_equal",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HistEqual failed"),
        )
    }

    /// VipsHistFind (hist_find), find image histogram
    /// returns `VipsImage` - Output histogram
    pub fn hist_find(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_hist_find(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("HistFind failed"),
            )
        }
    }

    pub fn hist_find_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "hist_find",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HistFind failed"),
        )
    }

    /// VipsHistFindIndexed (hist_find_indexed), find indexed image histogram
    /// index: `&VipsImage` -> Index image
    /// returns `VipsImage` - Output histogram
    pub fn hist_find_indexed(&self, index: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let index_in: *mut bindings::VipsImage = index.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_hist_find_indexed(
                inp_in,
                index_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("HistFindIndexed failed"),
            )
        }
    }

    pub fn hist_find_indexed_with_opts(
        &self,
        index: &VipsImage,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "hist_find_indexed",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "index",
                    VipsValue::Image(index),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HistFindIndexed failed"),
        )
    }

    /// VipsHistFindNDim (hist_find_ndim), find n-dimensional image histogram
    /// returns `VipsImage` - Output histogram
    pub fn hist_find_ndim(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_hist_find_ndim(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("HistFindNdim failed"),
            )
        }
    }

    pub fn hist_find_ndim_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "hist_find_ndim",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HistFindNdim failed"),
        )
    }

    /// VipsHistIsmonotonic (hist_ismonotonic), test for monotonicity
    /// inp: `&VipsImage` -> Input histogram image
    /// returns `bool` - true if in is monotonic
    pub fn hist_ismonotonic(&self) -> Result<bool> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut monotonic_out: i32 = 0;

            let vips_op_response = bindings::vips_hist_ismonotonic(
                inp_in,
                &mut monotonic_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                monotonic_out != 0,
                Error::OperationError("HistIsmonotonic failed"),
            )
        }
    }

    /// VipsHistLocal (hist_local), local histogram equalisation
    /// width: `i32` -> Window width in pixels
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Window height in pixels
    /// min: 1, max: 100000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn hist_local(&self, width: i32, height: i32) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let width_in: i32 = width;
            let height_in: i32 = height;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_hist_local(
                inp_in,
                &mut out_out,
                width_in,
                height_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("HistLocal failed"),
            )
        }
    }

    pub fn hist_local_with_opts(
        &self,
        width: i32,
        height: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "hist_local",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HistLocal failed"),
        )
    }

    /// VipsHistMatch (hist_match), match two histograms
    /// inp: `&VipsImage` -> Input histogram
    /// refp: `&VipsImage` -> Reference histogram
    /// returns `VipsImage` - Output image
    pub fn hist_match(&self, refp: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let refp_in: *mut bindings::VipsImage = refp.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_hist_match(
                inp_in,
                refp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("HistMatch failed"),
            )
        }
    }

    /// VipsHistNorm (hist_norm), normalise histogram
    /// returns `VipsImage` - Output image
    pub fn hist_norm(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_hist_norm(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("HistNorm failed"),
            )
        }
    }

    /// VipsHistPlot (hist_plot), plot histogram
    /// returns `VipsImage` - Output image
    pub fn hist_plot(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_hist_plot(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("HistPlot failed"),
            )
        }
    }

    /// VipsHoughCircle (hough_circle), find hough circle transform
    /// returns `VipsImage` - Output image
    pub fn hough_circle(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_hough_circle(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("HoughCircle failed"),
            )
        }
    }

    pub fn hough_circle_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "hough_circle",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HoughCircle failed"),
        )
    }

    /// VipsHoughLine (hough_line), find hough line transform
    /// returns `VipsImage` - Output image
    pub fn hough_line(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_hough_line(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("HoughLine failed"),
            )
        }
    }

    pub fn hough_line_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "hough_line",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HoughLine failed"),
        )
    }

    /// VipsIccExport (icc_export), output to device with ICC profile
    /// returns `VipsImage` - Output image
    pub fn icc_export(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_icc_export(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("IccExport failed"),
            )
        }
    }

    pub fn icc_export_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "icc_export",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("IccExport failed"),
        )
    }

    /// VipsIccImport (icc_import), import from device with ICC profile
    /// returns `VipsImage` - Output image
    pub fn icc_import(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_icc_import(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("IccImport failed"),
            )
        }
    }

    pub fn icc_import_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "icc_import",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("IccImport failed"),
        )
    }

    /// VipsIccTransform (icc_transform), transform between devices with ICC profiles
    /// output_profile: `&str` -> Filename to load output profile from
    /// returns `VipsImage` - Output image
    pub fn icc_transform(&self, output_profile: &str) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let output_profile_in: CString = utils::new_c_string(output_profile)?;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_icc_transform(
                inp_in,
                &mut out_out,
                output_profile_in.as_ptr(),
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("IccTransform failed"),
            )
        }
    }

    pub fn icc_transform_with_opts(
        &self,
        output_profile: &str,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "icc_transform",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "output_profile",
                    VipsValue::Str(output_profile),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("IccTransform failed"),
        )
    }

    /// VipsIdentity (identity), make a 1D image where pixel values are indexes
    /// returns `VipsImage` - Output image
    pub fn identity() -> Result<VipsImage> {
        unsafe {
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_identity(
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Identity failed"),
            )
        }
    }

    pub fn identity_with_opts(option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "identity",
            option.with(
                "out",
                VipsValue::MutImage(&mut out_out),
            ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Identity failed"),
        )
    }

    /// VipsIfthenelse (ifthenelse), ifthenelse an image
    /// cond: `&VipsImage` -> Condition input image
    /// in1: `&VipsImage` -> Source for TRUE pixels
    /// in2: `&VipsImage` -> Source for FALSE pixels
    /// returns `VipsImage` - Output image
    pub fn ifthenelse(&self, in1: &VipsImage, in2: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let cond_in: *mut bindings::VipsImage = self.ctx;
            let in_1_in: *mut bindings::VipsImage = in1.ctx;
            let in_2_in: *mut bindings::VipsImage = in2.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_ifthenelse(
                cond_in,
                in_1_in,
                in_2_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Ifthenelse failed"),
            )
        }
    }

    pub fn ifthenelse_with_opts(
        &self,
        in1: &VipsImage,
        in2: &VipsImage,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "ifthenelse",
            option
                .with(
                    "cond",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "in1",
                    VipsValue::Image(in1),
                )
                .with(
                    "in2",
                    VipsValue::Image(in2),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Ifthenelse failed"),
        )
    }

    /// VipsInsert (insert), insert image @sub into @main at @x, @y
    /// main: `&VipsImage` -> Main input image
    /// sub: `&VipsImage` -> Sub-image to insert into main image
    /// x: `i32` -> Left edge of sub in main
    /// min: -100000000, max: 100000000, default: 0
    /// y: `i32` -> Top edge of sub in main
    /// min: -100000000, max: 100000000, default: 0
    /// returns `VipsImage` - Output image
    pub fn insert(&self, sub: &VipsImage, x: i32, y: i32) -> Result<VipsImage> {
        unsafe {
            let main_in: *mut bindings::VipsImage = self.ctx;
            let sub_in: *mut bindings::VipsImage = sub.ctx;
            let x_in: i32 = x;
            let y_in: i32 = y;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_insert(
                main_in,
                sub_in,
                &mut out_out,
                x_in,
                y_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Insert failed"),
            )
        }
    }

    pub fn insert_with_opts(
        &self,
        sub: &VipsImage,
        x: i32,
        y: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "insert",
            option
                .with(
                    "main",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "sub",
                    VipsValue::Image(sub),
                )
                .with(
                    "x",
                    VipsValue::Int(x),
                )
                .with(
                    "y",
                    VipsValue::Int(y),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Insert failed"),
        )
    }

    /// VipsInvert (invert), invert an image
    /// returns `VipsImage` - Output image
    pub fn invert(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_invert(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Invert failed"),
            )
        }
    }

    /// VipsInvertlut (invertlut), build an inverted look-up table
    /// inp: `&VipsImage` -> Matrix of XY coordinates
    /// returns `VipsImage` - Output image
    pub fn invertlut(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_invertlut(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Invertlut failed"),
            )
        }
    }

    pub fn invertlut_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "invertlut",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Invertlut failed"),
        )
    }

    /// VipsInvfft (invfft), inverse FFT
    /// returns `VipsImage` - Output image
    pub fn invfft(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_invfft(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Invfft failed"),
            )
        }
    }

    pub fn invfft_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "invfft",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Invfft failed"),
        )
    }

    /// VipsJoin (join), join a pair of images
    /// in_1: `&VipsImage` -> First input image
    /// in2: `&VipsImage` -> Second input image
    /// direction: `Direction` -> Join left-right or up-down
    ///  `Horizontal` -> VIPS_DIRECTION_HORIZONTAL = 0 [DEFAULT]
    ///  `Vertical` -> VIPS_DIRECTION_VERTICAL = 1
    ///  `Last` -> VIPS_DIRECTION_LAST = 2
    /// returns `VipsImage` - Output image
    pub fn join(&self, in2: &VipsImage, direction: Direction) -> Result<VipsImage> {
        unsafe {
            let in_1_in: *mut bindings::VipsImage = self.ctx;
            let in_2_in: *mut bindings::VipsImage = in2.ctx;
            let direction_in: i32 = direction as i32;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_join(
                in_1_in,
                in_2_in,
                &mut out_out,
                direction_in
                    .try_into()
                    .unwrap(),
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Join failed"),
            )
        }
    }

    pub fn join_with_opts(
        &self,
        in2: &VipsImage,
        direction: Direction,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "join",
            option
                .with(
                    "in1",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "in2",
                    VipsValue::Image(in2),
                )
                .with(
                    "direction",
                    VipsValue::Int(direction as i32),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Join failed"),
        )
    }

    pub fn jp2kload(&self, filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "jp2kload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("jp2kload failed"),
        )
    }

    pub fn jp2kload_buffer(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "jp2kload_buffer",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "buffer",
                    VipsValue::Buffer(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("jp2kload_buffer failed"),
        )
    }

    pub fn jp2kload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "jp2kload_source",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("jp2kload_source failed"),
        )
    }

    pub fn jp2ksave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "jp2ksave",
            VOption::new()
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("jp2ksave failed"),
        )
    }

    pub fn jp2ksave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jp2ksave",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("jp2ksave_with_opts failed"),
        )
    }

    pub fn jp2ksave_buffer(&self) -> Result<Vec<u8>> {
        let mut blob = VipsBlob::from(null_mut());

        let vips_op_response = call(
            "jp2ksave_buffer",
            VOption::new()
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    VipsValue::MutBlob(&mut blob),
                ),
        );
        utils::result(
            vips_op_response,
            blob.into(),
            Error::OperationError("jp2ksave_buffer failed"),
        )
    }

    pub fn jp2ksave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut blob = VipsBlob::from(null_mut());

        let vips_op_response = call(
            "jp2ksave_buffer",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    VipsValue::MutBlob(&mut blob),
                ),
        );
        utils::result(
            vips_op_response,
            blob.into(),
            Error::OperationError("jp2ksave_buffer_with_opts failed"),
        )
    }

    pub fn jp2ksave_target(&self, target: &VipsTarget) -> Result<()> {
        let vips_op_response = call(
            "jp2ksave_target",
            VOption::new()
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    VipsValue::Target(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("jp2ksave_target failed"),
        )
    }

    pub fn jp2ksave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jp2ksave_target",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    VipsValue::Target(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("jp2ksave_target_with_opts failed"),
        )
    }

    /// VipsForeignLoadJpegFile (jpegload), load jpeg from file (.jpg, .jpeg, .jpe, .jfif), priority=50, is_a, get_flags, get_flags_filename, header, load
    /// filename: `&str` -> Filename to load from
    /// returns `VipsImage` - Output image
    pub fn jpegload(filename: &str) -> Result<VipsImage> {
        unsafe {
            let filename_in: CString = utils::new_c_string(filename)?;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_jpegload(
                filename_in.as_ptr(),
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Jpegload failed"),
            )
        }
    }

    pub fn jpegload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "jpegload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Jpegload failed"),
        )
    }

    /// VipsForeignLoadJpegBuffer (jpegload_buffer), load jpeg from buffer, priority=50, is_a_buffer, get_flags, get_flags_filename, header, load
    /// buffer: `&[u8]` -> Buffer to load from
    /// returns `VipsImage` - Output image
    pub fn jpegload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        unsafe {
            let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_jpegload_buffer(
                buffer_in,
                buffer.len() as u64,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("JpegloadBuffer failed"),
            )
        }
    }

    pub fn jpegload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "jpegload_buffer",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "buffer",
                    VipsValue::Buffer(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("JpegloadBuffer failed"),
        )
    }

    pub fn jpegload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "jpegload_source",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("jpegload_source failed"),
        )
    }

    /// VipsForeignSaveJpegFile (jpegsave), save image to jpeg file (.jpg, .jpeg, .jpe, .jfif), priority=0, rgb-cmyk
    /// inp: `&VipsImage` -> Image to save
    /// filename: `&str` -> Filename to save to
    pub fn jpegsave(&self, filename: &str) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let filename_in: CString = utils::new_c_string(filename)?;

            let vips_op_response = bindings::vips_jpegsave(
                inp_in,
                filename_in.as_ptr(),
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("Jpegsave failed"),
            )
        }
    }

    pub fn jpegsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jpegsave",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Jpegsave failed"),
        )
    }

    /// VipsForeignSaveJpegBuffer (jpegsave_buffer), save image to jpeg buffer (.jpg, .jpeg, .jpe, .jfif), priority=0, rgb-cmyk
    /// inp: `&VipsImage` -> Image to save
    /// returns `Vec<u8>` - Buffer to save to
    pub fn jpegsave_buffer(&self) -> Result<Vec<u8>> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut buffer_buf_size: u64 = 0;
            let mut buffer_out: *mut c_void = null_mut();

            let vips_op_response = bindings::vips_jpegsave_buffer(
                inp_in,
                &mut buffer_out,
                &mut buffer_buf_size,
                NULL,
            );
            utils::result(
                vips_op_response,
                utils::new_byte_array(
                    buffer_out,
                    buffer_buf_size,
                ),
                Error::OperationError("JpegsaveBuffer failed"),
            )
        }
    }

    pub fn jpegsave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut blob = VipsBlob::from(null_mut());

        let vips_op_response = call(
            "jpegsave_buffer",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    VipsValue::MutBlob(&mut blob),
                ),
        );
        utils::result(
            vips_op_response,
            blob.into(),
            Error::OperationError("JpegsaveBuffer failed"),
        )
    }

    /// VipsForeignSaveJpegMime (jpegsave_mime), save image to jpeg mime (.jpg, .jpeg, .jpe, .jfif), priority=0, rgb-cmyk
    /// inp: `&VipsImage` -> Image to save
    pub fn jpegsave_mime(&self) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;

            let vips_op_response = bindings::vips_jpegsave_mime(
                inp_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("JpegsaveMime failed"),
            )
        }
    }

    pub fn jpegsave_mime_with_opts(&self, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jpegsave_mime",
            option.with(
                "in",
                VipsValue::Image(&VipsImage::from(self.ctx)),
            ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("JpegsaveMime failed"),
        )
    }

    /// VipsForeignSaveJpegTarget (jpegsave_target), save image to jpeg target (.jpg, .jpeg, .jpe, .jfif), priority=0, rgb-cmyk
    /// inp: `&VipsImage` -> Image to save
    /// target: `&VipsTarget` -> Target to save to
    pub fn jpegsave_target(&self, target: &VipsTarget) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let target_in: *mut bindings::VipsTarget = target.ctx;

            let vips_op_response = bindings::vips_jpegsave_target(
                inp_in,
                target_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("JpegsaveTarget failed"),
            )
        }
    }

    pub fn jpegsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jpegsave_target",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    VipsValue::Target(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("JpegsaveTarget failed"),
        )
    }

    pub fn jxlload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "jxlload",
            VOption::new()
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("jxlload failed"),
        )
    }

    pub fn jxlload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "jxlload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("jxlload_with_opts failed"),
        )
    }

    pub fn jxlload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "jxlload_buffer",
            VOption::new()
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "buffer",
                    VipsValue::Buffer(buffer),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("jxlload_buffer failed"),
        )
    }

    pub fn jxlload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "jxlload_buffer",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "buffer",
                    VipsValue::Buffer(buffer),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("jxlload_buffer_with_opts failed"),
        )
    }

    pub fn jxlload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "jxlload_source",
            VOption::new()
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("jxlload_source failed"),
        )
    }

    pub fn jxlload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "jxlload_source",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("jxlload_source_with_opts failed"),
        )
    }

    pub fn jxlsave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "jxlsave",
            VOption::new()
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("jxlsave failed"),
        )
    }

    pub fn jxlsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jxlsave",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("jxlsave_with_opts failed"),
        )
    }

    pub fn jxlsave_buffer(&self) -> Result<Vec<u8>> {
        let mut blob = VipsBlob::from(null_mut());

        let vips_op_response = call(
            "jxlsave_buffer",
            VOption::new()
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    VipsValue::MutBlob(&mut blob),
                ),
        );
        utils::result(
            vips_op_response,
            blob.into(),
            Error::OperationError("jxlsave_buffer failed"),
        )
    }

    pub fn jxlsave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut blob = VipsBlob::from(null_mut());

        let vips_op_response = call(
            "jxlsave_buffer",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    VipsValue::MutBlob(&mut blob),
                ),
        );
        utils::result(
            vips_op_response,
            blob.into(),
            Error::OperationError("jxlsave_buffer_with_opts failed"),
        )
    }

    pub fn jxlsave_target(&self, target: &VipsTarget) -> Result<()> {
        let vips_op_response = call(
            "jxlsave_target",
            VOption::new()
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    VipsValue::Target(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("jxlsave_target failed"),
        )
    }

    pub fn jxlsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jxlsave_target",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    VipsValue::Target(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("jxlsave_target_with_opts failed"),
        )
    }

    /// VipsLabelregions (labelregions), label regions in an image
    /// returns `VipsImage` - Mask of region labels
    pub fn labelregions(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut mask_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_labelregions(
                inp_in,
                &mut mask_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: mask_out,
                },
                Error::OperationError("Labelregion failed"),
            )
        }
    }

    pub fn labelregions_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "labelregions",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "mask",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("labelregions_with_opts failed"),
        )
    }

    pub fn linear(&self, a: &[f64], b: &[f64]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "linear",
            VOption::new()
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "a",
                    VipsValue::DoubleArray(a),
                )
                .with(
                    "b",
                    VipsValue::DoubleArray(b),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Linear failed"),
        )
    }

    pub fn linear_with_opts(&self, a: &[f64], b: &[f64], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "linear",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "a",
                    VipsValue::DoubleArray(a),
                )
                .with(
                    "b",
                    VipsValue::DoubleArray(b),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Linear failed"),
        )
    }

    /// VipsLineCache (linecache), cache an image as a set of lines
    /// returns `VipsImage` - Output image
    pub fn linecache(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_linecache(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Linecache failed"),
            )
        }
    }

    pub fn linecache_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "linecache",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Linecache failed"),
        )
    }

    /// VipsLogmat (logmat), make a Laplacian of Gaussian image
    /// sigma: `f64` -> Radius of Gaussian
    /// min: 0.000001, max: 10000, default: 1
    /// min_ampl: `f64` -> Minimum amplitude of Gaussian
    /// min: 0.000001, max: 10000, default: 0.1
    /// returns `VipsImage` - Output image
    pub fn logmat(sigma: f64, min_ampl: f64) -> Result<VipsImage> {
        unsafe {
            let sigma_in: f64 = sigma;
            let min_ampl_in: f64 = min_ampl;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_logmat(
                &mut out_out,
                sigma_in,
                min_ampl_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Logmat failed"),
            )
        }
    }

    pub fn logmat_with_opts(sigma: f64, min_ampl: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "logmat",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "sigma",
                    VipsValue::Double(sigma),
                )
                .with(
                    "min_ampl",
                    VipsValue::Double(min_ampl),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Logmat failed"),
        )
    }

    pub fn magickload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "magickload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("magickload failed"),
        )
    }

    pub fn magickload_buffer(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "magickload_buffer",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "buffer",
                    VipsValue::Buffer(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("magickload_buffer failed"),
        )
    }

    pub fn magicksave(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "magicksave",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );

        utils::result(
            vips_op_response,
            (),
            Error::OperationError("magicksave failed"),
        )
    }

    pub fn magicksave_buffer(&self, option: VOption) -> Result<Vec<u8>> {
        let mut blob = VipsBlob::from(null_mut());

        let vips_op_response = call(
            "magicksave_buffer",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    VipsValue::MutBlob(&mut blob),
                ),
        );

        utils::result(
            vips_op_response,
            blob.into(),
            Error::OperationError("magicksave_buffer failed"),
        )
    }

    /// VipsMapim (mapim), resample with a map image
    /// index: `&VipsImage` -> Index pixels with this
    /// returns `VipsImage` - Output image
    pub fn mapim(&self, index: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let index_in: *mut bindings::VipsImage = index.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_mapim(
                inp_in,
                &mut out_out,
                index_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Mapim failed"),
            )
        }
    }

    pub fn mapim_with_opts(&self, index: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "mapim",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "index",
                    VipsValue::Image(index),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Mapim failed"),
        )
    }

    /// VipsMaplut (maplut), map an image though a lut
    /// lut: `&VipsImage` -> Look-up table image
    /// returns `VipsImage` - Output image
    pub fn maplut(&self, lut: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let lut_in: *mut bindings::VipsImage = lut.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_maplut(
                inp_in,
                &mut out_out,
                lut_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Maplut failed"),
            )
        }
    }

    pub fn maplut_with_opts(&self, lut: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "maplut",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "lut",
                    VipsValue::Image(lut),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Maplut failed"),
        )
    }

    /// VipsMaskButterworth (mask_butterworth), make a butterworth filter
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 1
    /// order: `f64` -> Filter order
    /// min: 1, max: 1000000, default: 1
    /// frequency_cutoff: `f64` -> Frequency cutoff
    /// min: 0, max: 1000000, default: 0.5
    /// amplitude_cutoff: `f64` -> Amplitude cutoff
    /// min: 0, max: 1, default: 0.5
    /// returns `VipsImage` - Output image
    pub fn mask_butterworth(
        width: i32,
        height: i32,
        order: f64,
        frequency_cutoff: f64,
        amplitude_cutoff: f64,
    ) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let order_in: f64 = order;
            let frequency_cutoff_in: f64 = frequency_cutoff;
            let amplitude_cutoff_in: f64 = amplitude_cutoff;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_mask_butterworth(
                &mut out_out,
                width_in,
                height_in,
                order_in,
                frequency_cutoff_in,
                amplitude_cutoff_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("MaskButterworth failed"),
            )
        }
    }

    pub fn mask_butterworth_with_opts(
        width: i32,
        height: i32,
        order: f64,
        frequency_cutoff: f64,
        amplitude_cutoff: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "mask_butterworth",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                )
                .with(
                    "order",
                    VipsValue::Double(order),
                )
                .with(
                    "frequency_cutoff",
                    VipsValue::Double(frequency_cutoff),
                )
                .with(
                    "amplitude_cutoff",
                    VipsValue::Double(amplitude_cutoff),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskButterworth failed"),
        )
    }

    /// VipsMaskButterworthBand (mask_butterworth_band), make a butterworth_band filter
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 1
    /// order: `f64` -> Filter order
    /// min: 1, max: 1000000, default: 1
    /// frequency_cutoff_x: `f64` -> Frequency cutoff x
    /// min: 0, max: 1000000, default: 0.5
    /// frequency_cutoff_y: `f64` -> Frequency cutoff y
    /// min: 0, max: 1000000, default: 0.5
    /// radius: `f64` -> Radius of circle
    /// min: 0, max: 1000000, default: 0.1
    /// amplitude_cutoff: `f64` -> Amplitude cutoff
    /// min: 0, max: 1, default: 0.5
    /// returns `VipsImage` - Output image
    pub fn mask_butterworth_band(
        width: i32,
        height: i32,
        order: f64,
        frequency_cutoff_x: f64,
        frequency_cutoff_y: f64,
        radius: f64,
        amplitude_cutoff: f64,
    ) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let order_in: f64 = order;
            let frequency_cutoff_x_in: f64 = frequency_cutoff_x;
            let frequency_cutoff_y_in: f64 = frequency_cutoff_y;
            let radius_in: f64 = radius;
            let amplitude_cutoff_in: f64 = amplitude_cutoff;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_mask_butterworth_band(
                &mut out_out,
                width_in,
                height_in,
                order_in,
                frequency_cutoff_x_in,
                frequency_cutoff_y_in,
                radius_in,
                amplitude_cutoff_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("MaskButterworthBand failed"),
            )
        }
    }

    pub fn mask_butterworth_band_with_opts(
        width: i32,
        height: i32,
        order: f64,
        frequency_cutoff_x: f64,
        frequency_cutoff_y: f64,
        radius: f64,
        amplitude_cutoff: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "mask_butterworth_band",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                )
                .with(
                    "order",
                    VipsValue::Double(order),
                )
                .with(
                    "frequency_cutoff_x",
                    VipsValue::Double(frequency_cutoff_x),
                )
                .with(
                    "frequency_cutoff_y",
                    VipsValue::Double(frequency_cutoff_y),
                )
                .with(
                    "radius",
                    VipsValue::Double(radius),
                )
                .with(
                    "amplitude_cutoff",
                    VipsValue::Double(amplitude_cutoff),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskButterworthBand failed"),
        )
    }

    /// VipsMaskButterworthRing (mask_butterworth_ring), make a butterworth ring filter
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 1
    /// order: `f64` -> Filter order
    /// min: 1, max: 1000000, default: 1
    /// frequency_cutoff: `f64` -> Frequency cutoff
    /// min: 0, max: 1000000, default: 0.5
    /// amplitude_cutoff: `f64` -> Amplitude cutoff
    /// min: 0, max: 1, default: 0.5
    /// ringwidth: `f64` -> Ringwidth
    /// min: 0, max: 1000000, default: 0.1
    /// returns `VipsImage` - Output image
    pub fn mask_butterworth_ring(
        width: i32,
        height: i32,
        order: f64,
        frequency_cutoff: f64,
        amplitude_cutoff: f64,
        ringwidth: f64,
    ) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let order_in: f64 = order;
            let frequency_cutoff_in: f64 = frequency_cutoff;
            let amplitude_cutoff_in: f64 = amplitude_cutoff;
            let ringwidth_in: f64 = ringwidth;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_mask_butterworth_ring(
                &mut out_out,
                width_in,
                height_in,
                order_in,
                frequency_cutoff_in,
                amplitude_cutoff_in,
                ringwidth_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("MaskButterworthRing failed"),
            )
        }
    }

    pub fn mask_butterworth_ring_with_opts(
        width: i32,
        height: i32,
        order: f64,
        frequency_cutoff: f64,
        amplitude_cutoff: f64,
        ringwidth: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "mask_butterworth_ring",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                )
                .with(
                    "order",
                    VipsValue::Double(order),
                )
                .with(
                    "frequency_cutoff",
                    VipsValue::Double(frequency_cutoff),
                )
                .with(
                    "amplitude_cutoff",
                    VipsValue::Double(amplitude_cutoff),
                )
                .with(
                    "ringwidth",
                    VipsValue::Double(ringwidth),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskButterworthRing failed"),
        )
    }

    /// VipsMaskFractal (mask_fractal), make fractal filter
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 1
    /// fractal_dimension: `f64` -> Fractal dimension
    /// min: 2, max: 3, default: 2.5
    /// returns `VipsImage` - Output image
    pub fn mask_fractal(width: i32, height: i32, fractal_dimension: f64) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let fractal_dimension_in: f64 = fractal_dimension;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_mask_fractal(
                &mut out_out,
                width_in,
                height_in,
                fractal_dimension_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("MaskFractal failed"),
            )
        }
    }

    pub fn mask_fractal_with_opts(
        width: i32,
        height: i32,
        fractal_dimension: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "mask_fractal",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                )
                .with(
                    "fractal_dimension",
                    VipsValue::Double(fractal_dimension),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskFractal failed"),
        )
    }

    /// VipsMaskGaussian (mask_gaussian), make a gaussian filter
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 1
    /// frequency_cutoff: `f64` -> Frequency cutoff
    /// min: 0, max: 1000000, default: 0.5
    /// amplitude_cutoff: `f64` -> Amplitude cutoff
    /// min: 0, max: 1, default: 0.5
    /// returns `VipsImage` - Output image
    pub fn mask_gaussian(
        width: i32,
        height: i32,
        frequency_cutoff: f64,
        amplitude_cutoff: f64,
    ) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let frequency_cutoff_in: f64 = frequency_cutoff;
            let amplitude_cutoff_in: f64 = amplitude_cutoff;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_mask_gaussian(
                &mut out_out,
                width_in,
                height_in,
                frequency_cutoff_in,
                amplitude_cutoff_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("MaskGaussian failed"),
            )
        }
    }

    pub fn mask_gaussian_with_opts(
        width: i32,
        height: i32,
        frequency_cutoff: f64,
        amplitude_cutoff: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "mask_gaussian",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                )
                .with(
                    "frequency_cutoff",
                    VipsValue::Double(frequency_cutoff),
                )
                .with(
                    "amplitude_cutoff",
                    VipsValue::Double(amplitude_cutoff),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskGaussian failed"),
        )
    }

    /// VipsMaskGaussianBand (mask_gaussian_band), make a gaussian filter
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 1
    /// frequency_cutoff_x: `f64` -> Frequency cutoff x
    /// min: 0, max: 1000000, default: 0.5
    /// frequency_cutoff_y: `f64` -> Frequency cutoff y
    /// min: 0, max: 1000000, default: 0.5
    /// radius: `f64` -> Radius of circle
    /// min: 0, max: 1000000, default: 0.1
    /// amplitude_cutoff: `f64` -> Amplitude cutoff
    /// min: 0, max: 1, default: 0.5
    /// returns `VipsImage` - Output image
    pub fn mask_gaussian_band(
        width: i32,
        height: i32,
        frequency_cutoff_x: f64,
        frequency_cutoff_y: f64,
        radius: f64,
        amplitude_cutoff: f64,
    ) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let frequency_cutoff_x_in: f64 = frequency_cutoff_x;
            let frequency_cutoff_y_in: f64 = frequency_cutoff_y;
            let radius_in: f64 = radius;
            let amplitude_cutoff_in: f64 = amplitude_cutoff;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_mask_gaussian_band(
                &mut out_out,
                width_in,
                height_in,
                frequency_cutoff_x_in,
                frequency_cutoff_y_in,
                radius_in,
                amplitude_cutoff_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("MaskGaussianBand failed"),
            )
        }
    }

    pub fn mask_gaussian_band_with_opts(
        width: i32,
        height: i32,
        frequency_cutoff_x: f64,
        frequency_cutoff_y: f64,
        radius: f64,
        amplitude_cutoff: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "mask_gaussian_band",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                )
                .with(
                    "frequency_cutoff_x",
                    VipsValue::Double(frequency_cutoff_x),
                )
                .with(
                    "frequency_cutoff_y",
                    VipsValue::Double(frequency_cutoff_y),
                )
                .with(
                    "radius",
                    VipsValue::Double(radius),
                )
                .with(
                    "amplitude_cutoff",
                    VipsValue::Double(amplitude_cutoff),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskGaussianBand failed"),
        )
    }

    /// VipsMaskGaussianRing (mask_gaussian_ring), make a gaussian ring filter
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 1
    /// frequency_cutoff: `f64` -> Frequency cutoff
    /// min: 0, max: 1000000, default: 0.5
    /// amplitude_cutoff: `f64` -> Amplitude cutoff
    /// min: 0, max: 1, default: 0.5
    /// ringwidth: `f64` -> Ringwidth
    /// min: 0, max: 1000000, default: 0.5
    /// returns `VipsImage` - Output image
    pub fn mask_gaussian_ring(
        width: i32,
        height: i32,
        frequency_cutoff: f64,
        amplitude_cutoff: f64,
        ringwidth: f64,
    ) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let frequency_cutoff_in: f64 = frequency_cutoff;
            let amplitude_cutoff_in: f64 = amplitude_cutoff;
            let ringwidth_in: f64 = ringwidth;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_mask_gaussian_ring(
                &mut out_out,
                width_in,
                height_in,
                frequency_cutoff_in,
                amplitude_cutoff_in,
                ringwidth_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("MaskGaussianRing failed"),
            )
        }
    }

    pub fn mask_gaussian_ring_with_opts(
        width: i32,
        height: i32,
        frequency_cutoff: f64,
        amplitude_cutoff: f64,
        ringwidth: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "mask_gaussian_ring",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                )
                .with(
                    "frequency_cutoff",
                    VipsValue::Double(frequency_cutoff),
                )
                .with(
                    "amplitude_cutoff",
                    VipsValue::Double(amplitude_cutoff),
                )
                .with(
                    "ringwidth",
                    VipsValue::Double(ringwidth),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskGaussianRing failed"),
        )
    }

    /// VipsMaskIdeal (mask_ideal), make an ideal filter
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 1
    /// frequency_cutoff: `f64` -> Frequency cutoff
    /// min: 0, max: 1000000, default: 0.5
    /// returns `VipsImage` - Output image
    pub fn mask_ideal(width: i32, height: i32, frequency_cutoff: f64) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let frequency_cutoff_in: f64 = frequency_cutoff;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_mask_ideal(
                &mut out_out,
                width_in,
                height_in,
                frequency_cutoff_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("MaskIdeal failed"),
            )
        }
    }

    pub fn mask_ideal_with_opts(
        width: i32,
        height: i32,
        frequency_cutoff: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "mask_ideal",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                )
                .with(
                    "frequency_cutoff",
                    VipsValue::Double(frequency_cutoff),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskIdeal failed"),
        )
    }

    /// VipsMaskIdealBand (mask_ideal_band), make an ideal band filter
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 1
    /// frequency_cutoff_x: `f64` -> Frequency cutoff x
    /// min: 0, max: 1000000, default: 0.5
    /// frequency_cutoff_y: `f64` -> Frequency cutoff y
    /// min: 0, max: 1000000, default: 0.5
    /// radius: `f64` -> Radius of circle
    /// min: 0, max: 1000000, default: 0.1
    /// returns `VipsImage` - Output image
    pub fn mask_ideal_band(
        width: i32,
        height: i32,
        frequency_cutoff_x: f64,
        frequency_cutoff_y: f64,
        radius: f64,
    ) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let frequency_cutoff_x_in: f64 = frequency_cutoff_x;
            let frequency_cutoff_y_in: f64 = frequency_cutoff_y;
            let radius_in: f64 = radius;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_mask_ideal_band(
                &mut out_out,
                width_in,
                height_in,
                frequency_cutoff_x_in,
                frequency_cutoff_y_in,
                radius_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("MaskIdealBand failed"),
            )
        }
    }

    pub fn mask_ideal_band_with_opts(
        width: i32,
        height: i32,
        frequency_cutoff_x: f64,
        frequency_cutoff_y: f64,
        radius: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "mask_ideal_band",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                )
                .with(
                    "frequency_cutoff_x",
                    VipsValue::Double(frequency_cutoff_x),
                )
                .with(
                    "frequency_cutoff_y",
                    VipsValue::Double(frequency_cutoff_y),
                )
                .with(
                    "radius",
                    VipsValue::Double(radius),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskIdealBand failed"),
        )
    }

    /// VipsMaskIdealRing (mask_ideal_ring), make an ideal ring filter
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 1
    /// frequency_cutoff: `f64` -> Frequency cutoff
    /// min: 0, max: 1000000, default: 0.5
    /// ringwidth: `f64` -> Ringwidth
    /// min: 0, max: 1000000, default: 0.5
    /// returns `VipsImage` - Output image
    pub fn mask_ideal_ring(
        width: i32,
        height: i32,
        frequency_cutoff: f64,
        ringwidth: f64,
    ) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let frequency_cutoff_in: f64 = frequency_cutoff;
            let ringwidth_in: f64 = ringwidth;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_mask_ideal_ring(
                &mut out_out,
                width_in,
                height_in,
                frequency_cutoff_in,
                ringwidth_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("MaskIdealRing failed"),
            )
        }
    }

    pub fn mask_ideal_ring_with_opts(
        width: i32,
        height: i32,
        frequency_cutoff: f64,
        ringwidth: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "mask_ideal_ring",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                )
                .with(
                    "frequency_cutoff",
                    VipsValue::Double(frequency_cutoff),
                )
                .with(
                    "ringwidth",
                    VipsValue::Double(ringwidth),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskIdealRing failed"),
        )
    }

    pub fn match_(
        &self,
        sec: &VipsImage,
        xr1: i32,
        yr1: i32,
        xs1: i32,
        ys1: i32,
        xr2: i32,
        yr2: i32,
        xs2: i32,
        ys2: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "match",
            option
                .with(
                    "ref",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "sec",
                    VipsValue::Image(sec),
                )
                .with(
                    "xr1",
                    VipsValue::Int(xr1),
                )
                .with(
                    "yr1",
                    VipsValue::Int(yr1),
                )
                .with(
                    "xs1",
                    VipsValue::Int(xs1),
                )
                .with(
                    "ys1",
                    VipsValue::Int(ys1),
                )
                .with(
                    "xr2",
                    VipsValue::Int(xr2),
                )
                .with(
                    "yr2",
                    VipsValue::Int(yr2),
                )
                .with(
                    "xs2",
                    VipsValue::Int(xs2),
                )
                .with(
                    "ys2",
                    VipsValue::Int(ys2),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Match failed"),
        )
    }

    /// VipsMath (math), apply a math operation to an image
    /// math: `OperationMath` -> Math to perform
    ///  `Sin` -> VIPS_OPERATION_MATH_SIN = 0 [DEFAULT]
    ///  `Co` -> VIPS_OPERATION_MATH_COS = 1
    ///  `Tan` -> VIPS_OPERATION_MATH_TAN = 2
    ///  `Asin` -> VIPS_OPERATION_MATH_ASIN = 3
    ///  `Aco` -> VIPS_OPERATION_MATH_ACOS = 4
    ///  `Atan` -> VIPS_OPERATION_MATH_ATAN = 5
    ///  `Log` -> VIPS_OPERATION_MATH_LOG = 6
    ///  `Log10` -> VIPS_OPERATION_MATH_LOG10 = 7
    ///  `Exp` -> VIPS_OPERATION_MATH_EXP = 8
    ///  `Exp10` -> VIPS_OPERATION_MATH_EXP10 = 9
    ///  `Sinh` -> VIPS_OPERATION_MATH_SINH = 10
    ///  `Cosh` -> VIPS_OPERATION_MATH_COSH = 11
    ///  `Tanh` -> VIPS_OPERATION_MATH_TANH = 12
    ///  `Asinh` -> VIPS_OPERATION_MATH_ASINH = 13
    ///  `Acosh` -> VIPS_OPERATION_MATH_ACOSH = 14
    ///  `Atanh` -> VIPS_OPERATION_MATH_ATANH = 15
    ///  `Last` -> VIPS_OPERATION_MATH_LAST = 16
    /// returns `VipsImage` - Output image
    pub fn math(&self, math: OperationMath) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let math_in: i32 = math as i32;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_math(
                inp_in,
                &mut out_out,
                math_in
                    .try_into()
                    .unwrap(),
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Math failed"),
            )
        }
    }

    /// VipsMath2 (math2), binary math operations
    /// left: `&VipsImage` -> Left-hand image argument
    /// right: `&VipsImage` -> Right-hand image argument
    /// math2: `OperationMath2` -> Math to perform
    ///  `Pow` -> VIPS_OPERATION_MATH2_POW = 0 [DEFAULT]
    ///  `Wop` -> VIPS_OPERATION_MATH2_WOP = 1
    ///  `Atan2` -> VIPS_OPERATION_MATH2_ATAN2 = 2
    ///  `Last` -> VIPS_OPERATION_MATH2_LAST = 3
    /// returns `VipsImage` - Output image
    pub fn math2(&self, right: &VipsImage, math2: OperationMath2) -> Result<VipsImage> {
        unsafe {
            let left_in: *mut bindings::VipsImage = self.ctx;
            let right_in: *mut bindings::VipsImage = right.ctx;
            let math_2_in: i32 = math2 as i32;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_math2(
                left_in,
                right_in,
                &mut out_out,
                math_2_in
                    .try_into()
                    .unwrap(),
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Math2 failed"),
            )
        }
    }

    /// VipsMath2Const (math2_const), binary math operations with a constant
    /// math_2: `OperationMath2` -> Math to perform
    ///  `Pow` -> VIPS_OPERATION_MATH2_POW = 0 [DEFAULT]
    ///  `Wop` -> VIPS_OPERATION_MATH2_WOP = 1
    ///  `Atan2` -> VIPS_OPERATION_MATH2_ATAN2 = 2
    ///  `Last` -> VIPS_OPERATION_MATH2_LAST = 3
    /// c: `&mut [f64]` -> Array of constants
    /// returns `VipsImage` - Output image
    pub fn math2_const(&self, math_2: OperationMath2, c: &[f64]) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let math_2_in: i32 = math_2 as i32;
            let c_in: *const f64 = c.as_ptr();
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_math2_const(
                inp_in,
                &mut out_out,
                math_2_in
                    .try_into()
                    .unwrap(),
                c_in as _,
                c.len() as i32,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Math2Const failed"),
            )
        }
    }

    pub fn matload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "matload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Matrixload failed"),
        )
    }

    /// VipsMatrixinvert (matrixinvert), invert an matrix
    /// inp: `&VipsImage` -> An square matrix
    /// returns `VipsImage` - Output matrix
    pub fn matrixinvert(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_matrixinvert(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Matrixinvert failed"),
            )
        }
    }

    /// VipsForeignLoadMatrixFile (matrixload), load matrix (.mat), priority=0, is_a, get_flags, get_flags_filename, header, load
    /// filename: `&str` -> Filename to load from
    /// returns `VipsImage` - Output image
    pub fn matrixload(filename: &str) -> Result<VipsImage> {
        unsafe {
            let filename_in: CString = utils::new_c_string(filename)?;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_matrixload(
                filename_in.as_ptr(),
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Matrixload failed"),
            )
        }
    }

    pub fn matrixload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "matrixload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Matrixload failed"),
        )
    }

    /// VipsForeignLoadMatrixSource (matrixload_source), load matrix, priority=0, is_a_source, get_flags, header, load
    /// source: `&VipsSource` -> Source to load from
    /// returns `VipsImage` - Output image
    pub fn matrixload_source(source: &VipsSource) -> Result<VipsImage> {
        unsafe {
            let source_in: *mut bindings::VipsSource = source.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_matrixload_source(
                source_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("MatrixloadSource failed"),
            )
        }
    }

    pub fn matrixload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "matrixload_source",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MatrixloadSource failed"),
        )
    }

    pub fn matrixmultiply(&self, right: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "matrixmultiply",
            option
                .with(
                    "left",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "right",
                    VipsValue::Image(right),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("matrixmultiply failed"),
        )
    }

    /// VipsForeignPrintMatrix (matrixprint), print matrix (.mat), priority=0, mono
    /// inp: `&VipsImage` -> Image to save
    pub fn matrixprint(&self) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;

            let vips_op_response = bindings::vips_matrixprint(
                inp_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("Matrixprint failed"),
            )
        }
    }

    pub fn matrixprint_with_opts(&self, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "matrixprint",
            option.with(
                "in",
                VipsValue::Image(&VipsImage::from(self.ctx)),
            ),
        );

        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Matrixprint failed"),
        )
    }

    /// VipsForeignSaveMatrixFile (matrixsave), save image to matrix (.mat), priority=0, mono
    /// inp: `&VipsImage` -> Image to save
    /// filename: `&str` -> Filename to save to
    pub fn matrixsave(&self, filename: &str) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let filename_in: CString = utils::new_c_string(filename)?;

            let vips_op_response = bindings::vips_matrixsave(
                inp_in,
                filename_in.as_ptr(),
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("Matrixsave failed"),
            )
        }
    }

    pub fn matrixsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "matrixsave",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );

        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Matrixsave failed"),
        )
    }

    /// VipsForeignSaveMatrixTarget (matrixsave_target), save image to matrix (.mat), priority=0, mono
    /// inp: `&VipsImage` -> Image to save
    /// target: `&VipsTarget` -> Target to save to
    pub fn matrixsave_target(&self, target: &VipsTarget) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let target_in: *mut bindings::VipsTarget = target.ctx;

            let vips_op_response = bindings::vips_matrixsave_target(
                inp_in,
                target_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("MatrixsaveTarget failed"),
            )
        }
    }

    pub fn matrixsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "matrixsave_target",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    VipsValue::Target(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("MatrixsaveTarget failed"),
        )
    }

    /// VipsMax (max), find image maximum
    /// returns `f64` - Output value
    pub fn max(&self) -> Result<f64> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: f64 = f64::from(0);

            let vips_op_response = bindings::vips_max(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                out_out,
                Error::OperationError("Max failed"),
            )
        }
    }

    pub fn max_with_opts(&self, option: VOption) -> Result<f64> {
        let mut out_out: f64 = 0.0;

        let vips_op_response = call(
            "max",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutDouble(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Max failed"),
        )
    }

    /// VipsMaxpair (maxpair), maximum of a pair of images
    /// left: `&VipsImage` -> Left-hand image argument
    /// right: `&VipsImage` -> Right-hand image argument
    /// returns `VipsImage` - Output image
    pub fn maxpair(&self, right: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let left_in: *mut bindings::VipsImage = self.ctx;
            let right_in: *mut bindings::VipsImage = right.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_maxpair(
                left_in,
                right_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Maxpair failed"),
            )
        }
    }

    /// VipsMeasure (measure), measure a set of patches on a color chart
    /// inp: `&VipsImage` -> Image to measure
    /// h: `i32` -> Number of patches across chart
    /// min: 1, max: 100000000, default: 1
    /// v: `i32` -> Number of patches down chart
    /// min: 1, max: 100000000, default: 1
    /// returns `VipsImage` - Output array of statistics
    pub fn measure(&self, h: i32, v: i32) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let h_in: i32 = h;
            let v_in: i32 = v;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_measure(
                inp_in,
                &mut out_out,
                h_in,
                v_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Measure failed"),
            )
        }
    }
    pub fn measure_with_opts(&self, h: i32, v: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "measure",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "h",
                    VipsValue::Int(h),
                )
                .with(
                    "v",
                    VipsValue::Int(v),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Measure failed"),
        )
    }

    pub fn median(&self, size: i32) -> Result<VipsImage> {
        let inp_in: *mut bindings::VipsImage = self.ctx;
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = unsafe {
            bindings::vips_median(
                inp_in,
                &mut out_out,
                size,
            )
        };
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("Globalbalance failed"),
        )
    }

    /// VipsMerge (merge), merge two images
    /// refp: `&VipsImage` -> Reference image
    /// sec: `&VipsImage` -> Secondary image
    /// direction: `Direction` -> Horizontal or vertical merge
    ///  `Horizontal` -> VIPS_DIRECTION_HORIZONTAL = 0 [DEFAULT]
    ///  `Vertical` -> VIPS_DIRECTION_VERTICAL = 1
    ///  `Last` -> VIPS_DIRECTION_LAST = 2
    /// dx: `i32` -> Horizontal displacement from sec to ref
    /// min: -100000000, max: 1000000000, default: 1
    /// dy: `i32` -> Vertical displacement from sec to ref
    /// min: -100000000, max: 1000000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn merge(
        &self,
        sec: &VipsImage,
        direction: Direction,
        dx: i32,
        dy: i32,
    ) -> Result<VipsImage> {
        unsafe {
            let refp_in: *mut bindings::VipsImage = self.ctx;
            let sec_in: *mut bindings::VipsImage = sec.ctx;
            let direction_in: i32 = direction as i32;
            let dx_in: i32 = dx;
            let dy_in: i32 = dy;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_merge(
                refp_in,
                sec_in,
                &mut out_out,
                direction_in
                    .try_into()
                    .unwrap(),
                dx_in,
                dy_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Merge failed"),
            )
        }
    }

    pub fn merge_with_opts(
        &self,
        sec: &VipsImage,
        direction: Direction,
        dx: i32,
        dy: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "merge",
            option
                .with(
                    "ref",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "sec",
                    VipsValue::Image(sec),
                )
                .with(
                    "direction",
                    VipsValue::Int(direction as i32),
                )
                .with(
                    "dx",
                    VipsValue::Int(dx),
                )
                .with(
                    "dy",
                    VipsValue::Int(dy),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Merge failed"),
        )
    }

    /// VipsMin (min), find image minimum
    /// returns `f64` - Output value
    pub fn min(&self) -> Result<f64> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: f64 = f64::from(0);

            let vips_op_response = bindings::vips_min(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                out_out,
                Error::OperationError("Min failed"),
            )
        }
    }

    pub fn min_with_opts(&self, option: VOption) -> Result<f64> {
        let mut out_out: f64 = 0.0;

        let vips_op_response = call(
            "min",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutDouble(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Min failed"),
        )
    }

    /// VipsMinpair (minpair), minimum of a pair of images
    /// left: `&VipsImage` -> Left-hand image argument
    /// right: `&VipsImage` -> Right-hand image argument
    /// returns `VipsImage` - Output image
    pub fn minpair(&self, right: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let left_in: *mut bindings::VipsImage = self.ctx;
            let right_in: *mut bindings::VipsImage = right.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_minpair(
                left_in,
                right_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Minpair failed"),
            )
        }
    }

    /// VipsMorph (morph), morphology operation
    /// mask: `&VipsImage` -> Input matrix image
    /// morph: `OperationMorphology` -> Morphological operation to perform
    ///  `Erode` -> VIPS_OPERATION_MORPHOLOGY_ERODE = 0 [DEFAULT]
    ///  `Dilate` -> VIPS_OPERATION_MORPHOLOGY_DILATE = 1
    ///  `Last` -> VIPS_OPERATION_MORPHOLOGY_LAST = 2
    /// returns `VipsImage` - Output image
    pub fn morph(&self, mask: &VipsImage, morph: OperationMorphology) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mask_in: *mut bindings::VipsImage = mask.ctx;
            let morph_in: i32 = morph as i32;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_morph(
                inp_in,
                &mut out_out,
                mask_in,
                morph_in
                    .try_into()
                    .unwrap(),
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Morph failed"),
            )
        }
    }

    /// VipsMosaic (mosaic), mosaic two images
    /// refp: `&VipsImage` -> Reference image
    /// sec: `&VipsImage` -> Secondary image
    /// direction: `Direction` -> Horizontal or vertical mosaic
    ///  `Horizontal` -> VIPS_DIRECTION_HORIZONTAL = 0 [DEFAULT]
    ///  `Vertical` -> VIPS_DIRECTION_VERTICAL = 1
    ///  `Last` -> VIPS_DIRECTION_LAST = 2
    /// xref: `i32` -> Position of reference tie-point
    /// min: 0, max: 1000000000, default: 1
    /// yref: `i32` -> Position of reference tie-point
    /// min: 0, max: 1000000000, default: 1
    /// xsec: `i32` -> Position of secondary tie-point
    /// min: 0, max: 1000000000, default: 1
    /// ysec: `i32` -> Position of secondary tie-point
    /// min: 0, max: 1000000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn mosaic(
        &self,
        sec: &VipsImage,
        direction: Direction,
        xref: i32,
        yref: i32,
        xsec: i32,
        ysec: i32,
    ) -> Result<VipsImage> {
        unsafe {
            let refp_in: *mut bindings::VipsImage = self.ctx;
            let sec_in: *mut bindings::VipsImage = sec.ctx;
            let direction_in: i32 = direction as i32;
            let xref_in: i32 = xref;
            let yref_in: i32 = yref;
            let xsec_in: i32 = xsec;
            let ysec_in: i32 = ysec;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_mosaic(
                refp_in,
                sec_in,
                &mut out_out,
                direction_in
                    .try_into()
                    .unwrap(),
                xref_in,
                yref_in,
                xsec_in,
                ysec_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Mosaic failed"),
            )
        }
    }

    pub fn mosaic_with_opts(
        &self,
        sec: &VipsImage,
        direction: Direction,
        xref: i32,
        yref: i32,
        xsec: i32,
        ysec: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "mosaic",
            option
                .with(
                    "ref",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "sec",
                    VipsValue::Image(sec),
                )
                .with(
                    "direction",
                    VipsValue::Int(direction as i32),
                )
                .with(
                    "xref",
                    VipsValue::Int(xref),
                )
                .with(
                    "yref",
                    VipsValue::Int(yref),
                )
                .with(
                    "xsec",
                    VipsValue::Int(xsec),
                )
                .with(
                    "ysec",
                    VipsValue::Int(ysec),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Mosaic failed"),
        )
    }

    /// VipsMosaic1 (mosaic1), first-order mosaic of two images
    /// refp: `&VipsImage` -> Reference image
    /// sec: `&VipsImage` -> Secondary image
    /// direction: `Direction` -> Horizontal or vertical mosaic
    ///  `Horizontal` -> VIPS_DIRECTION_HORIZONTAL = 0 [DEFAULT]
    ///  `Vertical` -> VIPS_DIRECTION_VERTICAL = 1
    ///  `Last` -> VIPS_DIRECTION_LAST = 2
    /// xr1: `i32` -> Position of first reference tie-point
    /// min: -1000000000, max: 1000000000, default: 1
    /// yr1: `i32` -> Position of first reference tie-point
    /// min: -1000000000, max: 1000000000, default: 1
    /// xs1: `i32` -> Position of first secondary tie-point
    /// min: -1000000000, max: 1000000000, default: 1
    /// ys1: `i32` -> Position of first secondary tie-point
    /// min: -1000000000, max: 1000000000, default: 1
    /// xr2: `i32` -> Position of second reference tie-point
    /// min: -1000000000, max: 1000000000, default: 1
    /// yr2: `i32` -> Position of second reference tie-point
    /// min: -1000000000, max: 1000000000, default: 1
    /// xs2: `i32` -> Position of second secondary tie-point
    /// min: -1000000000, max: 1000000000, default: 1
    /// ys2: `i32` -> Position of second secondary tie-point
    /// min: -1000000000, max: 1000000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn mosaic1(
        &self,
        sec: &VipsImage,
        direction: Direction,
        xr1: i32,
        yr1: i32,
        xs1: i32,
        ys1: i32,
        xr2: i32,
        yr2: i32,
        xs2: i32,
        ys2: i32,
    ) -> Result<VipsImage> {
        unsafe {
            let refp_in: *mut bindings::VipsImage = self.ctx;
            let sec_in: *mut bindings::VipsImage = sec.ctx;
            let direction_in: i32 = direction as i32;
            let xr_1_in: i32 = xr1;
            let yr_1_in: i32 = yr1;
            let xs_1_in: i32 = xs1;
            let ys_1_in: i32 = ys1;
            let xr_2_in: i32 = xr2;
            let yr_2_in: i32 = yr2;
            let xs_2_in: i32 = xs2;
            let ys_2_in: i32 = ys2;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_mosaic1(
                refp_in,
                sec_in,
                &mut out_out,
                direction_in
                    .try_into()
                    .unwrap(),
                xr_1_in,
                yr_1_in,
                xs_1_in,
                ys_1_in,
                xr_2_in,
                yr_2_in,
                xs_2_in,
                ys_2_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Mosaic1 failed"),
            )
        }
    }

    pub fn mosaic1_with_opts(
        &self,
        sec: &VipsImage,
        direction: Direction,
        xr1: i32,
        yr1: i32,
        xs1: i32,
        ys1: i32,
        xr2: i32,
        yr2: i32,
        xs2: i32,
        ys2: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "mosaic1",
            option
                .with(
                    "ref",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "sec",
                    VipsValue::Image(sec),
                )
                .with(
                    "direction",
                    VipsValue::Int(direction as i32),
                )
                .with(
                    "xr1",
                    VipsValue::Int(xr1),
                )
                .with(
                    "yr1",
                    VipsValue::Int(yr1),
                )
                .with(
                    "xs1",
                    VipsValue::Int(xs1),
                )
                .with(
                    "ys1",
                    VipsValue::Int(ys1),
                )
                .with(
                    "xr2",
                    VipsValue::Int(xr2),
                )
                .with(
                    "yr2",
                    VipsValue::Int(yr2),
                )
                .with(
                    "xs2",
                    VipsValue::Int(xs2),
                )
                .with(
                    "ys2",
                    VipsValue::Int(ys2),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Mosaic1 failed"),
        )
    }

    /// VipsMsb (msb), pick most-significant byte from an image
    /// returns `VipsImage` - Output image
    pub fn msb(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_msb(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Msb failed"),
            )
        }
    }

    pub fn msb_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "msb",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Msb failed"),
        )
    }

    /// VipsMultiply (multiply), multiply two images
    /// left: `&VipsImage` -> Left-hand image argument
    /// right: `&VipsImage` -> Right-hand image argument
    /// returns `VipsImage` - Output image
    pub fn multiply(&self, right: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let left_in: *mut bindings::VipsImage = self.ctx;
            let right_in: *mut bindings::VipsImage = right.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_multiply(
                left_in,
                right_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Multiply failed"),
            )
        }
    }

    pub fn niftiload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "niftiload",
            VOption::new()
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("niftiload failed"),
        )
    }

    pub fn niftiload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "niftiload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("niftiload_with_opts failed"),
        )
    }

    pub fn niftiload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "niftiload_source",
            VOption::new()
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("niftiload_source failed"),
        )
    }

    pub fn niftiload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "niftiload_source",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("niftiload_source_with_opts failed"),
        )
    }

    pub fn niftisave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "niftisave",
            VOption::new()
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("niftisave failed"),
        )
    }

    pub fn niftisave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "niftisave",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("niftisave_with_opts failed"),
        )
    }

    pub fn openexrload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "openexrload",
            VOption::new()
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("openexrload failed"),
        )
    }

    pub fn openexrload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "openexrload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("openexrload_with_opts failed"),
        )
    }

    pub fn openslideload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "openslideload",
            VOption::new()
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("openslideload failed"),
        )
    }

    pub fn openslideload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "openslideload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("openslideload_with_opts failed"),
        )
    }

    pub fn openslideload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "openslideload_source",
            VOption::new()
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("openslideload_source failed"),
        )
    }

    pub fn openslideload_source_with_opts(
        source: &VipsSource,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "openslideload_source",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("openslideload_source_with_opts failed"),
        )
    }

    pub fn pdfload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "pdfload",
            VOption::new()
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Pdfload failed"),
        )
    }

    pub fn pdfload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "pdfload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Pdfload failed"),
        )
    }

    pub fn pdfload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "pdfload_buffer",
            VOption::new()
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "buffer",
                    VipsValue::Buffer(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("PdfloadBuffer failed"),
        )
    }

    pub fn pdfload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "pdfload_buffer",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "buffer",
                    VipsValue::Buffer(buffer),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("PdfloadBuffer failed"),
        )
    }

    pub fn pdfload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "pdfload_source",
            VOption::new()
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("pdfload_source failed"),
        )
    }

    pub fn pdfload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "pdfload_source",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("pdfload_source_with_opts failed"),
        )
    }

    /// VipsPercent (percent), find threshold for percent of pixels
    /// percent: `f64` -> Percent of pixels
    /// min: 0, max: 100, default: 50
    /// returns `i32` - Threshold above which lie percent of pixels
    pub fn percent(&self, percent: f64) -> Result<i32> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let percent_in: f64 = percent;
            let mut threshold_out: i32 = 0;

            let vips_op_response = bindings::vips_percent(
                inp_in,
                percent_in,
                &mut threshold_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                threshold_out,
                Error::OperationError("Percent failed"),
            )
        }
    }

    /// VipsPerlin (perlin), make a perlin noise image
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn perlin(width: i32, height: i32) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_perlin(
                &mut out_out,
                width_in,
                height_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Perlin failed"),
            )
        }
    }

    pub fn perlin_with_opts(width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "perlin",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Perlin failed"),
        )
    }

    /// VipsPhasecor (phasecor), calculate phase correlation
    /// in2: `&VipsImage` -> Second input image
    /// returns `VipsImage` - Output image
    pub fn phasecor(&self, in2: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let in_2_in: *mut bindings::VipsImage = in2.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_phasecor(
                inp_in,
                in_2_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Phasecor failed"),
            )
        }
    }

    /// VipsForeignLoadPngFile (pngload), load png from file (.png), priority=200, is_a, get_flags, get_flags_filename, header, load
    /// filename: `&str` -> Filename to load from
    /// returns `VipsImage` - Output image
    pub fn pngload(filename: &str) -> Result<VipsImage> {
        unsafe {
            let filename_in: CString = utils::new_c_string(filename)?;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_pngload(
                filename_in.as_ptr(),
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Pngload failed"),
            )
        }
    }

    pub fn pngload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "pngload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Pngload failed"),
        )
    }

    /// VipsForeignLoadPngBuffer (pngload_buffer), load png from buffer, priority=200, is_a_buffer, get_flags, get_flags_filename, header, load
    /// buffer: `&[u8]` -> Buffer to load from
    /// returns `VipsImage` - Output image
    pub fn pngload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        unsafe {
            let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_pngload_buffer(
                buffer_in,
                buffer.len() as u64,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("PngloadBuffer failed"),
            )
        }
    }

    pub fn pngload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "pngload_buffer",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "buffer",
                    VipsValue::Buffer(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("PngloadBuffer failed"),
        )
    }

    /// VipsForeignLoadPngSource (pngload_source), load png from source, priority=200, is_a_source, get_flags, get_flags_filename, header, load
    /// source: `&VipsSource` -> Source to load from
    /// returns `VipsImage` - Output image
    pub fn pngload_source(source: &VipsSource) -> Result<VipsImage> {
        unsafe {
            let source_in: *mut bindings::VipsSource = source.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_pngload_source(
                source_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("PngloadSource failed"),
            )
        }
    }

    pub fn pngload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "pngload_source",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("PngloadSource failed"),
        )
    }

    /// VipsForeignSavePngFile (pngsave), save image to png file (.png), priority=0, rgba
    /// inp: `&VipsImage` -> Image to save
    /// filename: `&str` -> Filename to save to
    pub fn pngsave(&self, filename: &str) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let filename_in: CString = utils::new_c_string(filename)?;

            let vips_op_response = bindings::vips_pngsave(
                inp_in,
                filename_in.as_ptr(),
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("Pngsave failed"),
            )
        }
    }

    pub fn pngsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "pngsave",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Pngsave failed"),
        )
    }

    /// VipsForeignSavePngBuffer (pngsave_buffer), save image to png buffer (.png), priority=0, rgba
    /// inp: `&VipsImage` -> Image to save
    /// returns `Vec<u8>` - Buffer to save to
    pub fn pngsave_buffer(&self) -> Result<Vec<u8>> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut buffer_buf_size: u64 = 0;
            let mut buffer_out: *mut c_void = null_mut();

            let vips_op_response = bindings::vips_pngsave_buffer(
                inp_in,
                &mut buffer_out,
                &mut buffer_buf_size,
                NULL,
            );
            utils::result(
                vips_op_response,
                utils::new_byte_array(
                    buffer_out,
                    buffer_buf_size,
                ),
                Error::OperationError("PngsaveBuffer failed"),
            )
        }
    }

    pub fn pngsave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut blob = VipsBlob::from(null_mut());

        let vips_op_response = call(
            "pngsave_buffer",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    VipsValue::MutBlob(&mut blob),
                ),
        );

        utils::result(
            vips_op_response,
            blob.into(),
            Error::OperationError("PngsaveBuffer failed"),
        )
    }

    /// VipsForeignSavePngTarget (pngsave_target), save image to target as PNG (.png), priority=0, rgba
    /// inp: `&VipsImage` -> Image to save
    /// target: `&VipsTarget` -> Target to save to
    pub fn pngsave_target(&self, target: &VipsTarget) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let target_in: *mut bindings::VipsTarget = target.ctx;

            let vips_op_response = bindings::vips_pngsave_target(
                inp_in,
                target_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("PngsaveTarget failed"),
            )
        }
    }

    pub fn pngsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "pngsave_target",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    VipsValue::Target(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("PngsaveTarget failed"),
        )
    }

    /// VipsForeignLoadPpmFile (ppmload), load ppm from file (.pbm, .pgm, .ppm, .pfm, .pnm), priority=200, untrusted, is_a, get_flags, header, load
    /// filename: `&str` -> Filename to load from
    /// returns `VipsImage` - Output image
    pub fn ppmload(filename: &str) -> Result<VipsImage> {
        unsafe {
            let filename_in: CString = utils::new_c_string(filename)?;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_ppmload(
                filename_in.as_ptr(),
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Ppmload failed"),
            )
        }
    }

    pub fn ppmload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "ppmload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Ppmload failed"),
        )
    }

    pub fn ppmload_buffer(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "ppmload_buffer",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "buffer",
                    VipsValue::Buffer(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ppmload_buffer failed"),
        )
    }

    /// VipsForeignLoadPpmSource (ppmload_source), load ppm base class (.pbm, .pgm, .ppm, .pfm, .pnm), priority=200, untrusted, is_a_source, get_flags, header, load
    /// source: `&VipsSource` -> Source to load from
    /// returns `VipsImage` - Output image
    pub fn ppmload_source(source: &VipsSource) -> Result<VipsImage> {
        unsafe {
            let source_in: *mut bindings::VipsSource = source.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_ppmload_source(
                source_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("PpmloadSource failed"),
            )
        }
    }

    pub fn ppmload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "ppmload_source",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("PpmloadSource failed"),
        )
    }

    /// VipsForeignSavePpmFile (ppmsave), save image to ppm file (.pbm, .pgm, .ppm, .pfm, .pnm), priority=0, any
    /// inp: `&VipsImage` -> Image to save
    /// filename: `&str` -> Filename to save to
    pub fn ppmsave(&self, filename: &str) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let filename_in: CString = utils::new_c_string(filename)?;

            let vips_op_response = bindings::vips_ppmsave(
                inp_in,
                filename_in.as_ptr(),
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("Ppmsave failed"),
            )
        }
    }

    pub fn ppmsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "ppmsave",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Ppmsave failed"),
        )
    }

    /// VipsForeignSavePpmTarget (ppmsave_target), save to ppm (.ppm), priority=0, any
    /// inp: `&VipsImage` -> Image to save
    /// target: `&VipsTarget` -> Target to save to
    pub fn ppmsave_target(&self, target: &VipsTarget) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let target_in: *mut bindings::VipsTarget = target.ctx;

            let vips_op_response = bindings::vips_ppmsave_target(
                inp_in,
                target_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("PpmsaveTarget failed"),
            )
        }
    }

    pub fn ppmsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "ppmsave_target",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    VipsValue::Target(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("PpmsaveTarget failed"),
        )
    }

    /// VipsPremultiply (premultiply), premultiply image alpha
    /// returns `VipsImage` - Output image
    pub fn premultiply(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_premultiply(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Premultiply failed"),
            )
        }
    }

    pub fn premultiply_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "premultiply",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Premultiply failed"),
        )
    }

    /// VipsPrewitt (prewitt), Prewitt edge detector
    /// returns `VipsImage` - Output image
    pub fn prewitt(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_prewitt(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Prewitt failed"),
            )
        }
    }

    /// VipsProfile (profile), find image profiles
    /// Tuple (
    /// VipsImage - First non-zero pixel in column
    /// VipsImage - First non-zero pixel in row
    ///)
    pub fn profile(
        &self,
    ) -> Result<(
        VipsImage,
        VipsImage,
    )> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut columns_out: *mut bindings::VipsImage = null_mut();
            let mut rows_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_profile(
                inp_in,
                &mut columns_out,
                &mut rows_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                (
                    VipsImage {
                        ctx: columns_out,
                    },
                    VipsImage {
                        ctx: rows_out,
                    },
                ),
                Error::OperationError("Profile failed"),
            )
        }
    }

    /// VipsProfileLoad (profile_load), load named ICC profile
    /// name: `&str` -> Profile name
    /// returns `Vec<u8>` - Loaded profile
    pub fn profile_load(name: &str) -> Result<Vec<u8>> {
        unsafe {
            let name_in: CString = utils::new_c_string(name)?;
            let mut profile_out: *mut bindings::VipsBlob = null_mut();

            let vips_op_response = bindings::vips_profile_load(
                name_in.as_ptr(),
                &mut profile_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsBlob {
                    ctx: profile_out,
                }
                .into(),
                Error::OperationError("ProfileLoad failed"),
            )
        }
    }

    /// VipsProject (project), find image projections
    /// Tuple (
    /// VipsImage - Sums of columns
    /// VipsImage - Sums of rows
    ///)
    pub fn project(
        &self,
    ) -> Result<(
        VipsImage,
        VipsImage,
    )> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut columns_out: *mut bindings::VipsImage = null_mut();
            let mut rows_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_project(
                inp_in,
                &mut columns_out,
                &mut rows_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                (
                    VipsImage {
                        ctx: columns_out,
                    },
                    VipsImage {
                        ctx: rows_out,
                    },
                ),
                Error::OperationError("Project failed"),
            )
        }
    }

    /// VipsQuadratic (quadratic), resample an image with a quadratic transform
    /// coeff: `&VipsImage` -> Coefficient matrix
    /// returns `VipsImage` - Output image
    pub fn quadratic(&self, coeff: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let coeff_in: *mut bindings::VipsImage = coeff.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_quadratic(
                inp_in,
                &mut out_out,
                coeff_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Quadratic failed"),
            )
        }
    }

    pub fn quadratic_with_opts(&self, coeff: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "quadratic",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "coeff",
                    VipsValue::Image(coeff),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Quadratic failed"),
        )
    }

    /// VipsRad2float (rad2float), unpack Radiance coding to float RGB
    /// returns `VipsImage` - Output image
    pub fn rad2float(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_rad2float(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Rad2Float failed"),
            )
        }
    }

    /// VipsForeignLoadRadFile (radload), load a Radiance image from a file (.hdr), priority=-50, untrusted, is_a, get_flags, get_flags_filename, header, load
    /// filename: `&str` -> Filename to load from
    /// returns `VipsImage` - Output image
    pub fn radload(filename: &str) -> Result<VipsImage> {
        unsafe {
            let filename_in: CString = utils::new_c_string(filename)?;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_radload(
                filename_in.as_ptr(),
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Radload failed"),
            )
        }
    }

    pub fn radload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "radload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Radload failed"),
        )
    }

    /// VipsForeignLoadRadBuffer (radload_buffer), load rad from buffer, priority=-50, untrusted, is_a_buffer, get_flags, get_flags_filename, header, load
    /// buffer: `&[u8]` -> Buffer to load from
    /// returns `VipsImage` - Output image
    pub fn radload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        unsafe {
            let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_radload_buffer(
                buffer_in,
                buffer.len() as u64,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("RadloadBuffer failed"),
            )
        }
    }

    pub fn radload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "radload_buffer",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "buffer",
                    VipsValue::Buffer(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("RadloadBuffer failed"),
        )
    }

    /// VipsForeignLoadRadSource (radload_source), load rad from source, priority=-50, untrusted, is_a_source, get_flags, get_flags_filename, header, load
    /// source: `&VipsSource` -> Source to load from
    /// returns `VipsImage` - Output image
    pub fn radload_source(source: &VipsSource) -> Result<VipsImage> {
        unsafe {
            let source_in: *mut bindings::VipsSource = source.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_radload_source(
                source_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("RadloadSource failed"),
            )
        }
    }

    pub fn radload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "radload_source",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("RadloadSource failed"),
        )
    }

    /// VipsForeignSaveRadFile (radsave), save image to Radiance file (.hdr), priority=0, rgb
    /// inp: `&VipsImage` -> Image to save
    /// filename: `&str` -> Filename to save to
    pub fn radsave(&self, filename: &str) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let filename_in: CString = utils::new_c_string(filename)?;

            let vips_op_response = bindings::vips_radsave(
                inp_in,
                filename_in.as_ptr(),
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("Radsave failed"),
            )
        }
    }

    pub fn radsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "radsave",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Radsave failed"),
        )
    }

    /// VipsForeignSaveRadBuffer (radsave_buffer), save image to Radiance buffer (.hdr), priority=0, rgb
    /// inp: `&VipsImage` -> Image to save
    /// returns `Vec<u8>` - Buffer to save to
    pub fn radsave_buffer(&self) -> Result<Vec<u8>> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut buffer_buf_size: u64 = 0;
            let mut buffer_out: *mut c_void = null_mut();

            let vips_op_response = bindings::vips_radsave_buffer(
                inp_in,
                &mut buffer_out,
                &mut buffer_buf_size,
                NULL,
            );
            utils::result(
                vips_op_response,
                utils::new_byte_array(
                    buffer_out,
                    buffer_buf_size,
                ),
                Error::OperationError("RadsaveBuffer failed"),
            )
        }
    }

    pub fn radsave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut blob = VipsBlob::from(null_mut());

        let vips_op_response = call(
            "radsave_buffer",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    VipsValue::MutBlob(&mut blob),
                ),
        );

        utils::result(
            vips_op_response,
            blob.into(),
            Error::OperationError("RadsaveBuffer failed"),
        )
    }

    /// VipsForeignSaveRadTarget (radsave_target), save image to Radiance target (.hdr), priority=0, rgb
    /// inp: `&VipsImage` -> Image to save
    /// target: `&VipsTarget` -> Target to save to
    pub fn radsave_target(&self, target: &VipsTarget) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let target_in: *mut bindings::VipsTarget = target.ctx;

            let vips_op_response = bindings::vips_radsave_target(
                inp_in,
                target_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("RadsaveTarget failed"),
            )
        }
    }

    pub fn radsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "radsave_target",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    VipsValue::Target(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("RadsaveTarget failed"),
        )
    }

    /// VipsRank (rank), rank filter
    /// width: `i32` -> Window width in pixels
    /// min: 1, max: 100000, default: 11
    /// height: `i32` -> Window height in pixels
    /// min: 1, max: 100000, default: 11
    /// index: `i32` -> Select pixel at index
    /// min: 0, max: 100000000, default: 50
    /// returns `VipsImage` - Output image
    pub fn rank(&self, width: i32, height: i32, index: i32) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let width_in: i32 = width;
            let height_in: i32 = height;
            let index_in: i32 = index;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_rank(
                inp_in,
                &mut out_out,
                width_in,
                height_in,
                index_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Rank failed"),
            )
        }
    }

    /// VipsForeignLoadRaw (rawload), load raw data from a file, priority=0, untrusted, get_flags, get_flags_filename, header
    /// filename: `&str` -> Filename to load from
    /// width: `i32` -> Image width in pixels
    /// min: 0, max: 100000000, default: 0
    /// height: `i32` -> Image height in pixels
    /// min: 0, max: 100000000, default: 0
    /// bands: `i32` -> Number of bands in image
    /// min: 0, max: 100000000, default: 0
    /// returns `VipsImage` - Output image
    pub fn rawload(filename: &str, width: i32, height: i32, bands: i32) -> Result<VipsImage> {
        unsafe {
            let filename_in: CString = utils::new_c_string(filename)?;
            let width_in: i32 = width;
            let height_in: i32 = height;
            let bands_in: i32 = bands;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_rawload(
                filename_in.as_ptr(),
                &mut out_out,
                width_in,
                height_in,
                bands_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Rawload failed"),
            )
        }
    }
    pub fn rawload_with_opts(
        filename: &str,
        width: i32,
        height: i32,
        bands: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "rawload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                )
                .with(
                    "bands",
                    VipsValue::Int(bands),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Rawload failed"),
        )
    }

    /// VipsForeignSaveRaw (rawsave), save image to raw file (.raw), priority=0, any
    /// inp: `&VipsImage` -> Image to save
    /// filename: `&str` -> Filename to save to
    pub fn rawsave(&self, filename: &str) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let filename_in: CString = utils::new_c_string(filename)?;

            let vips_op_response = bindings::vips_rawsave(
                inp_in,
                filename_in.as_ptr(),
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("Rawsave failed"),
            )
        }
    }
    pub fn rawsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "rawsave",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Rawsave failed"),
        )
    }

    /// VipsForeignSaveRawBuffer (rawsave_buffer), write raw image to buffer (.raw), priority=0, any
    /// inp: `&VipsImage` -> Image to save
    /// returns `Vec<u8>` - Buffer to save to
    pub fn rawsave_buffer(&self) -> Result<Vec<u8>> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut buffer_buf_size: u64 = 0;
            let mut buffer_out: *mut c_void = null_mut();

            let vips_op_response = bindings::vips_rawsave_buffer(
                inp_in,
                &mut buffer_out,
                &mut buffer_buf_size,
                NULL,
            );
            utils::result(
                vips_op_response,
                utils::new_byte_array(
                    buffer_out,
                    buffer_buf_size,
                ),
                Error::OperationError("RawsaveBuffer failed"),
            )
        }
    }

    pub fn rawsave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut blob = VipsBlob::from(null_mut());

        let vips_op_response = call(
            "rawsave_buffer",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    VipsValue::MutBlob(&mut blob),
                ),
        );

        utils::result(
            vips_op_response,
            blob.into(),
            Error::OperationError("RawsaveBuffer failed"),
        )
    }

    /// VipsForeignSaveRawTarget (rawsave_target), write raw image to target (.raw), priority=0, any
    /// inp: `&VipsImage` -> Image to save
    /// target: `&VipsTarget` -> Target to save to
    pub fn rawsave_target(&self, target: &VipsTarget) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let target_in: *mut bindings::VipsTarget = target.ctx;

            let vips_op_response = bindings::vips_rawsave_target(
                inp_in,
                target_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("RawsaveTarget failed"),
            )
        }
    }

    pub fn rawsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "rawsave_target",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    VipsValue::Target(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("RawsaveTarget failed"),
        )
    }

    /// VipsRecomb (recomb), linear recombination with matrix
    /// m: `&VipsImage` -> Matrix of coefficients
    /// returns `VipsImage` - Output image
    pub fn recomb(&self, m: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let m_in: *mut bindings::VipsImage = m.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_recomb(
                inp_in,
                &mut out_out,
                m_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Recomb failed"),
            )
        }
    }

    /// VipsReduce (reduce), reduce an image
    /// hshrink: `f64` -> Horizontal shrink factor
    /// min: 1, max: 1000000, default: 1
    /// vshrink: `f64` -> Vertical shrink factor
    /// min: 1, max: 1000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn reduce(&self, hshrink: f64, vshrink: f64) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let hshrink_in: f64 = hshrink;
            let vshrink_in: f64 = vshrink;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_reduce(
                inp_in,
                &mut out_out,
                hshrink_in,
                vshrink_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Reduce failed"),
            )
        }
    }

    pub fn reduce_with_opts(
        &self,
        hshrink: f64,
        vshrink: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "reduce",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "hshrink",
                    VipsValue::Double(hshrink),
                )
                .with(
                    "vshrink",
                    VipsValue::Double(vshrink),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Reduce failed"),
        )
    }

    /// VipsReduceh (reduceh), shrink an image horizontally
    /// hshrink: `f64` -> Horizontal shrink factor
    /// min: 1, max: 1000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn reduceh(&self, hshrink: f64) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let hshrink_in: f64 = hshrink;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_reduceh(
                inp_in,
                &mut out_out,
                hshrink_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Reduceh failed"),
            )
        }
    }

    pub fn reduceh_with_opts(&self, hshrink: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "reduceh",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "hshrink",
                    VipsValue::Double(hshrink),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Reduceh failed"),
        )
    }

    /// VipsReducev (reducev), shrink an image vertically
    /// vshrink: `f64` -> Vertical shrink factor
    /// min: 1, max: 1000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn reducev(&self, vshrink: f64) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let vshrink_in: f64 = vshrink;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_reducev(
                inp_in,
                &mut out_out,
                vshrink_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Reducev failed"),
            )
        }
    }

    pub fn reducev_with_opts(&self, vshrink: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "reducev",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "vshrink",
                    VipsValue::Double(vshrink),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Reducev failed"),
        )
    }

    /// VipsRelational (relational), relational operation on two images
    /// left: `&VipsImage` -> Left-hand image argument
    /// right: `&VipsImage` -> Right-hand image argument
    /// relational: `OperationRelational` -> Relational to perform
    ///  `Equal` -> VIPS_OPERATION_RELATIONAL_EQUAL = 0 [DEFAULT]
    ///  `Noteq` -> VIPS_OPERATION_RELATIONAL_NOTEQ = 1
    ///  `Less` -> VIPS_OPERATION_RELATIONAL_LESS = 2
    ///  `Lesseq` -> VIPS_OPERATION_RELATIONAL_LESSEQ = 3
    ///  `More` -> VIPS_OPERATION_RELATIONAL_MORE = 4
    ///  `Moreeq` -> VIPS_OPERATION_RELATIONAL_MOREEQ = 5
    ///  `Last` -> VIPS_OPERATION_RELATIONAL_LAST = 6
    /// returns `VipsImage` - Output image
    pub fn relational(
        &self,
        right: &VipsImage,
        relational: OperationRelational,
    ) -> Result<VipsImage> {
        unsafe {
            let left_in: *mut bindings::VipsImage = self.ctx;
            let right_in: *mut bindings::VipsImage = right.ctx;
            let relational_in: i32 = relational as i32;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_relational(
                left_in,
                right_in,
                &mut out_out,
                relational_in
                    .try_into()
                    .unwrap(),
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Relational failed"),
            )
        }
    }

    /// VipsRelationalConst (relational_const), relational operations against a constant
    /// relational: `OperationRelational` -> Relational to perform
    ///  `Equal` -> VIPS_OPERATION_RELATIONAL_EQUAL = 0 [DEFAULT]
    ///  `Noteq` -> VIPS_OPERATION_RELATIONAL_NOTEQ = 1
    ///  `Less` -> VIPS_OPERATION_RELATIONAL_LESS = 2
    ///  `Lesseq` -> VIPS_OPERATION_RELATIONAL_LESSEQ = 3
    ///  `More` -> VIPS_OPERATION_RELATIONAL_MORE = 4
    ///  `Moreeq` -> VIPS_OPERATION_RELATIONAL_MOREEQ = 5
    ///  `Last` -> VIPS_OPERATION_RELATIONAL_LAST = 6
    /// c: `&[f64]` -> Array of constants
    /// returns `VipsImage` - Output image
    pub fn relational_const(
        &self,
        relational: OperationRelational,
        c: &[f64],
    ) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let relational_in: i32 = relational as i32;
            let c_in: *const f64 = c.as_ptr();
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_relational_const(
                inp_in,
                &mut out_out,
                relational_in
                    .try_into()
                    .unwrap(),
                c_in as _,
                c.len() as i32,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("RelationalConst failed"),
            )
        }
    }

    /// VipsRemainder (remainder), remainder after integer division of two images
    /// left: `&VipsImage` -> Left-hand image argument
    /// right: `&VipsImage` -> Right-hand image argument
    /// returns `VipsImage` - Output image
    pub fn remainder(&self, right: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let left_in: *mut bindings::VipsImage = self.ctx;
            let right_in: *mut bindings::VipsImage = right.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_remainder(
                left_in,
                right_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Remainder failed"),
            )
        }
    }

    /// VipsRemainderConst (remainder_const), remainder after integer division of an image and a constant
    /// c: `&[f64]` -> Array of constants
    /// returns `VipsImage` - Output image
    pub fn remainder_const(&self, c: &[f64]) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let c_in: *const f64 = c.as_ptr();
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_remainder_const(
                inp_in,
                &mut out_out,
                c_in as _,
                c.len() as i32,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("RemainderConst failed"),
            )
        }
    }

    pub fn remosaic(&self, old_str: &str, new_str: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "remosaic",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "old_str",
                    VipsValue::Str(old_str),
                )
                .with(
                    "new_str",
                    VipsValue::Str(new_str),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("remosaic failed"),
        )
    }

    /// VipsReplicate (replicate), replicate an image
    /// across: `i32` -> Repeat this many times horizontally
    /// min: 1, max: 1000000, default: 1
    /// down: `i32` -> Repeat this many times vertically
    /// min: 1, max: 1000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn replicate(&self, across: i32, down: i32) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let across_in: i32 = across;
            let down_in: i32 = down;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_replicate(
                inp_in,
                &mut out_out,
                across_in,
                down_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Replicate failed"),
            )
        }
    }

    /// VipsResize (resize), resize an image
    /// scale: `f64` -> Scale image by this factor
    /// min: 0, max: 10000000, default: 0
    /// returns `VipsImage` - Output image
    pub fn resize(&self, scale: f64) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let scale_in: f64 = scale;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_resize(
                inp_in,
                &mut out_out,
                scale_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Resize failed"),
            )
        }
    }

    pub fn resize_with_opts(&self, scale: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "resize",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "scale",
                    VipsValue::Double(scale),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Resize failed"),
        )
    }

    /// VipsRot (rot), rotate an image
    /// angle: `Angle` -> Angle to rotate image
    ///  `D0` -> VIPS_ANGLE_D0 = 0
    ///  `D90` -> VIPS_ANGLE_D90 = 1 [DEFAULT]
    ///  `D180` -> VIPS_ANGLE_D180 = 2
    ///  `D270` -> VIPS_ANGLE_D270 = 3
    ///  `Last` -> VIPS_ANGLE_LAST = 4
    /// returns `VipsImage` - Output image
    pub fn rot(&self, angle: Angle) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let angle_in: i32 = angle as i32;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_rot(
                inp_in,
                &mut out_out,
                angle_in
                    .try_into()
                    .unwrap(),
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Rot failed"),
            )
        }
    }

    /// VipsRot45 (rot45), rotate an image
    /// returns `VipsImage` - Output image
    pub fn rot45(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_rot45(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Rot45 failed"),
            )
        }
    }

    pub fn rot45_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "rot45",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Rot45 failed"),
        )
    }

    /// VipsRotate (rotate), rotate an image by a number of degrees
    /// angle: `f64` -> Rotate clockwise by this many degrees
    /// min: -10000000, max: 10000000, default: 0
    /// returns `VipsImage` - Output image
    pub fn rotate(&self, angle: f64) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let angle_in: f64 = angle;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_rotate(
                inp_in,
                &mut out_out,
                angle_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Rotate failed"),
            )
        }
    }

    pub fn rotate_with_opts(&self, angle: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "rotate",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "angle",
                    VipsValue::Double(angle),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Rotate failed"),
        )
    }

    /// VipsRound (round), perform a round function on an image
    /// round: `OperationRound` -> Rounding operation to perform
    ///  `Rint` -> VIPS_OPERATION_ROUND_RINT = 0 [DEFAULT]
    ///  `Ceil` -> VIPS_OPERATION_ROUND_CEIL = 1
    ///  `Floor` -> VIPS_OPERATION_ROUND_FLOOR = 2
    ///  `Last` -> VIPS_OPERATION_ROUND_LAST = 3
    /// returns `VipsImage` - Output image
    pub fn round(&self, round: OperationRound) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let round_in: i32 = round as i32;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_round(
                inp_in,
                &mut out_out,
                round_in
                    .try_into()
                    .unwrap(),
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Round failed"),
            )
        }
    }

    /// VipssRGB2HSV (sRGB2HSV), transform sRGB to HSV
    /// returns `VipsImage` - Output image
    pub fn sRGB2HSV(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_sRGB2HSV(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("SRgb2Hsv failed"),
            )
        }
    }

    pub fn sRGB2scRGB(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_sRGB2scRGB(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("SRgb2ScRgb failed"),
            )
        }
    }

    /// VipsscRGB2BW (scRGB2BW), convert scRGB to BW
    /// returns `VipsImage` - Output image
    pub fn scRGB2BW(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_scRGB2BW(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("ScRgb2Bw failed"),
            )
        }
    }

    pub fn scRGB2BW_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "scRGB2BW",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ScRgb2Bw failed"),
        )
    }

    /// VipsscRGB2XYZ (scRGB2XYZ), transform scRGB to XYZ
    /// returns `VipsImage` - Output image
    pub fn scRGB2XYZ(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_scRGB2XYZ(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("ScRgb2Xyz failed"),
            )
        }
    }

    /// VipsscRGB2sRGB (scRGB2sRGB), convert an scRGB image to sRGB
    /// returns `VipsImage` - Output image
    pub fn scRGB2sRGB(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_scRGB2sRGB(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("ScRgb2SRgb failed"),
            )
        }
    }

    pub fn scRGB2sRGB_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "scRGB2sRGB",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ScRgb2SRgb failed"),
        )
    }

    /// VipsScale (scale), scale an image to uchar
    /// returns `VipsImage` - Output image
    pub fn scale(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_scale(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Scale failed"),
            )
        }
    }

    pub fn scale_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "scale",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Scale failed"),
        )
    }

    /// VipsScharr (scharr), Scharr edge detector
    /// returns `VipsImage` - Output image
    pub fn scharr(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_scharr(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Scharr failed"),
            )
        }
    }

    /// VipsSdf (sdf), create an SDF image
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 1
    /// shape: `SdfShape` -> SDF shape to create
    ///  `Circle` -> VIPS_SDF_SHAPE_CIRCLE = 0 [DEFAULT]
    ///  `Box` -> VIPS_SDF_SHAPE_BOX = 1
    ///  `RoundedBox` -> VIPS_SDF_SHAPE_ROUNDED_BOX = 2
    ///  `Line` -> VIPS_SDF_SHAPE_LINE = 3
    ///  `Last` -> VIPS_SDF_SHAPE_LAST = 4
    /// returns `VipsImage` - Output image
    pub fn sdf(width: i32, height: i32, shape: SdfShape) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let shape_in: i32 = shape as i32;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_sdf(
                &mut out_out,
                width_in,
                height_in,
                shape_in
                    .try_into()
                    .unwrap(),
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Sdf failed"),
            )
        }
    }

    pub fn sdf_with_opts(
        width: i32,
        height: i32,
        shape: SdfShape,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "sdf",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                )
                .with(
                    "shape",
                    VipsValue::Int(shape as i32),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Sdf failed"),
        )
    }

    /// VipsSequential (sequential), check sequential access
    /// returns `VipsImage` - Output image
    pub fn sequential(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_sequential(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Sequential failed"),
            )
        }
    }
    pub fn sequential_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "sequential",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Sequential failed"),
        )
    }

    /// VipsSharpen (sharpen), unsharp masking for print
    /// returns `VipsImage` - Output image
    pub fn sharpen(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_sharpen(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Sharpen failed"),
            )
        }
    }

    pub fn sharpen_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "sharpen",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Sharpen failed"),
        )
    }

    /// VipsShrink (shrink), shrink an image
    /// hshrink: `f64` -> Horizontal shrink factor
    /// min: 1, max: 1000000, default: 1
    /// vshrink: `f64` -> Vertical shrink factor
    /// min: 1, max: 1000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn shrink(&self, hshrink: f64, vshrink: f64) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let hshrink_in: f64 = hshrink;
            let vshrink_in: f64 = vshrink;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_shrink(
                inp_in,
                &mut out_out,
                hshrink_in,
                vshrink_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Shrink failed"),
            )
        }
    }

    pub fn shrink_with_opts(
        &self,
        hshrink: f64,
        vshrink: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "shrink",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "hshrink",
                    VipsValue::Double(hshrink),
                )
                .with(
                    "vshrink",
                    VipsValue::Double(vshrink),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Shrink failed"),
        )
    }

    /// VipsShrinkh (shrinkh), shrink an image horizontally
    /// hshrink: `i32` -> Horizontal shrink factor
    /// min: 1, max: 1000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn shrinkh(&self, hshrink: i32) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let hshrink_in: i32 = hshrink;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_shrinkh(
                inp_in,
                &mut out_out,
                hshrink_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Shrinkh failed"),
            )
        }
    }

    pub fn shrinkh_with_opts(&self, hshrink: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "shrinkh",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "hshrink",
                    VipsValue::Int(hshrink),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Shrinkh failed"),
        )
    }

    /// VipsShrinkv (shrinkv), shrink an image vertically
    /// vshrink: `i32` -> Vertical shrink factor
    /// min: 1, max: 1000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn shrinkv(&self, vshrink: i32) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let vshrink_in: i32 = vshrink;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_shrinkv(
                inp_in,
                &mut out_out,
                vshrink_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Shrinkv failed"),
            )
        }
    }

    pub fn shrinkv_with_opts(&self, vshrink: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "shrinkv",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "vshrink",
                    VipsValue::Int(vshrink),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Shrinkv failed"),
        )
    }

    /// VipsSign (sign), unit vector of pixel
    /// returns `VipsImage` - Output image
    pub fn sign(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_sign(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Sign failed"),
            )
        }
    }

    /// VipsSimilarity (similarity), similarity transform of an image
    /// returns `VipsImage` - Output image
    pub fn similarity(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_similarity(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Similarity failed"),
            )
        }
    }

    pub fn similarity_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "similarity",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Similarity failed"),
        )
    }

    /// VipsSines (sines), make a 2D sine wave
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn sines(width: i32, height: i32) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_sines(
                &mut out_out,
                width_in,
                height_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Sine failed"),
            )
        }
    }

    pub fn sines_with_opts(width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "sines",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("sines_with_opts failed"),
        )
    }

    /// VipsSmartcrop (smartcrop), extract an area from an image
    /// input: `&VipsImage` -> Input image
    /// width: `i32` -> Width of extract area
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Height of extract area
    /// min: 1, max: 100000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn smartcrop(&self, width: i32, height: i32) -> Result<VipsImage> {
        unsafe {
            let input_in: *mut bindings::VipsImage = self.ctx;
            let width_in: i32 = width;
            let height_in: i32 = height;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_smartcrop(
                input_in,
                &mut out_out,
                width_in,
                height_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Smartcrop failed"),
            )
        }
    }

    pub fn smartcrop_with_opts(
        &self,
        width: i32,
        height: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "smartcrop",
            option
                .with(
                    "input",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Smartcrop failed"),
        )
    }

    /// VipsSobel (sobel), Sobel edge detector
    /// returns `VipsImage` - Output image
    pub fn sobel(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_sobel(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Sobel failed"),
            )
        }
    }

    /// VipsSpcor (spcor), spatial correlation
    /// refp: `&VipsImage` -> Input reference image
    /// returns `VipsImage` - Output image
    pub fn spcor(&self, refp: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let refp_in: *mut bindings::VipsImage = refp.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_spcor(
                inp_in,
                refp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Spcor failed"),
            )
        }
    }

    /// VipsSpectrum (spectrum), make displayable power spectrum
    /// returns `VipsImage` - Output image
    pub fn spectrum(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_spectrum(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Spectrum failed"),
            )
        }
    }

    /// VipsStats (stats), find many image stats
    /// returns `VipsImage` - Output array of statistics
    pub fn stats(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_stats(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Stat failed"),
            )
        }
    }

    /// VipsStdif (stdif), statistical difference
    /// width: `i32` -> Window width in pixels
    /// min: 1, max: 256, default: 11
    /// height: `i32` -> Window height in pixels
    /// min: 1, max: 256, default: 11
    /// returns `VipsImage` - Output image
    pub fn stdif(&self, width: i32, height: i32) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let width_in: i32 = width;
            let height_in: i32 = height;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_stdif(
                inp_in,
                &mut out_out,
                width_in,
                height_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Stdif failed"),
            )
        }
    }

    pub fn stdif_with_opts(&self, width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "stdif",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Stdif failed"),
        )
    }

    /// VipsSubsample (subsample), subsample an image
    /// input: `&VipsImage` -> Input image
    /// xfac: `i32` -> Horizontal subsample factor
    /// min: 1, max: 100000000, default: 1
    /// yfac: `i32` -> Vertical subsample factor
    /// min: 1, max: 100000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn subsample(&self, xfac: i32, yfac: i32) -> Result<VipsImage> {
        unsafe {
            let input_in: *mut bindings::VipsImage = self.ctx;
            let xfac_in: i32 = xfac;
            let yfac_in: i32 = yfac;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_subsample(
                input_in,
                &mut out_out,
                xfac_in,
                yfac_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Subsample failed"),
            )
        }
    }

    pub fn subsample_with_opts(&self, xfac: i32, yfac: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "subsample",
            option
                .with(
                    "input",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "xfac",
                    VipsValue::Int(xfac),
                )
                .with(
                    "yfac",
                    VipsValue::Int(yfac),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Subsample failed"),
        )
    }

    /// VipsSubtract (subtract), subtract two images
    /// left: `&VipsImage` -> Left-hand image argument
    /// right: `&VipsImage` -> Right-hand image argument
    /// returns `VipsImage` - Output image
    pub fn subtract(&self, right: &VipsImage) -> Result<VipsImage> {
        unsafe {
            let left_in: *mut bindings::VipsImage = self.ctx;
            let right_in: *mut bindings::VipsImage = right.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_subtract(
                left_in,
                right_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Subtract failed"),
            )
        }
    }

    /// VipsSum (sum), sum an array of images
    /// inp: `&[VipsImage]` -> Array of input images
    /// returns `VipsImage` - Output image
    pub fn sum(inp: &[VipsImage]) -> Result<VipsImage> {
        unsafe {
            let (inp_len, mut inp_in) = {
                let len = inp.len();
                let mut input = Vec::new();
                for img in inp {
                    input.push(img.ctx)
                }
                (
                    len as i32,
                    input,
                )
            };
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_sum(
                inp_in.as_mut_ptr(),
                &mut out_out,
                inp_len,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Sum failed"),
            )
        }
    }

    /// VipsForeignLoadSvgFile (svgload), load SVG with rsvg (.svg, .svgz, .svg.gz), priority=-5, untrusted, is_a, get_flags, get_flags_filename, header, load
    /// filename: `&str` -> Filename to load from
    /// returns `VipsImage` - Output image
    pub fn svgload(filename: &str) -> Result<VipsImage> {
        unsafe {
            let filename_in: CString = utils::new_c_string(filename)?;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_svgload(
                filename_in.as_ptr(),
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Svgload failed"),
            )
        }
    }

    pub fn svgload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "svgload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Svgload failed"),
        )
    }

    /// VipsForeignLoadSvgBuffer (svgload_buffer), load SVG with rsvg, priority=-5, untrusted, is_a_buffer, get_flags, get_flags_filename, header, load
    /// buffer: `&[u8]` -> Buffer to load from
    /// returns `VipsImage` - Output image
    pub fn svgload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        unsafe {
            let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_svgload_buffer(
                buffer_in,
                buffer.len() as u64,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("SvgloadBuffer failed"),
            )
        }
    }

    pub fn svgload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "svgload_buffer",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "buffer",
                    VipsValue::Buffer(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("SvgloadBuffer failed"),
        )
    }

    pub fn svgload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "svgload_source",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("svgload_source failed"),
        )
    }

    pub fn switch_image(tests: &[VipsImage], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "switch",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "tests",
                    VipsValue::ImageArray(tests),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Switch failed"),
        )
    }

    /// VipsSystem (system), run an external command
    /// cmd_format: `&str` -> Command to run
    pub fn system(cmd_format: &str) -> Result<()> {
        unsafe {
            let cmd_format_in: CString = utils::new_c_string(cmd_format)?;

            let vips_op_response = bindings::vips_system(
                cmd_format_in.as_ptr(),
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("System failed"),
            )
        }
    }

    pub fn system_with_opts(cmd_format: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "system",
            option.with(
                "cmd_format",
                VipsValue::Str(cmd_format),
            ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("System failed"),
        )
    }

    /// VipsText (text), make a text image
    /// text: `&str` -> Text to render
    /// returns `VipsImage` - Output image
    pub fn text(text: &str) -> Result<VipsImage> {
        unsafe {
            let text_in: CString = utils::new_c_string(text)?;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_text(
                &mut out_out,
                text_in.as_ptr(),
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Text failed"),
            )
        }
    }

    pub fn text_with_opts(text: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "text",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "text",
                    VipsValue::Str(text),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Text failed"),
        )
    }

    /// VipsThumbnailFile (thumbnail), generate thumbnail from file
    /// filename: `&str` -> Filename to read from
    /// width: `i32` -> Size to this width
    /// min: 1, max: 100000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn thumbnail(filename: &str, width: i32) -> Result<VipsImage> {
        unsafe {
            let filename_in: CString = utils::new_c_string(filename)?;
            let width_in: i32 = width;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_thumbnail(
                filename_in.as_ptr(),
                &mut out_out,
                width_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Thumbnail failed"),
            )
        }
    }

    pub fn thumbnail_with_opts(filename: &str, width: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "thumbnail",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Thumbnail failed"),
        )
    }

    /// VipsThumbnailBuffer (thumbnail_buffer), generate thumbnail from buffer
    /// buffer: `&[u8]` -> Buffer to load from
    /// width: `i32` -> Size to this width
    /// min: 1, max: 100000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn thumbnail_buffer(buffer: &[u8], width: i32) -> Result<VipsImage> {
        unsafe {
            let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
            let width_in: i32 = width;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_thumbnail_buffer(
                buffer_in,
                buffer.len() as u64,
                &mut out_out,
                width_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("ThumbnailBuffer failed"),
            )
        }
    }

    pub fn thumbnail_buffer_with_opts(
        buffer: &[u8],
        width: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "thumbnail_buffer",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "buffer",
                    VipsValue::Buffer(buffer),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ThumbnailBuffer failed"),
        )
    }

    /// VipsThumbnailImage (thumbnail_image), generate thumbnail from image
    /// width: `i32` -> Size to this width
    /// min: 1, max: 100000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn thumbnail_image(&self, width: i32) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let width_in: i32 = width;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_thumbnail_image(
                inp_in,
                &mut out_out,
                width_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("ThumbnailImage failed"),
            )
        }
    }

    pub fn thumbnail_image_with_opts(&self, width: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "thumbnail_image",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ThumbnailImage failed"),
        )
    }

    /// VipsThumbnailSource (thumbnail_source), generate thumbnail from source
    /// source: `&VipsSource` -> Source to load from
    /// width: `i32` -> Size to this width
    /// min: 1, max: 100000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn thumbnail_source(source: &VipsSource, width: i32) -> Result<VipsImage> {
        unsafe {
            let source_in: *mut bindings::VipsSource = source.ctx;
            let width_in: i32 = width;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_thumbnail_source(
                source_in,
                &mut out_out,
                width_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("ThumbnailSource failed"),
            )
        }
    }

    pub fn thumbnail_source_with_opts(
        source: &VipsSource,
        width: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "thumbnail_source",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ThumbnailSource failed"),
        )
    }

    /// VipsForeignLoadTiffFile (tiffload), load tiff from file (.tif, .tiff), priority=50, is_a, get_flags, get_flags_filename, header, load
    /// filename: `&str` -> Filename to load from
    /// returns `VipsImage` - Output image
    pub fn tiffload(filename: &str) -> Result<VipsImage> {
        unsafe {
            let filename_in: CString = utils::new_c_string(filename)?;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_tiffload(
                filename_in.as_ptr(),
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Tiffload failed"),
            )
        }
    }

    pub fn tiffload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "tiffload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Tiffload failed"),
        )
    }

    /// VipsForeignLoadTiffBuffer (tiffload_buffer), load tiff from buffer, priority=50, is_a_buffer, get_flags, get_flags_filename, header, load
    /// buffer: `&[u8]` -> Buffer to load from
    /// returns `VipsImage` - Output image
    pub fn tiffload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        unsafe {
            let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_tiffload_buffer(
                buffer_in,
                buffer.len() as u64,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("TiffloadBuffer failed"),
            )
        }
    }

    pub fn tiffload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "tiffload_buffer",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "buffer",
                    VipsValue::Buffer(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("TiffloadBuffer failed"),
        )
    }

    /// VipsForeignLoadTiffSource (tiffload_source), load tiff from source, priority=50, is_a_source, get_flags, get_flags_filename, header, load
    /// source: `&VipsSource` -> Source to load from
    /// returns `VipsImage` - Output image
    pub fn tiffload_source(source: &VipsSource) -> Result<VipsImage> {
        unsafe {
            let source_in: *mut bindings::VipsSource = source.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_tiffload_source(
                source_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("TiffloadSource failed"),
            )
        }
    }

    pub fn tiffload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "tiffload_source",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("TiffloadSource failed"),
        )
    }

    /// VipsForeignSaveTiffFile (tiffsave), save image to tiff file (.tif, .tiff), priority=0, any
    /// inp: `&VipsImage` -> Image to save
    /// filename: `&str` -> Filename to save to
    pub fn tiffsave(&self, filename: &str) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let filename_in: CString = utils::new_c_string(filename)?;

            let vips_op_response = bindings::vips_tiffsave(
                inp_in,
                filename_in.as_ptr(),
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("Tiffsave failed"),
            )
        }
    }

    pub fn tiffsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "tiffsave",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Tiffsave failed"),
        )
    }

    /// VipsForeignSaveTiffBuffer (tiffsave_buffer), save image to tiff buffer (.tif, .tiff), priority=0, any
    /// inp: `&VipsImage` -> Image to save
    /// returns `Vec<u8>` - Buffer to save to
    pub fn tiffsave_buffer(&self) -> Result<Vec<u8>> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut buffer_buf_size: u64 = 0;
            let mut buffer_out: *mut c_void = null_mut();

            let vips_op_response = bindings::vips_tiffsave_buffer(
                inp_in,
                &mut buffer_out,
                &mut buffer_buf_size,
                NULL,
            );
            utils::result(
                vips_op_response,
                utils::new_byte_array(
                    buffer_out,
                    buffer_buf_size,
                ),
                Error::OperationError("TiffsaveBuffer failed"),
            )
        }
    }

    pub fn tiffsave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut blob = VipsBlob::from(null_mut());

        let vips_op_response = call(
            "tiffsave_buffer",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    VipsValue::MutBlob(&mut blob),
                ),
        );

        utils::result(
            vips_op_response,
            blob.into(),
            Error::OperationError("TiffsaveBuffer failed"),
        )
    }

    /// VipsForeignSaveTiffTarget (tiffsave_target), save image to tiff target (.tif, .tiff), priority=0, any
    /// inp: `&VipsImage` -> Image to save
    /// target: `&VipsTarget` -> Target to save to
    pub fn tiffsave_target(&self, target: &VipsTarget) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let target_in: *mut bindings::VipsTarget = target.ctx;

            let vips_op_response = bindings::vips_tiffsave_target(
                inp_in,
                target_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("TiffsaveTarget failed"),
            )
        }
    }
    pub fn tiffsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "tiffsave_target",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    VipsValue::Target(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("TiffsaveTarget failed"),
        )
    }

    /// VipsTileCache (tilecache), cache an image as a set of tiles
    /// returns `VipsImage` - Output image
    pub fn tilecache(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_tilecache(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Tilecache failed"),
            )
        }
    }

    pub fn tilecache_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "tilecache",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Tilecache failed"),
        )
    }

    /// VipsTonelut (tonelut), build a look-up table
    /// returns `VipsImage` - Output image
    pub fn tonelut() -> Result<VipsImage> {
        unsafe {
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_tonelut(
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Tonelut failed"),
            )
        }
    }

    pub fn tonelut_with_opts(option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "tonelut",
            option.with(
                "out",
                VipsValue::MutImage(&mut out_out),
            ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Tonelut failed"),
        )
    }

    /// VipsTranspose3d (transpose3d), transpose3d an image
    /// returns `VipsImage` - Output image
    pub fn transpose3d(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_transpose3d(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Transpose3D failed"),
            )
        }
    }

    pub fn transpose3d_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "transpose3d",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Transpose3D failed"),
        )
    }

    /// VipsUnpremultiply (unpremultiply), unpremultiply image alpha
    /// returns `VipsImage` - Output image
    pub fn unpremultiply(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_unpremultiply(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Unpremultiply failed"),
            )
        }
    }

    pub fn unpremultiply_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "unpremultiply",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Unpremultiply failed"),
        )
    }

    /// VipsForeignLoadVipsFile (vipsload), load vips from file (.v, .vips), priority=200, untrusted, is_a, get_flags, get_flags_filename, header
    /// filename: `&str` -> Filename to load from
    /// returns `VipsImage` - Output image
    pub fn vipsload(filename: &str) -> Result<VipsImage> {
        unsafe {
            let filename_in: CString = utils::new_c_string(filename)?;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_vipsload(
                filename_in.as_ptr(),
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Vipsload failed"),
            )
        }
    }

    pub fn vipsload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "vipsload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Vipsload failed"),
        )
    }

    /// VipsForeignLoadVipsSource (vipsload_source), load vips from source, priority=200, untrusted, is_a_source, get_flags, get_flags_filename, header
    /// source: `&VipsSource` -> Source to load from
    /// returns `VipsImage` - Output image
    pub fn vipsload_source(source: &VipsSource) -> Result<VipsImage> {
        unsafe {
            let source_in: *mut bindings::VipsSource = source.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_vipsload_source(
                source_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("VipsloadSource failed"),
            )
        }
    }

    pub fn vipsload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "vipsload_source",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("VipsloadSource failed"),
        )
    }

    /// VipsForeignSaveVipsFile (vipssave), save image to file in vips format (.v, .vips), priority=0, any
    /// inp: `&VipsImage` -> Image to save
    /// filename: `&str` -> Filename to save to
    pub fn vipssave(&self, filename: &str) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let filename_in: CString = utils::new_c_string(filename)?;

            let vips_op_response = bindings::vips_vipssave(
                inp_in,
                filename_in.as_ptr(),
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("Vipssave failed"),
            )
        }
    }

    pub fn vipssave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "vipssave",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Vipssave failed"),
        )
    }

    /// VipsForeignSaveVipsTarget (vipssave_target), save image to target in vips format (.v, .vips), priority=0, any
    /// inp: `&VipsImage` -> Image to save
    /// target: `&VipsTarget` -> Target to save to
    pub fn vipssave_target(&self, target: &VipsTarget) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let target_in: *mut bindings::VipsTarget = target.ctx;

            let vips_op_response = bindings::vips_vipssave_target(
                inp_in,
                target_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("VipssaveTarget failed"),
            )
        }
    }

    pub fn vipssave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "vipssave_target",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    VipsValue::Target(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("VipssaveTarget failed"),
        )
    }

    /// VipsForeignLoadWebpFile (webpload), load webp from file (.webp), priority=200, is_a, get_flags, get_flags_filename, header, load
    /// filename: `&str` -> Filename to load from
    /// returns `VipsImage` - Output image
    pub fn webpload(filename: &str) -> Result<VipsImage> {
        unsafe {
            let filename_in: CString = utils::new_c_string(filename)?;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_webpload(
                filename_in.as_ptr(),
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Webpload failed"),
            )
        }
    }
    pub fn webpload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "webpload",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Webpload failed"),
        )
    }

    /// VipsForeignLoadWebpBuffer (webpload_buffer), load webp from buffer, priority=200, is_a_buffer, get_flags, get_flags_filename, header, load
    /// buffer: `&[u8]` -> Buffer to load from
    /// returns `VipsImage` - Output image
    pub fn webpload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        unsafe {
            let buffer_in: *mut c_void = buffer.as_ptr() as *mut c_void;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_webpload_buffer(
                buffer_in,
                buffer.len() as u64,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("WebploadBuffer failed"),
            )
        }
    }

    pub fn webpload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "webpload_buffer",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "buffer",
                    VipsValue::Buffer(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("WebploadBuffer failed"),
        )
    }

    /// VipsForeignLoadWebpSource (webpload_source), load webp from source, priority=200, is_a_source, get_flags, get_flags_filename, header, load
    /// source: `&VipsSource` -> Source to load from
    /// returns `VipsImage` - Output image
    pub fn webpload_source(source: &VipsSource) -> Result<VipsImage> {
        unsafe {
            let source_in: *mut bindings::VipsSource = source.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_webpload_source(
                source_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("WebploadSource failed"),
            )
        }
    }

    pub fn webpload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "webpload_source",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "source",
                    VipsValue::Source(source),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("WebploadSource failed"),
        )
    }

    /// VipsForeignSaveWebpFile (webpsave), save as WebP (.webp), priority=0, rgba-only
    /// inp: `&VipsImage` -> Image to save
    /// filename: `&str` -> Filename to save to
    pub fn webpsave(&self, filename: &str) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let filename_in: CString = utils::new_c_string(filename)?;

            let vips_op_response = bindings::vips_webpsave(
                inp_in,
                filename_in.as_ptr(),
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("Webpsave failed"),
            )
        }
    }

    pub fn webpsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "webpsave",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    VipsValue::Str(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Webpsave failed"),
        )
    }

    /// VipsForeignSaveWebpBuffer (webpsave_buffer), save as WebP (.webp), priority=0, rgba-only
    /// inp: `&VipsImage` -> Image to save
    /// returns `Vec<u8>` - Buffer to save to
    pub fn webpsave_buffer(&self) -> Result<Vec<u8>> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut buffer_buf_size: u64 = 0;
            let mut buffer_out: *mut c_void = null_mut();

            let vips_op_response = bindings::vips_webpsave_buffer(
                inp_in,
                &mut buffer_out,
                &mut buffer_buf_size,
                NULL,
            );
            utils::result(
                vips_op_response,
                utils::new_byte_array(
                    buffer_out,
                    buffer_buf_size,
                ),
                Error::OperationError("WebpsaveBuffer failed"),
            )
        }
    }

    pub fn webpsave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut blob = VipsBlob::from(null_mut());

        let vips_op_response = call(
            "webpsave_buffer",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    VipsValue::MutBlob(&mut blob),
                ),
        );

        utils::result(
            vips_op_response,
            blob.into(),
            Error::OperationError("WebpsaveBuffer failed"),
        )
    }

    /// VipsForeignSaveWebpMime (webpsave_mime), save image to webp mime (.webp), priority=0, rgba-only
    /// inp: `&VipsImage` -> Image to save
    pub fn webpsave_mime(&self) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;

            let vips_op_response = bindings::vips_webpsave_mime(
                inp_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("WebpsaveMime failed"),
            )
        }
    }

    pub fn webpsave_mime_with_opts(&self, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "webpsave_mime",
            option.with(
                "in",
                VipsValue::Image(&VipsImage::from(self.ctx)),
            ),
        );

        utils::result(
            vips_op_response,
            (),
            Error::OperationError("WebpsaveMime failed"),
        )
    }

    /// VipsForeignSaveWebpTarget (webpsave_target), save as WebP (.webp), priority=0, rgba-only
    /// inp: `&VipsImage` -> Image to save
    /// target: `&VipsTarget` -> Target to save to
    pub fn webpsave_target(&self, target: &VipsTarget) -> Result<()> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let target_in: *mut bindings::VipsTarget = target.ctx;

            let vips_op_response = bindings::vips_webpsave_target(
                inp_in,
                target_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                (),
                Error::OperationError("WebpsaveTarget failed"),
            )
        }
    }

    pub fn webpsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "webpsave_target",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    VipsValue::Target(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("WebpsaveTarget failed"),
        )
    }

    /// VipsWorley (worley), make a worley noise image
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn worley(width: i32, height: i32) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_worley(
                &mut out_out,
                width_in,
                height_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Worley failed"),
            )
        }
    }

    pub fn worley_with_opts(width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "worley",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Worley failed"),
        )
    }

    /// VipsWrap (wrap), wrap image origin
    /// returns `VipsImage` - Output image
    pub fn wrap(&self) -> Result<VipsImage> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_wrap(
                inp_in,
                &mut out_out,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Wrap failed"),
            )
        }
    }

    pub fn wrap_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "wrap",
            option
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Wrap failed"),
        )
    }

    /// VipsXyz (xyz), make an image where pixel values are coordinates
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 64
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 64
    /// returns `VipsImage` - Output image
    pub fn xyz(width: i32, height: i32) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_xyz(
                &mut out_out,
                width_in,
                height_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Xyz failed"),
            )
        }
    }

    pub fn xyz_with_opts(width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "xyz",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Xyz failed"),
        )
    }

    /// VipsZone (zone), make a zone plate
    /// width: `i32` -> Image width in pixels
    /// min: 1, max: 100000000, default: 1
    /// height: `i32` -> Image height in pixels
    /// min: 1, max: 100000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn zone(width: i32, height: i32) -> Result<VipsImage> {
        unsafe {
            let width_in: i32 = width;
            let height_in: i32 = height;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_zone(
                &mut out_out,
                width_in,
                height_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Zone failed"),
            )
        }
    }

    pub fn zone_with_opts(width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "zone",
            option
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .with(
                    "width",
                    VipsValue::Int(width),
                )
                .with(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Zone failed"),
        )
    }

    /// VipsZoom (zoom), zoom an image
    /// input: `&VipsImage` -> Input image
    /// xfac: `i32` -> Horizontal zoom factor
    /// min: 1, max: 100000000, default: 1
    /// yfac: `i32` -> Vertical zoom factor
    /// min: 1, max: 100000000, default: 1
    /// returns `VipsImage` - Output image
    pub fn zoom(&self, xfac: i32, yfac: i32) -> Result<VipsImage> {
        unsafe {
            let input_in: *mut bindings::VipsImage = self.ctx;
            let xfac_in: i32 = xfac;
            let yfac_in: i32 = yfac;
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_zoom(
                input_in,
                &mut out_out,
                xfac_in,
                yfac_in,
                NULL,
            );
            utils::result(
                vips_op_response,
                VipsImage {
                    ctx: out_out,
                },
                Error::OperationError("Zoom failed"),
            )
        }
    }
}
