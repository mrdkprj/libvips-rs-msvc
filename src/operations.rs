#![allow(clippy::too_many_arguments)]
#![allow(clippy::upper_case_acronyms)]
use crate::bindings;
use crate::error::*;
use crate::utils;
use crate::voption::{call, VOption, VipsValue};
use crate::Result;
use crate::VipsBlob;
use crate::VipsImage;
use crate::VipsSource;
use crate::VipsTarget;
use std::ffi::c_void;
use std::ffi::CString;
use std::ptr::null_mut;

const NULL: *const std::ffi::c_void = null_mut();

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Access {
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0
    Random = 0,
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    Sequential = 1,
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    SequentialUnbuffered = 2,
    ///  `Last` -> VIPS_ACCESS_LAST = 3
    Last = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Align {
    ///  `Low` -> VIPS_ALIGN_LOW = 0
    Low = 0,
    ///  `Centre` -> VIPS_ALIGN_CENTRE = 1
    Centre = 1,
    ///  `High` -> VIPS_ALIGN_HIGH = 2
    High = 2,
    ///  `Last` -> VIPS_ALIGN_LAST = 3
    Last = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, Eq, PartialEq)]
pub enum Angle {
    ///  `D0` -> VIPS_ANGLE_D0 = 0
    D0 = 0,
    ///  `D90` -> VIPS_ANGLE_D90 = 1
    D90 = 1,
    ///  `D180` -> VIPS_ANGLE_D180 = 2
    D180 = 2,
    ///  `D270` -> VIPS_ANGLE_D270 = 3
    D270 = 3,
    ///  `Last` -> VIPS_ANGLE_LAST = 4
    Last = 4,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Angle45 {
    ///  `D0` -> VIPS_ANGLE45_D0 = 0
    D0 = 0,
    ///  `D45` -> VIPS_ANGLE45_D45 = 1
    D45 = 1,
    ///  `D90` -> VIPS_ANGLE45_D90 = 2
    D90 = 2,
    ///  `D135` -> VIPS_ANGLE45_D135 = 3
    D135 = 3,
    ///  `D180` -> VIPS_ANGLE45_D180 = 4
    D180 = 4,
    ///  `D225` -> VIPS_ANGLE45_D225 = 5
    D225 = 5,
    ///  `D270` -> VIPS_ANGLE45_D270 = 6
    D270 = 6,
    ///  `D315` -> VIPS_ANGLE45_D315 = 7
    D315 = 7,
    ///  `Last` -> VIPS_ANGLE45_LAST = 8
    Last = 8,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq)]
