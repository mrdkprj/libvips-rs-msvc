#![allow(clippy::all)]
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
use std::ptr::null_mut;

impl VipsImage {
    pub fn CMC2LCh(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "CMC2LCh",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Cmc2LChError,
        )
    }

    pub fn CMYK2XYZ(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "CMYK2XYZ",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Cmyk2XyzError,
        )
    }

    pub fn HSV2sRGB(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "HSV2sRGB",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Hsv2SRgbError,
        )
    }

    pub fn LCh2CMC(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "LCh2CMC",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LCh2CmcError,
        )
    }

    pub fn LCh2Lab(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "LCh2Lab",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LCh2LabError,
        )
    }

    pub fn Lab2LCh(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "Lab2LCh",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Lab2LChError,
        )
    }

    pub fn Lab2LabQ(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "Lab2LabQ",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Lab2LabQError,
        )
    }

    pub fn Lab2LabS(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "Lab2LabS",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Lab2LabSError,
        )
    }

    pub fn Lab2XYZ(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "Lab2XYZ",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Lab2XyzError,
        )
    }

    pub fn LabQ2Lab(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "LabQ2Lab",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LabQ2LabError,
        )
    }

    pub fn LabQ2LabS(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "LabQ2LabS",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LabQ2LabSError,
        )
    }

    pub fn LabQ2sRGB(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "LabQ2sRGB",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LabQ2SRgbError,
        )
    }

    pub fn LabS2Lab(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "LabS2Lab",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LabS2LabError,
        )
    }

    pub fn LabS2LabQ(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "LabS2LabQ",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LabS2LabQError,
        )
    }

    pub fn XYZ2CMYK(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "XYZ2CMYK",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Xyz2CmykError,
        )
    }

    pub fn XYZ2Lab(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "XYZ2Lab",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Xyz2LabError,
        )
    }

    pub fn XYZ2Yxy(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "XYZ2Yxy",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Xyz2YxyError,
        )
    }

    pub fn XYZ2scRGB(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "XYZ2scRGB",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Xyz2ScRgbError,
        )
    }

    pub fn Yxy2XYZ(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "Yxy2XYZ",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Yxy2XyzError,
        )
    }

    pub fn abs(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "abs",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::AbError,
        )
    }

    pub fn add(&self, right: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "add",
            option
                .set(
                    "left",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "right",
                    VipsValue::Image(right.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::AddError,
        )
    }

    pub fn addalpha(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "addalpha",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::AddalphaError,
        )
    }

    pub fn affine(&self, matrix: &[f64], option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "affine",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "matrix",
                    VipsValue::DoubleArray(matrix),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::AffineError,
        )
    }

    pub fn analyzeload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "analyzeload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::AnalyzeloadError,
        )
    }

    pub fn arrayjoin(&self, inps: &[VipsImage], option: VOption) -> Result<VipsImage> {
        let mut inp = Vec::new();
        for inp_in in inps {
            inp.push(inp_in.ctx);
        }
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "arrayjoin",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "in",
                    VipsValue::ImageArray(&inp),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ArrayjoinError,
        )
    }

    pub fn autorot(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "autorot",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::AutorotError,
        )
    }

    pub fn avg(&self, option: VOption) -> Result<f64> {
        let mut out: f64 = 0.0;

        let vips_op_response = call(
            "avg",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutDouble(&mut out),
                ),
        );

        utils::result(
            vips_op_response,
            out,
            Error::AvgError,
        )
    }

    pub fn bandbool(&self, boolean: OperationBoolean, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "bandbool",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "boolean",
                    VipsValue::Int(boolean as i32),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BandboolError,
        )
    }

    pub fn bandfold(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "bandfold",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BandfoldError,
        )
    }

    pub fn bandjoin(&self, inps: &[VipsImage], option: VOption) -> Result<VipsImage> {
        let mut inp = Vec::new();
        for inp_in in inps {
            inp.push(inp_in.ctx);
        }

        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "bandjoin",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "in",
                    VipsValue::ImageArray(&inp),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BandjoinError,
        )
    }

    pub fn bandjoin_const(&self, c: &[f64], option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "bandjoin_const",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "c",
                    VipsValue::DoubleArray(c),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BandjoinConstError,
        )
    }

    pub fn bandmean(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "bandmean",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BandmeanError,
        )
    }

    pub fn bandrank(&self, inps: &[VipsImage], option: VOption) -> Result<VipsImage> {
        let mut inp = Vec::new();
        for inp_in in inps {
            inp.push(inp_in.ctx);
        }
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "bandrank",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "in",
                    VipsValue::ImageArray(&inp),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BandrankError,
        )
    }

    pub fn bandunfold(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "bandunfold",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BandunfoldError,
        )
    }

    pub fn black(&self, width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "black",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BlackError,
        )
    }

    pub fn boolean(
        &self,
        right: &VipsImage,
        boolean: OperationBoolean,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "boolean",
            option
                .set(
                    "left",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "right",
                    VipsValue::Image(right.ctx),
                )
                .set(
                    "boolean",
                    VipsValue::Int(boolean as i32),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BooleanError,
        )
    }

    pub fn boolean_const(
        &self,
        boolean: OperationBoolean,
        c: &[f64],
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "boolean_const",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "boolean",
                    VipsValue::Int(boolean as i32),
                )
                .set(
                    "c",
                    VipsValue::DoubleArray(c),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BooleanConstError,
        )
    }

    pub fn buildlut(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "buildlut",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::BuildlutError,
        )
    }

    pub fn byteswap(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "byteswap",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ByteswapError,
        )
    }

    pub fn canny(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "canny",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CannyError,
        )
    }

    pub fn case_image(&self, cases: &[VipsImage], option: VOption) -> Result<VipsImage> {
        let mut inp = Vec::new();
        for case in cases {
            inp.push(case.ctx);
        }
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "case",
            option
                .set(
                    "index",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "cases",
                    VipsValue::ImageArray(&inp),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CaseError,
        )
    }

    pub fn cast(&self, format: BandFormat, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "cast",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "format",
                    VipsValue::Int(format as i32),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CastError,
        )
    }

    pub fn clamp(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "clamp",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ClampError,
        )
    }

    pub fn colourspace(&self, space: Interpretation, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "colourspace",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "space",
                    VipsValue::Int(space as i32),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ColourspaceError,
        )
    }

    pub fn compass(&self, mask: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "compass",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "mask",
                    VipsValue::Image(mask.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CompassError,
        )
    }

    pub fn complex(&self, cmplx: OperationComplex, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "complex",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "cmplx",
                    VipsValue::Int(cmplx as i32),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ComplexError,
        )
    }

    pub fn complex2(
        &self,
        right: &VipsImage,
        cmplx: OperationComplex2,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "complex2",
            option
                .set(
                    "left",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "right",
                    VipsValue::Image(right.ctx),
                )
                .set(
                    "cmplx",
                    VipsValue::Int(cmplx as i32),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Complex2Error,
        )
    }

    pub fn complexform(&self, right: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "complexform",
            option
                .set(
                    "left",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "right",
                    VipsValue::Image(right.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ComplexformError,
        )
    }

    pub fn complexget(&self, get: OperationComplexget, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "complexget",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "get",
                    VipsValue::Int(get as i32),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ComplexgetError,
        )
    }

    pub fn composite(
        &self,
        inps: &[VipsImage],
        mode: &[i32],
        option: VOption,
    ) -> Result<VipsImage> {
        let mut inp = Vec::new();
        for inp_in in inps {
            inp.push(inp_in.ctx);
        }
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "composite",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "in",
                    VipsValue::ImageArray(&inp),
                )
                .set(
                    "mode",
                    VipsValue::IntArray(mode),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CompositeError,
        )
    }

    pub fn composite2(
        &self,
        overlay: &VipsImage,
        mode: BlendMode,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "composite2",
            option
                .set(
                    "base",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "overlay",
                    VipsValue::Image(overlay.ctx),
                )
                .set(
                    "mode",
                    VipsValue::Int(mode as i32),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Composite2Error,
        )
    }

    pub fn conv(&self, mask: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "conv",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "mask",
                    VipsValue::Image(mask.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ConvError,
        )
    }

    pub fn conva(&self, mask: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "conva",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "mask",
                    VipsValue::Image(mask.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ConvaError,
        )
    }

    pub fn convasep(&self, mask: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "convasep",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "mask",
                    VipsValue::Image(mask.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ConvasepError,
        )
    }

    pub fn convf(&self, mask: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "convf",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "mask",
                    VipsValue::Image(mask.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ConvfError,
        )
    }

    pub fn convi(&self, mask: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "convi",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "mask",
                    VipsValue::Image(mask.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ConviError,
        )
    }

    pub fn convsep(&self, mask: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "convsep",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "mask",
                    VipsValue::Image(mask.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ConvsepError,
        )
    }

    pub fn copy(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "copy",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CopyError,
        )
    }

    pub fn countlines(&self, direction: Direction, option: VOption) -> Result<f64> {
        let mut nolines: f64 = 0.0;

        let vips_op_response = call(
            "countlines",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "nolines",
                    VipsValue::MutDouble(&mut nolines),
                )
                .set(
                    "direction",
                    VipsValue::Int(direction as i32),
                ),
        );

        utils::result(
            vips_op_response,
            nolines,
            Error::CountlineError,
        )
    }

    pub fn crop(
        &self,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "crop",
            option
                .set(
                    "input",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "left",
                    VipsValue::Int(left),
                )
                .set(
                    "top",
                    VipsValue::Int(top),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CropError,
        )
    }

    pub fn csvload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "csvload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CsvloadError,
        )
    }

    pub fn csvload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "csvload_source",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "source",
                    VipsValue::Source(source.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::CsvloadSourceError,
        )
    }

    pub fn csvsave(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "csvsave",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::CsvsaveError,
        )
    }

    pub fn csvsave_target(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "csvsave_target",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "target",
                    VipsValue::Target(target.ctx),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::CsvsaveTargetError,
        )
    }

    pub fn dE00(&self, right: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "dE00",
            option
                .set(
                    "left",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "right",
                    VipsValue::Image(right.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::DE00Error,
        )
    }

    pub fn dE76(&self, right: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "dE76",
            option
                .set(
                    "left",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "right",
                    VipsValue::Image(right.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::DE76Error,
        )
    }

    pub fn dECMC(&self, right: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "dECMC",
            option
                .set(
                    "left",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "right",
                    VipsValue::Image(right.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::DEcmcError,
        )
    }

    pub fn deviate(&self, option: VOption) -> Result<f64> {
        let mut out: f64 = 0.0;

        let vips_op_response = call(
            "deviate",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutDouble(&mut out),
                ),
        );

        utils::result(
            vips_op_response,
            out,
            Error::DeviateError,
        )
    }

    pub fn divide(&self, right: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "divide",
            option
                .set(
                    "left",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "right",
                    VipsValue::Image(right.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::DivideError,
        )
    }

    pub fn draw_circle(
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
                .set(
                    "image",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "ink",
                    VipsValue::DoubleArray(ink),
                )
                .set(
                    "cx",
                    VipsValue::Int(cx),
                )
                .set(
                    "cy",
                    VipsValue::Int(cy),
                )
                .set(
                    "radius",
                    VipsValue::Int(radius),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::DrawCircleError,
        )
    }

    pub fn draw_flood(&self, ink: &[f64], x: i32, y: i32, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "draw_flood",
            option
                .set(
                    "image",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "ink",
                    VipsValue::DoubleArray(ink),
                )
                .set(
                    "x",
                    VipsValue::Int(x),
                )
                .set(
                    "y",
                    VipsValue::Int(y),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::DrawFloodError,
        )
    }

    pub fn draw_image(&self, sub: &VipsImage, x: i32, y: i32, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "draw_image",
            option
                .set(
                    "image",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "sub",
                    VipsValue::Image(sub.ctx),
                )
                .set(
                    "x",
                    VipsValue::Int(x),
                )
                .set(
                    "y",
                    VipsValue::Int(y),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::DrawImageError,
        )
    }

    pub fn draw_line(
        &self,
        ink: &[f64],
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        option: VOption,
    ) -> Result<()> {
        let vips_op_response = call(
            "draw_line",
            option
                .set(
                    "image",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "ink",
                    VipsValue::DoubleArray(ink),
                )
                .set(
                    "x1",
                    VipsValue::Int(x1),
                )
                .set(
                    "y1",
                    VipsValue::Int(y1),
                )
                .set(
                    "x2",
                    VipsValue::Int(x2),
                )
                .set(
                    "y2",
                    VipsValue::Int(y2),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::DrawLineError,
        )
    }

    pub fn draw_mask(
        &self,
        ink: &[f64],
        mask: VipsImage,
        x: i32,
        y: i32,
        option: VOption,
    ) -> Result<()> {
        let vips_op_response = call(
            "draw_mask",
            option
                .set(
                    "image",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "ink",
                    VipsValue::DoubleArray(ink),
                )
                .set(
                    "mask",
                    VipsValue::Image(mask.ctx),
                )
                .set(
                    "x",
                    VipsValue::Int(x),
                )
                .set(
                    "y",
                    VipsValue::Int(y),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::DrawMaskError,
        )
    }

    pub fn draw_rect(
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
                .set(
                    "image",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "ink",
                    VipsValue::DoubleArray(ink),
                )
                .set(
                    "left",
                    VipsValue::Int(left),
                )
                .set(
                    "top",
                    VipsValue::Int(top),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::DrawRectError,
        )
    }

    pub fn draw_smudge(
        &self,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
        option: VOption,
    ) -> Result<()> {
        let vips_op_response = call(
            "draw_smudge",
            option
                .set(
                    "image",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "left",
                    VipsValue::Int(left),
                )
                .set(
                    "top",
                    VipsValue::Int(top),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::DrawSmudgeError,
        )
    }

    pub fn dzsave(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "dzsave",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("dzsave failed"),
        )
    }

    pub fn dzsave_buffer(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out: *mut bindings::VipsBlob = null_mut();

        let vips_op_response = call(
            "dzsave_buffer",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "buffer",
                    VipsValue::MutBlob(&mut buffer_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsBlob {
                ctx: buffer_out,
            }
            .into(),
            Error::OperationError("dzsave_buffer failed"),
        )
    }

    pub fn dzsave_target(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "dzsave_target",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "target",
                    VipsValue::Target(target.ctx),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("dzsave_target failed"),
        )
    }

    pub fn embed(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "embed",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "x",
                    VipsValue::Int(x),
                )
                .set(
                    "y",
                    VipsValue::Int(y),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::EmbedError,
        )
    }

    pub fn extract_area(
        &self,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "extract_area",
            option
                .set(
                    "input",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "left",
                    VipsValue::Int(left),
                )
                .set(
                    "top",
                    VipsValue::Int(top),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ExtractAreaError,
        )
    }

    pub fn extract_band(&self, band: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "extract_band",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "band",
                    VipsValue::Int(band),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ExtractBandError,
        )
    }

    pub fn eye(&self, width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "eye",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::EyeError,
        )
    }

    pub fn falsecolour(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "falsecolour",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::FalsecolourError,
        )
    }

    pub fn fastcor(&self, ref_in: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "fastcor",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "ref",
                    VipsValue::Image(ref_in.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::FastcorError,
        )
    }

    pub fn fill_nearest(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "fill_nearest",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::FillNearestError,
        )
    }

    pub fn find_trim(
        &self,
        top: &mut i32,
        width: &mut i32,
        height: &mut i32,
        option: VOption,
    ) -> Result<i32> {
        let mut left: i32 = 0;

        let vips_op_response = call(
            "find_trim",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "left",
                    VipsValue::MutInt(&mut left),
                )
                .set(
                    "top",
                    VipsValue::MutInt(top),
                )
                .set(
                    "width",
                    VipsValue::MutInt(width),
                )
                .set(
                    "height",
                    VipsValue::MutInt(height),
                ),
        );

        utils::result(
            vips_op_response,
            left,
            Error::FindTrimError,
        )
    }

    pub fn fitsload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "fitsload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("fitsloade failed"),
        )
    }

    pub fn fitsload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "fitsload_source",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "source",
                    VipsValue::Source(source.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("fitsload_source failed"),
        )
    }

    pub fn fitssave(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "fitssave",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("fitssave failed"),
        )
    }

    pub fn flatten(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "flatten",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::FlattenError,
        )
    }

    pub fn flip(&self, direction: Direction, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "flip",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "direction",
                    VipsValue::Int(direction as i32),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::FlipError,
        )
    }

    pub fn float2rad(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "float2rad",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Float2RadError,
        )
    }

    pub fn fractsurf(
        &self,
        width: i32,
        height: i32,
        fractal_dimension: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "fractsurf",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                )
                .set(
                    "fractal_dimension",
                    VipsValue::Double(fractal_dimension),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::FractsurfError,
        )
    }

    pub fn freqmult(&self, mask: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "freqmult",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "mask",
                    VipsValue::Image(mask.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::FreqmultError,
        )
    }

    pub fn fwfft(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "fwfft",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::FwfftError,
        )
    }

    pub fn gamma(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "gamma",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GammaError,
        )
    }

    pub fn gaussblur(&self, sigma: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "gaussblur",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "sigma",
                    VipsValue::Double(sigma),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GaussblurError,
        )
    }

    pub fn gaussmat(&self, sigma: f64, min_ampl: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "gaussmat",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "sigma",
                    VipsValue::Double(sigma),
                )
                .set(
                    "min_ampl",
                    VipsValue::Double(min_ampl),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GaussmatError,
        )
    }

    pub fn gaussnoise(&self, width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "gaussnoise",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GaussnoiseError,
        )
    }

    pub fn getpoint(&self, x: i32, y: i32, option: VOption) -> Result<Vec<f64>> {
        let mut out: Vec<f64> = Vec::new();

        let vips_op_response = call(
            "getpoint",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out_array",
                    VipsValue::MutDoubleArray(&mut out),
                )
                .set(
                    "x",
                    VipsValue::Int(x),
                )
                .set(
                    "y",
                    VipsValue::Int(y),
                ),
        );

        utils::result(
            vips_op_response,
            out,
            Error::GetpointError,
        )
    }

    pub fn gifload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "gifload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GifloadError,
        )
    }

    pub fn gifload_buffer(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "gifload_buffer",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "buffer",
                    VipsValue::Blob(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GifloadBufferError,
        )
    }

    pub fn gifload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "gifload_source",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "source",
                    VipsValue::Source(source.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GifloadSourceError,
        )
    }

    pub fn gifsave(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "gifsave",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::GifsaveError,
        )
    }

    pub fn gifsave_buffer(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out: *mut bindings::VipsBlob = null_mut();

        let vips_op_response = call(
            "gifsave_buffer",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "buffer",
                    VipsValue::MutBlob(&mut buffer_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsBlob {
                ctx: buffer_out,
            }
            .into(),
            Error::GifsaveBufferError,
        )
    }

    pub fn gifsave_target(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "gifsave_target",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "target",
                    VipsValue::Target(target.ctx),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::GifsaveTargetError,
        )
    }

    pub fn globalbalance(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "globalbalance",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GlobalbalanceError,
        )
    }

    pub fn gravity(
        &self,
        direction: CompassDirection,
        width: i32,
        height: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "gravity",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "direction",
                    VipsValue::Int(direction as i32),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GravityError,
        )
    }

    pub fn grey(&self, width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "grey",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GreyError,
        )
    }

    pub fn grid(
        &self,
        tile_height: i32,
        across: i32,
        down: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "grid",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "tile_height",
                    VipsValue::Int(tile_height),
                )
                .set(
                    "across",
                    VipsValue::Int(across),
                )
                .set(
                    "down",
                    VipsValue::Int(down),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::GridError,
        )
    }

    pub fn heifload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "heifload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HeifloadError,
        )
    }

    pub fn heifload_buffer(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "heifload_buffer",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "buffer",
                    VipsValue::Blob(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HeifloadBufferError,
        )
    }

    pub fn heifload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "heifload_source",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "source",
                    VipsValue::Source(source.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HeifloadSourceError,
        )
    }

    pub fn heifsave(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "heifsave",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::HeifsaveError,
        )
    }

    pub fn heifsave_buffer(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out: *mut bindings::VipsBlob = null_mut();

        let vips_op_response = call(
            "heifsave_buffer",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "buffer",
                    VipsValue::MutBlob(&mut buffer_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsBlob {
                ctx: buffer_out,
            }
            .into(),
            Error::HeifsaveBufferError,
        )
    }

    pub fn heifsave_target(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "heifsave_target",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "target",
                    VipsValue::Target(target.ctx),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::HeifsaveTargetError,
        )
    }

    pub fn hist_cum(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "hist_cum",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistCumError,
        )
    }

    pub fn hist_entropy(&self, option: VOption) -> Result<f64> {
        let mut out: f64 = 0.0;

        let vips_op_response = call(
            "hist_entropy",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutDouble(&mut out),
                ),
        );

        utils::result(
            vips_op_response,
            out,
            Error::HistEntropyError,
        )
    }

    pub fn hist_equal(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "hist_equal",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistEqualError,
        )
    }

    pub fn hist_find(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "hist_find",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistFindError,
        )
    }

    pub fn hist_find_indexed(&self, index: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "hist_find_indexed",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "index",
                    VipsValue::Image(index.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistFindIndexedError,
        )
    }

    pub fn hist_find_ndim(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "hist_find_ndim",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistFindNdimError,
        )
    }

    pub fn hist_ismonotonic(&self, option: VOption) -> Result<bool> {
        let mut monotonic = false;

        let vips_op_response = call(
            "hist_ismonotonic",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "monotonic",
                    VipsValue::MutBool(&mut monotonic),
                ),
        );

        utils::result(
            vips_op_response,
            monotonic,
            Error::HistIsmonotonicError,
        )
    }

    pub fn hist_local(&self, width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "hist_local",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistLocalError,
        )
    }

    pub fn hist_match(&self, ref_in: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "hist_match",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "ref",
                    VipsValue::Image(ref_in.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistMatchError,
        )
    }

    pub fn hist_norm(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "hist_norm",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistNormError,
        )
    }

    pub fn hist_plot(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "hist_plot",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HistPlotError,
        )
    }

    pub fn hough_circle(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "hough_circle",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HoughCircleError,
        )
    }

    pub fn hough_line(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "hough_line",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::HoughLineError,
        )
    }

    pub fn icc_export(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "icc_export",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::IccExportError,
        )
    }

    pub fn icc_import(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "icc_import",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::IccImportError,
        )
    }

    pub fn icc_transform(&self, output_profile: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "icc_transform",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "output_profile",
                    VipsValue::String(output_profile),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::IccTransformError,
        )
    }

    pub fn identity(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "identity",
            option.set(
                "out",
                VipsValue::MutImage(&mut out_out),
            ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::IdentityError,
        )
    }

    pub fn ifthenelse(
        &self,
        in1: &VipsImage,
        in2: &VipsImage,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "ifthenelse",
            option
                .set(
                    "cond",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "in1",
                    VipsValue::Image(in1.ctx),
                )
                .set(
                    "in2",
                    VipsValue::Image(in2.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::IfthenelseError,
        )
    }

    pub fn insert(&self, sub: &VipsImage, x: i32, y: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "insert",
            option
                .set(
                    "main",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "sub",
                    VipsValue::Image(sub.ctx),
                )
                .set(
                    "x",
                    VipsValue::Int(x),
                )
                .set(
                    "y",
                    VipsValue::Int(y),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::InsertError,
        )
    }

    pub fn invert(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "invert",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::InvertError,
        )
    }

    pub fn invertlut(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "invertlut",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::InvertlutError,
        )
    }

    pub fn invfft(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "invfft",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::InvfftError,
        )
    }

    pub fn join(
        &self,
        in2: &VipsImage,
        direction: Direction,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "join",
            option
                .set(
                    "in1",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "in2",
                    VipsValue::Image(in2.ctx),
                )
                .set(
                    "direction",
                    VipsValue::Int(direction as i32),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::JoinError,
        )
    }

    pub fn jp2kload(&self, filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "jp2kload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("jp2kload failed"),
        )
    }

    pub fn jp2kload_buffer(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "jp2kload_buffer",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "buffer",
                    VipsValue::Blob(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("jp2kload_buffer failed"),
        )
    }

    pub fn jp2kload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "jp2kload_source",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "source",
                    VipsValue::Source(source.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("jp2kload_source failed"),
        )
    }

    pub fn jp2ksave(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jp2ksave",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("jp2ksave failed"),
        )
    }

    pub fn jp2ksave_buffer(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out: *mut bindings::VipsBlob = null_mut();

        let vips_op_response = call(
            "jp2ksave_buffer",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "buffer",
                    VipsValue::MutBlob(&mut buffer_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsBlob {
                ctx: buffer_out,
            }
            .into(),
            Error::OperationError("jp2ksave_buffer failed"),
        )
    }

    pub fn jp2ksave_target(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jp2ksave_target",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "target",
                    VipsValue::Target(target.ctx),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("jp2ksave_target"),
        )
    }

    pub fn jpegload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "jpegload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::JpegloadError,
        )
    }

    pub fn jpegload_buffer(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "jpegload_buffer",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "buffer",
                    VipsValue::Blob(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::JpegloadBufferError,
        )
    }

    pub fn jpegload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "jpegload_source",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "source",
                    VipsValue::Source(source.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("JpegloadSourceError"),
        )
    }

    pub fn jpegsave(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jpegsave",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::JpegsaveError,
        )
    }

    pub fn jpegsave_buffer(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out: *mut bindings::VipsBlob = null_mut();

        let vips_op_response = call(
            "jpegsave_buffer",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "buffer",
                    VipsValue::MutBlob(&mut buffer_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsBlob {
                ctx: buffer_out,
            }
            .into(),
            Error::JpegsaveBufferError,
        )
    }

    pub fn jpegsave_mime(&self, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jpegsave_mime",
            option.set(
                "in",
                VipsValue::Image(self.ctx),
            ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::JpegsaveMimeError,
        )
    }

    pub fn jpegsave_target(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jpegsave_target",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "target",
                    VipsValue::Target(target.ctx),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::JpegsaveTargetError,
        )
    }

    pub fn jxlload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "jxlload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("jxlload"),
        )
    }

    pub fn jxlload_buffer(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "jxlload_buffer",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "buffer",
                    VipsValue::Blob(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("jxlload_buffer"),
        )
    }

    pub fn jxlload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "jxlload_source",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "source",
                    VipsValue::Source(source.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("jxlload_source"),
        )
    }

    pub fn jxlsave(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jxlsave",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("jxlsave"),
        )
    }

    pub fn jxlsave_buffer(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out: *mut bindings::VipsBlob = null_mut();

        let vips_op_response = call(
            "jxlsave_buffer",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "buffer",
                    VipsValue::MutBlob(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            VipsBlob {
                ctx: buffer_out,
            }
            .into(),
            Error::OperationError("jxlsave_buffer"),
        )
    }

    pub fn jxlsave_target(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jxlsave_target",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "target",
                    VipsValue::Target(target.ctx),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("jxlsave_target"),
        )
    }

    pub fn labelregions(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "labelregions",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "mask",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("labelregions"),
        )
    }

    pub fn linear(&self, a: &[f64], b: &[f64], option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "linear",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "a",
                    VipsValue::DoubleArray(a),
                )
                .set(
                    "b",
                    VipsValue::DoubleArray(b),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LinearError,
        )
    }

    pub fn linecache(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "linecache",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LinecacheError,
        )
    }

    pub fn logmat(&self, sigma: f64, min_ampl: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "logmat",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "sigma",
                    VipsValue::Double(sigma),
                )
                .set(
                    "min_ampl",
                    VipsValue::Double(min_ampl),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::LogmatError,
        )
    }

    pub fn magickload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "magickload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("magickload"),
        )
    }

    pub fn magickload_buffer(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "magickload_buffer",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "buffer",
                    VipsValue::Blob(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("magickload_buffer"),
        )
    }

    pub fn magicksave(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "magicksave",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            (),
            Error::OperationError("magicksave"),
        )
    }

    pub fn magicksave_buffer(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out: *mut bindings::VipsBlob = null_mut();

        let vips_op_response = call(
            "magicksave_buffer",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "buffer",
                    VipsValue::MutBlob(&mut buffer_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsBlob {
                ctx: buffer_out,
            }
            .into(),
            Error::OperationError("magicksave_buffer"),
        )
    }

    pub fn mapim(&self, index: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "mapim",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "index",
                    VipsValue::Image(index.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MapimError,
        )
    }

    pub fn maplut(&self, lut: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "maplut",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "lut",
                    VipsValue::Image(lut.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaplutError,
        )
    }

    pub fn mask_butterworth(
        width: i32,
        height: i32,
        order: f64,
        frequency_cutoff: f64,
        amplitude_cutoff: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "mask_butterworth",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                )
                .set(
                    "order",
                    VipsValue::Double(order),
                )
                .set(
                    "frequency_cutoff",
                    VipsValue::Double(frequency_cutoff),
                )
                .set(
                    "amplitude_cutoff",
                    VipsValue::Double(amplitude_cutoff),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskButterworthError,
        )
    }

    pub fn mask_butterworth_band(
        width: i32,
        height: i32,
        order: f64,
        frequency_cutoff_x: f64,
        frequency_cutoff_y: f64,
        radius: f64,
        amplitude_cutoff: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "mask_butterworth_band",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                )
                .set(
                    "order",
                    VipsValue::Double(order),
                )
                .set(
                    "frequency_cutoff_x",
                    VipsValue::Double(frequency_cutoff_x),
                )
                .set(
                    "frequency_cutoff_y",
                    VipsValue::Double(frequency_cutoff_y),
                )
                .set(
                    "radius",
                    VipsValue::Double(radius),
                )
                .set(
                    "amplitude_cutoff",
                    VipsValue::Double(amplitude_cutoff),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskButterworthBandError,
        )
    }

    pub fn mask_butterworth_ring(
        width: i32,
        height: i32,
        order: f64,
        frequency_cutoff: f64,
        amplitude_cutoff: f64,
        ringwidth: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "mask_butterworth_ring",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                )
                .set(
                    "order",
                    VipsValue::Double(order),
                )
                .set(
                    "frequency_cutoff",
                    VipsValue::Double(frequency_cutoff),
                )
                .set(
                    "amplitude_cutoff",
                    VipsValue::Double(amplitude_cutoff),
                )
                .set(
                    "ringwidth",
                    VipsValue::Double(ringwidth),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskButterworthRingError,
        )
    }

    pub fn mask_fractal(
        width: i32,
        height: i32,
        fractal_dimension: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "mask_fractal",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                )
                .set(
                    "fractal_dimension",
                    VipsValue::Double(fractal_dimension),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskFractalError,
        )
    }

    pub fn mask_gaussian(
        width: i32,
        height: i32,
        frequency_cutoff: f64,
        amplitude_cutoff: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "mask_gaussian",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                )
                .set(
                    "frequency_cutoff",
                    VipsValue::Double(frequency_cutoff),
                )
                .set(
                    "amplitude_cutoff",
                    VipsValue::Double(amplitude_cutoff),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskGaussianError,
        )
    }

    pub fn mask_gaussian_band(
        width: i32,
        height: i32,
        frequency_cutoff_x: f64,
        frequency_cutoff_y: f64,
        radius: f64,
        amplitude_cutoff: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "mask_gaussian_band",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                )
                .set(
                    "frequency_cutoff_x",
                    VipsValue::Double(frequency_cutoff_x),
                )
                .set(
                    "frequency_cutoff_y",
                    VipsValue::Double(frequency_cutoff_y),
                )
                .set(
                    "radius",
                    VipsValue::Double(radius),
                )
                .set(
                    "amplitude_cutoff",
                    VipsValue::Double(amplitude_cutoff),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskGaussianBandError,
        )
    }

    pub fn mask_gaussian_ring(
        width: i32,
        height: i32,
        frequency_cutoff: f64,
        amplitude_cutoff: f64,
        ringwidth: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "mask_gaussian_ring",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                )
                .set(
                    "frequency_cutoff",
                    VipsValue::Double(frequency_cutoff),
                )
                .set(
                    "amplitude_cutoff",
                    VipsValue::Double(amplitude_cutoff),
                )
                .set(
                    "ringwidth",
                    VipsValue::Double(ringwidth),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskGaussianRingError,
        )
    }

    pub fn mask_ideal(
        width: i32,
        height: i32,
        frequency_cutoff: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "mask_ideal",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                )
                .set(
                    "frequency_cutoff",
                    VipsValue::Double(frequency_cutoff),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskIdealError,
        )
    }

    pub fn mask_ideal_band(
        width: i32,
        height: i32,
        frequency_cutoff_x: f64,
        frequency_cutoff_y: f64,
        radius: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "mask_ideal_band",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                )
                .set(
                    "frequency_cutoff_x",
                    VipsValue::Double(frequency_cutoff_x),
                )
                .set(
                    "frequency_cutoff_y",
                    VipsValue::Double(frequency_cutoff_y),
                )
                .set(
                    "radius",
                    VipsValue::Double(radius),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskIdealBandError,
        )
    }

    pub fn mask_ideal_ring(
        width: i32,
        height: i32,
        frequency_cutoff: f64,
        ringwidth: f64,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "mask_ideal_ring",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                )
                .set(
                    "frequency_cutoff",
                    VipsValue::Double(frequency_cutoff),
                )
                .set(
                    "ringwidth",
                    VipsValue::Double(ringwidth),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaskIdealRingError,
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
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "match",
            option
                .set(
                    "ref",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "sec",
                    VipsValue::Image(sec.ctx),
                )
                .set(
                    "xr1",
                    VipsValue::Int(xr1),
                )
                .set(
                    "yr1",
                    VipsValue::Int(yr1),
                )
                .set(
                    "xs1",
                    VipsValue::Int(xs1),
                )
                .set(
                    "ys1",
                    VipsValue::Int(ys1),
                )
                .set(
                    "xr2",
                    VipsValue::Int(xr2),
                )
                .set(
                    "yr2",
                    VipsValue::Int(yr2),
                )
                .set(
                    "xs2",
                    VipsValue::Int(xs2),
                )
                .set(
                    "ys2",
                    VipsValue::Int(ys2),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MatchError,
        )
    }

    pub fn math(&self, math: OperationMath, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "math",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "math",
                    VipsValue::Int(math as i32),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MathError,
        )
    }

    pub fn math2(
        &self,
        right: &VipsImage,
        math2: OperationMath2,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "math2",
            option
                .set(
                    "left",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "right",
                    VipsValue::Image(right.ctx),
                )
                .set(
                    "math2",
                    VipsValue::Int(math2 as i32),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Math2Error,
        )
    }

    pub fn math2_const(
        &self,
        math2: OperationMath2,
        c: &[f64],
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "math2_const",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "math2",
                    VipsValue::Int(math2 as i32),
                )
                .set(
                    "c",
                    VipsValue::DoubleArray(c),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Math2ConstError,
        )
    }

    pub fn matload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "matload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MatrixloadError,
        )
    }

    pub fn matrixinvert(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "matrixinvert",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MatrixinvertError,
        )
    }

    pub fn matrixload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "matrixload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MatrixloadError,
        )
    }

    pub fn matrixload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "matrixload_source",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "source",
                    VipsValue::Source(source.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MatrixloadSourceError,
        )
    }

    pub fn matrixmultiply(&self, right: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "matrixmultiply",
            option
                .set(
                    "left",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "right",
                    VipsValue::Image(right.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("matrixmultiply"),
        )
    }

    pub fn matrixprint(&self, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "matrixprint",
            option.set(
                "in",
                VipsValue::Image(self.ctx),
            ),
        );

        utils::result(
            vips_op_response,
            (),
            Error::MatrixprintError,
        )
    }

    pub fn matrixsave(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "matrixsave",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            (),
            Error::MatrixsaveError,
        )
    }

    pub fn matrixsave_target(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "matrixsave_target",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "target",
                    VipsValue::Target(target.ctx),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::MatrixsaveTargetError,
        )
    }

    pub fn max(&self, option: VOption) -> Result<f64> {
        let mut out_out: f64 = 0.0;

        let vips_op_response = call(
            "max",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutDouble(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::MaxError,
        )
    }

    pub fn maxpair(&self, right: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "maxpair",
            option
                .set(
                    "left",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "right",
                    VipsValue::Image(right.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MaxpairError,
        )
    }

    pub fn measure(&self, h: i32, v: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "measure",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "h",
                    VipsValue::Int(h),
                )
                .set(
                    "v",
                    VipsValue::Int(v),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MeasureError,
        )
    }

    pub fn merge(
        &self,
        sec: &VipsImage,
        direction: Direction,
        dx: i32,
        dy: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "merge",
            option
                .set(
                    "ref",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "sec",
                    VipsValue::Image(sec.ctx),
                )
                .set(
                    "direction",
                    VipsValue::Int(direction as i32),
                )
                .set(
                    "dx",
                    VipsValue::Int(dx),
                )
                .set(
                    "dy",
                    VipsValue::Int(dy),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MergeError,
        )
    }

    pub fn min(&self, option: VOption) -> Result<f64> {
        let mut out_out: f64 = 0.0;

        let vips_op_response = call(
            "min",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutDouble(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            out_out,
            Error::MinError,
        )
    }

    pub fn minpair(&self, right: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "minpair",
            option
                .set(
                    "left",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "right",
                    VipsValue::Image(right.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MinpairError,
        )
    }

    pub fn morph(
        &self,
        mask: &VipsImage,
        morph: OperationMorphology,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "morph",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "mask",
                    VipsValue::Image(mask.ctx),
                )
                .set(
                    "morph",
                    VipsValue::Int(morph as i32),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MorphError,
        )
    }

    pub fn mosaic(
        &self,
        sec: &VipsImage,
        direction: Direction,
        xref: i32,
        yref: i32,
        xsec: i32,
        ysec: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "mosaic",
            option
                .set(
                    "ref",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "sec",
                    VipsValue::Image(sec.ctx),
                )
                .set(
                    "direction",
                    VipsValue::Int(direction as i32),
                )
                .set(
                    "xref",
                    VipsValue::Int(xref),
                )
                .set(
                    "yref",
                    VipsValue::Int(yref),
                )
                .set(
                    "xsec",
                    VipsValue::Int(xsec),
                )
                .set(
                    "ysec",
                    VipsValue::Int(ysec),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MosaicError,
        )
    }

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
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "mosaic1",
            option
                .set(
                    "ref",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "sec",
                    VipsValue::Image(sec.ctx),
                )
                .set(
                    "direction",
                    VipsValue::Int(direction as i32),
                )
                .set(
                    "xr1",
                    VipsValue::Int(xr1),
                )
                .set(
                    "yr1",
                    VipsValue::Int(yr1),
                )
                .set(
                    "xs1",
                    VipsValue::Int(xs1),
                )
                .set(
                    "ys1",
                    VipsValue::Int(ys1),
                )
                .set(
                    "xr2",
                    VipsValue::Int(xr2),
                )
                .set(
                    "yr2",
                    VipsValue::Int(yr2),
                )
                .set(
                    "xs2",
                    VipsValue::Int(xs2),
                )
                .set(
                    "ys2",
                    VipsValue::Int(ys2),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Mosaic1Error,
        )
    }

    pub fn msb(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "msb",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MsbError,
        )
    }

    pub fn multiply(&self, right: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "multiply",
            option
                .set(
                    "left",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "right",
                    VipsValue::Image(right.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::MultiplyError,
        )
    }

    pub fn niftiload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "niftiload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("niftiload"),
        )
    }

    pub fn niftiload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "niftiload_source",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "source",
                    VipsValue::Source(source.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("niftiload_source"),
        )
    }

    pub fn niftisave(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "niftisave",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            (),
            Error::OperationError("niftisave"),
        )
    }

    pub fn openexrload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "openexrload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("openexrload"),
        )
    }

    pub fn openslideload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "openslideload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("openslideload"),
        )
    }

    pub fn openslideload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "openslideload_source",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "source",
                    VipsValue::Source(source.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("openslideload_source"),
        )
    }

    pub fn pdfload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "pdfload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PdfloadError,
        )
    }

    pub fn pdfload_buffer(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "pdfload_buffer",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "buffer",
                    VipsValue::Blob(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PdfloadBufferError,
        )
    }

    pub fn pdfload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "pdfload_source",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "source",
                    VipsValue::Source(source.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("pdfload_source"),
        )
    }

    pub fn percent(&self, percent: f64, option: VOption) -> Result<i32> {
        let mut threshold: i32 = 0;

        let vips_op_response = call(
            "percent",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "threshold",
                    VipsValue::MutInt(&mut threshold),
                )
                .set(
                    "percent",
                    VipsValue::Double(percent),
                ),
        );
        utils::result(
            vips_op_response,
            threshold,
            Error::PercentError,
        )
    }

    pub fn perlin(&self, width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "perlin",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PerlinError,
        )
    }

    pub fn phasecor(&self, in2: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "phasecor",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "in2",
                    VipsValue::Image(in2.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PhasecorError,
        )
    }

    pub fn pngload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "pngload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PngloadError,
        )
    }

    pub fn pngload_buffer(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "pngload_buffer",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "buffer",
                    VipsValue::Blob(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PngloadBufferError,
        )
    }

    pub fn pngload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "pngload_source",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "source",
                    VipsValue::Source(source.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PngloadSourceError,
        )
    }

    pub fn pngsave(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "pngsave",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::PngsaveError,
        )
    }

    pub fn pngsave_buffer(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out: *mut bindings::VipsBlob = null_mut();

        let vips_op_response = call(
            "pngsave_buffer",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "buffer",
                    VipsValue::MutBlob(&mut buffer_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsBlob {
                ctx: buffer_out,
            }
            .into(),
            Error::PngsaveBufferError,
        )
    }

    pub fn pngsave_target(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "pngsave_target",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "target",
                    VipsValue::Target(target.ctx),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::PngsaveTargetError,
        )
    }

    pub fn ppmload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "ppmload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PpmloadError,
        )
    }

    pub fn ppmload_buffer(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "ppmload_buffer",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "buffer",
                    VipsValue::Blob(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("ppmload_buffer"),
        )
    }

    pub fn ppmload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "ppmload_source",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "source",
                    VipsValue::Source(source.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PpmloadSourceError,
        )
    }

    pub fn ppmsave(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "ppmsave",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::PpmsaveError,
        )
    }

    pub fn ppmsave_target(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "ppmsave_target",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "target",
                    VipsValue::Target(target.ctx),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::PpmsaveTargetError,
        )
    }

    pub fn premultiply(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "premultiply",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PremultiplyError,
        )
    }

    pub fn prewitt(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "prewitt",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::PrewittError,
        )
    }

    pub fn profile(&self, rows: &mut VipsImage, option: VOption) -> Result<VipsImage> {
        let mut columns: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "profile",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "columns",
                    VipsValue::MutImage(&mut columns),
                )
                .set(
                    "rows",
                    VipsValue::MutImage(&mut rows.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: columns,
            },
            Error::ProfileError,
        )
    }

    pub fn profile_load(name: &str, option: VOption) -> Result<Vec<u8>> {
        let mut profile_out: *mut bindings::VipsBlob = null_mut();

        let vips_op_response = call(
            "profile_load",
            option
                .set(
                    "profile",
                    VipsValue::MutBlob(&mut profile_out),
                )
                .set(
                    "name",
                    VipsValue::String(name),
                ),
        );

        utils::result(
            vips_op_response,
            VipsBlob {
                ctx: profile_out,
            }
            .into(),
            Error::ProfileLoadError,
        )
    }

    pub fn project(&self, rows: &mut VipsImage, option: VOption) -> Result<VipsImage> {
        let mut columns: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "project",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "columns",
                    VipsValue::MutImage(&mut columns),
                )
                .set(
                    "rows",
                    VipsValue::MutImage(&mut rows.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: columns,
            },
            Error::ProjectError,
        )
    }

    pub fn quadratic(&self, coeff: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "quadratic",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "coeff",
                    VipsValue::Image(coeff.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::QuadraticError,
        )
    }

    pub fn rad2float(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "rad2float",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Rad2FloatError,
        )
    }

    pub fn radload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "radload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RadloadError,
        )
    }

    pub fn radload_buffer(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "radload_buffer",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "buffer",
                    VipsValue::Blob(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RadloadBufferError,
        )
    }

    pub fn radload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "radload_source",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "source",
                    VipsValue::Source(source.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RadloadSourceError,
        )
    }

    pub fn radsave(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "radsave",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::RadsaveError,
        )
    }

    pub fn radsave_buffer(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out: *mut bindings::VipsBlob = null_mut();

        let vips_op_response = call(
            "radsave_buffer",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "buffer",
                    VipsValue::MutBlob(&mut buffer_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsBlob {
                ctx: buffer_out,
            }
            .into(),
            Error::RadsaveBufferError,
        )
    }

    pub fn radsave_target(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "radsave_target",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "target",
                    VipsValue::Target(target.ctx),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::RadsaveTargetError,
        )
    }

    pub fn rank(&self, width: i32, height: i32, index: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "rank",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                )
                .set(
                    "index",
                    VipsValue::Int(index),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RankError,
        )
    }

    pub fn rawload(
        filename: &str,
        width: i32,
        height: i32,
        bands: i32,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "rawload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                )
                .set(
                    "bands",
                    VipsValue::Int(bands),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RawloadError,
        )
    }

    pub fn rawsave(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "rawsave",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::RawsaveError,
        )
    }

    pub fn rawsave_buffer(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out: *mut bindings::VipsBlob = null_mut();

        let vips_op_response = call(
            "rawsave_buffer",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "buffer",
                    VipsValue::MutBlob(&mut buffer_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsBlob {
                ctx: buffer_out,
            }
            .into(),
            Error::RawsaveBufferError,
        )
    }

    pub fn rawsave_target(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "rawsave_target",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "target",
                    VipsValue::Target(target.ctx),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::RawsaveTargetError,
        )
    }

    pub fn recomb(&self, m: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "recomb",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "m",
                    VipsValue::Image(m.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RecombError,
        )
    }

    pub fn reduce(&self, hshrink: f64, vshrink: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "reduce",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "hshrink",
                    VipsValue::Double(hshrink),
                )
                .set(
                    "vshrink",
                    VipsValue::Double(vshrink),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ReduceError,
        )
    }

    pub fn reduceh(&self, hshrink: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "reduceh",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "hshrink",
                    VipsValue::Double(hshrink),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ReducehError,
        )
    }

    pub fn reducev(&self, vshrink: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "reducev",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "vshrink",
                    VipsValue::Double(vshrink),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ReducevError,
        )
    }

    pub fn relational(
        &self,
        right: &VipsImage,
        relational: OperationRelational,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "relational",
            option
                .set(
                    "left",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "right",
                    VipsValue::Image(right.ctx),
                )
                .set(
                    "relational",
                    VipsValue::Int(relational as i32),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RelationalError,
        )
    }

    pub fn relational_const(
        &self,
        relational: OperationRelational,
        c: &[f64],
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "relational_const",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "relational",
                    VipsValue::Int(relational as i32),
                )
                .set(
                    "c",
                    VipsValue::DoubleArray(c),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RelationalConstError,
        )
    }

    pub fn remainder(&self, right: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "remainder",
            option
                .set(
                    "left",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "right",
                    VipsValue::Image(right.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RemainderError,
        )
    }

    pub fn remainder_const(&self, c: &[f64], option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "remainder_const",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "c",
                    VipsValue::DoubleArray(c),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RemainderConstError,
        )
    }

    pub fn remosaic(&self, old_str: &str, new_str: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "remosaic",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "old_str",
                    VipsValue::String(old_str),
                )
                .set(
                    "new_str",
                    VipsValue::String(new_str),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("remosaic"),
        )
    }

    pub fn replicate(&self, across: i32, down: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "replicate",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "across",
                    VipsValue::Int(across),
                )
                .set(
                    "down",
                    VipsValue::Int(down),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ReplicateError,
        )
    }

    pub fn resize(&self, scale: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "resize",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "scale",
                    VipsValue::Double(scale),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ResizeError,
        )
    }

    pub fn rot(&self, angle: Angle, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "rot",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "angle",
                    VipsValue::Int(angle as i32),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RotError,
        )
    }

    pub fn rot45(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "rot45",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Rot45Error,
        )
    }

    pub fn rotate(&self, angle: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "rotate",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "angle",
                    VipsValue::Double(angle),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RotateError,
        )
    }

    pub fn round(&self, round: OperationRound, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "round",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "round",
                    VipsValue::Int(round as i32),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::RoundError,
        )
    }

    pub fn sRGB2HSV(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "sRGB2HSV",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SRgb2HsvError,
        )
    }

    pub fn sRGB2scRGB(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "sRGB2scRGB",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SRgb2ScRgbError,
        )
    }

    pub fn scRGB2BW(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "scRGB2BW",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ScRgb2BwError,
        )
    }

    pub fn scRGB2XYZ(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "scRGB2XYZ",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ScRgb2XyzError,
        )
    }

    pub fn scRGB2sRGB(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "scRGB2sRGB",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ScRgb2SRgbError,
        )
    }

    pub fn scale(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "scale",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ScaleError,
        )
    }

    pub fn scharr(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "scharr",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ScharrError,
        )
    }

    pub fn sdf(
        &self,
        width: i32,
        height: i32,
        shape: SdfShape,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "sdf",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                )
                .set(
                    "shape",
                    VipsValue::Int(shape as i32),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SdfError,
        )
    }

    pub fn sequential(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "sequential",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SequentialError,
        )
    }

    pub fn sharpen(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "sharpen",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SharpenError,
        )
    }

    pub fn shrink(&self, hshrink: f64, vshrink: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "shrink",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "hshrink",
                    VipsValue::Double(hshrink),
                )
                .set(
                    "vshrink",
                    VipsValue::Double(vshrink),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ShrinkError,
        )
    }

    pub fn shrinkh(&self, hshrink: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "shrinkh",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "hshrink",
                    VipsValue::Int(hshrink),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ShrinkhError,
        )
    }

    pub fn shrinkv(&self, vshrink: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "shrinkv",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "vshrink",
                    VipsValue::Int(vshrink),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ShrinkvError,
        )
    }

    pub fn sign(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "sign",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SignError,
        )
    }

    pub fn similarity(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "similarity",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SimilarityError,
        )
    }

    pub fn sines(&self, width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "sines",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("sines"),
        )
    }

    pub fn smartcrop(&self, width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "smartcrop",
            option
                .set(
                    "input",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SmartcropError,
        )
    }

    pub fn sobel(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "sobel",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SobelError,
        )
    }

    pub fn spcor(&self, ref_in: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "spcor",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "ref",
                    VipsValue::Image(ref_in.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SpcorError,
        )
    }

    pub fn spectrum(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "spectrum",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SpectrumError,
        )
    }

    pub fn stats(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "stats",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("stats"),
        )
    }

    pub fn stdif(&self, width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "stdif",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::StdifError,
        )
    }

    pub fn subsample(&self, xfac: i32, yfac: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "subsample",
            option
                .set(
                    "input",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "xfac",
                    VipsValue::Int(xfac),
                )
                .set(
                    "yfac",
                    VipsValue::Int(yfac),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SubsampleError,
        )
    }

    pub fn subtract(&self, right: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "subtract",
            option
                .set(
                    "left",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "right",
                    VipsValue::Image(right.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SubtractError,
        )
    }

    pub fn sum(&self, inps: &[VipsImage], option: VOption) -> Result<VipsImage> {
        let mut inp = Vec::new();
        for inp_in in inps {
            inp.push(inp_in.ctx);
        }
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "sum",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "in",
                    VipsValue::ImageArray(&inp),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SumError,
        )
    }

    pub fn svgload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "svgload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SvgloadError,
        )
    }

    pub fn svgload_buffer(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "svgload_buffer",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "buffer",
                    VipsValue::Blob(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SvgloadBufferError,
        )
    }

    pub fn svgload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "svgload_source",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "source",
                    VipsValue::Source(source.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::OperationError("svgload_source"),
        )
    }

    pub fn switch_image(tests: &[VipsImage], option: VOption) -> Result<VipsImage> {
        let mut inp = Vec::new();
        for inp_in in tests {
            inp.push(inp_in.ctx);
        }
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "switch",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "tests",
                    VipsValue::ImageArray(&inp),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::SwitchError,
        )
    }

    pub fn system(&self, cmd_format: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "system",
            option.set(
                "cmd_format",
                VipsValue::String(cmd_format),
            ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::SystemError,
        )
    }

    pub fn text(&self, text: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "text",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "text",
                    VipsValue::String(text),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::TextError,
        )
    }

    pub fn thumbnail(&self, filename: &str, width: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "thumbnail",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ThumbnailError,
        )
    }

    pub fn thumbnail_buffer(buffer: &[u8], width: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "thumbnail_buffer",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "buffer",
                    VipsValue::Blob(buffer),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ThumbnailBufferError,
        )
    }

    pub fn thumbnail_image(&self, width: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "thumbnail_image",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ThumbnailImageError,
        )
    }

    pub fn thumbnail_source(source: &VipsSource, width: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "thumbnail_source",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "source",
                    VipsValue::Source(source.ctx),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ThumbnailSourceError,
        )
    }

    pub fn tiffload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "tiffload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::TiffloadError,
        )
    }

    pub fn tiffload_buffer(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "tiffload_buffer",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "buffer",
                    VipsValue::Blob(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::TiffloadBufferError,
        )
    }

    pub fn tiffload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "tiffload_source",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "source",
                    VipsValue::Source(source.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::TiffloadSourceError,
        )
    }

    pub fn tiffsave(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "tiffsave",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::TiffsaveError,
        )
    }

    pub fn tiffsave_buffer(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out: *mut bindings::VipsBlob = null_mut();

        let vips_op_response = call(
            "tiffsave_buffer",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "buffer",
                    VipsValue::MutBlob(&mut buffer_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsBlob {
                ctx: buffer_out,
            }
            .into(),
            Error::TiffsaveBufferError,
        )
    }

    pub fn tiffsave_target(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "tiffsave_target",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "target",
                    VipsValue::Target(target.ctx),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::TiffsaveTargetError,
        )
    }

    pub fn tilecache(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "tilecache",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::TilecacheError,
        )
    }

    pub fn tonelut(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "tonelut",
            option.set(
                "out",
                VipsValue::MutImage(&mut out_out),
            ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::TonelutError,
        )
    }

    pub fn transpose3d(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "transpose3d",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::Transpose3DError,
        )
    }

    pub fn unpremultiply(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "unpremultiply",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::UnpremultiplyError,
        )
    }

    pub fn vipsload(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "vipsload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::VipsloadError,
        )
    }

    pub fn vipsload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "vipsload_source",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "source",
                    VipsValue::Source(source.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::VipsloadSourceError,
        )
    }

    pub fn vipssave(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "vipssave",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::VipssaveError,
        )
    }

    pub fn vipssave_target(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "vipssave_target",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "target",
                    VipsValue::Target(target.ctx),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::VipssaveTargetError,
        )
    }

    pub fn webpload(&self, filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "webpload",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::WebploadError,
        )
    }

    pub fn webpload_buffer(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "webpload_buffer",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "buffer",
                    VipsValue::Blob(buffer),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::WebploadBufferError,
        )
    }

    pub fn webpload_source(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "webpload_source",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "source",
                    VipsValue::Source(source.ctx),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::WebploadSourceError,
        )
    }

    pub fn webpsave(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "webpsave",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "filename",
                    VipsValue::String(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::WebpsaveError,
        )
    }

    pub fn webpsave_buffer(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out: *mut bindings::VipsBlob = null_mut();

        let vips_op_response = call(
            "webpsave_buffer",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "buffer",
                    VipsValue::MutBlob(&mut buffer_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsBlob {
                ctx: buffer_out,
            }
            .into(),
            Error::WebpsaveBufferError,
        )
    }

    pub fn webpsave_mime(&self, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "webpsave_mime",
            option.set(
                "in",
                VipsValue::Image(self.ctx),
            ),
        );

        utils::result(
            vips_op_response,
            (),
            Error::WebpsaveMimeError,
        )
    }

    pub fn webpsave_target(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "webpsave_target",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "target",
                    VipsValue::Target(target.ctx),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::WebpsaveTargetError,
        )
    }

    pub fn worley(&self, width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "worley",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::WorleyError,
        )
    }

    pub fn wrap(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "wrap",
            option
                .set(
                    "in",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::WrapError,
        )
    }

    pub fn xyz(&self, width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "xyz",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::XyzError,
        )
    }

    pub fn zone(&self, width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "zone",
            option
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "width",
                    VipsValue::Int(width),
                )
                .set(
                    "height",
                    VipsValue::Int(height),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ZoneError,
        )
    }

    pub fn zoom(&self, xfac: i32, yfac: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out: *mut bindings::VipsImage = null_mut();

        let vips_op_response = call(
            "zoom",
            option
                .set(
                    "input",
                    VipsValue::Image(self.ctx),
                )
                .set(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                )
                .set(
                    "xfac",
                    VipsValue::Int(xfac),
                )
                .set(
                    "yfac",
                    VipsValue::Int(yfac),
                ),
        );

        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out,
            },
            Error::ZoomError,
        )
    }
}