pub enum BandFormat {
    ///  `Notset` -> VIPS_FORMAT_NOTSET = -1
    Notset = -1,
    ///  `Uchar` -> VIPS_FORMAT_UCHAR = 0
    Uchar = 0,
    ///  `Char` -> VIPS_FORMAT_CHAR = 1
    Char = 1,
    ///  `Ushort` -> VIPS_FORMAT_USHORT = 2
    Ushort = 2,
    ///  `Short` -> VIPS_FORMAT_SHORT = 3
    Short = 3,
    ///  `Uint` -> VIPS_FORMAT_UINT = 4
    Uint = 4,
    ///  `Int` -> VIPS_FORMAT_INT = 5
    Int = 5,
    ///  `Float` -> VIPS_FORMAT_FLOAT = 6
    Float = 6,
    ///  `Complex` -> VIPS_FORMAT_COMPLEX = 7
    Complex = 7,
    ///  `Double` -> VIPS_FORMAT_DOUBLE = 8
    Double = 8,
    ///  `Dpcomplex` -> VIPS_FORMAT_DPCOMPLEX = 9
    Dpcomplex = 9,
    ///  `Last` -> VIPS_FORMAT_LAST = 10
    Last = 10,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum BlendMode {
    ///  `Clear` -> VIPS_BLEND_MODE_CLEAR = 0
    Clear = 0,
    ///  `Source` -> VIPS_BLEND_MODE_SOURCE = 1
    Source = 1,
    ///  `Over` -> VIPS_BLEND_MODE_OVER = 2
    Over = 2,
    ///  `In` -> VIPS_BLEND_MODE_IN = 3
    In = 3,
    ///  `Out` -> VIPS_BLEND_MODE_OUT = 4
    Out = 4,
    ///  `Atop` -> VIPS_BLEND_MODE_ATOP = 5
    Atop = 5,
    ///  `Dest` -> VIPS_BLEND_MODE_DEST = 6
    Dest = 6,
    ///  `DestOver` -> VIPS_BLEND_MODE_DEST_OVER = 7
    DestOver = 7,
    ///  `DestIn` -> VIPS_BLEND_MODE_DEST_IN = 8
    DestIn = 8,
    ///  `DestOut` -> VIPS_BLEND_MODE_DEST_OUT = 9
    DestOut = 9,
    ///  `DestAtop` -> VIPS_BLEND_MODE_DEST_ATOP = 10
    DestAtop = 10,
    ///  `Xor` -> VIPS_BLEND_MODE_XOR = 11
    Xor = 11,
    ///  `Add` -> VIPS_BLEND_MODE_ADD = 12
    Add = 12,
    ///  `Saturate` -> VIPS_BLEND_MODE_SATURATE = 13
    Saturate = 13,
    ///  `Multiply` -> VIPS_BLEND_MODE_MULTIPLY = 14
    Multiply = 14,
    ///  `Screen` -> VIPS_BLEND_MODE_SCREEN = 15
    Screen = 15,
    ///  `Overlay` -> VIPS_BLEND_MODE_OVERLAY = 16
    Overlay = 16,
    ///  `Darken` -> VIPS_BLEND_MODE_DARKEN = 17
    Darken = 17,
    ///  `Lighten` -> VIPS_BLEND_MODE_LIGHTEN = 18
    Lighten = 18,
    ///  `ColourDodge` -> VIPS_BLEND_MODE_COLOUR_DODGE = 19
    ColourDodge = 19,
    ///  `ColourBurn` -> VIPS_BLEND_MODE_COLOUR_BURN = 20
    ColourBurn = 20,
    ///  `HardLight` -> VIPS_BLEND_MODE_HARD_LIGHT = 21
    HardLight = 21,
    ///  `SoftLight` -> VIPS_BLEND_MODE_SOFT_LIGHT = 22
    SoftLight = 22,
    ///  `Difference` -> VIPS_BLEND_MODE_DIFFERENCE = 23
    Difference = 23,
    ///  `Exclusion` -> VIPS_BLEND_MODE_EXCLUSION = 24
    Exclusion = 24,
    ///  `Last` -> VIPS_BLEND_MODE_LAST = 25
    Last = 25,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Coding {
    ///  `Error` -> VIPS_CODING_ERROR = -1
    Error = -1,
    ///  `None` -> VIPS_CODING_NONE = 0
    None = 0,
    ///  `Labq` -> VIPS_CODING_LABQ = 2
    Labq = 2,
    ///  `Rad` -> VIPS_CODING_RAD = 6
    Rad = 6,
    ///  `Last` -> VIPS_CODING_LAST = 7
    Last = 7,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Combine {
    ///  `Max` -> VIPS_COMBINE_MAX = 0
    Max = 0,
    ///  `Sum` -> VIPS_COMBINE_SUM = 1
    Sum = 1,
    ///  `Min` -> VIPS_COMBINE_MIN = 2
    Min = 2,
    ///  `Last` -> VIPS_COMBINE_LAST = 3
    Last = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum CombineMode {
    ///  `Set` -> VIPS_COMBINE_MODE_SET = 0
    Set = 0,
    ///  `Add` -> VIPS_COMBINE_MODE_ADD = 1
    Add = 1,
    ///  `Last` -> VIPS_COMBINE_MODE_LAST = 2
    Last = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum CompassDirection {
    ///  `Centre` -> VIPS_COMPASS_DIRECTION_CENTRE = 0
    Centre = 0,
    ///  `North` -> VIPS_COMPASS_DIRECTION_NORTH = 1
    North = 1,
    ///  `East` -> VIPS_COMPASS_DIRECTION_EAST = 2
    East = 2,
    ///  `South` -> VIPS_COMPASS_DIRECTION_SOUTH = 3
    South = 3,
    ///  `West` -> VIPS_COMPASS_DIRECTION_WEST = 4
    West = 4,
    ///  `NorthEast` -> VIPS_COMPASS_DIRECTION_NORTH_EAST = 5
    NorthEast = 5,
    ///  `SouthEast` -> VIPS_COMPASS_DIRECTION_SOUTH_EAST = 6
    SouthEast = 6,
    ///  `SouthWest` -> VIPS_COMPASS_DIRECTION_SOUTH_WEST = 7
    SouthWest = 7,
    ///  `NorthWest` -> VIPS_COMPASS_DIRECTION_NORTH_WEST = 8
    NorthWest = 8,
    ///  `Last` -> VIPS_COMPASS_DIRECTION_LAST = 9
    Last = 9,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Direction {
    ///  `Horizontal` -> VIPS_DIRECTION_HORIZONTAL = 0
    Horizontal = 0,
    ///  `Vertical` -> VIPS_DIRECTION_VERTICAL = 1
    Vertical = 1,
    ///  `Last` -> VIPS_DIRECTION_LAST = 2
    Last = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, Eq)]
pub enum Extend {
    ///  `Black` -> VIPS_EXTEND_BLACK = 0
    Black = 0,
    ///  `Copy` -> VIPS_EXTEND_COPY = 1
    Copy = 1,
    ///  `Repeat` -> VIPS_EXTEND_REPEAT = 2
    Repeat = 2,
    ///  `Mirror` -> VIPS_EXTEND_MIRROR = 3
    Mirror = 3,
    ///  `White` -> VIPS_EXTEND_WHITE = 4
    White = 4,
    ///  `Background` -> VIPS_EXTEND_BACKGROUND = 5
    Background = 5,
    ///  `Last` -> VIPS_EXTEND_LAST = 6
    Last = 6,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum FailOn {
    ///  `None` -> VIPS_FAIL_ON_NONE = 0
    None = 0,
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    Truncated = 1,
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    Error = 2,
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    Warning = 3,
    ///  `Last` -> VIPS_FAIL_ON_LAST = 4
    Last = 4,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialOrd, PartialEq)]
pub enum ForeignDzDepth {
    ///  `Onepixel` -> VIPS_FOREIGN_DZ_DEPTH_ONEPIXEL = 0
    Onepixel = 0,
    ///  `Onetile` -> VIPS_FOREIGN_DZ_DEPTH_ONETILE = 1
    Onetile = 1,
    ///  `One` -> VIPS_FOREIGN_DZ_DEPTH_ONE = 2
    One = 2,
    ///  `Last` -> VIPS_FOREIGN_DZ_DEPTH_LAST = 3
    Last = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignFlags {
    ///  `None` -> VIPS_FOREIGN_NONE = 0
    None = 0,
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    Partial = 1,
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    Bigendian = 2,
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    Sequential = 4,
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    All = 7,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignHeifCompression {
    ///  `Hevc` -> VIPS_FOREIGN_HEIF_COMPRESSION_HEVC = 1
    Hevc = 1,
    ///  `Avc` -> VIPS_FOREIGN_HEIF_COMPRESSION_AVC = 2
    Avc = 2,
    ///  `Jpeg` -> VIPS_FOREIGN_HEIF_COMPRESSION_JPEG = 3
    Jpeg = 3,
    ///  `Av1` -> VIPS_FOREIGN_HEIF_COMPRESSION_AV1 = 4
    Av1 = 4,
    ///  `Last` -> VIPS_FOREIGN_HEIF_COMPRESSION_LAST = 5
    Last = 5,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignHeifEncoder {
    ///  `Auto` -> VIPS_FOREIGN_HEIF_ENCODER_AUTO = 0
    Auto = 0,
    ///  `Aom` -> VIPS_FOREIGN_HEIF_ENCODER_AOM = 1
    Aom = 1,
    ///  `Rav1E` -> VIPS_FOREIGN_HEIF_ENCODER_RAV1E = 2
    Rav1E = 2,
    ///  `Svt` -> VIPS_FOREIGN_HEIF_ENCODER_SVT = 3
    Svt = 3,
    ///  `X265` -> VIPS_FOREIGN_HEIF_ENCODER_X265 = 4
    X265 = 4,
    ///  `Last` -> VIPS_FOREIGN_HEIF_ENCODER_LAST = 5
    Last = 5,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignKeep {
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    None = 0,
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    Exif = 1,
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    Xmp = 2,
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    Iptc = 4,
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    Icc = 8,
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    Other = 16,
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 31
    All = 31,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq)]
pub enum ForeignDzLayout {
    Dz = 0,
    Zoomify = 1,
    Google = 2,
    Iiif = 3,
    Iiif3 = 4,
    Last = 5,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignDzContainer {
    Fs = 0,
    Zip = 1,
    Szi = 2,
    Last = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignPngFilter {
    ///  `None` -> VIPS_FOREIGN_PNG_FILTER_NONE = 8
    None = 8,
    ///  `Sub` -> VIPS_FOREIGN_PNG_FILTER_SUB = 16
    Sub = 16,
    ///  `Up` -> VIPS_FOREIGN_PNG_FILTER_UP = 32
    Up = 32,
    ///  `Avg` -> VIPS_FOREIGN_PNG_FILTER_AVG = 64
    Avg = 64,
    ///  `Paeth` -> VIPS_FOREIGN_PNG_FILTER_PAETH = 128
    Paeth = 128,
    ///  `All` -> VIPS_FOREIGN_PNG_FILTER_ALL = 248
    All = 248,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignPpmFormat {
    ///  `Pbm` -> VIPS_FOREIGN_PPM_FORMAT_PBM = 0
    Pbm = 0,
    ///  `Pgm` -> VIPS_FOREIGN_PPM_FORMAT_PGM = 1
    Pgm = 1,
    ///  `Ppm` -> VIPS_FOREIGN_PPM_FORMAT_PPM = 2
    Ppm = 2,
    ///  `Pfm` -> VIPS_FOREIGN_PPM_FORMAT_PFM = 3
    Pfm = 3,
    ///  `Pnm` -> VIPS_FOREIGN_PPM_FORMAT_PNM = 4
    Pnm = 4,
    ///  `Last` -> VIPS_FOREIGN_PPM_FORMAT_LAST = 5
    Last = 5,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignSubsample {
    ///  `Auto` -> VIPS_FOREIGN_SUBSAMPLE_AUTO = 0
    Auto = 0,
    ///  `On` -> VIPS_FOREIGN_SUBSAMPLE_ON = 1
    On = 1,
    ///  `Off` -> VIPS_FOREIGN_SUBSAMPLE_OFF = 2
    Off = 2,
    ///  `Last` -> VIPS_FOREIGN_SUBSAMPLE_LAST = 3
    Last = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum ForeignTiffCompression {
    ///  `None` -> VIPS_FOREIGN_TIFF_COMPRESSION_NONE = 0
    None = 0,
    ///  `Jpeg` -> VIPS_FOREIGN_TIFF_COMPRESSION_JPEG = 1
    Jpeg = 1,
    ///  `Deflate` -> VIPS_FOREIGN_TIFF_COMPRESSION_DEFLATE = 2
    Deflate = 2,
    ///  `Packbit` -> VIPS_FOREIGN_TIFF_COMPRESSION_PACKBITS = 3
    Packbit = 3,
    ///  `Ccittfax4` -> VIPS_FOREIGN_TIFF_COMPRESSION_CCITTFAX4 = 4
    Ccittfax4 = 4,
    ///  `Lzw` -> VIPS_FOREIGN_TIFF_COMPRESSION_LZW = 5
    Lzw = 5,
    ///  `Webp` -> VIPS_FOREIGN_TIFF_COMPRESSION_WEBP = 6
    Webp = 6,
    ///  `Zstd` -> VIPS_FOREIGN_TIFF_COMPRESSION_ZSTD = 7
    Zstd = 7,
    ///  `Jp2K` -> VIPS_FOREIGN_TIFF_COMPRESSION_JP2K = 8
    Jp2K = 8,
    ///  `Last` -> VIPS_FOREIGN_TIFF_COMPRESSION_LAST = 9
    Last = 9,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum ForeignTiffPredictor {
    ///  `None` -> VIPS_FOREIGN_TIFF_PREDICTOR_NONE = 1
    None = 1,
    ///  `Horizontal` -> VIPS_FOREIGN_TIFF_PREDICTOR_HORIZONTAL = 2
    Horizontal = 2,
    ///  `Float` -> VIPS_FOREIGN_TIFF_PREDICTOR_FLOAT = 3
    Float = 3,
    ///  `Last` -> VIPS_FOREIGN_TIFF_PREDICTOR_LAST = 4
    Last = 4,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignTiffResunit {
    ///  `Cm` -> VIPS_FOREIGN_TIFF_RESUNIT_CM = 0
    Cm = 0,
    ///  `Inch` -> VIPS_FOREIGN_TIFF_RESUNIT_INCH = 1
    Inch = 1,
    ///  `Last` -> VIPS_FOREIGN_TIFF_RESUNIT_LAST = 2
    Last = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum ForeignWebpPreset {
    ///  `Default` -> VIPS_FOREIGN_WEBP_PRESET_DEFAULT = 0
    Default = 0,
    ///  `Picture` -> VIPS_FOREIGN_WEBP_PRESET_PICTURE = 1
    Picture = 1,
    ///  `Photo` -> VIPS_FOREIGN_WEBP_PRESET_PHOTO = 2
    Photo = 2,
    ///  `Drawing` -> VIPS_FOREIGN_WEBP_PRESET_DRAWING = 3
    Drawing = 3,
    ///  `Icon` -> VIPS_FOREIGN_WEBP_PRESET_ICON = 4
    Icon = 4,
    ///  `Text` -> VIPS_FOREIGN_WEBP_PRESET_TEXT = 5
    Text = 5,
    ///  `Last` -> VIPS_FOREIGN_WEBP_PRESET_LAST = 6
    Last = 6,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Intent {
    ///  `Perceptual` -> VIPS_INTENT_PERCEPTUAL = 0
    Perceptual = 0,
    ///  `Relative` -> VIPS_INTENT_RELATIVE = 1
    Relative = 1,
    ///  `Saturation` -> VIPS_INTENT_SATURATION = 2
    Saturation = 2,
    ///  `Absolute` -> VIPS_INTENT_ABSOLUTE = 3
    Absolute = 3,
    ///  `Last` -> VIPS_INTENT_LAST = 4
    Last = 4,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Interesting {
    ///  `None` -> VIPS_INTERESTING_NONE = 0
    None = 0,
    ///  `Centre` -> VIPS_INTERESTING_CENTRE = 1
    Centre = 1,
    ///  `Entropy` -> VIPS_INTERESTING_ENTROPY = 2
    Entropy = 2,
    ///  `Attention` -> VIPS_INTERESTING_ATTENTION = 3
    Attention = 3,
    ///  `Low` -> VIPS_INTERESTING_LOW = 4
    Low = 4,
    ///  `High` -> VIPS_INTERESTING_HIGH = 5
    High = 5,
    ///  `All` -> VIPS_INTERESTING_ALL = 6
    All = 6,
    ///  `Last` -> VIPS_INTERESTING_LAST = 7
    Last = 7,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, Eq)]
pub enum Interpretation {
    ///  `Error` -> VIPS_INTERPRETATION_ERROR = -1
    Error = -1,
    ///  `Multiband` -> VIPS_INTERPRETATION_MULTIBAND = 0
    Multiband = 0,
    ///  `BW` -> VIPS_INTERPRETATION_B_W = 1
    BW = 1,
    ///  `Histogram` -> VIPS_INTERPRETATION_HISTOGRAM = 10
    Histogram = 10,
    ///  `Xyz` -> VIPS_INTERPRETATION_XYZ = 12
    Xyz = 12,
    ///  `Lab` -> VIPS_INTERPRETATION_LAB = 13
    Lab = 13,
    ///  `Cmyk` -> VIPS_INTERPRETATION_CMYK = 15
    Cmyk = 15,
    ///  `Labq` -> VIPS_INTERPRETATION_LABQ = 16
    Labq = 16,
    ///  `Rgb` -> VIPS_INTERPRETATION_RGB = 17
    Rgb = 17,
    ///  `Cmc` -> VIPS_INTERPRETATION_CMC = 18
    Cmc = 18,
    ///  `Lch` -> VIPS_INTERPRETATION_LCH = 19
    Lch = 19,
    ///  `Lab` -> VIPS_INTERPRETATION_LABS = 21
    Labs = 21,
    ///  `Srgb` -> VIPS_INTERPRETATION_sRGB = 22
    Srgb = 22,
    ///  `Yxy` -> VIPS_INTERPRETATION_YXY = 23
    Yxy = 23,
    ///  `Fourier` -> VIPS_INTERPRETATION_FOURIER = 24
    Fourier = 24,
    ///  `Rgb16` -> VIPS_INTERPRETATION_RGB16 = 25
    Rgb16 = 25,
    ///  `Grey16` -> VIPS_INTERPRETATION_GREY16 = 26
    Grey16 = 26,
    ///  `Matrix` -> VIPS_INTERPRETATION_MATRIX = 27
    Matrix = 27,
    ///  `Scrgb` -> VIPS_INTERPRETATION_scRGB = 28
    Scrgb = 28,
    ///  `Hsv` -> VIPS_INTERPRETATION_HSV = 29
    Hsv = 29,
    ///  `Last` -> VIPS_INTERPRETATION_LAST = 30
    Last = 30,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Kernel {
    ///  `Nearest` -> VIPS_KERNEL_NEAREST = 0
    Nearest = 0,
    ///  `Linear` -> VIPS_KERNEL_LINEAR = 1
    Linear = 1,
    ///  `Cubic` -> VIPS_KERNEL_CUBIC = 2
    Cubic = 2,
    ///  `Mitchell` -> VIPS_KERNEL_MITCHELL = 3
    Mitchell = 3,
    ///  `Lanczos2` -> VIPS_KERNEL_LANCZOS2 = 4
    Lanczos2 = 4,
    ///  `Lanczos3` -> VIPS_KERNEL_LANCZOS3 = 5
    Lanczos3 = 5,
    ///  `Last` -> VIPS_KERNEL_MKS2013  = 6
    Mks2013 = 6,
    ///  `Last` -> VIPS_KERNEL_MKS2021  = 7
    Mks2021 = 7,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum OperationBoolean {
    ///  `And` -> VIPS_OPERATION_BOOLEAN_AND = 0
    And = 0,
    ///  `Or` -> VIPS_OPERATION_BOOLEAN_OR = 1
    Or = 1,
    ///  `Eor` -> VIPS_OPERATION_BOOLEAN_EOR = 2
    Eor = 2,
    ///  `Lshift` -> VIPS_OPERATION_BOOLEAN_LSHIFT = 3
    Lshift = 3,
    ///  `Rshift` -> VIPS_OPERATION_BOOLEAN_RSHIFT = 4
    Rshift = 4,
    ///  `Last` -> VIPS_OPERATION_BOOLEAN_LAST = 5
    Last = 5,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum OperationComplex {
    ///  `Polar` -> VIPS_OPERATION_COMPLEX_POLAR = 0
    Polar = 0,
    ///  `Rect` -> VIPS_OPERATION_COMPLEX_RECT = 1
    Rect = 1,
    ///  `Conj` -> VIPS_OPERATION_COMPLEX_CONJ = 2
    Conj = 2,
    ///  `Last` -> VIPS_OPERATION_COMPLEX_LAST = 3
    Last = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum OperationComplex2 {
    ///  `CrossPhase` -> VIPS_OPERATION_COMPLEX2_CROSS_PHASE = 0
    CrossPhase = 0,
    ///  `Last` -> VIPS_OPERATION_COMPLEX2_LAST = 1
    Last = 1,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum OperationComplexget {
    ///  `Real` -> VIPS_OPERATION_COMPLEXGET_REAL = 0
    Real = 0,
    ///  `Imag` -> VIPS_OPERATION_COMPLEXGET_IMAG = 1
    Imag = 1,
    ///  `Last` -> VIPS_OPERATION_COMPLEXGET_LAST = 2
    Last = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum OperationMath {
    ///  `Sin` -> VIPS_OPERATION_MATH_SIN = 0
    Sin = 0,
    ///  `Co` -> VIPS_OPERATION_MATH_COS = 1
    Co = 1,
    ///  `Tan` -> VIPS_OPERATION_MATH_TAN = 2
    Tan = 2,
    ///  `Asin` -> VIPS_OPERATION_MATH_ASIN = 3
    Asin = 3,
    ///  `Aco` -> VIPS_OPERATION_MATH_ACOS = 4
    Aco = 4,
    ///  `Atan` -> VIPS_OPERATION_MATH_ATAN = 5
    Atan = 5,
    ///  `Log` -> VIPS_OPERATION_MATH_LOG = 6
    Log = 6,
    ///  `Log10` -> VIPS_OPERATION_MATH_LOG10 = 7
    Log10 = 7,
    ///  `Exp` -> VIPS_OPERATION_MATH_EXP = 8
    Exp = 8,
    ///  `Exp10` -> VIPS_OPERATION_MATH_EXP10 = 9
    Exp10 = 9,
    ///  `Sinh` -> VIPS_OPERATION_MATH_SINH = 10
    Sinh = 10,
    ///  `Cosh` -> VIPS_OPERATION_MATH_COSH = 11
    Cosh = 11,
    ///  `Tanh` -> VIPS_OPERATION_MATH_TANH = 12
    Tanh = 12,
    ///  `Asinh` -> VIPS_OPERATION_MATH_ASINH = 13
    Asinh = 13,
    ///  `Acosh` -> VIPS_OPERATION_MATH_ACOSH = 14
    Acosh = 14,
    ///  `Atanh` -> VIPS_OPERATION_MATH_ATANH = 15
    Atanh = 15,
    ///  `Last` -> VIPS_OPERATION_MATH_LAST = 16
    Last = 16,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum OperationMath2 {
    ///  `Pow` -> VIPS_OPERATION_MATH2_POW = 0
    Pow = 0,
    ///  `Wop` -> VIPS_OPERATION_MATH2_WOP = 1
    Wop = 1,
    ///  `Atan2` -> VIPS_OPERATION_MATH2_ATAN2 = 2
    Atan2 = 2,
    ///  `Last` -> VIPS_OPERATION_MATH2_LAST = 3
    Last = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum OperationMorphology {
    ///  `Erode` -> VIPS_OPERATION_MORPHOLOGY_ERODE = 0
    Erode = 0,
    ///  `Dilate` -> VIPS_OPERATION_MORPHOLOGY_DILATE = 1
    Dilate = 1,
    ///  `Last` -> VIPS_OPERATION_MORPHOLOGY_LAST = 2
    Last = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum OperationRelational {
    ///  `Equal` -> VIPS_OPERATION_RELATIONAL_EQUAL = 0
    Equal = 0,
    ///  `Noteq` -> VIPS_OPERATION_RELATIONAL_NOTEQ = 1
    Noteq = 1,
    ///  `Less` -> VIPS_OPERATION_RELATIONAL_LESS = 2
    Less = 2,
    ///  `Lesseq` -> VIPS_OPERATION_RELATIONAL_LESSEQ = 3
    Lesseq = 3,
    ///  `More` -> VIPS_OPERATION_RELATIONAL_MORE = 4
    More = 4,
    ///  `Moreeq` -> VIPS_OPERATION_RELATIONAL_MOREEQ = 5
    Moreeq = 5,
    ///  `Last` -> VIPS_OPERATION_RELATIONAL_LAST = 6
    Last = 6,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum OperationRound {
    ///  `Rint` -> VIPS_OPERATION_ROUND_RINT = 0
    Rint = 0,
    ///  `Ceil` -> VIPS_OPERATION_ROUND_CEIL = 1
    Ceil = 1,
    ///  `Floor` -> VIPS_OPERATION_ROUND_FLOOR = 2
    Floor = 2,
    ///  `Last` -> VIPS_OPERATION_ROUND_LAST = 3
    Last = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum PCS {
    ///  `Lab` -> VIPS_PCS_LAB = 0
    Lab = 0,
    ///  `Xyz` -> VIPS_PCS_XYZ = 1
    Xyz = 1,
    ///  `Last` -> VIPS_PCS_LAST = 2
    Last = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Precision {
    ///  `Integer` -> VIPS_PRECISION_INTEGER = 0
    Integer = 0,
    ///  `Float` -> VIPS_PRECISION_FLOAT = 1
    Float = 1,
    ///  `Approximate` -> VIPS_PRECISION_APPROXIMATE = 2
    Approximate = 2,
    ///  `Last` -> VIPS_PRECISION_LAST = 3
    Last = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum RegionShrink {
    ///  `Mean` -> VIPS_REGION_SHRINK_MEAN = 0
    Mean = 0,
    ///  `Median` -> VIPS_REGION_SHRINK_MEDIAN = 1
    Median = 1,
    ///  `Mode` -> VIPS_REGION_SHRINK_MODE = 2
    Mode = 2,
    ///  `Max` -> VIPS_REGION_SHRINK_MAX = 3
    Max = 3,
    ///  `Min` -> VIPS_REGION_SHRINK_MIN = 4
    Min = 4,
    ///  `Nearest` -> VIPS_REGION_SHRINK_NEAREST = 5
    Nearest = 5,
    ///  `Last` -> VIPS_REGION_SHRINK_LAST = 6
    Last = 6,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum SdfShape {
    ///  `Circle` -> VIPS_SDF_SHAPE_CIRCLE = 0
    Circle = 0,
    ///  `Box` -> VIPS_SDF_SHAPE_BOX = 1
    Box = 1,
    ///  `RoundedBox` -> VIPS_SDF_SHAPE_ROUNDED_BOX = 2
    RoundedBox = 2,
    ///  `Line` -> VIPS_SDF_SHAPE_LINE = 3
    Line = 3,
    ///  `Last` -> VIPS_SDF_SHAPE_LAST = 4
    Last = 4,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum Size {
    ///  `Both` -> VIPS_SIZE_BOTH = 0
    Both = 0,
    ///  `Up` -> VIPS_SIZE_UP = 1
    Up = 1,
    ///  `Down` -> VIPS_SIZE_DOWN = 2
    Down = 2,
    ///  `Force` -> VIPS_SIZE_FORCE = 3
    Force = 3,
    ///  `Last` -> VIPS_SIZE_LAST = 4
    Last = 4,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
pub enum TextWrap {
    ///  `Word` -> VIPS_TEXT_WRAP_WORD = 0
    Word = 0,
    ///  `Char` -> VIPS_TEXT_WRAP_CHAR = 1
    Char = 1,
    ///  `WordChar` -> VIPS_TEXT_WRAP_WORD_CHAR = 2
    WordChar = 2,
    ///  `None` -> VIPS_TEXT_WRAP_NONE = 3
    None = 3,
    ///  `Last` -> VIPS_TEXT_WRAP_LAST = 4
    Last = 4,
}

impl VipsImage {
    /// Transform LCh to CMC
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

    /// Transform CMYK to XYZ
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

    /// Transform HSV to sRGB
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

    /// Transform LCh to CMC
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

    /// Transform LCh to Lab
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

    /// Transform Lab to LCh
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

    /// Transform float Lab to LabQ coding
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

    /// Transform float Lab to signed short
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

    /// Transform CIELAB to XYZ
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

    /// Transform CIELAB to XYZ
    ///
    /// Optional arguments
    ///
    /// temp: `&[f64]`, colour temperature.
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

    /// Unpack a LabQ image to float Lab
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

    /// Unpack a LabQ image to short Lab
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

    /// Convert a LabQ image to sRGB
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

    /// Transform signed short Lab to float
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

    /// Transform short Lab to LabQ coding
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

    /// Transform XYZ to CMYK
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

    /// Transform XYZ to Lab
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

    /// Transform XYZ to Lab
    ///
    /// Optional arguments
    ///
    /// temp: `&[f64]`, colour temperature.
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

    /// Transform XYZ to Yxy
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

    /// Transform XYZ to scRGB
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

    /// Transform Yxy to XYZ
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

    /// Absolute value of an image
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
            Error::OperationError("Abs failed"),
        )
    }

    /// Add this image to right image
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

    // Alias for operator overload
    pub(crate) fn add_image(&self, right: &VipsImage) -> Result<VipsImage> {
        self.add(right)
    }

    /// Append an alpha channel
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

    /// Affine transform of an image
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

    /// Affine transform of an image
    ///
    /// Optional arguments
    ///
    /// interpolate: `Interpolate`, interpolate pixels with this
    ///
    /// oarea: `&[f64]`, output rectangle
    ///
    /// idx: `f64`, input horizontal offset
    ///
    /// idy: `f64`, input vertical offset
    ///
    /// odx: `f64`, output horizontal offset
    ///
    /// ody: `f64`, output vertical offset
    ///
    /// extend: `VipsExtend`, how to generate new pixels
    ///
    /// background: `&[f64]` colour for new pixels
    ///
    /// premultiplied: `bool`, images are already premultiplied
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

    /// Load an Analyze 6.0 file.
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

    /// Load an Analyze 6.0 file.
    pub fn analyzeload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "analyzeload",
            option
                .with(
                    "filename",
                    VipsValue::Str(filename),
                )
                .with(
                    "out",
                    VipsValue::MutImage(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Analyzeload failed"),
        )
    }

    /// Join an array of images
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

    /// Join an array of images
    ///
    /// Optional arguments
    ///
    /// across: `i32`, number of images per row
    ///
    /// shim: `i32`, space between images, in pixels
    ///
    /// background: `&[f64]`, background ink colour
    ///
    /// halign: `Align`, low, centre or high alignment
    ///
    /// valign: `Align`, low, centre or high alignment
    ///
    /// hspacing: `i32`, horizontal distance between images
    ///
    /// vspacing: `i32`, vertical distance between images
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

    /// Autorotate image by exif tag
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

    /// Autorotate image by exif tag
    ///
    /// Optional arguments
    ///
    /// angle: `Angle`, output, the image was rotated by
    ///
    /// flip: `bool`, output, whether the image was flipped.
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

    /// Find image average
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

    /// Boolean operation across image bands
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

    /// Fold up an image horizontally
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

    /// Fold up an image horizontally
    ///
    /// Optional arguments
    ///
    /// factor: `i32`, fold by this factor
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

    /// Join a set of images together, bandwise.
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

    /// Join this image with a set of images, bandwise.
    pub fn bandjoin_with(&self, others: &[VipsImage]) -> Result<VipsImage> {
        unsafe {
            let mut inp_in = Vec::new();
            inp_in.push(self.ctx);
            for img in others {
                inp_in.push(img.ctx)
            }
            let mut out_out: *mut bindings::VipsImage = null_mut();

            let vips_op_response = bindings::vips_bandjoin(
                inp_in.as_mut_ptr(),
                &mut out_out,
                inp_in.len() as i32,
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

    /// Append a set of constant bands to an image.
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

    /// Band-wise average
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

    /// Band-wise rank of a set of images
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

    /// Band-wise rank of a set of images
    ///
    /// Optional arguments
    ///
    /// index: `i32`, pick this index from list of sorted values
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

    /// Unfold image bands into x axis
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

    /// Unfold image bands into x axis
    ///
    /// Optional arguments
    ///
    /// factor: `i32`, unfold by this factor
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

    /// Make a black image
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

    /// Make a black image
    ///
    /// Optional arguments
    ///
    /// bands: `i32`, output bands
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

    /// Boolean operation on two images
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

    /// Boolean operations against a constant
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

    /// Build a look-up table
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

    /// Byteswap an image
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

    /// Canny edge detector
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

    /// Canny edge detector
    ///
    /// Optional arguments
    ///
    /// sigma: `f64`, sigma for gaussian blur
    ///
    /// precision: `Precision`, calculation accuracy
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

    /// Use values in index to select pixels from cases
    pub fn case(&self, cases: &[VipsImage]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "case",
            VOption::new()
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

    /// Cast an image
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

    /// Cast an image
    ///
    /// Optional arguments
    ///
    /// shift: `bool`, integer values are shifted
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

    /// Clamps pixel values to a range, by default 0 - 1.
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

    /// Clamps pixel values to a range, by default 0 - 1.
    ///
    /// Optional arguments
    ///
    /// min: `f64`, minimum value
    ///
    /// max: `f64`, maximum value
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

    /// Convert to a new colorspace
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

    /// Convert to a new colorspace
    ///
    /// Optional arguments
    ///
    /// source_space: `Interpretation`, input colour space
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

    /// Convolve with rotating mask
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

    /// Convolve with rotating mask
    ///
    /// Optional arguments
    ///
    /// times: `i32`, how many times to rotate and convolve
    ///
    /// angle: `Angle`, rotate mask by this much between colvolutions
    ///
    /// combine: `Combine`, combine results like this
    ///
    /// precision: `Precision`, precision for blur, default float
    ///
    /// layers: `i32`, number of layers for approximation
    ///
    /// cluster: `i32`, cluster lines closer than this distance
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

    /// Perform a complex operation on an image
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

    /// Complex binary operations on two images
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

    /// Form a complex image from two real images
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

    /// Get a component from a complex image
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

    /// Blend an array of images with an array of blend modes
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

    /// Blend an array of images with an array of blend modes
    ///
    /// Optional arguments
    ///
    /// compositing_space: `Interpretation` to composite in
    ///
    /// premultiplied: `bool`, images are already premultiplied
    ///
    /// x: `&[i32]`, array of (n - 1) x coordinates
    ///
    /// y: `&[i32]`, array of (n - 1) y coordinates
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

    /// Blend a pair of images with a blend mode
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

    /// Blend a pair of images with a blend mode
    ///
    /// Optional arguments
    ///
    /// compositing_space: `Interpretation` to composite in
    ///
    /// premultiplied: `bool`, images are already premultiplied
    ///
    /// x: `i32`, position of overlay
    ///
    /// y: `i32`, position of overlay.
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

    /// Convolution operation
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

    /// Convolution operation
    ///
    /// Optional arguments
    ///
    /// precision: `Precision`, calculation accuracy
    ///
    /// layers: `i32`, number of layers for approximation
    ///
    /// cluster: `i32`, cluster lines closer than this distance
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

    /// Approximate integer convolution
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

    /// Approximate integer convolution
    ///
    /// Optional arguments
    ///
    /// layers: `i32`, number of layers for approximation
    ///
    /// cluster: `i32`, cluster lines closer than this distance
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

    /// Approximate separable integer convolution
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

    /// Approximate separable integer convolution
    ///
    /// Optional arguments
    ///
    /// layers: `i32`, number of layers for approximation
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

    /// Float convolution operation
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
    /// Int convolution operation
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

    /// Separable convolution operation
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

    /// Separable convolution operation
    ///
    /// Optional arguments
    ///
    /// precision: `Precision`, calculation accuracy
    ///
    /// layers: `i32`, number of layers for approximation
    ///
    /// cluster: `i32`, cluster lines closer than this distance
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

    /// Copy an image
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

    /// Copy an image
    ///
    /// Optional arguments
    ///
    /// width: `i32`, set image width
    ///
    /// height: `i32`, set image height
    ///
    /// bands: `i32`, set image bands
    ///
    /// format: `BandFormat`, set image format
    ///
    /// coding: `Coding`, set image coding
    ///
    /// interpretation: `Interpretation`, set image interpretation
    ///
    /// xres: `f64`, set image xres
    ///
    /// yres: `f64`, set image yres
    ///
    /// xoffset: `i32`, set image xoffset
    ///
    /// yoffset: `i32`, set image yoffset
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

    /// Count lines in an image
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

    /// A synonym for vips_extract_area()
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

    /// Load a CSV (comma-separated values) file.
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

    /// Load a CSV (comma-separated values) file.
    ///
    /// Optional arguments
    ///
    /// skip: `i32`, skip this many lines at start of file
    ///
    /// lines: `i32`, read this many lines from file
    ///
    /// whitespace: `&str`, set of whitespace characters
    ///
    /// separator: `&str`, set of separator characters
    ///
    /// fail_on: `FailOn`, types of read error to fail on
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

    /// Exactly as vips_csvload(), but read from a source.
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

    /// Exactly as vips_csvload(), but read from a source.
    ///
    /// Optional arguments
    ///
    /// skip: `i32`, skip this many lines at start of file
    ///
    /// lines: `i32`, read this many lines from file
    ///
    /// whitespace: &str, set of whitespace characters
    ///
    /// separator: &str, set of separator characters
    ///
    /// fail_on: FailOn, types of read error to fail on
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

    /// Writes the pixels in in to the filename as CSV (comma-separated values).
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

    /// Writes the pixels in in to the filename as CSV (comma-separated values).
    ///
    /// Optional arguments
    ///
    /// separator: `&str`, separator string
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

    /// As vips_csvsave(), but save to a target.
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

    /// As vips_csvsave(), but save to a target.
    ///
    /// Optional arguments
    ///
    /// separator: `&str`, separator string
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

    /// Calculate dE00
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

    /// Calculate dE76
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

    /// Calculate dECMC
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

    /// Find image standard deviation
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

    /// Divide two images
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

    /// Draw a circle on an image
    ///
    /// ink: `&[f64]` -> Color for pixels
    ///
    /// cx: `i32` -> Centre of draw_circle
    ///
    /// cy: `i32` -> Centre of draw_circle
    ///
    /// radius: `i32` -> Radius in pixels
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

    /// Draw a circle on an image
    ///
    /// ink: `&[f64]` -> Color for pixels
    ///
    /// cx: `i32` -> Centre of draw_circle
    ///
    /// cy: `i32` -> Centre of draw_circle
    ///
    /// radius: `i32` -> Radius in pixels
    ///
    /// Optional arguments
    ///
    /// fill: `bool`, fill the draw_circle
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

    /// Flood-fill an area
    ///
    /// ink: `&[f64]` -> Color for pixels
    ///
    /// x: `i32` -> DrawFlood start point
    ///
    /// y: `i32` -> DrawFlood start point
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

    /// Flood-fill an area
    ///
    /// ink: `&[f64]` -> Color for pixels
    ///
    /// x: `i32` -> DrawFlood start point
    ///
    /// y: `i32` -> DrawFlood start point
    ///
    /// Optional arguments
    ///
    /// test: `&VipsImage`, test this image
    ///
    /// equal: `bool`, fill while equal to edge
    ///
    /// left: `i32`, output left edge of bounding box of modified area
    ///
    /// top: `i32`, output top edge of bounding box of modified area
    ///
    /// width: `i32`, output width of bounding box of modified area
    ///
    /// height: `i32`, output height of bounding box of modified area
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

    /// Paint an image into another image
    ///
    /// sub: `&VipsImage` -> Sub-image to insert into main image
    ///
    /// x: `i32` -> Draw image here
    ///
    /// y: `i32` -> Draw image here
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

    /// Paint an image into another image
    ///
    /// sub: `&VipsImage` -> Sub-image to insert into main image
    ///
    /// x: `i32` -> Draw image here
    ///
    /// y: `i32` -> Draw image here
    ///
    /// Optional arguments
    ///
    /// mode: `CombineMode`, how to combine pixels
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

    /// Draw a line on an image
    ///
    /// ink: `&[f64]` -> Color for pixels
    ///
    /// x1: `i32` -> Start of draw_line
    ///
    /// y1: `i32` -> Start of draw_line
    ///
    /// x2: `i32` -> End of draw_line
    ///
    /// y2: `i32` -> End of draw_line
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

    /// Draw a mask on an image
    ///
    /// ink: `&[f64]` -> Color for pixels
    ///
    /// mask: `&VipsImage` -> Mask of pixels to draw
    ///
    /// x: `i32` -> Draw mask here
    ///
    /// y: `i32` -> Draw mask here
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

    /// Paint a rectangle on an image
    ///
    /// ink: `&[f64]` -> Color for pixels
    ///
    /// left: `i32` -> Rect to fill
    ///
    /// top: `i32` -> Rect to fill
    ///
    /// width: `i32` -> Rect to fill
    ///
    /// height: `i32` -> Rect to fill
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

    /// Paint a rectangle on an image
    ///
    /// ink: `&[f64]` -> Color for pixels
    ///
    /// left: `i32` -> Rect to fill
    ///
    /// top: `i32` -> Rect to fill
    ///
    /// width: `i32` -> Rect to fill
    ///
    /// height: `i32` -> Rect to fill
    ///
    /// Optional arguments
    ///
    /// fill: `bool`, fill the rect
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

    /// Blur a rectangle on an image
    ///
    /// left: `i32` -> Rect to fill
    ///
    /// top: `i32` -> Rect to fill
    ///
    /// width: `i32` -> Rect to fill
    ///
    /// height: `i32` -> Rect to fill
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

    /// Save an image as a set of tiles at various resolutions.
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

    /// Save an image as a set of tiles at various resolutions.
    ///
    /// Optional arguments
    ///
    /// basename: `&str`, base part of name
    ///
    /// layout: `ForeignDzLayout`, directory layout convention
    ///
    /// suffix: `&str`, suffix for tiles
    ///
    /// overlap: `i32`, set tile overlap
    ///
    /// tile_size: `i32`, set tile size
    ///
    /// background: `&[f64]`, background colour
    ///
    /// depth: `ForeignDzDepth`, how deep to make the pyramid
    ///
    /// centre: `bool`, centre the tiles
    ///
    /// angle: `Angle`, rotate the image by this much
    ///
    /// container: `ForeignDzContainer`, set container type
    ///
    /// compression: `i32`, zip deflate compression level
    ///
    /// region_shrink: `RegionShrink`, how to shrink each 2x2 region
    ///
    /// skip_blanks: `i32`, skip tiles which are nearly equal to the background
    ///
    /// id: `&str`, id for IIIF properties
    ///
    /// Q: `i32`, quality factor
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

    /// As vips_dzsave(), but save to a memory buffer.
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

    /// As vips_dzsave(), but save to a memory buffer.
    ///
    /// Optional arguments
    ///
    /// basename: `&str`, base part of name
    ///
    /// layout: `ForeignDzLayout`, directory layout convention
    ///
    /// suffix: `&str`, suffix for tiles
    ///
    /// overlap: `i32`, set tile overlap
    ///
    /// tile_size: `i32`, set tile size
    ///
    /// background: `&[f64]`, background colour
    ///
    /// depth: `ForeignDzDepth`, how deep to make the pyramid
    ///
    /// centre: `bool`, centre the tiles
    ///
    /// angle: `Angle`, rotate the image by this much
    ///
    /// container: `ForeignDzContainer`, set container type
    ///
    /// compression: `i32`, zip deflate compression level
    ///
    /// region_shrink: `RegionShrink`, how to shrink each 2x2 region
    ///
    /// skip_blanks: `i32`, skip tiles which are nearly equal to the background
    ///
    /// id: `&str`, id for IIIF properties
    ///
    /// Q: `i32`, quality factor
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

    /// As vips_dzsave(), but save to a target.
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

    /// As vips_dzsave(), but save to a target.
    ///
    /// Optional arguments
    ///
    /// basename: `&str`, base part of name
    ///
    /// layout: `ForeignDzLayout`, directory layout convention
    ///
    /// suffix: `&str`, suffix for tiles
    ///
    /// overlap: `i32`, set tile overlap
    ///
    /// tile_size: `i32`, set tile size
    ///
    /// background: `&[f64]`, background colour
    ///
    /// depth: `ForeignDzDepth`, how deep to make the pyramid
    ///
    /// centre: `bool`, centre the tiles
    ///
    /// angle: `Angle`, rotate the image by this much
    ///
    /// container: `ForeignDzContainer`, set container type
    ///
    /// compression: `i32`, zip deflate compression level
    ///
    /// region_shrink: `RegionShrink`, how to shrink each 2x2 region
    ///
    /// skip_blanks: `i32`, skip tiles which are nearly equal to the background
    ///
    /// id: `&str`, id for IIIF properties
    ///
    /// Q: `i32`, quality factor
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

    /// Embed an image in a larger image
    ///
    /// x: `i32` -> Left edge of input in output
    ///
    /// y: `i32` -> Top edge of input in output
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
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

    /// Embed an image in a larger image
    ///
    /// x: `i32` -> Left edge of input in output
    ///
    /// y: `i32` -> Top edge of input in output
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// Optional arguments
    ///
    /// extend: `Extend` to generate the edge pixels (default: VIPS_EXTEND_BLACK)
    ///
    /// background: `&[f64]` colour for edge pixels
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

    /// Extract an area from an image
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

    /// Extract band from an image
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

    /// Extract band from an image
    ///
    /// Optional arguments
    ///
    /// n: `i32`, number of bands to extract
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

    /// Make an image showing the eye's spatial response
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

    /// Make an image showing the eye's spatial response
    ///
    /// Optional arguments
    ///
    /// factor: `f64`, maximum spatial frequency
    ///
    /// uchar: `bool`, output a uchar image
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

    /// False-color an image
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

    /// Fast correlation
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

    /// Fill image zeros with nearest non-zero pixel
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

    /// Fill image zeros with nearest non-zero pixel
    ///
    /// Optional arguments
    ///
    /// distance: `&mut VipsImage`, output, image of distance to nearest non-zero pixel
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

    /// Search an image for non-edge areas
    ///
    /// Tuple (
    ///
    /// i32 - Left edge of image
    ///
    /// i32 - Top edge of extract area
    ///
    /// i32 - Width of extract area
    ///
    /// i32 - Height of extract area
    ///
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
            let mut left_out: i32 = 0;
            let mut top_out: i32 = 0;
            let mut width_out: i32 = 0;
            let mut height_out: i32 = 0;

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

    /// Search an image for non-edge areas
    ///
    /// Tuple (
    ///
    /// i32 - Left edge of image
    ///
    /// i32 - Top edge of extract area
    ///
    /// i32 - Width of extract area
    ///
    /// i32 - Height of extract area
    ///
    ///)
    ///
    /// Optional arguments
    ///
    /// threshold: `f64`, background / object threshold
    ///
    /// background: `&[f64]`, background colour
    ///
    /// line_art: `bool`, enable line art mode
    pub fn find_trim_with_opts(
        &self,
        option: VOption,
    ) -> Result<(
        i32,
        i32,
        i32,
        i32,
    )> {
        let mut left: i32 = 0;
        let mut top: i32 = 0;
        let mut width: i32 = 0;
        let mut height: i32 = 0;

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

    /// Read a FITS image file into a VIPS image.
    pub fn fitsload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "fitsload",
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
            Error::OperationError("fitsloade failed"),
        )
    }

    /// Read a FITS image file into a VIPS image.
    pub fn fitsload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
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

    /// Exactly as vips_fitsload(), but read from a source.
    pub fn fitsload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "fitsload_source",
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
            Error::OperationError("fitsload_source failed"),
        )
    }

    /// Exactly as vips_fitsload(), but read from a source.
    pub fn fitsload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
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

    /// Write a VIPS image to a file in FITS format.
    pub fn fitssave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "fitssave",
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
            Error::OperationError("fitssave failed"),
        )
    }

    /// Write a VIPS image to a file in FITS format.
    pub fn fitssave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
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

    /// Flatten alpha out of an image
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

    /// Flatten alpha out of an image
    ///
    /// Optional arguments
    ///
    /// background: `&[f64]` colour for new pixels
    ///
    /// max_alpha: `f64`, maximum value for alpha
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

    /// Flip an image
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

    /// Transform float RGB to Radiance coding
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

    /// Make a fractal surface
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

    /// Frequency-domain filtering
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

    /// Forward FFT
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

    /// Gamma an image
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

    /// Gamma an image
    ///
    /// Optional arguments
    ///
    /// exponent: `f64`, gamma, default 1.0 / 2.4
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

    /// Gaussian blur
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

    /// Gaussian blur
    ///
    /// Optional arguments
    ///
    /// precision: `Precision`, precision for blur, default int
    ///
    /// min_ampl: `f64`, minimum amplitude, default 0.2
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

    /// Make a gaussian image
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

    /// Make a gaussian image
    ///
    /// Optional arguments
    ///
    /// precision: `Precision`, precision for blur, default int
    ///
    /// separable: `bool`, generate a separable gaussian
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

    /// Make a gaussnoise image
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

    /// Make a gaussnoise image
    ///
    /// Optional arguments
    ///
    /// mean: `f64`, mean of generated pixels
    ///
    /// sigma: `f64`, standard deviation of generated pixels
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

    /// Read a point from an image
    pub fn getpoint(&self, x: i32, y: i32) -> Result<Vec<f64>> {
        unsafe {
            let inp_in: *mut bindings::VipsImage = self.ctx;
            let mut out_array_size: i32 = 0;
            let mut out_array: *mut f64 = null_mut();

            let vips_op_response = bindings::vips_getpoint(
                inp_in,
                &mut out_array,
                &mut out_array_size,
                x,
                y,
                NULL,
            );
            utils::result(
                vips_op_response,
                utils::new_double_array(
                    out_array,
                    out_array_size
                        .try_into()
                        .unwrap(),
                ),
                Error::OperationError("Gaussnoise failed"),
            )
        }
    }

    /// Read a GIF file into a libvips image.
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

    /// Read a GIF file into a libvips image.
    ///
    /// Optional arguments
    ///
    /// page: `i32`, page (frame) to read
    ///
    /// n: `i32`, load this many pages
    ///
    /// fail_on: `FailOn`, types of read error to fail on
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

    /// Exactly as vips_gifload(), but read from a memory buffer.
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

    /// Exactly as vips_gifload(), but read from a memory buffer.
    ///
    /// Optional arguments
    ///
    /// page: `i32`, page (frame) to read
    ///
    /// n: `i32`, load this many pages
    ///
    /// fail_on: `FailOn`, types of read error to fail on
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

    /// Exactly as vips_gifload(), but read from a source.
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

    /// Exactly as vips_gifload(), but read from a source.
    ///
    /// Optional arguments
    ///
    /// page: `i32`, page (frame) to read
    ///
    /// n: `i32`, load this many pages
    ///
    /// fail_on: `FailOn`, types of read error to fail on
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

    /// Write to a file in GIF format.
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

    /// Write to a file in GIF format.
    ///
    /// Optional arguments
    ///
    /// dither: `f64`, quantisation dithering level
    ///
    /// effort: `i32`, quantisation CPU effort
    ///
    /// bitdepth: `i32`, number of bits per pixel
    ///
    /// interframe_maxerror: `f64`, maximum inter-frame error for transparency
    ///
    /// reuse: `bool`, reuse palette from input
    ///
    /// interlace: `bool`, write an interlaced (progressive) GIF
    ///
    /// interpalette_maxerror: `f64`, maximum inter-palette error for palette reusage
    ///
    /// keep_duplicate_frames: `bool`, keep duplicate frames in the output instead of combining them
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

    /// As vips_gifsave(), but save to a memory buffer.
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

    /// As vips_gifsave(), but save to a memory buffer.
    ///
    /// Optional arguments
    ///
    /// dither: `f64`, quantisation dithering level
    ///
    /// effort: `i32`, quantisation CPU effort
    ///
    /// bitdepth: `i32`, number of bits per pixel
    ///
    /// interframe_maxerror: `f64`, maximum inter-frame error for transparency
    ///
    /// reuse: `bool`, reuse palette from input
    ///
    /// interlace: `bool`, write an interlaced (progressive) GIF
    ///
    /// interpalette_maxerror: `f64`, maximum inter-palette error for palette reusage
    ///
    /// keep_duplicate_frames: `bool`, keep duplicate frames in the output instead of combining them
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

    /// As vips_gifsave(), but save to a target.
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

    /// As vips_gifsave(), but save to a target.
    ///
    /// Optional arguments
    ///
    /// dither: `f64`, quantisation dithering level
    ///
    /// effort: `i32`, quantisation CPU effort
    ///
    /// bitdepth: `i32`, number of bits per pixel
    ///
    /// interframe_maxerror: `f64`, maximum inter-frame error for transparency
    ///
    /// reuse: `bool`, reuse palette from input
    ///
    /// interlace: `bool`, write an interlaced (progressive) GIF
    ///
    /// interpalette_maxerror: `f64`, maximum inter-palette error for palette reusage
    ///
    /// keep_duplicate_frames: `bool`, keep duplicate frames in the output instead of combining them
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

    /// Global balance an image mosaic
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

    /// Global balance an image mosaic
    ///
    /// Optional arguments
    ///
    /// gamma: `f64`, gamma of source images
    ///
    /// int_output: `bool`, TRUE for integer image output
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

    /// Place an image within a larger image with a certain gravity
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

    /// Place an image within a larger image with a certain gravity
    ///
    /// Optional arguments
    ///
    /// extend: `Extend` to generate the edge pixels (default: VIPS_EXTEND_BLACK)
    ///
    /// background: `&[f64]` colour for edge pixels
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

    /// Make a grey ramp image
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

    /// Make a grey ramp image
    ///
    /// Optional arguments
    ///
    /// uchar: `bool`, output a uchar image
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

    /// Grid an image
    ///
    /// tile_height: `i32` -> Chop into tiles this high
    ///
    /// across: `i32` -> Number of tiles across
    ///
    /// down: `i32` -> Number of tiles down
    ///
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

    /// Read a HEIF image file into a VIPS image.
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

    /// Read a HEIF image file into a VIPS image.
    ///
    /// Optional arguments
    ///
    /// page: `i32`, page (top-level image number) to read
    ///
    /// n: `i32`, load this many pages
    ///
    /// thumbnail: `bool`, fetch thumbnail instead of image
    ///
    /// unlimited: `bool`, remove all denial of service limits
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

    /// Read a HEIF image file into a VIPS image from a memory buffer.
    /// buffer: `&[u8]` -> Buffer to load from
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

    /// Read a HEIF image file into a VIPS image from a memory buffer.
    ///
    /// Optional arguments
    ///
    /// page: `i32`, page (top-level image number) to read
    ///
    /// n: `i32`, load this many pages
    ///
    /// thumbnail: `bool`, fetch thumbnail instead of image
    ///
    /// unlimited: `bool`, remove all denial of service limits
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

    /// Exactly as vips_heifload(), but read from a source.
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

    /// Exactly as vips_heifload(), but read from a source.
    ///
    /// Optional arguments
    ///
    /// page: `i32`, page (top-level image number) to read
    ///
    /// n: `i32`, load this many pages
    ///
    /// thumbnail: `bool`, fetch thumbnail instead of image
    ///
    /// unlimited: `bool`, remove all denial of service limits
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

    /// Write a VIPS image to a file in HEIF format.
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

    /// Write a VIPS image to a file in HEIF format.
    ///
    /// Optional arguments
    ///
    /// Q: `i32`, quality factor
    ///
    /// bitdepth: `i32`, set write bit depth to 8, 10, or 12 bits
    ///
    /// lossless: `bool`, enable lossless encoding
    ///
    /// compression: `ForeignHeifCompression`, write with this compression
    ///
    /// effort: `i32`, encoding effort
    ///
    /// subsample_mode: `ForeignSubsample`, chroma subsampling mode
    ///
    /// encoder: `ForeignHeifEncoder`, select encoder to use
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

    /// As vips_heifsave(), but save to a memory buffer.
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

    /// As vips_heifsave(), but save to a memory buffer.
    ///
    /// Optional arguments
    ///
    /// Q: `i32`, quality factor
    ///
    /// bitdepth: `i32`, set write bit depth to 8, 10, or 12 bits
    ///
    /// lossless: `bool`, enable lossless encoding
    ///
    /// compression: `ForeignHeifCompression`, write with this compression
    ///
    /// effort: `i32`, encoding effort
    ///
    /// subsample_mode: `ForeignSubsample`, chroma subsampling mode
    ///
    /// encoder: `ForeignHeifEncoder`, select encoder to use
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

    /// As vips_heifsave(), but save to a target.
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

    /// As vips_heifsave(), but save to a target.
    ///
    /// Optional arguments
    ///
    /// Q: `i32`, quality factor
    ///
    /// bitdepth: `i32`, set write bit depth to 8, 10, or 12 bits
    ///
    /// lossless: `bool`, enable lossless encoding
    ///
    /// compression: `ForeignHeifCompression`, write with this compression
    ///
    /// effort: `i32`, encoding effort
    ///
    /// subsample_mode: `ForeignSubsample`, chroma subsampling mode
    ///
    /// encoder: `ForeignHeifEncoder`, select encoder to use
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

    /// Form cumulative histogram
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

    /// Estimate image entropy
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

    /// Histogram equalisation
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

    /// Histogram equalisation
    ///
    /// Optional arguments
    ///
    /// band: `i32`, band to equalise.
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

    /// Find image histogram
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

    /// Find image histogram
    ///
    /// Optional arguments
    ///
    /// band: `i32`, band to equalise
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

    /// Find indexed image histogram
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

    /// Find indexed image histogram
    ///
    /// Optional arguments
    ///
    /// combine: `Combine`, combine bins like this
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

    /// Find n-dimensional image histogram
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

    /// Find n-dimensional image histogram
    ///
    /// Optional arguments
    ///
    /// bins: `i32`, number of bins to make on each axis
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

    /// Test for monotonicity
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

    /// Local histogram equalisation
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

    /// Local histogram equalisation
    ///
    /// Optional arguments
    ///
    /// max_slope: `i32`, maximum brightening
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

    /// Match two histograms
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

    /// Normalise histogram
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

    /// Plot histogram
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

    /// Find hough circle transform
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

    /// Find hough circle transform
    ///
    /// Optional arguments
    ///
    /// scale: `i32`, scale down dimensions by this much
    ///
    /// min_radius: `i32`, smallest radius to search for
    ///
    /// max_radius: `i32`, largest radius to search for
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

    /// Find hough line transform
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

    /// Find hough line transform
    ///
    /// Optional arguments
    ///
    /// width: `i32`, horizontal size of parameter space
    ///
    /// height: `i32`, vertical size of parameter space
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

    /// Output to device with ICC profile
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

    /// Output to device with ICC profile
    ///
    /// Optional arguments
    ///
    /// pcs: `PCS`, use XYZ or LAB PCS
    ///
    /// intent: `Intent`, transform with this intent
    ///
    /// black_point_compensation: `bool`, enable black point compensation
    ///
    /// output_profile: `&str`, get the output profile from here
    ///
    /// depth: `i32`, depth of output image in bits.
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

    /// Import from device with ICC profile
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

    /// Import from device with ICC profile
    ///
    /// Optional arguments
    ///
    /// pcs: `PCS`, use XYZ or LAB PCS
    ///
    /// intent: `Intent`, transform with this intent
    ///
    /// black_point_compensation: `bool`, enable black point compensation
    ///
    /// embedded: `bool`, use profile embedded in input image
    ///
    /// input_profile: `&str`, get the input profile from here.
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

    /// Transform between devices with ICC profiles
    ///
    /// output_profile: `&str` -> Filename to load output profile from
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

    /// Transform between devices with ICC profiles
    ///
    /// output_profile: `&str` -> Filename to load output profile from
    ///
    /// Optional arguments
    ///
    /// pcs: `PCS`, use XYZ or LAB PCS
    ///
    /// intent: `Intent`, transform with this intent
    ///
    /// black_point_compensation: `bool`, enable black point compensation
    ///
    /// embedded: `bool`, use profile embedded in input image
    ///
    /// input_profile: `&str`, get the input profile from here
    ///
    /// depth: `i32`, depth of output image in bits.
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

    /// Make a 1D image where pixel values are indexes
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

    /// Make a 1D image where pixel values are indexes
    ///
    /// Optional arguments
    ///
    /// bands: `i32`, number of bands to create
    ///
    /// ushort: `bool`, TRUE for an unsigned short identity
    ///
    /// size: `i32`, number of LUT elements for a ushort image
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

    /// Ifthenelse an image
    ///
    /// in1: `&VipsImage` -> Source for TRUE pixels
    ///
    /// in2: `&VipsImage` -> Source for FALSE pixels
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

    /// Ifthenelse an image
    ///
    /// in1: `&VipsImage` -> Source for TRUE pixels
    ///
    /// in2: `&VipsImage` -> Source for FALSE pixels
    ///
    /// Optional arguments
    ///
    /// blend: `bool`, blend smoothly between in1 and in2
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

    /// Insert sub into this image at position x, y.
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

    /// Insert sub into this image at position x, y.
    ///
    /// Optional arguments
    ///
    /// expand: `f64`, expand output to hold whole of both images
    ///
    /// background: `&[f64]`, colour for new pixels
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

    /// Invert an image
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

    /// Build an inverted look-up table
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

    /// Build an inverted look-up table
    ///
    /// Optional arguments
    ///
    /// size: `i32`, generate this much
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

    /// Inverse FFT
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

    /// Inverse FFT
    ///
    /// Optional arguments
    ///
    /// real: `bool`, only output the real part
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

    /// Join a pair of images
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

    /// Join a pair of images
    ///
    /// Optional arguments
    ///
    /// expand: `bool`, TRUE to expand the output image to hold all of the input pixels
    ///
    /// shim: `i32`, space between images, in pixels
    ///
    /// background: `&[f64]`, background ink colour
    ///
    /// align: `Align`, low, centre or high alignment
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

    /// Read a JPEG2000 image.
    pub fn jp2kload(&self, filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "jp2kload",
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
            Error::OperationError("jp2kload failed"),
        )
    }

    /// Read a JPEG2000 image.
    ///
    /// Optional arguments
    ///
    /// page: `i32`, load this page
    ///
    /// oneshot: `bool`, load pages in one-shot mode
    ///
    /// fail_on: `FailOn`, types of read error to fail on
    pub fn jp2kload_with_opts(&self, filename: &str, option: VOption) -> Result<VipsImage> {
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

    /// Exactly as vips_jp2kload(), but read from a buffer.
    pub fn jp2kload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "jp2kload_buffer",
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
            Error::OperationError("jp2kload_buffer failed"),
        )
    }

    /// Exactly as vips_jp2kload(), but read from a buffer.
    ///
    /// Optional arguments
    ///
    /// page: `i32`, load this page
    ///
    /// oneshot: `bool`, load pages in one-shot mode
    ///
    /// fail_on: `FailOn`, types of read error to fail on
    pub fn jp2kload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
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

    /// Exactly as vips_jp2kload(), but read from a source.
    pub fn jp2kload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "jp2kload_source",
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
            Error::OperationError("jp2kload_source failed"),
        )
    }

    /// Exactly as vips_jp2kload(), but read from a source.
    ///
    /// Optional arguments
    ///
    /// page: `i32`, load this page
    ///
    /// oneshot: `bool`, load pages in one-shot mode
    ///
    /// fail_on: `FailOn`, types of read error to fail on
    pub fn jp2kload_source_with_opts(source: &VipsSource, options: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "jp2kload_source",
            options
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

    /// Write a VIPS image to a file in JPEG2000 format.
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

    /// Write a VIPS image to a file in JPEG2000 format.
    ///
    /// Optional arguments
    ///
    /// Q: `i32`, quality factor
    ///
    /// lossless: `bool`, enables lossless compression
    ///
    /// tile_width: `i32`, tile width
    ///
    /// tile_height: `i32`, tile width
    ///
    /// subsample_mode: `ForeignSubsample`, chroma subsampling mode
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
            Error::OperationError("jp2ksave failed"),
        )
    }

    /// Save jp2 image to buffer
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

    /// Save jp2 image to buffer
    ///
    /// Optional arguments
    ///
    /// Q: `i32`, quality factor
    ///
    /// lossless: `bool`, enables lossless compression
    ///
    /// tile_width: `i32`, tile width
    ///
    /// tile_height: `i32`, tile width
    ///
    /// subsample_mode: `ForeignSubsample`, chroma subsampling mode
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
            Error::OperationError("jp2ksave_buffer failed"),
        )
    }

    /// As vips_jp2ksave(), but save to a target.
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

    /// As vips_jp2ksave(), but save to a target.
    ///
    /// Optional arguments
    ///
    /// Q: `i32`, quality factor
    ///
    /// lossless: `bool`, enables lossless compression
    ///
    /// tile_width: `i32`, tile width
    ///
    /// tile_height: `i32`, tile width
    ///
    /// subsample_mode: `ForeignSubsample`, chroma subsampling mode
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
            Error::OperationError("jp2ksave_target failed"),
        )
    }

    /// Read a JPEG file into a VIPS image.
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

    /// Read a JPEG file into a VIPS image.
    ///
    /// Optional arguments
    ///
    /// shrink: `i32`, shrink by this much on load
    ///
    /// fail_on: `FailOn`, types of read error to fail on
    ///
    /// autorotate: `bool`, use exif Orientation tag to rotate the image during load
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

    /// Read a JPEG-formatted memory block into a VIPS image.
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

    /// Read a JPEG-formatted memory block into a VIPS image.
    ///
    /// Optional arguments
    ///
    /// shrink: `i32`, shrink by this much on load
    ///
    /// fail_on: `FailOn`, types of read error to fail on
    ///
    /// autorotate: `bool`, use exif Orientation tag to rotate the image during load
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

    /// Read a JPEG-formatted memory block into a VIPS image from source.
    pub fn jpegload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "jpegload_source",
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
            Error::OperationError("jpegload_source failed"),
        )
    }

    /// Read a JPEG-formatted memory block into a VIPS image from source.
    ///
    /// Optional arguments
    ///
    /// shrink: `i32`, shrink by this much on load
    ///
    /// fail_on: `FailOn`, types of read error to fail on
    ///
    /// autorotate: `bool`, use exif Orientation tag to rotate the image during load
    pub fn jpegload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
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

    /// Write a VIPS image to a file as JPEG.
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

    /// Write a VIPS image to a file as JPEG.
    ///
    /// Optional arguments
    ///
    /// Q: `i32`, quality factor
    ///
    /// optimize_coding: `bool`, compute optimal Huffman coding tables
    ///
    /// interlace: `bool`, write an interlaced (progressive) jpeg
    ///
    /// subsample_mode: `ForeignSubsample`, chroma subsampling mode
    ///
    /// trellis_quant: `bool`, apply trellis quantisation to each 8x8 block
    ///
    /// overshoot_deringing: `bool`, overshoot samples with extreme values
    ///
    /// optimize_scans: `bool`, split DCT coefficients into separate scans
    ///
    /// quant_table: `i32`, quantization table index
    ///
    /// restart_interval: `i32`, restart interval in mcu
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

    /// As vips_jpegsave(), but save to a memory buffer.
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

    /// As vips_jpegsave(), but save to a memory buffer.
    ///
    /// Optional arguments
    ///
    /// Q: `i32`, quality factor
    ///
    /// optimize_coding: `bool`, compute optimal Huffman coding tables
    ///
    /// interlace: `bool`, write an interlaced (progressive) jpeg
    ///
    /// subsample_mode: `ForeignSubsample`, chroma subsampling mode
    ///
    /// trellis_quant: `bool`, apply trellis quantisation to each 8x8 block
    ///
    /// overshoot_deringing: `bool`, overshoot samples with extreme values
    ///
    /// optimize_scans: `bool`, split DCT coefficients into separate scans
    ///
    /// quant_table: `i32`, quantization table index
    ///
    /// restart_interval: `i32`, restart interval in mcu
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

    /// As vips_jpegsave(), but save as a mime jpeg on stdout.
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

    /// As vips_jpegsave(), but save as a mime jpeg on stdout.
    ///
    /// Optional arguments
    ///
    /// Q: `i32`, quality factor
    ///
    /// optimize_coding: `bool`, compute optimal Huffman coding tables
    ///
    /// interlace: `bool`, write an interlaced (progressive) jpeg
    ///
    /// subsample_mode: `ForeignSubsample`, chroma subsampling mode
    ///
    /// trellis_quant: `bool`, apply trellis quantisation to each 8x8 block
    ///
    /// overshoot_deringing: `bool`, overshoot samples with extreme values
    ///
    /// optimize_scans: `bool`, split DCT coefficients into separate scans
    ///
    /// quant_table: `i32`, quantization table index
    ///
    /// restart_interval: `i32`, restart interval in mcu
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

    /// As vips_jpegsave(), but save to a target.
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

    /// As vips_jpegsave(), but save to a target.
    ///
    /// Optional arguments
    ///
    /// Q: `i32`, quality factor
    ///
    /// optimize_coding: `bool`, compute optimal Huffman coding tables
    ///
    /// interlace: `bool`, write an interlaced (progressive) jpeg
    ///
    /// subsample_mode: `ForeignSubsample`, chroma subsampling mode
    ///
    /// trellis_quant: `bool`, apply trellis quantisation to each 8x8 block
    ///
    /// overshoot_deringing: `bool`, overshoot samples with extreme values
    ///
    /// optimize_scans: `bool`, split DCT coefficients into separate scans
    ///
    /// quant_table: `i32`, quantization table index
    ///
    /// restart_interval: `i32`, restart interval in mcu
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

    /// Load jxl from file
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

    /// Load jxl from file
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
            Error::OperationError("jxlload failed"),
        )
    }

    /// Load jxl from buffer
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

    /// Load jxl from buffer
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
            Error::OperationError("jxlload_buffer failed"),
        )
    }

    /// Load jxl from source
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

    /// Load jxl from source
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
            Error::OperationError("jxlload_source failed"),
        )
    }

    /// Save jxl image to file
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

    /// Save jxl image to file
    ///
    /// Optional arguments
    ///
    /// tier: `i32`, decode speed tier
    ///
    /// distance: `f64`, maximum encoding error
    ///
    /// effort: `i32`, encoding effort
    ///
    /// lossless: `bool`, enables lossless compression
    ///
    /// Q: `i32`, quality setting.
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

    /// Save jxl image to buffer
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

    /// Save jxl image to buffer
    ///
    /// Optional arguments
    ///
    /// tier: `i32`, decode speed tier
    ///
    /// distance: `f64`, maximum encoding error
    ///
    /// effort: `i32`, encoding effort
    ///
    /// lossless: `bool`, enables lossless compression
    ///
    /// Q: `i32`, quality setting.
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
            Error::OperationError("jxlsave_buffer failed"),
        )
    }

    /// Save jxl image to target
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

    /// Save jxl image to target
    ///
    /// Optional arguments
    ///
    /// tier: `i32`, decode speed tier
    ///
    /// distance: `f64`, maximum encoding error
    ///
    /// effort: `i32`, encoding effort
    ///
    /// lossless: `bool`, enables lossless compression
    ///
    /// Q: `i32`, quality setting.
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
            Error::OperationError("jxlsave_target failed"),
        )
    }

    /// Label regions in an image
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

    /// Label regions in an image
    ///
    /// Optional arguments
    ///
    /// segments: `i32`, output, number of regions found
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
            Error::OperationError("Labelregion failed"),
        )
    }

    /// Pass an image through a linear transform, ie. (out = in * a + b).
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

    /// Pass an image through a linear transform, ie. (out = in * a + b).
    ///
    /// Optional arguments
    ///
    /// uchar: `bool`, output uchar pixels
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

    /// Cache an image as a set of lines
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

    /// Cache an image as a set of lines
    ///
    /// Optional arguments
    ///
    /// access: `Access`, hint expected access pattern
    ///
    /// tile_height: `i32`, height of tiles in cache
    ///
    /// threaded: `bool`, allow many threads
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

    /// Make a Laplacian of Gaussian image
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

    /// Make a Laplacian of Gaussian image
    ///
    /// Optional arguments
    ///
    /// separable: `bool`, generate a separable mask
    ///
    /// precision: `Precision` for out
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

    /// Resample with a map image
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

    /// Resample with a map image
    ///
    /// Optional arguments
    ///
    /// interpolate: `Interpolate`, interpolate pixels with this
    ///
    /// extend: `Extend`, how to generate new pixels
    ///
    /// background: `&[f64]`, colour for new pixels
    ///
    /// premultiplied: `bool`, images are already premultiplied
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

    /// Map an image though a lut
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

    /// Map an image though a lut
    ///
    /// Optional arguments
    ///
    /// band: `i32`, apply one-band lut to this band of in
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

    /// Make a butterworth filter
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

    /// Make a butterworth filter
    ///
    /// Optional arguments
    ///
    /// nodc: `bool`, don’t set the DC pixel
    ///
    /// reject: `bool`, invert the filter sense
    ///
    /// optical: `bool`, coordinates in optical space
    ///
    /// uchar: `bool`, output a uchar image
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

    /// Make a butterworth_band filter
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

    /// Make a butterworth_band filter
    ///
    /// Optional arguments
    ///
    /// nodc: `bool`, don’t set the DC pixel
    ///
    /// reject: `bool`, invert the filter sense
    ///
    /// optical: `bool`, coordinates in optical space
    ///
    /// uchar: `bool`, output a uchar image
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

    /// Make a butterworth ring filter
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

    /// Make a butterworth ring filter
    ///
    /// Optional arguments
    ///
    /// nodc: `bool`, don’t set the DC pixel
    ///
    /// reject: `bool`, invert the filter sense
    ///
    /// optical: `bool`, coordinates in optical space
    ///
    /// uchar: `bool`, output a uchar image
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

    /// Make fractal filter
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

    /// Make fractal filter
    ///
    /// Optional arguments
    ///
    /// nodc: `bool`, don’t set the DC pixel
    ///
    /// reject: `bool`, invert the filter sense
    ///
    /// optical: `bool`, coordinates in optical space
    ///
    /// uchar: `bool`, output a uchar image
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

    /// Make a gaussian filter
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

    /// Make a gaussian filter
    ///
    /// Optional arguments
    ///
    /// nodc: `bool`, don’t set the DC pixel
    ///
    /// reject: `bool`, invert the filter sense
    ///
    /// optical: `bool`, coordinates in optical space
    ///
    /// uchar: `bool`, output a uchar image
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

    /// Make a gaussian filter
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

    /// Make a gaussian filter
    ///
    /// Optional arguments
    ///
    /// nodc: `bool`, don’t set the DC pixel
    ///
    /// reject: `bool`, invert the filter sense
    ///
    /// optical: `bool`, coordinates in optical space
    ///
    /// uchar: `bool`, output a uchar image
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

    /// Make a gaussian ring filter
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

    /// Make a gaussian ring filter
    ///
    /// Optional arguments
    ///
    /// nodc: `bool`, don’t set the DC pixel
    ///
    /// reject: `bool`, invert the filter sense
    ///
    /// optical: `bool`, coordinates in optical space
    ///
    /// uchar: `bool`, output a uchar image
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

    /// Make an ideal filter
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

    /// Make an ideal filter
    ///
    /// Optional arguments
    ///
    /// nodc: `bool`, don’t set the DC pixel
    ///
    /// reject: `bool`, invert the filter sense
    ///
    /// optical: `bool`, coordinates in optical space
    ///
    /// uchar: `bool`, output a uchar image
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

    /// Make an ideal band filter
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

    /// Make an ideal band filter
    ///
    /// Optional arguments
    ///
    /// nodc: `bool`, don’t set the DC pixel
    ///
    /// reject: `bool`, invert the filter sense
    ///
    /// optical: `bool`, coordinates in optical space
    ///
    /// uchar: `bool`, output a uchar image
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

    /// Make an ideal ring filter
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

    /// Make an ideal ring filter
    ///
    /// Optional arguments
    ///
    /// nodc: `bool`, don’t set the DC pixel
    ///
    /// reject: `bool`, invert the filter sense
    ///
    /// optical: `bool`, coordinates in optical space
    ///
    /// uchar: `bool`, output a uchar image
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

    /// Rotate and translate sec so that the tie-points line up.
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
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "match",
            VOption::new()
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

    /// Rotate and translate sec so that the tie-points line up.
    ///
    /// Optional arguments
    ///
    /// search: `bool`, search to improve tie-points
    ///
    /// hwindow: `i32`, half window size
    ///
    /// harea: `i32`, half search size
    ///
    /// interpolate: `Interpolate`, interpolate pixels with this.
    pub fn match_with_opts(
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

    /// Apply a math operation to an image
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

    /// Binary math operations
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

    /// Binary math operations with a constant
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

    /// Invert an matrix
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

    /// Reads a matrix from a file.
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

    /// Reads a matrix from a file.
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

    /// Exactly as vips_matrixload(), but read from a source.
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

    /// Exactly as vips_matrixload(), but read from a source.
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

    /// Multiplies two matrix images.
    pub fn matrixmultiply(&self, right: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "matrixmultiply",
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
            Error::OperationError("matrixmultiply failed"),
        )
    }

    /// Print in to %stdout in matrix format.
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

    /// Write image to filename in matrix format.
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

    /// As vips_matrixsave(), but save to a target.
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

    /// Find image maximum
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

    /// Find image maximum
    ///
    /// Optional arguments
    ///
    /// x: `i32`, output, horizontal position of maximum
    ///
    /// y: `i32`, output, vertical position of maximum
    ///
    /// size: `i32`, number of maxima to find
    ///
    /// out_array: `&[f64]`, output, array of maximum values
    ///
    /// x_array: `&[i32]`, output, corresponding horizontal positions
    ///
    /// y_array: `&[i32]`. output, corresponding vertical positions
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

    pub fn maxpos(&self) -> Result<(f64, f64)> {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;

        let vips_op_response = call(
            "max",
            VOption::new()
                .with(
                    "in",
                    VipsValue::Image(&VipsImage::from(self.ctx)),
                )
                .with(
                    "x",
                    VipsValue::MutDouble(&mut x),
                )
                .with(
                    "y",
                    VipsValue::MutDouble(&mut y),
                ),
        );
        utils::result(
            vips_op_response,
            (x, y),
            Error::OperationError("maxpos failed"),
        )
    }

    /// Maximum of a pair of images
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

    /// Measure a set of patches on a color chart
    ///
    /// h: `i32` -> Number of patches across chart
    ///
    /// v: `i32` -> Number of patches down chart
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

    /// Measure a set of patches on a color chart
    ///
    /// h: `i32` -> Number of patches across chart
    ///
    /// v: `i32` -> Number of patches down chart
    ///
    /// Optional arguments
    ///
    /// left: `i32`, area of image containing chart
    ///
    /// top: `i32`, area of image containing chart
    ///
    /// width: `i32`, area of image containing chart
    ///
    /// height: `i32`, area of image containing chart
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

    /// A convenience function equivalent to: vips_rank(in, out, size, size, (size * size) / 2);
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
            Error::OperationError("median failed"),
        )
    }

    /// Merge two images
    ///
    /// sec: `&VipsImage` -> Secondary image
    ///
    /// direction: `Direction` -> Horizontal or vertical merge
    ///
    /// dx: `i32` -> Horizontal displacement from sec to image
    ///
    /// dy: `i32` -> Vertical displacement from sec to image
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

    /// Merge two images
    ///
    /// sec: `&VipsImage` -> Secondary image
    ///
    /// direction: `Direction` -> Horizontal or vertical merge
    ///
    /// dx: `i32` -> Horizontal displacement from sec to image
    ///
    /// dy: `i32` -> Vertical displacement from sec to image
    ///
    /// Optional arguments
    ///
    /// mblend: `i32`, maximum blend size
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

    /// Find image minimum
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

    /// Find image minimum
    ///
    /// Optional arguments
    ///
    /// x: `i32`, output, horizontal position of minimum
    ///
    /// y: `i32`, output, vertical position of minimum
    ///
    /// size: `i32`, number of minima to find
    ///
    /// out_array: `&[f64]`, output, array of minimum values
    ///
    /// x_array: `&[i32]`, output, corresponding horizontal positions
    ///
    /// y_array: `&[i32]`, output, corresponding vertical positions
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

    /// Minimum of a pair of images
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

    /// Morphology operation
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

    /// Mosaic two images
    ///
    /// sec: `&VipsImage` -> Secondary image
    ///
    /// direction: `Direction` -> Horizontal or vertical mosaic
    ///
    /// xref: `i32` -> Position of image tie-point
    ///
    /// yref: `i32` -> Position of image tie-point
    ///
    /// xsec: `i32` -> Position of secondary image tie-point
    ///
    /// ysec: `i32` -> Position of secondary image tie-point
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

    /// Mosaic two images
    ///
    /// sec: `&VipsImage` -> Secondary image
    ///
    /// direction: `Direction` -> Horizontal or vertical mosaic
    ///
    /// xref: `i32` -> Position of image tie-point
    ///
    /// yref: `i32` -> Position of image tie-point
    ///
    /// xsec: `i32` -> Position of secondary image tie-point
    ///
    /// ysec: `i32` -> Position of secondary image tie-point
    ///
    /// Optional arguments
    ///
    /// bandno: `i32`, band to search for features
    ///
    /// hwindow: `i32`, half window size
    ///
    /// harea: `i32`, half search size
    ///
    /// mblend: `i32`, maximum blend size
    ///
    /// dx0: `i32`, output, detected displacement
    ///
    /// dy0: `i32`, output, detected displacement
    ///
    /// scale1: `f64`, output, detected first order scale
    ///
    /// angle1: `f64`, output, detected first order rotation
    ///
    /// dx1: `f64`, output, detected first order displacement
    ///
    /// dy1: `f64`, output, detected first order displacement
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

    /// First-order mosaic of two images
    ///
    /// sec: `&VipsImage` -> Secondary image
    ///
    /// direction: `Direction` -> Horizontal or vertical mosaic
    ///
    /// xr1: `i32` -> Position of first image tie-point
    ///
    /// yr1: `i32` -> Position of first image tie-point
    ///
    /// xs1: `i32` -> Position of first secondary tie-point
    ///
    /// ys1: `i32` -> Position of first secondary tie-point
    ///
    /// xr2: `i32` -> Position of second image tie-point
    ///
    /// yr2: `i32` -> Position of second image tie-point
    ///
    /// xs2: `i32` -> Position of second secondary tie-point
    ///
    /// ys2: `i32` -> Position of second secondary tie-point
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

    /// First-order mosaic of two images
    ///
    /// sec: `&VipsImage` -> Secondary image
    ///
    /// direction: `Direction` -> Horizontal or vertical mosaic
    ///
    /// xr1: `i32` -> Position of first image tie-point
    ///
    /// yr1: `i32` -> Position of first image tie-point
    ///
    /// xs1: `i32` -> Position of first secondary tie-point
    ///
    /// ys1: `i32` -> Position of first secondary tie-point
    ///
    /// xr2: `i32` -> Position of second image tie-point
    ///
    /// yr2: `i32` -> Position of second image tie-point
    ///
    /// xs2: `i32` -> Position of second secondary tie-point
    ///
    /// ys2: `i32` -> Position of second secondary tie-point
    ///
    /// Optional arguments
    ///
    /// search: `bool`, search to improve tie-points
    ///
    /// hwindow: `i32`, half window size
    ///
    /// harea: `i32`, half search size
    ///
    /// interpolate: `Interpolate`, interpolate pixels with this
    ///
    /// mblend: `i32`, maximum blend size
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

    /// Pick most-significant byte from an image
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

    /// Pick most-significant byte from an image
    ///
    /// Optional arguments
    ///
    /// band: `i32`, msb just this band
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

    /// Multiply two images
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

    /// Read a OpenEXR file into a VIPS image.
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

    /// Read a OpenEXR file into a VIPS image.
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
            Error::OperationError("openexrload failed"),
        )
    }

    /// Read a virtual slide supported by the OpenSlide library into a VIPS image.
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

    /// Read a virtual slide supported by the OpenSlide library into a VIPS image.
    ///
    /// Optional arguments
    ///
    /// level: `i32`, load this level
    ///
    /// associated: `&str`, load this associated image
    ///
    /// attach_associated: `bool`, attach all associated images as metadata
    ///
    /// autocrop: `bool`, crop to image bounds
    ///
    /// rgb: `bool`, output RGB (not RGBA) pixels
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
            Error::OperationError("openslideload failed"),
        )
    }

    /// Exactly as vips_openslideload(), but read from a source.
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

    /// Exactly as vips_openslideload(), but read from a source.
    ///
    /// Optional arguments
    ///
    /// level: `i32`, load this level
    ///
    /// associated: `&str`, load this associated image
    ///
    /// attach_associated: `bool`, attach all associated images as metadata
    ///
    /// autocrop: `bool`, crop to image bounds
    ///
    /// rgb: `bool`, output RGB (not RGBA) pixels
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
            Error::OperationError("openslideload_source failed"),
        )
    }

    /// Render a PDF file into a VIPS image.
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

    /// Render a PDF file into a VIPS image.
    ///
    /// Optional arguments
    ///
    /// page: `i32`, load this page, numbered from zero
    ///
    /// n: `i32`, load this many pages
    ///
    /// dpi: `f64`, render at this DPI
    ///
    /// scale: `f64`, scale render by this factor
    ///
    /// background: `&[f64]`, background colour
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

    /// Read a PDF-formatted memory buffer into a VIPS image.
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

    /// Read a PDF-formatted memory buffer into a VIPS image.
    ///
    /// Optional arguments
    ///
    /// page: `i32`, load this page, numbered from zero
    ///
    /// n: `i32`, load this many pages
    ///
    /// dpi: `f64`, render at this DPI
    ///
    /// scale: `f64`, scale render by this factor
    ///
    /// background: `&[f64]`, background colour
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

    /// Exactly as vips_pdfload(), but read from a source.
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

    /// Exactly as vips_pdfload(), but read from a source.
    ///
    /// Optional arguments
    ///
    /// page: `i32`, load this page, numbered from zero
    ///
    /// n: `i32`, load this many pages
    ///
    /// dpi: `f64`, render at this DPI
    ///
    /// scale: `f64`, scale render by this factor
    ///
    /// background: `&[f64]`, background colour
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

    /// Find threshold for percent of pixels
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

    /// Make a perlin noise image
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

    /// Make a perlin noise image
    ///
    /// Optional arguments
    ///
    /// cell_size: `i32`, size of Perlin cells
    ///
    /// uchar: `bool`, output a uchar image
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

    /// Calculate phase correlation
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

    /// Read a PNG file into a VIPS image.
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

    /// Read a PNG file into a VIPS image.
    ///
    /// Optional arguments
    ///
    /// fail_on: `FailOn`, types of read error to fail on
    ///
    /// unlimited: `bool`, Remove all denial of service limits
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

    /// Exactly as vips_pngload(), but read from a PNG-formatted memory block.
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

    /// Exactly as vips_pngload(), but read from a PNG-formatted memory block.
    ///
    /// Optional arguments
    ///
    /// fail_on: `FailOn`, types of read error to fail on
    ///
    /// unlimited: `bool`, Remove all denial of service limits
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

    /// Exactly as vips_pngload(), but read from a source.
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

    /// Exactly as vips_pngload(), but read from a source.
    ///
    /// Optional arguments
    ///
    /// fail_on: `FailOn`, types of read error to fail on
    ///
    /// unlimited: `bool`, Remove all denial of service limits
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

    /// Write a VIPS image to a file as PNG.
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

    /// Write a VIPS image to a file as PNG.
    ///
    /// Optional arguments
    ///
    /// compression: `i32`, compression level
    ///
    /// interlace: `bool`, interlace image
    ///
    /// filter: `ForeignPngFilter`, row filter flag(s)
    ///
    /// palette: `bool`, enable quantisation to 8bpp palette
    ///
    /// Q: `i32`, quality for 8bpp quantisation
    ///
    /// dither: `f64`, amount of dithering for 8bpp quantization
    ///
    /// bitdepth: `i32`, set write bit depth to 1, 2, 4, 8 or 16
    ///
    /// effort: `i32`, quantisation CPU effort
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

    /// As vips_pngsave(), but save to a memory buffer.
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

    /// As vips_pngsave(), but save to a memory buffer.
    ///
    /// Optional arguments
    ///
    /// compression: `i32`, compression level
    ///
    /// interlace: `bool`, interlace image
    ///
    /// filter: `ForeignPngFilter`, row filter flag(s)
    ///
    /// palette: `bool`, enable quantisation to 8bpp palette
    ///
    /// Q: `i32`, quality for 8bpp quantisation
    ///
    /// dither: `f64`, amount of dithering for 8bpp quantization
    ///
    /// bitdepth: `i32`, set write bit depth to 1, 2, 4, 8 or 16
    ///
    /// effort: `i32`, quantisation CPU effort
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

    /// As vips_pngsave(), but save to a target.
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

    /// As vips_pngsave(), but save to a target.
    ///
    /// Optional arguments
    ///
    /// compression: `i32`, compression level
    ///
    /// interlace: `bool`, interlace image
    ///
    /// filter: `ForeignPngFilter`, row filter flag(s)
    ///
    /// palette: `bool`, enable quantisation to 8bpp palette
    ///
    /// Q: `i32`, quality for 8bpp quantisation
    ///
    /// dither: `f64`, amount of dithering for 8bpp quantization
    ///
    /// bitdepth: `i32`, set write bit depth to 1, 2, 4, 8 or 16
    ///
    /// effort: `i32`, quantisation CPU effort
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

    /// Read a PPM/PBM/PGM/PFM file into a VIPS image.
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

    /// Read a PPM/PBM/PGM/PFM file into a VIPS image.
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

    /// Exactly as vips_ppmload(), but read from a memory source.
    pub fn ppmload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "ppmload_buffer",
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
            Error::OperationError("ppmload_buffer failed"),
        )
    }

    /// Exactly as vips_ppmload(), but read from a memory source.
    pub fn ppmload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
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

    /// Exactly as vips_ppmload(), but read from a source.
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

    /// Exactly as vips_ppmload(), but read from a source.
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

    /// Write a VIPS image to a file as PPM.
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

    /// Write a VIPS image to a file as PPM.
    ///
    /// Optional arguments
    ///
    /// format: `ForeignPpmFormat`, format to save in
    ///
    /// ascii: `bool`, save as ASCII rather than binary
    ///
    /// bitdepth: `i32`, bitdepth to save at
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

    /// As vips_ppmsave(), but save to a target.
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

    /// As vips_ppmsave(), but save to a target.
    ///
    /// Optional arguments
    ///
    /// format: `ForeignPpmFormat`, format to save in
    ///
    /// ascii: `bool`, save as ASCII rather than binary
    ///
    /// bitdepth: `i32`, bitdepth to save at
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

    /// Premultiply image alpha
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

    /// Premultiply image alpha
    ///
    /// Optional arguments
    ///
    /// max_alpha: `f64`, maximum value for alpha
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

    /// Prewitt edge detector
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

    /// Find image profiles
    ///
    /// Tuple (
    ///
    /// VipsImage - First non-zero pixel in column
    ///
    /// VipsImage - First non-zero pixel in row
    ///
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

    /// Load named ICC profile
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

    /// Find image projections
    ///
    /// Tuple (
    ///
    /// VipsImage - Sums of columns
    ///
    /// VipsImage - Sums of rows
    ///
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

    /// Resample an image with a quadratic transform
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

    /// Resample an image with a quadratic transform
    ///
    /// Optional arguments
    ///
    /// interpolate: `Interpolate`, use this interpolator (default bilinear)
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

    /// Unpack Radiance coding to float RGB
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

    /// Read a Radiance (HDR) file into a VIPS image.
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

    /// Read a Radiance (HDR) file into a VIPS image.
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

    /// Exactly as vips_radload(), but read from a HDR-formatted memory block.
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
                Error::OperationError("radload_buffer failed"),
            )
        }
    }

    /// Exactly as vips_radload(), but read from a HDR-formatted memory block.
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
            Error::OperationError("radload_buffer failed"),
        )
    }

    /// Exactly as vips_radload(), but read from a source.
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

    /// Exactly as vips_radload(), but read from a source.
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

    /// Write a VIPS image in Radiance (HDR) format.
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

    /// Write a VIPS image in Radiance (HDR) format.
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

    /// As vips_radsave(), but save to a memory buffer.
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

    /// As vips_radsave(), but save to a memory buffer.
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

    /// As vips_radsave(), but save to a target.
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

    /// As vips_radsave(), but save to a target.
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

    /// Rank filter
    ///
    /// width: `i32` -> Window width in pixels
    ///
    /// height: `i32` -> Window height in pixels
    ///
    /// index: `i32` -> Select pixel at index
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

    /// Mmaps the file, setting up out so that access to that image will read from the file.
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

    /// Mmaps the file, setting up out so that access to that image will read from the file.
    /// Optional arguments
    ///
    /// offset: `u64`, offset in bytes from start of file
    ///
    /// format: `BandFormat`, set image format
    ///
    /// interpretation: `Interpretation`, set image interpretation
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

    /// Writes the pixels in image to the file filename with no header or other metadata.
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

    /// Writes the pixels in image to the file filename with no header or other metadata.
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

    /// As vips_rawsave(), but save to a memory buffer.
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

    /// As vips_rawsave(), but save to a memory buffer.
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

    /// As vips_rawsave(), but save to a target.
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

    /// As vips_rawsave(), but save to a target.
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

    /// Linear recombination with matrix
    ///
    /// m: `&VipsImage` -> Matrix of coefficients
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

    /// Reduce an image
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

    /// Reduce an image
    ///
    /// Optional arguments
    ///
    /// kernel: `Kernel`, kernel to interpolate with (default: VIPS_KERNEL_LANCZOS3)
    ///
    /// gap: `f64`, reducing gap to use (default: 0.0)
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

    /// Shrink an image horizontally
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

    /// Shrink an image horizontally
    ///
    /// Optional arguments
    ///
    /// kernel: `Kernel`, kernel to interpolate with (default: VIPS_KERNEL_LANCZOS3)
    ///
    /// gap: `f64`, reducing gap to use (default: 0.0)
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

    /// Shrink an image vertically
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

    /// Shrink an image vertically
    ///
    /// Optional arguments
    ///
    /// kernel: `Kernel`, kernel to interpolate with (default: VIPS_KERNEL_LANCZOS3)
    ///
    /// gap: `f64`, reducing gap to use (default: 0.0)
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

    /// Relational operation on two images
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

    /// Relational operations against a constant
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

    /// Remainder after integer division of two images
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

    /// Remainder after integer division of an image and a constant
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

    /// Takes apart the mosaiced image and rebuilds it, substituting images.
    ///
    /// old_str:&str, Gamma of source images.
    ///
    /// new_str:&str, Gamma of source images.
    pub fn remosaic(&self, old_str: &str, new_str: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "remosaic",
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

    /// Replicate an image
    ///
    /// across: `i32` -> Repeat this many times horizontally
    ///
    /// down: `i32` -> Repeat this many times vertically
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

    /// Resize an image
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

    /// Resize an image
    ///
    /// Optional arguments
    ///
    /// vscale: `f64`, vertical scale factor
    ///
    /// kernel: `Kernel`, kernel to reduce with (default: VIPS_KERNEL_LANCZOS3)
    ///
    /// gap: `f64`, reducing gap to use (default: 2.0)
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

    /// Rotate an image
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

    /// Rotate an image
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

    /// Rotate an image
    ///
    /// Optional arguments
    ///
    /// angle: `Angle45`, rotation angle
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

    /// Rotate an image by a number of degrees
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

    /// Rotate an image by a number of degrees
    ///
    /// Optional arguments
    ///
    /// interpolate: `Interpolate`, interpolate pixels with this
    ///
    /// background: `&[f64]`, colour for new pixels
    ///
    /// idx: `f64`, input horizontal offset
    ///
    /// idy: `f64`, input vertical offset
    ///
    /// odx: `f64`, output horizontal offset
    ///
    /// ody: `f64`, output vertical offset
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

    /// Perform a round function on an image
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

    /// Transform sRGB to HSV
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

    /// Convert sRGB to scRGB.
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

    /// Convert scRGB to BW
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

    /// Convert scRGB to BW
    ///
    /// Optional arguments
    ///
    /// depth: `i32`, depth of output image in bits
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

    /// Transform scRGB to XYZ
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

    /// Convert scRGB to sRGB
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

    /// Convert scRGB to sRGB
    ///
    /// Optional arguments
    ///
    /// depth: `i32`, depth of output image in bits
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

    /// Scale an image to uchar
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

    /// Scale an image to uchar
    ///
    /// Optional arguments
    ///
    /// log: `bool`, log scale pixels
    ///
    /// exp: `f64`, exponent for log scale
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

    /// Scharr edge detector
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

    /// Create an SDF image
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

    /// Create an SDF image
    ///
    /// Optional arguments
    ///
    /// a: `&[f64]`, first point
    ///
    /// b: `&[f64]`, second point
    ///
    /// r: `f64`, radius
    ///
    /// corners: `&[f64]`, corner radii
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

    /// Check sequential access
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

    /// Check sequential access
    ///
    /// Optional arguments
    ///
    /// tile_height: `i32`, height of cache strips
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

    /// Unsharp masking for print
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

    /// Unsharp masking for print
    ///
    /// Optional arguments
    ///
    /// sigma: `f64`, sigma of gaussian
    ///
    /// x1: `f64`, flat/jaggy threshold
    ///
    /// y2: `f64`, maximum amount of brightening
    ///
    /// y3: `f64`, maximum amount of darkening
    ///
    /// m1: `f64`, slope for flat areas
    ///
    /// m2: `f64`, slope for jaggy areas
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

    /// Shrink an image
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

    /// Shrink an image
    ///
    /// Optional arguments
    ///
    /// ceil: `bool`, round-up output dimensions
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

    /// Shrink an image horizontally
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

    /// Shrink an image horizontally
    ///
    /// Optional arguments
    ///
    /// ceil: `bool`, round-up output dimensions
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

    /// Shrink an image vertically
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

    /// Shrink an image vertically
    ///
    /// Optional arguments
    ///
    /// ceil: `bool`, round-up output dimensions
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

    /// Unit vector of pixel
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

    /// Similarity transform of an image
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

    /// Similarity transform of an image
    ///
    /// Optional arguments
    ///
    /// scale: `f64`, scale by this factor
    ///
    /// angle: `f64`, rotate by this many degrees clockwise
    ///
    /// interpolate: `Interpolate`, interpolate pixels with this
    ///
    /// background: `&[f64]`, colour for new pixels
    ///
    /// idx: `f64`, input horizontal offset
    ///
    /// idy: `f64`, input vertical offset
    ///
    /// odx: `f64`, output horizontal offset
    ///
    /// ody: `f64`, output vertical offset
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

    /// Make a 2D sine wave
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
                Error::OperationError("Sines failed"),
            )
        }
    }

    /// Make a 2D sine wave
    ///
    /// Optional arguments
    ///
    /// hfreq: `f64`, horizontal frequency
    ///
    /// vreq: `f64`, vertical frequency
    ///
    /// uchar: `bool`, output a uchar image
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
            Error::OperationError("Sines failed"),
        )
    }

    /// Extract an area from an image
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

    /// Extract an area from an image
    ///
    /// Optional arguments
    ///
    /// interesting: `Interesting` to use to find interesting areas (default: VIPS_INTERESTING_ATTENTION)
    ///
    /// premultiplied: `bool`, input image already has premultiplied alpha
    ///
    /// attention_x: `i32`, output, horizontal position of attention centre when using attention based cropping
    ///
    /// attention_y: `i32`, output, vertical position of attention centre when using attention based cropping
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

    /// Sobel edge detector
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

    /// Spatial correlation
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

    /// Make displayable power spectrum
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

    /// Find many image stats
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
                Error::OperationError("Stats failed"),
            )
        }
    }

    /// Statistical difference
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

    /// Statistical difference
    ///
    /// Optional arguments
    ///
    /// a: `f64`, weight of new mean
    ///
    /// m0: `f64`, target mean
    ///
    /// b: `f64`, weight of new deviation
    ///
    /// s0: `f64`, target deviation
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

    /// Subsample an image
    ///
    /// xfac: `i32` -> Horizontal subsample factor
    ///
    /// yfac: `i32` -> Vertical subsample factor
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

    /// Subsample an image
    ///
    /// xfac: `i32` -> Horizontal subsample factor
    ///
    /// yfac: `i32` -> Vertical subsample factor
    ///
    /// Optional arguments
    ///
    /// point: `bool`, turn on point sample mode
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

    /// Subtract two images
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

    /// Sum an array of images
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

    /// Render a SVG file into a VIPS image.
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

    /// Render a SVG file into a VIPS image.
    ///
    /// Optional arguments
    ///
    /// dpi: `f64`, render at this DPI
    ///
    /// scale: `f64`, scale render by this factor
    ///
    /// unlimited: `bool`, allow SVGs of any size
    ///
    /// stylesheet: `&str`, custom CSS
    ///
    /// high_bitdepth: `bool`, enable scRGB 128-bit output
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

    /// Read a SVG-formatted memory block into a VIPS image.
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

    /// Read a SVG-formatted memory block into a VIPS image.
    ///
    /// Optional arguments
    ///
    /// dpi: `f64`, render at this DPI
    ///
    /// scale: `f64`, scale render by this factor
    ///
    /// unlimited: `bool`, allow SVGs of any size
    ///
    /// stylesheet: `&str`, custom CSS
    ///
    /// high_bitdepth: `bool`, enable scRGB 128-bit output
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

    /// Exactly as vips_svgload(), but read from a source.
    pub fn svgload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "svgload_source",
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
            Error::OperationError("svgload_source failed"),
        )
    }

    /// Exactly as vips_svgload(), but read from a source.
    pub fn svgload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
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

    /// The tests images are evaluated and at each point the index of the first non-zero value is written to out
    pub fn switch(tests: &[VipsImage]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());

        let vips_op_response = call(
            "switch",
            VOption::new()
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

    /// Run an external command
    ///
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

    /// Run an external command
    ///
    /// cmd_format: `&str` -> Command to run
    ///
    /// Optional arguments
    ///
    /// in: `&[f64]`, array of input images
    ///
    /// out: `&mut VipsImage`, output, image
    ///
    /// in_format: `&str`, write input files like this
    ///
    /// out_format: `&str`, write output filename like this
    ///
    /// log: `&str`, output, stdout of command is returned here.
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

    /// Make a text image
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

    /// Make a text image
    ///
    /// Optional arguments
    ///
    /// font: `&str`, font to render with
    ///
    /// fontfile: `&str`, load this font file
    ///
    /// width: `i32`, image should be no wider than this many pixels
    ///
    /// height: `i32`, image should be no higher than this many pixels
    ///
    /// align: `Align`, set justification alignment
    ///
    /// justify: `bool`, justify lines
    ///
    /// dpi: `i32`, render at this resolution
    ///
    /// autofit_dpi: `&mut i32`, output, auto-fitted DPI
    ///
    /// rgba: `bool`, enable RGBA output
    ///
    /// spacing: `i32`, space lines by this in points
    ///
    /// wrap: `TextWrap`, wrap lines on characters or words
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

    /// Generate thumbnail from file
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

    /// Generate thumbnail from file
    ///
    /// Optional arguments
    ///
    /// height: `i32`, target height in pixels
    ///
    /// size: `Size`, upsize, downsize, both or force
    ///
    /// no_rotate: `bool`, don’t rotate upright using orientation tag
    ///
    /// crop: `Interesting`, shrink and crop to fill target
    ///
    /// linear: `bool`, perform shrink in linear light
    ///
    /// input_profile: `&str`, fallback input ICC profile
    ///
    /// output_profile: `&str`, output ICC profile
    ///
    /// intent: `Intent`, rendering intent
    ///
    /// fail_on: `FailOn`, load error types to fail on
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

    /// Generate thumbnail from buffer
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

    /// Generate thumbnail from buffer
    ///
    /// Optional arguments
    ///
    /// height: `i32`, target height in pixels
    ///
    /// size: `Size`, upsize, downsize, both or force
    ///
    /// no_rotate: `bool`, don’t rotate upright using orientation tag
    ///
    /// crop: `Interesting`, shrink and crop to fill target
    ///
    /// linear: `bool`, perform shrink in linear light
    ///
    /// input_profile: `&str`, fallback input ICC profile
    ///
    /// output_profile: `&str`, output ICC profile
    ///
    /// intent: `Intent`, rendering intent
    ///
    /// fail_on: `FailOn`, load error types to fail on
    ///
    /// option_string: `&str`, extra loader options
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

    /// Generate thumbnail from image
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

    /// Generate thumbnail from image
    ///
    /// Optional arguments
    ///
    /// height: `i32`, target height in pixels
    ///
    /// size: `Size`, upsize, downsize, both or force
    ///
    /// no_rotate: `bool`, don’t rotate upright using orientation tag
    ///
    /// crop: `Interesting`, shrink and crop to fill target
    ///
    /// linear: `bool`, perform shrink in linear light
    ///
    /// input_profile: `&str`, fallback input ICC profile
    ///
    /// output_profile: `&str`, output ICC profile
    ///
    /// intent: `Intent`, rendering intent
    ///
    /// fail_on: `FailOn`, load error types to fail on
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

    /// Generate thumbnail from source
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

    /// Generate thumbnail from source
    ///
    /// Optional arguments
    ///
    /// height: `i32`, target height in pixels
    ///
    /// size: `Size`, upsize, downsize, both or force
    ///
    /// no_rotate: `bool`, don’t rotate upright using orientation tag
    ///
    /// crop: `Interesting`, shrink and crop to fill target
    ///
    /// linear: `bool`, perform shrink in linear light
    ///
    /// input_profile: `&str`, fallback input ICC profile
    ///
    /// output_profile: `&str`, output ICC profile
    ///
    /// intent: `Intent`, rendering intent
    ///
    /// fail_on: `FailOn`, load error types to fail on
    ///
    /// option_string: `&str`, extra loader options
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

    /// Read a TIFF file into a VIPS image.
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

    /// Read a TIFF file into a VIPS image.
    ///
    /// Optional arguments
    ///
    /// page: `i32`, load this page
    ///
    /// n: `i32`, load this many pages
    ///
    /// autorotate: `bool`, use orientation tag to rotate the image during load
    ///
    /// subifd: `i32`, select this subifd index
    ///
    /// fail_on: `FailOn`, types of read error to fail on
    ///
    /// unlimited: `bool`, remove all denial of service limits
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

    /// Read a TIFF-formatted memory block into a VIPS image.
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

    /// Read a TIFF-formatted memory block into a VIPS image.
    ///
    /// Optional arguments
    ///
    /// page: `i32`, load this page
    ///
    /// n: `i32`, load this many pages
    ///
    /// autorotate: `bool`, use orientation tag to rotate the image during load
    ///
    /// subifd: `i32`, select this subifd index
    ///
    /// fail_on: `FailOn`, types of read error to fail on
    ///
    /// unlimited: `bool`, remove all denial of service limits
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

    /// Exactly as vips_tiffload(), but read from a source.
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

    /// Exactly as vips_tiffload(), but read from a source.
    ///
    /// Optional arguments
    ///
    /// page: `i32`, load this page
    ///
    /// n: `i32`, load this many pages
    ///
    /// autorotate: `bool`, use orientation tag to rotate the image during load
    ///
    /// subifd: `i32`, select this subifd index
    ///
    /// fail_on: `FailOn`, types of read error to fail on
    ///
    /// unlimited: `bool`, remove all denial of service limits
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

    /// Write a VIPS image to a file as TIFF.
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

    /// Write a VIPS image to a file as TIFF.
    ///
    /// Optional arguments
    ///
    /// compression: `ForeignTiffCompression`, write with this compression
    ///
    /// Q: `i32`, quality factor
    ///
    /// predictor: `ForeignTiffPredictor`, use this predictor
    ///
    /// tile: `bool`, set TRUE to write a tiled tiff
    ///
    /// tile_width: `i32`, for tile size
    ///
    /// tile_height: `i32`, for tile size
    ///
    /// pyramid: `bool`, write an image pyramid
    ///
    /// bitdepth: `i32`, change bit depth to 1,2, or 4 bit
    ///
    /// miniswhite: `bool`, write 1-bit images as MINISWHITE
    ///
    /// resunit: `ForeignTiffResunit` for resolution unit
    ///
    /// xres: `f64`, horizontal resolution in pixels/mm
    ///
    /// yres: `f64`, vertical resolution in pixels/mm
    ///
    /// bigtiff: `bool`, write a BigTiff file
    ///
    /// properties: `bool`, set TRUE to write an IMAGEDESCRIPTION tag
    ///
    /// region_shrink: `RegionShrink` How to shrink each 2x2 region.
    ///
    /// level: `i32`, Zstd or Deflate (zlib) compression level
    ///
    /// lossless: `bool`, WebP lossless mode
    ///
    /// depth: `ForeignDzDepth` how deep to make the pyramid
    ///
    /// subifd: `bool`, write pyr layers as sub-ifds
    ///
    /// premultiply: `bool`, write premultiplied alpha
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

    /// As vips_tiffsave(), but save to a memory buffer.
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

    /// As vips_tiffsave(), but save to a memory buffer.
    ///
    /// Optional arguments
    ///
    /// compression: `ForeignTiffCompression`, write with this compression
    ///
    /// Q: `i32`, quality factor
    ///
    /// predictor: `ForeignTiffPredictor`, use this predictor
    ///
    /// tile: `bool`, set TRUE to write a tiled tiff
    ///
    /// tile_width: `i32`, for tile size
    ///
    /// tile_height: `i32`, for tile size
    ///
    /// pyramid: `bool`, write an image pyramid
    ///
    /// bitdepth: `i32`, change bit depth to 1,2, or 4 bit
    ///
    /// miniswhite: `bool`, write 1-bit images as MINISWHITE
    ///
    /// resunit: `ForeignTiffResunit` for resolution unit
    ///
    /// xres: `f64`, horizontal resolution in pixels/mm
    ///
    /// yres: `f64`, vertical resolution in pixels/mm
    ///
    /// bigtiff: `bool`, write a BigTiff file
    ///
    /// properties: `bool`, set TRUE to write an IMAGEDESCRIPTION tag
    ///
    /// region_shrink: `RegionShrink` How to shrink each 2x2 region.
    ///
    /// level: `i32`, Zstd or Deflate (zlib) compression level
    ///
    /// lossless: `bool`, WebP lossless mode
    ///
    /// depth: `ForeignDzDepth` how deep to make the pyramid
    ///
    /// subifd: `bool`, write pyr layers as sub-ifds
    ///
    /// premultiply: `bool`, write premultiplied alpha
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

    /// As vips_tiffsave(), but save to a target.
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

    /// As vips_tiffsave(), but save to a target.
    ///
    /// Optional arguments
    ///
    /// compression: `ForeignTiffCompression`, write with this compression
    ///
    /// Q: `i32`, quality factor
    ///
    /// predictor: `ForeignTiffPredictor`, use this predictor
    ///
    /// tile: `bool`, set TRUE to write a tiled tiff
    ///
    /// tile_width: `i32`, for tile size
    ///
    /// tile_height: `i32`, for tile size
    ///
    /// pyramid: `bool`, write an image pyramid
    ///
    /// bitdepth: `i32`, change bit depth to 1,2, or 4 bit
    ///
    /// miniswhite: `bool`, write 1-bit images as MINISWHITE
    ///
    /// resunit: `ForeignTiffResunit` for resolution unit
    ///
    /// xres: `f64`, horizontal resolution in pixels/mm
    ///
    /// yres: `f64`, vertical resolution in pixels/mm
    ///
    /// bigtiff: `bool`, write a BigTiff file
    ///
    /// properties: `bool`, set TRUE to write an IMAGEDESCRIPTION tag
    ///
    /// region_shrink: `RegionShrink` How to shrink each 2x2 region.
    ///
    /// level: `i32`, Zstd or Deflate (zlib) compression level
    ///
    /// lossless: `bool`, WebP lossless mode
    ///
    /// depth: `ForeignDzDepth` how deep to make the pyramid
    ///
    /// subifd: `bool`, write pyr layers as sub-ifds
    ///
    /// premultiply: `bool`, write premultiplied alpha
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

    /// Cache an image as a set of tiles
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

    /// Cache an image as a set of tiles
    ///
    /// Optional arguments
    ///
    /// tile_width: `i32`, width of tiles in cache
    ///
    /// tile_height: `i32`, height of tiles in cache
    ///
    /// max_tiles: `i32`, maximum number of tiles to cache
    ///
    /// access: `Access`, hint expected access pattern
    ///
    /// threaded: `bool`, allow many threads
    ///
    /// persistent: `bool`, don’t drop cache at end of computation
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

    /// Build a look-up table
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

    /// Build a look-up table
    ///
    /// Optional arguments
    ///
    /// in_max: `i32`, input range
    ///
    /// out_max: `i32`, output range
    ///
    /// Lb: `f64`, black-point [0-100]
    ///
    /// Lw: `f64`, white-point [0-100]
    ///
    /// Ps: `f64`, shadow point (eg. 0.2)
    ///
    /// Pm: `f64`, mid-tone point (eg. 0.5)
    ///
    /// Ph: `f64`, highlight point (eg. 0.8)
    ///
    /// S: `f64`, shadow adjustment (+/- 30)
    ///
    /// M: `f64`, mid-tone adjustment (+/- 30)
    ///
    /// H: `f64`, highlight adjustment (+/- 30).
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

    /// Transpose3d an image
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

    /// Transpose3d an image
    ///
    /// Optional arguments
    ///
    /// page_height: `i32`, size of each input page
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

    /// Unpremultiply image alpha
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

    /// Unpremultiply image alpha
    ///
    /// Optional arguments
    ///
    /// max_alpha: `f64`, maximum value for alpha
    ///
    /// alpha_band: `i32`, band containing alpha data
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

    /// Read in a vips image.
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

    /// Read in a vips image.
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

    /// Exactly as vips_vipsload(), but read from a source.
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

    /// Exactly as vips_vipsload(), but read from a source.
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

    /// Write image to filename in VIPS format.
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

    /// Write image to filename in VIPS format.
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

    /// As vips_vipssave(), but save to a target.
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

    /// As vips_vipssave(), but save to a target.
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

    /// Read a WebP file into a VIPS image.
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

    /// Read a WebP file into a VIPS image.
    ///
    /// Optional arguments
    ///
    /// page: `i32`, page (frame) to read
    ///
    /// n: `i32`, load this many pages
    ///
    /// scale: `bool`, scale by this much on load
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

    /// Read a WebP-formatted memory block into a VIPS image.
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

    /// Read a WebP-formatted memory block into a VIPS image.
    ///
    /// Optional arguments
    ///
    /// page: `i32`, page (frame) to read
    ///
    /// n: `i32`, load this many pages
    ///
    /// scale: `bool`, scale by this much on load
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

    /// Exactly as vips_webpload(), but read from a source.
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

    /// Exactly as vips_webpload(), but read from a source.
    ///
    /// Optional arguments
    ///
    /// page: `i32`, page (frame) to read
    ///
    /// n: `i32`, load this many pages
    ///
    /// scale: `bool`, scale by this much on load
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

    /// Write an image to a file in WebP format.
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

    /// Write an image to a file in WebP format.
    ///
    /// Optional arguments
    ///
    /// Q: `i32`, quality factor
    ///
    /// lossless: `bool`, enables lossless compression
    ///
    /// preset: `ForeignWebpPreset`, choose lossy compression preset
    ///
    /// smart_subsample: `bool`, enables high quality chroma subsampling
    ///
    /// smart_deblock: `bool`, enables auto-adjusting of the deblocking filter
    ///
    /// near_lossless: `bool`, preprocess in lossless mode (controlled by Q)
    ///
    /// alpha_q: `i32`, set alpha quality in lossless mode
    ///
    /// effort: `i32`, level of CPU effort to reduce file size
    ///
    /// target_size: `i32`, desired target size in bytes
    ///
    /// passes: `i32`, number of entropy-analysis passes
    ///
    /// min_size: `bool`, minimise size
    ///
    /// mixed: `bool`, allow both lossy and lossless encoding
    ///
    /// kmin: `i32`, minimum number of frames between keyframes
    ///
    /// kmax: `i32`, maximum number of frames between keyframes
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

    /// As vips_webpsave(), but save to a memory buffer.
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

    /// As vips_webpsave(), but save to a memory buffer.
    ///
    /// Optional arguments
    ///
    /// Q: `i32`, quality factor
    ///
    /// lossless: `bool`, enables lossless compression
    ///
    /// preset: `ForeignWebpPreset`, choose lossy compression preset
    ///
    /// smart_subsample: `bool`, enables high quality chroma subsampling
    ///
    /// smart_deblock: `bool`, enables auto-adjusting of the deblocking filter
    ///
    /// near_lossless: `bool`, preprocess in lossless mode (controlled by Q)
    ///
    /// alpha_q: `i32`, set alpha quality in lossless mode
    ///
    /// effort: `i32`, level of CPU effort to reduce file size
    ///
    /// target_size: `i32`, desired target size in bytes
    ///
    /// passes: `i32`, number of entropy-analysis passes
    ///
    /// min_size: `bool`, minimise size
    ///
    /// mixed: `bool`, allow both lossy and lossless encoding
    ///
    /// kmin: `i32`, minimum number of frames between keyframes
    ///
    /// kmax: `i32`, maximum number of frames between keyframes
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

    /// As vips_webpsave(), but save as a mime webp on stdout.
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

    /// As vips_webpsave(), but save as a mime webp on stdout.
    ///
    /// Optional arguments
    ///
    /// Q: `i32`, quality factor
    ///
    /// lossless: `bool`, enables lossless compression
    ///
    /// preset: `ForeignWebpPreset`, choose lossy compression preset
    ///
    /// smart_subsample: `bool`, enables high quality chroma subsampling
    ///
    /// smart_deblock: `bool`, enables auto-adjusting of the deblocking filter
    ///
    /// near_lossless: `bool`, preprocess in lossless mode (controlled by Q)
    ///
    /// alpha_q: `i32`, set alpha quality in lossless mode
    ///
    /// effort: `i32`, level of CPU effort to reduce file size
    ///
    /// target_size: `i32`, desired target size in bytes
    ///
    /// passes: `i32`, number of entropy-analysis passes
    ///
    /// min_size: `bool`, minimise size
    ///
    /// mixed: `bool`, allow both lossy and lossless encoding
    ///
    /// kmin: `i32`, minimum number of frames between keyframes
    ///
    /// kmax: `i32`, maximum number of frames between keyframes
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

    /// As vips_webpsave(), but save to a target.
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

    /// As vips_webpsave(), but save to a target.
    ///
    /// Optional arguments
    ///
    /// Q: `i32`, quality factor
    ///
    /// lossless: `bool`, enables lossless compression
    ///
    /// preset: `ForeignWebpPreset`, choose lossy compression preset
    ///
    /// smart_subsample: `bool`, enables high quality chroma subsampling
    ///
    /// smart_deblock: `bool`, enables auto-adjusting of the deblocking filter
    ///
    /// near_lossless: `bool`, preprocess in lossless mode (controlled by Q)
    ///
    /// alpha_q: `i32`, set alpha quality in lossless mode
    ///
    /// effort: `i32`, level of CPU effort to reduce file size
    ///
    /// target_size: `i32`, desired target size in bytes
    ///
    /// passes: `i32`, number of entropy-analysis passes
    ///
    /// min_size: `bool`, minimise size
    ///
    /// mixed: `bool`, allow both lossy and lossless encoding
    ///
    /// kmin: `i32`, minimum number of frames between keyframes
    ///
    /// kmax: `i32`, maximum number of frames between keyframes
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

    /// Make a worley noise image
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

    /// Make a worley noise image
    ///
    /// Optional arguments
    ///
    /// cell_size: `i32`, size of Worley cells
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

    /// Wrap image origin
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

    /// Wrap image origin
    ///
    /// Optional arguments
    ///
    /// x: `i32`, horizontal displacement
    ///
    /// y: `i32`, vertical displacement
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

    /// Make an image where pixel values are coordinates
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

    /// Make an image where pixel values are coordinates
    ///
    /// Optional arguments
    ///
    /// csize: `i32`, size for third dimension
    ///
    /// dsize: `i32`, size for fourth dimension
    ///
    /// esize: `i32`, size for fifth dimension
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

    /// Make a zone plate
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

    /// Make a zone plate
    ///
    /// Optional arguments
    ///
    /// uchar: `bool`, output a uchar image
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

    /// Zoom an image
    ///
    /// xfac: `i32` -> Horizontal zoom factor
    ///
    /// yfac: `i32` -> Vertical zoom factor
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
