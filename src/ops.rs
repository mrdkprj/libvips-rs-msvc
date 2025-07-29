#![allow(clippy::too_many_arguments)]
#![allow(clippy::upper_case_acronyms)]
use crate::error::*;
use crate::utils;
use crate::v_value;
use crate::voption::{call, VOption};
use crate::Result;
use crate::VipsBlob;
use crate::VipsImage;
use crate::VipsSource;
use crate::VipsTarget;
use std::ffi::c_void;
use std::ptr::null_mut;

const NULL: *const c_void = null_mut();

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum CombineMode {
    ///  `Set` -> VIPS_COMBINE_MODE_SET = 0
    Set = 0,
    ///  `Add` -> VIPS_COMBINE_MODE_ADD = 1
    Add = 1,
    ///  `Last` -> VIPS_COMBINE_MODE_LAST = 2
    Last = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum Direction {
    ///  `Horizontal` -> VIPS_DIRECTION_HORIZONTAL = 0
    Horizontal = 0,
    ///  `Vertical` -> VIPS_DIRECTION_VERTICAL = 1
    Vertical = 1,
    ///  `Last` -> VIPS_DIRECTION_LAST = 2
    Last = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum ForeignDzContainer {
    ///  `Fs` -> VIPS_FOREIGN_DZ_CONTAINER_FS = 0
    Fs = 0,
    ///  `Zip` -> VIPS_FOREIGN_DZ_CONTAINER_ZIP = 1
    Zip = 1,
    ///  `Szi` -> VIPS_FOREIGN_DZ_CONTAINER_SZI = 2
    Szi = 2,
    ///  `Last` -> VIPS_FOREIGN_DZ_CONTAINER_LAST = 3
    Last = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum ForeignDzLayout {
    ///  `Dz` -> VIPS_FOREIGN_DZ_LAYOUT_DZ = 0
    Dz = 0,
    ///  `Zoomify` -> VIPS_FOREIGN_DZ_LAYOUT_ZOOMIFY = 1
    Zoomify = 1,
    ///  `Google` -> VIPS_FOREIGN_DZ_LAYOUT_GOOGLE = 2
    Google = 2,
    ///  `Iiif` -> VIPS_FOREIGN_DZ_LAYOUT_IIIF = 3
    Iiif = 3,
    ///  `Iiif3` -> VIPS_FOREIGN_DZ_LAYOUT_IIIF3 = 4
    Iiif3 = 4,
    ///  `Last` -> VIPS_FOREIGN_DZ_LAYOUT_LAST = 5
    Last = 5,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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
    ///  `Packbits` -> VIPS_FOREIGN_TIFF_COMPRESSION_PACKBITS = 3
    Packbits = 3,
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum ForeignTiffResunit {
    ///  `Cm` -> VIPS_FOREIGN_TIFF_RESUNIT_CM = 0
    Cm = 0,
    ///  `Inch` -> VIPS_FOREIGN_TIFF_RESUNIT_INCH = 1
    Inch = 1,
    ///  `Last` -> VIPS_FOREIGN_TIFF_RESUNIT_LAST = 2
    Last = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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
    ///  `Labs` -> VIPS_INTERPRETATION_LABS = 21
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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
    ///  `Last` -> VIPS_KERNEL_LAST = 6
    Last = 6,
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum OperationComplex2 {
    ///  `CrossPhase` -> VIPS_OPERATION_COMPLEX2_CROSS_PHASE = 0
    CrossPhase = 0,
    ///  `Last` -> VIPS_OPERATION_COMPLEX2_LAST = 1
    Last = 1,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum OperationComplexget {
    ///  `Real` -> VIPS_OPERATION_COMPLEXGET_REAL = 0
    Real = 0,
    ///  `Imag` -> VIPS_OPERATION_COMPLEXGET_IMAG = 1
    Imag = 1,
    ///  `Last` -> VIPS_OPERATION_COMPLEXGET_LAST = 2
    Last = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum OperationMath {
    ///  `Sin` -> VIPS_OPERATION_MATH_SIN = 0
    Sin = 0,
    ///  `Cos` -> VIPS_OPERATION_MATH_COS = 1
    Cos = 1,
    ///  `Tan` -> VIPS_OPERATION_MATH_TAN = 2
    Tan = 2,
    ///  `Asin` -> VIPS_OPERATION_MATH_ASIN = 3
    Asin = 3,
    ///  `Acos` -> VIPS_OPERATION_MATH_ACOS = 4
    Acos = 4,
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum OperationMorphology {
    ///  `Erode` -> VIPS_OPERATION_MORPHOLOGY_ERODE = 0
    Erode = 0,
    ///  `Dilate` -> VIPS_OPERATION_MORPHOLOGY_DILATE = 1
    Dilate = 1,
    ///  `Last` -> VIPS_OPERATION_MORPHOLOGY_LAST = 2
    Last = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum PCS {
    ///  `Lab` -> VIPS_PCS_LAB = 0
    Lab = 0,
    ///  `Xyz` -> VIPS_PCS_XYZ = 1
    Xyz = 1,
    ///  `Last` -> VIPS_PCS_LAST = 2
    Last = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
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
    /// VipsCMC2LCh (CMC2LCh), transform LCh to CMC
    /// returns `VipsImage` - Output image
    pub fn CMC2LCh(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "CMC2LCh",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Cmc2LCh (vips_CMC2LCh) failed"),
        )
    }

    /// VipsCMYK2XYZ (CMYK2XYZ), transform CMYK to XYZ
    /// returns `VipsImage` - Output image
    pub fn CMYK2XYZ(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "CMYK2XYZ",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Cmyk2Xyz (vips_CMYK2XYZ) failed"),
        )
    }

    /// VipsHSV2sRGB (HSV2sRGB), transform HSV to sRGB
    /// returns `VipsImage` - Output image
    pub fn HSV2sRGB(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "HSV2sRGB",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Hsv2SRgb (vips_HSV2sRGB) failed"),
        )
    }

    /// VipsLCh2CMC (LCh2CMC), transform LCh to CMC
    /// returns `VipsImage` - Output image
    pub fn LCh2CMC(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "LCh2CMC",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("LCh2Cmc (vips_LCh2CMC) failed"),
        )
    }

    /// VipsLCh2Lab (LCh2Lab), transform LCh to Lab
    /// returns `VipsImage` - Output image
    pub fn LCh2Lab(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "LCh2Lab",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("LCh2Lab (vips_LCh2Lab) failed"),
        )
    }

    /// VipsLab2LCh (Lab2LCh), transform Lab to LCh
    /// returns `VipsImage` - Output image
    pub fn Lab2LCh(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "Lab2LCh",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Lab2LCh (vips_Lab2LCh) failed"),
        )
    }

    /// VipsLab2LabQ (Lab2LabQ), transform float Lab to LabQ coding
    /// returns `VipsImage` - Output image
    pub fn Lab2LabQ(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "Lab2LabQ",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Lab2LabQ (vips_Lab2LabQ) failed"),
        )
    }

    /// VipsLab2LabS (Lab2LabS), transform float Lab to signed short
    /// returns `VipsImage` - Output image
    pub fn Lab2LabS(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "Lab2LabS",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Lab2LabSs (vips_Lab2LabS) failed"),
        )
    }

    /// VipsLab2XYZ (Lab2XYZ), transform CIELAB to XYZ
    /// returns `VipsImage` - Output image
    pub fn Lab2XYZ(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "Lab2XYZ",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Lab2Xyz (vips_Lab2XYZ) failed"),
        )
    }

    /// VipsLab2XYZ (Lab2XYZ), transform CIELAB to XYZ
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// temp: `&[f64]` -> Color temperature
    pub fn Lab2XYZ_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "Lab2XYZ",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Lab2Xyz (vips_Lab2XYZ) failed"),
        )
    }

    /// VipsLabQ2Lab (LabQ2Lab), unpack a LabQ image to float Lab
    /// returns `VipsImage` - Output image
    pub fn LabQ2Lab(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "LabQ2Lab",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("LabQ2Lab (vips_LabQ2Lab) failed"),
        )
    }

    /// VipsLabQ2LabS (LabQ2LabS), unpack a LabQ image to short Lab
    /// returns `VipsImage` - Output image
    pub fn LabQ2LabS(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "LabQ2LabS",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("LabQ2LabSs (vips_LabQ2LabS) failed"),
        )
    }

    /// VipsLabQ2sRGB (LabQ2sRGB), convert a LabQ image to sRGB
    /// returns `VipsImage` - Output image
    pub fn LabQ2sRGB(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "LabQ2sRGB",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("LabQ2SRgb (vips_LabQ2sRGB) failed"),
        )
    }

    /// VipsLabS2Lab (LabS2Lab), transform signed short Lab to float
    /// returns `VipsImage` - Output image
    pub fn LabS2Lab(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "LabS2Lab",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("LabS2Lab (vips_LabS2Lab) failed"),
        )
    }

    /// VipsLabS2LabQ (LabS2LabQ), transform short Lab to LabQ coding
    /// returns `VipsImage` - Output image
    pub fn LabS2LabQ(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "LabS2LabQ",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("LabS2LabQ (vips_LabS2LabQ) failed"),
        )
    }

    /// VipsXYZ2CMYK (XYZ2CMYK), transform XYZ to CMYK
    /// returns `VipsImage` - Output image
    pub fn XYZ2CMYK(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "XYZ2CMYK",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Xyz2Cmyk (vips_XYZ2CMYK) failed"),
        )
    }

    /// VipsXYZ2Lab (XYZ2Lab), transform XYZ to Lab
    /// returns `VipsImage` - Output image
    pub fn XYZ2Lab(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "XYZ2Lab",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Xyz2Lab (vips_XYZ2Lab) failed"),
        )
    }

    /// VipsXYZ2Lab (XYZ2Lab), transform XYZ to Lab
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// temp: `&[f64]` -> Colour temperature
    pub fn XYZ2Lab_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "XYZ2Lab",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Xyz2Lab (vips_XYZ2Lab) failed"),
        )
    }

    /// VipsXYZ2Yxy (XYZ2Yxy), transform XYZ to Yxy
    /// returns `VipsImage` - Output image
    pub fn XYZ2Yxy(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "XYZ2Yxy",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Xyz2Yxy (vips_XYZ2Yxy) failed"),
        )
    }

    /// VipsXYZ2scRGB (XYZ2scRGB), transform XYZ to scRGB
    /// returns `VipsImage` - Output image
    pub fn XYZ2scRGB(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "XYZ2scRGB",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Xyz2ScRgb (vips_XYZ2scRGB) failed"),
        )
    }

    /// VipsYxy2XYZ (Yxy2XYZ), transform Yxy to XYZ
    /// returns `VipsImage` - Output image
    pub fn Yxy2XYZ(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "Yxy2XYZ",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Yxy2Xyz (vips_Yxy2XYZ) failed"),
        )
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Abs (vips_abs) failed"),
        )
    }

    /// VipsAdd (add), add two images
    /// returns `VipsImage` - Output image
    ///
    /// right: `&VipsImage` -> Right-hand image argument
    pub fn add(&self, right: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "add",
            VOption::new()
                .with(
                    "left",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "right",
                    v_value!(right),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Add (vips_add) failed"),
        )
    }

    /// VipsAddAlpha (addalpha), append an alpha channel
    /// returns `VipsImage` - Output image
    pub fn addalpha(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "addalpha",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Addalpha (vips_addalpha) failed"),
        )
    }

    /// VipsAffine (affine), affine transform of an image
    /// returns `VipsImage` - Output image
    ///
    /// matrix: `&[f64]` -> Transformation matrix
    pub fn affine(&self, matrix: &[f64]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "affine",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "matrix",
                    v_value!(matrix),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Affine (vips_affine) failed"),
        )
    }

    /// VipsAffine (affine), affine transform of an image
    /// returns `VipsImage` - Output image
    ///
    /// matrix: `&[f64]` -> Transformation matrix
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// interpolate: `&VipsInterpolate` -> Interpolate pixels with this
    ///
    /// oarea: `&[i32]` -> Area of output to generate
    ///
    /// odx: `f64` -> Horizontal output displacement
    ///
    /// ody: `f64` -> Vertical output displacement
    ///
    /// idx: `f64` -> Horizontal input displacement
    ///
    /// idy: `f64` -> Vertical input displacement
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// premultiplied: `bool` -> Images have premultiplied alpha
    ///
    /// extend: `Extend` -> How to generate the extra pixels
    pub fn affine_with_opts(&self, matrix: &[f64], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "affine",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "matrix",
                    v_value!(matrix),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Affine (vips_affine) failed"),
        )
    }

    /// VipsForeignLoadAnalyze (analyzeload), load an Analyze6 image (.img, .hdr), priority=-50, untrusted, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn analyzeload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "analyzeload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Analyzeload (vips_analyzeload) failed"),
        )
    }

    /// VipsForeignLoadAnalyze (analyzeload), load an Analyze6 image (.img, .hdr), priority=-50, untrusted, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn analyzeload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "analyzeload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Analyzeload (vips_analyzeload) failed"),
        )
    }

    /// VipsArrayjoin (arrayjoin), join an array of images
    /// returns `VipsImage` - Output image
    ///
    /// inp: `&[VipsImage]` -> Array of input images
    pub fn arrayjoin(inp: &[VipsImage]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "arrayjoin",
            VOption::new()
                .with(
                    "in",
                    v_value!(inp),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Arrayjoin (vips_arrayjoin) failed"),
        )
    }

    /// VipsArrayjoin (arrayjoin), join an array of images
    /// returns `VipsImage` - Output image
    ///
    /// inp: `&[VipsImage]` -> Array of input images
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// across: `i32` -> Number of images across grid
    ///
    /// shim: `i32` -> Pixels between images
    ///
    /// background: `&[f64]` -> Colour for new pixels
    ///
    /// halign: `Align` -> Align on the left, centre or right
    ///
    /// valign: `Align` -> Align on the top, centre or bottom
    ///
    /// hspacing: `i32` -> Horizontal spacing between images
    ///
    /// vspacing: `i32` -> Vertical spacing between images
    pub fn arrayjoin_with_opts(inp: &[VipsImage], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "arrayjoin",
            option
                .with(
                    "in",
                    v_value!(inp),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Arrayjoin (vips_arrayjoin) failed"),
        )
    }

    /// VipsAutorot (autorot), autorotate image by exif tag
    /// returns `VipsImage` - Output image
    pub fn autorot(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "autorot",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Autorot (vips_autorot) failed"),
        )
    }

    /// VipsAutorot (autorot), autorotate image by exif tag
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// angle: `Angle` -> Angle image was rotated by
    ///
    /// flip: `&mut bool` -> Whether the image was flipped or not
    pub fn autorot_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "autorot",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Autorot (vips_autorot) failed"),
        )
    }

    /// VipsAvg (avg), find image average
    /// returns `f64` - Output value
    pub fn avg(&self) -> Result<f64> {
        let mut out_out: f64 = 0.0;
        let vips_op_response = call(
            "avg",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Avg (vips_avg) failed"),
        )
    }

    /// VipsBandbool (bandbool), boolean operation across image bands
    /// returns `VipsImage` - Output image
    ///
    /// boolean: `OperationBoolean` -> Boolean to perform
    pub fn bandbool(&self, boolean: OperationBoolean) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "bandbool",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "boolean",
                    v_value!(boolean as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Bandbool (vips_bandbool) failed"),
        )
    }

    /// VipsBandfold (bandfold), fold up x axis into bands
    /// returns `VipsImage` - Output image
    pub fn bandfold(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "bandfold",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Bandfold (vips_bandfold) failed"),
        )
    }

    /// VipsBandfold (bandfold), fold up x axis into bands
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// factor: `i32` -> Fold by this factor
    pub fn bandfold_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "bandfold",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Bandfold (vips_bandfold) failed"),
        )
    }

    /// VipsBandjoin (bandjoin), bandwise join a set of images
    /// returns `VipsImage` - Output image
    ///
    /// inp: `&[VipsImage]` -> Array of input images
    pub fn bandjoin(inp: &[VipsImage]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "bandjoin",
            VOption::new()
                .with(
                    "in",
                    v_value!(inp),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Bandjoin (vips_bandjoin) failed"),
        )
    }

    /// VipsBandjoinConst (bandjoin_const), append a constant band to an image
    /// returns `VipsImage` - Output image
    ///
    /// c: `&[f64]` -> Array of constants to add
    pub fn bandjoin_const(&self, c: &[f64]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "bandjoin_const",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "c",
                    v_value!(c),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("BandjoinConst (vips_bandjoin_const) failed"),
        )
    }

    /// VipsBandmean (bandmean), band-wise average
    /// returns `VipsImage` - Output image
    pub fn bandmean(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "bandmean",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Bandmean (vips_bandmean) failed"),
        )
    }

    /// VipsBandrank (bandrank), band-wise rank of a set of images
    /// returns `VipsImage` - Output image
    ///
    /// inp: `&[VipsImage]` -> Array of input images
    pub fn bandrank(inp: &[VipsImage]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "bandrank",
            VOption::new()
                .with(
                    "in",
                    v_value!(inp),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Bandrank (vips_bandrank) failed"),
        )
    }

    /// VipsBandrank (bandrank), band-wise rank of a set of images
    /// returns `VipsImage` - Output image
    ///
    /// inp: `&[VipsImage]` -> Array of input images
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// index: `i32` -> Select this band element from sorted list
    pub fn bandrank_with_opts(inp: &[VipsImage], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "bandrank",
            option
                .with(
                    "in",
                    v_value!(inp),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Bandrank (vips_bandrank) failed"),
        )
    }

    /// VipsBandunfold (bandunfold), unfold image bands into x axis
    /// returns `VipsImage` - Output image
    pub fn bandunfold(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "bandunfold",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Bandunfold (vips_bandunfold) failed"),
        )
    }

    /// VipsBandunfold (bandunfold), unfold image bands into x axis
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// factor: `i32` -> Unfold by this factor
    pub fn bandunfold_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "bandunfold",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Bandunfold (vips_bandunfold) failed"),
        )
    }

    /// VipsBlack (black), make a black image
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    pub fn black(width: i32, height: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "black",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Black (vips_black) failed"),
        )
    }

    /// VipsBlack (black), make a black image
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// bands: `i32` -> Number of bands in image
    pub fn black_with_opts(width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "black",
            option
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Black (vips_black) failed"),
        )
    }

    /// VipsBoolean (boolean), boolean operation on two images
    /// returns `VipsImage` - Output image
    ///
    /// right: `&VipsImage` -> Right-hand image argument
    ///
    /// boolean: `OperationBoolean` -> Boolean to perform
    pub fn boolean(&self, right: &VipsImage, boolean: OperationBoolean) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "boolean",
            VOption::new()
                .with(
                    "left",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "right",
                    v_value!(right),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "boolean",
                    v_value!(boolean as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Boolean (vips_boolean) failed"),
        )
    }

    /// VipsBooleanConst (boolean_const), boolean operations against a constant
    /// returns `VipsImage` - Output image
    ///
    /// boolean: `OperationBoolean` -> Boolean to perform
    ///
    /// c: `&[f64]` -> Array of constants
    pub fn boolean_const(&self, boolean: OperationBoolean, c: &[f64]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "boolean_const",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "boolean",
                    v_value!(boolean as i32),
                )
                .with(
                    "c",
                    v_value!(c),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("BooleanConst (vips_boolean_const) failed"),
        )
    }

    /// VipsBuildlut (buildlut), build a look-up table
    /// returns `VipsImage` - Output image
    pub fn buildlut(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "buildlut",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Buildlut (vips_buildlut) failed"),
        )
    }

    /// VipsByteswap (byteswap), byteswap an image
    /// returns `VipsImage` - Output image
    pub fn byteswap(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "byteswap",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Byteswap (vips_byteswap) failed"),
        )
    }

    /// VipsCanny (canny), Canny edge detector
    /// returns `VipsImage` - Output image
    pub fn canny(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "canny",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Canny (vips_canny) failed"),
        )
    }

    /// VipsCanny (canny), Canny edge detector
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// sigma: `f64` -> Sigma of Gaussian
    ///
    /// precision: `Precision` -> Convolve with this precision
    pub fn canny_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "canny",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Canny (vips_canny) failed"),
        )
    }

    /// VipsCase (case), use pixel values to pick cases from an array of images
    /// returns `VipsImage` - Output image
    ///
    /// cases: `&[VipsImage]` -> Array of case images
    pub fn case(&self, cases: &[VipsImage]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "case",
            VOption::new()
                .with(
                    "index",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "cases",
                    v_value!(cases),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Case (vips_case) failed"),
        )
    }

    /// VipsCast (cast), cast an image
    /// returns `VipsImage` - Output image
    ///
    /// format: `BandFormat` -> Format to cast to
    pub fn cast(&self, format: BandFormat) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "cast",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "format",
                    v_value!(format as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Cast (vips_cast) failed"),
        )
    }

    /// VipsCast (cast), cast an image
    /// returns `VipsImage` - Output image
    ///
    /// format: `BandFormat` -> Format to cast to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// shift: `bool` -> Shift integer values up and down
    pub fn cast_with_opts(&self, format: BandFormat, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "cast",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "format",
                    v_value!(format as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Cast (vips_cast) failed"),
        )
    }

    /// VipsClamp (clamp), clamp values of an image
    /// returns `VipsImage` - Output image
    pub fn clamp(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "clamp",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Clamp (vips_clamp) failed"),
        )
    }

    /// VipsClamp (clamp), clamp values of an image
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// min: `f64` -> Minimum value
    ///
    /// max: `f64` -> Maximum value
    pub fn clamp_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "clamp",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Clamp (vips_clamp) failed"),
        )
    }

    /// VipsColourspace (colourspace), convert to a new colorspace
    /// returns `VipsImage` - Output image
    ///
    /// space: `Interpretation` -> Destination color space
    pub fn colourspace(&self, space: Interpretation) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "colourspace",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "space",
                    v_value!(space as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Colourspace (vips_colourspace) failed"),
        )
    }

    /// VipsColourspace (colourspace), convert to a new colorspace
    /// returns `VipsImage` - Output image
    ///
    /// space: `Interpretation` -> Destination color space
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// source_space: `Interpretation` -> Source color space
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "space",
                    v_value!(space as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Colourspace (vips_colourspace) failed"),
        )
    }

    /// VipsCompass (compass), convolve with rotating mask
    /// returns `VipsImage` - Output image
    ///
    /// mask: `&VipsImage` -> Input matrix image
    pub fn compass(&self, mask: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "compass",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "mask",
                    v_value!(mask),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Compass (vips_compass) failed"),
        )
    }

    /// VipsCompass (compass), convolve with rotating mask
    /// returns `VipsImage` - Output image
    ///
    /// mask: `&VipsImage` -> Input matrix image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// times: `i32` -> Rotate and convolve this many times
    ///
    /// angle: `Angle45` -> Rotate mask by this much between convolutions
    ///
    /// combine: `Combine` -> Combine convolution results like this
    ///
    /// precision: `Precision` -> Convolve with this precision
    ///
    /// layers: `i32` -> Use this many layers in approximation
    ///
    /// cluster: `i32` -> Cluster lines closer than this in approximation
    pub fn compass_with_opts(&self, mask: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "compass",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "mask",
                    v_value!(mask),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Compass (vips_compass) failed"),
        )
    }

    /// VipsComplex2 (complex2), complex binary operations on two images
    /// returns `VipsImage` - Output image
    ///
    /// right: `&VipsImage` -> Right-hand image argument
    ///
    /// cmplx: `OperationComplex2` -> Binary complex operation to perform
    pub fn complex2(&self, right: &VipsImage, cmplx: OperationComplex2) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "complex2",
            VOption::new()
                .with(
                    "left",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "right",
                    v_value!(right),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "cmplx",
                    v_value!(cmplx as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Complex2 (vips_complex2) failed"),
        )
    }

    /// VipsComplex (complex), perform a complex operation on an image
    /// returns `VipsImage` - Output image
    ///
    /// cmplx: `OperationComplex` -> Complex to perform
    pub fn complex(&self, cmplx: OperationComplex) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "complex",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "cmplx",
                    v_value!(cmplx as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Complex (vips_complex) failed"),
        )
    }

    /// VipsComplexform (complexform), form a complex image from two real images
    /// returns `VipsImage` - Output image
    ///
    /// right: `&VipsImage` -> Right-hand image argument
    pub fn complexform(&self, right: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "complexform",
            VOption::new()
                .with(
                    "left",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "right",
                    v_value!(right),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Complexform (vips_complexform) failed"),
        )
    }

    /// VipsComplexget (complexget), get a component from a complex image
    /// returns `VipsImage` - Output image
    ///
    /// get: `OperationComplexget` -> Complex to perform
    pub fn complexget(&self, get: OperationComplexget) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "complexget",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "get",
                    v_value!(get as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Complexget (vips_complexget) failed"),
        )
    }

    /// VipsComposite2 (composite2), blend a pair of images with a blend mode
    /// returns `VipsImage` - Output image
    ///
    /// overlay: `&VipsImage` -> Overlay image
    ///
    /// mode: `BlendMode` -> VipsBlendMode to join with
    pub fn composite2(&self, overlay: &VipsImage, mode: BlendMode) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "composite2",
            VOption::new()
                .with(
                    "base",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "overlay",
                    v_value!(overlay),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "mode",
                    v_value!(mode as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Composite2 (vips_composite2) failed"),
        )
    }

    /// VipsComposite2 (composite2), blend a pair of images with a blend mode
    /// returns `VipsImage` - Output image
    ///
    /// overlay: `&VipsImage` -> Overlay image
    ///
    /// mode: `BlendMode` -> VipsBlendMode to join with
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// x: `i32` -> x position of overlay
    ///
    /// y: `i32` -> y position of overlay
    ///
    /// compositing_space: `Interpretation` -> Composite images in this colour space
    ///
    /// premultiplied: `bool` -> Images have premultiplied alpha
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "overlay",
                    v_value!(overlay),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "mode",
                    v_value!(mode as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Composite2 (vips_composite2) failed"),
        )
    }

    /// VipsComposite (composite), blend an array of images with an array of blend modes
    /// returns `VipsImage` - Output image
    ///
    /// inp: `&[VipsImage]` -> Array of input images
    ///
    /// mode: `&[i32]` -> Array of VipsBlendMode to join with
    pub fn composite(inp: &[VipsImage], mode: &[i32]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "composite",
            VOption::new()
                .with(
                    "in",
                    v_value!(inp),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "mode",
                    v_value!(mode),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Composite (vips_composite) failed"),
        )
    }

    /// VipsComposite (composite), blend an array of images with an array of blend modes
    /// returns `VipsImage` - Output image
    ///
    /// inp: `&[VipsImage]` -> Array of input images
    ///
    /// mode: `&[i32]` -> Array of VipsBlendMode to join with
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// x: `&[i32]` -> Array of x coordinates to join at
    ///
    /// y: `&[i32]` -> Array of y coordinates to join at
    ///
    /// compositing_space: `Interpretation` -> Composite images in this colour space
    ///
    /// premultiplied: `bool` -> Images have premultiplied alpha
    pub fn composite_with_opts(
        inp: &[VipsImage],
        mode: &[i32],
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "composite",
            option
                .with(
                    "in",
                    v_value!(inp),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "mode",
                    v_value!(mode),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Composite (vips_composite) failed"),
        )
    }

    /// VipsConv (conv), convolution operation
    /// returns `VipsImage` - Output image
    ///
    /// mask: `&VipsImage` -> Input matrix image
    pub fn conv(&self, mask: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "conv",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "mask",
                    v_value!(mask),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Conv (vips_conv) failed"),
        )
    }

    /// VipsConv (conv), convolution operation
    /// returns `VipsImage` - Output image
    ///
    /// mask: `&VipsImage` -> Input matrix image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// precision: `Precision` -> Convolve with this precision
    ///
    /// layers: `i32` -> Use this many layers in approximation
    ///
    /// cluster: `i32` -> Cluster lines closer than this in approximation
    pub fn conv_with_opts(&self, mask: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "conv",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "mask",
                    v_value!(mask),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Conv (vips_conv) failed"),
        )
    }

    /// VipsConva (conva), approximate integer convolution
    /// returns `VipsImage` - Output image
    ///
    /// mask: `&VipsImage` -> Input matrix image
    pub fn conva(&self, mask: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "conva",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "mask",
                    v_value!(mask),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Conva (vips_conva) failed"),
        )
    }

    /// VipsConva (conva), approximate integer convolution
    /// returns `VipsImage` - Output image
    ///
    /// mask: `&VipsImage` -> Input matrix image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// layers: `i32` -> Use this many layers in approximation
    ///
    /// cluster: `i32` -> Cluster lines closer than this in approximation
    pub fn conva_with_opts(&self, mask: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "conva",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "mask",
                    v_value!(mask),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Conva (vips_conva) failed"),
        )
    }

    /// VipsConvasep (convasep), approximate separable integer convolution
    /// returns `VipsImage` - Output image
    ///
    /// mask: `&VipsImage` -> Input matrix image
    pub fn convasep(&self, mask: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "convasep",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "mask",
                    v_value!(mask),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Convasep (vips_convasep) failed"),
        )
    }

    /// VipsConvasep (convasep), approximate separable integer convolution
    /// returns `VipsImage` - Output image
    ///
    /// mask: `&VipsImage` -> Input matrix image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// layers: `i32` -> Use this many layers in approximation
    pub fn convasep_with_opts(&self, mask: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "convasep",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "mask",
                    v_value!(mask),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Convasep (vips_convasep) failed"),
        )
    }

    /// VipsConvf (convf), float convolution operation
    /// returns `VipsImage` - Output image
    ///
    /// mask: `&VipsImage` -> Input matrix image
    pub fn convf(&self, mask: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "convf",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "mask",
                    v_value!(mask),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Convf (vips_convf) failed"),
        )
    }

    /// VipsConvi (convi), int convolution operation
    /// returns `VipsImage` - Output image
    ///
    /// mask: `&VipsImage` -> Input matrix image
    pub fn convi(&self, mask: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "convi",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "mask",
                    v_value!(mask),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Convi (vips_convi) failed"),
        )
    }

    /// VipsConvsep (convsep), separable convolution operation
    /// returns `VipsImage` - Output image
    ///
    /// mask: `&VipsImage` -> Input matrix image
    pub fn convsep(&self, mask: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "convsep",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "mask",
                    v_value!(mask),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Convsep (vips_convsep) failed"),
        )
    }

    /// VipsConvsep (convsep), separable convolution operation
    /// returns `VipsImage` - Output image
    ///
    /// mask: `&VipsImage` -> Input matrix image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// precision: `Precision` -> Convolve with this precision
    ///
    /// layers: `i32` -> Use this many layers in approximation
    ///
    /// cluster: `i32` -> Cluster lines closer than this in approximation
    pub fn convsep_with_opts(&self, mask: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "convsep",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "mask",
                    v_value!(mask),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Convsep (vips_convsep) failed"),
        )
    }

    /// VipsCopy (copy), copy an image
    /// returns `VipsImage` - Output image
    pub fn copy(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "copy",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Copy (vips_copy) failed"),
        )
    }

    /// VipsCopy (copy), copy an image
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// bands: `i32` -> Number of bands in image
    ///
    /// format: `BandFormat` -> Pixel format in image
    ///
    /// coding: `Coding` -> Pixel coding
    ///
    /// interpretation: `Interpretation` -> Pixel interpretation
    ///
    /// xres: `f64` -> Horizontal resolution in pixels/mm
    ///
    /// yres: `f64` -> Vertical resolution in pixels/mm
    ///
    /// xoffset: `i32` -> Horizontal offset of origin
    ///
    /// yoffset: `i32` -> Vertical offset of origin
    pub fn copy_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "copy",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Copy (vips_copy) failed"),
        )
    }

    /// VipsCountlines (countlines), count lines in an image
    /// returns `f64` - Number of lines
    ///
    /// direction: `Direction` -> Countlines left-right or up-down
    pub fn countlines(&self, direction: Direction) -> Result<f64> {
        let mut nolines_out: f64 = 0.0;
        let vips_op_response = call(
            "countlines",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "nolines",
                    v_value!(&mut nolines_out),
                )
                .with(
                    "direction",
                    v_value!(direction as i32),
                ),
        );
        utils::result(
            vips_op_response,
            nolines_out,
            Error::OperationError("Countlines (vips_countlines) failed"),
        )
    }

    /// VipsForeignLoadCsvFile (csvload), load csv (.csv), priority=0, untrusted, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn csvload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "csvload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Csvload (vips_csvload) failed"),
        )
    }

    /// VipsForeignLoadCsvFile (csvload), load csv (.csv), priority=0, untrusted, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// skip: `i32` -> Skip this many lines at the start of the file
    ///
    /// lines: `i32` -> Read this many lines from the file
    ///
    /// whitespace: `&str` -> Set of whitespace characters
    ///
    /// separator: `&str` -> Set of separator characters
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn csvload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "csvload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Csvload (vips_csvload) failed"),
        )
    }

    /// VipsForeignLoadCsvSource (csvload_source), load csv, priority=0, untrusted, is_a_source, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    pub fn csvload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "csvload_source",
            VOption::new()
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("CsvloadSource (vips_csvload_source) failed"),
        )
    }

    /// VipsForeignLoadCsvSource (csvload_source), load csv, priority=0, untrusted, is_a_source, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// skip: `i32` -> Skip this many lines at the start of the file
    ///
    /// lines: `i32` -> Read this many lines from the file
    ///
    /// whitespace: `&str` -> Set of whitespace characters
    ///
    /// separator: `&str` -> Set of separator characters
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn csvload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "csvload_source",
            option
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("CsvloadSource (vips_csvload_source) failed"),
        )
    }

    /// VipsForeignSaveCsvFile (csvsave), save image to csv (.csv), priority=0, mono
    ///
    /// filename: `&str` -> Filename to save to
    pub fn csvsave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "csvsave",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Csvsave (vips_csvsave) failed"),
        )
    }

    /// VipsForeignSaveCsvFile (csvsave), save image to csv (.csv), priority=0, mono
    ///
    /// filename: `&str` -> Filename to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// separator: `&str` -> Separator characters
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn csvsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "csvsave",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Csvsave (vips_csvsave) failed"),
        )
    }

    /// VipsForeignSaveCsvTarget (csvsave_target), save image to csv (.csv), priority=0, mono
    ///
    /// target: `&VipsTarget` -> Target to save to
    pub fn csvsave_target(&self, target: &VipsTarget) -> Result<()> {
        let vips_op_response = call(
            "csvsave_target",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("CsvsaveTarget (vips_csvsave_target) failed"),
        )
    }

    /// VipsForeignSaveCsvTarget (csvsave_target), save image to csv (.csv), priority=0, mono
    ///
    /// target: `&VipsTarget` -> Target to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// separator: `&str` -> Separator characters
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn csvsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "csvsave_target",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("CsvsaveTarget (vips_csvsave_target) failed"),
        )
    }

    /// VipsdE00 (dE00), calculate dE00
    /// returns `VipsImage` - Output image
    ///
    /// right: `&VipsImage` -> Right-hand input image
    pub fn dE00(&self, right: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "dE00",
            VOption::new()
                .with(
                    "left",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "right",
                    v_value!(right),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("DE00 (vips_dE00) failed"),
        )
    }

    /// VipsdE76 (dE76), calculate dE76
    /// returns `VipsImage` - Output image
    ///
    /// right: `&VipsImage` -> Right-hand input image
    pub fn dE76(&self, right: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "dE76",
            VOption::new()
                .with(
                    "left",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "right",
                    v_value!(right),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("DE76 (vips_dE76) failed"),
        )
    }

    /// VipsdECMC (dECMC), calculate dECMC
    /// returns `VipsImage` - Output image
    ///
    /// right: `&VipsImage` -> Right-hand input image
    pub fn dECMC(&self, right: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "dECMC",
            VOption::new()
                .with(
                    "left",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "right",
                    v_value!(right),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("DEcmc (vips_dECMC) failed"),
        )
    }

    /// VipsDeviate (deviate), find image standard deviation
    /// returns `f64` - Output value
    pub fn deviate(&self) -> Result<f64> {
        let mut out_out: f64 = 0.0;
        let vips_op_response = call(
            "deviate",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Deviate (vips_deviate) failed"),
        )
    }

    /// VipsDivide (divide), divide two images
    /// returns `VipsImage` - Output image
    ///
    /// right: `&VipsImage` -> Right-hand image argument
    pub fn divide(&self, right: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "divide",
            VOption::new()
                .with(
                    "left",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "right",
                    v_value!(right),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Divide (vips_divide) failed"),
        )
    }

    /// VipsDrawCircle (draw_circle), draw a circle on an image
    ///
    /// ink: `&[f64]` -> Color for pixels
    ///
    /// cx: `i32` -> Centre of draw_circle
    ///
    /// cy: `i32` -> Centre of draw_circle
    ///
    /// radius: `i32` -> Radius in pixels
    pub fn draw_circle(&self, ink: &[f64], cx: i32, cy: i32, radius: i32) -> Result<()> {
        let vips_op_response = call(
            "draw_circle",
            VOption::new()
                .with(
                    "image",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "ink",
                    v_value!(ink),
                )
                .with(
                    "cx",
                    v_value!(cx),
                )
                .with(
                    "cy",
                    v_value!(cy),
                )
                .with(
                    "radius",
                    v_value!(radius),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("DrawCircle (vips_draw_circle) failed"),
        )
    }

    /// VipsDrawCircle (draw_circle), draw a circle on an image
    ///
    /// ink: `&[f64]` -> Color for pixels
    ///
    /// cx: `i32` -> Centre of draw_circle
    ///
    /// cy: `i32` -> Centre of draw_circle
    ///
    /// radius: `i32` -> Radius in pixels
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// fill: `bool` -> Draw a solid object
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "ink",
                    v_value!(ink),
                )
                .with(
                    "cx",
                    v_value!(cx),
                )
                .with(
                    "cy",
                    v_value!(cy),
                )
                .with(
                    "radius",
                    v_value!(radius),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("DrawCircle (vips_draw_circle) failed"),
        )
    }

    /// VipsDrawFlood (draw_flood), flood-fill an area
    ///
    /// ink: `&[f64]` -> Color for pixels
    ///
    /// x: `i32` -> DrawFlood start point
    ///
    /// y: `i32` -> DrawFlood start point
    pub fn draw_flood(&self, ink: &[f64], x: i32, y: i32) -> Result<()> {
        let vips_op_response = call(
            "draw_flood",
            VOption::new()
                .with(
                    "image",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "ink",
                    v_value!(ink),
                )
                .with(
                    "x",
                    v_value!(x),
                )
                .with(
                    "y",
                    v_value!(y),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("DrawFlood (vips_draw_flood) failed"),
        )
    }

    /// VipsDrawFlood (draw_flood), flood-fill an area
    ///
    /// ink: `&[f64]` -> Color for pixels
    ///
    /// x: `i32` -> DrawFlood start point
    ///
    /// y: `i32` -> DrawFlood start point
    ///
    /// <ins>Optional arguments</ins>
    ///

    ///
    /// equal: `bool` -> DrawFlood while equal to edge
    ///
    /// left: `&mut i32` -> Left edge of modified area
    ///
    /// top: `&mut i32` -> Top edge of modified area
    ///
    /// width: `&mut i32` -> Width of modified area
    ///
    /// height: `&mut i32` -> Height of modified area
    pub fn draw_flood_with_opts(&self, ink: &[f64], x: i32, y: i32, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "draw_flood",
            option
                .with(
                    "image",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "ink",
                    v_value!(ink),
                )
                .with(
                    "x",
                    v_value!(x),
                )
                .with(
                    "y",
                    v_value!(y),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("DrawFlood (vips_draw_flood) failed"),
        )
    }

    /// VipsDrawImage (draw_image), paint an image into another image
    ///
    /// sub: `&VipsImage` -> Sub-image to insert into main image
    ///
    /// x: `i32` -> Draw image here
    ///
    /// y: `i32` -> Draw image here
    pub fn draw_image(&self, sub: &VipsImage, x: i32, y: i32) -> Result<()> {
        let vips_op_response = call(
            "draw_image",
            VOption::new()
                .with(
                    "image",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "sub",
                    v_value!(sub),
                )
                .with(
                    "x",
                    v_value!(x),
                )
                .with(
                    "y",
                    v_value!(y),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("DrawImage (vips_draw_image) failed"),
        )
    }

    /// VipsDrawImage (draw_image), paint an image into another image
    ///
    /// sub: `&VipsImage` -> Sub-image to insert into main image
    ///
    /// x: `i32` -> Draw image here
    ///
    /// y: `i32` -> Draw image here
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// mode: `CombineMode` -> Combining mode
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "sub",
                    v_value!(sub),
                )
                .with(
                    "x",
                    v_value!(x),
                )
                .with(
                    "y",
                    v_value!(y),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("DrawImage (vips_draw_image) failed"),
        )
    }

    /// VipsDrawLine (draw_line), draw a line on an image
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
        let vips_op_response = call(
            "draw_line",
            VOption::new()
                .with(
                    "image",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "ink",
                    v_value!(ink),
                )
                .with(
                    "x1",
                    v_value!(x1),
                )
                .with(
                    "y1",
                    v_value!(y1),
                )
                .with(
                    "x2",
                    v_value!(x2),
                )
                .with(
                    "y2",
                    v_value!(y2),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("DrawLine (vips_draw_line) failed"),
        )
    }

    /// VipsDrawMask (draw_mask), draw a mask on an image
    ///
    /// ink: `&[f64]` -> Color for pixels
    ///
    /// mask: `&VipsImage` -> Mask of pixels to draw
    ///
    /// x: `i32` -> Draw mask here
    ///
    /// y: `i32` -> Draw mask here
    pub fn draw_mask(&self, ink: &[f64], mask: &VipsImage, x: i32, y: i32) -> Result<()> {
        let vips_op_response = call(
            "draw_mask",
            VOption::new()
                .with(
                    "image",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "ink",
                    v_value!(ink),
                )
                .with(
                    "mask",
                    v_value!(mask),
                )
                .with(
                    "x",
                    v_value!(x),
                )
                .with(
                    "y",
                    v_value!(y),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("DrawMask (vips_draw_mask) failed"),
        )
    }

    /// VipsDrawRect (draw_rect), paint a rectangle on an image
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
        let vips_op_response = call(
            "draw_rect",
            VOption::new()
                .with(
                    "image",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "ink",
                    v_value!(ink),
                )
                .with(
                    "left",
                    v_value!(left),
                )
                .with(
                    "top",
                    v_value!(top),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("DrawRect (vips_draw_rect) failed"),
        )
    }

    /// VipsDrawRect (draw_rect), paint a rectangle on an image
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
    /// <ins>Optional arguments</ins>
    ///
    /// fill: `bool` -> Draw a solid object
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "ink",
                    v_value!(ink),
                )
                .with(
                    "left",
                    v_value!(left),
                )
                .with(
                    "top",
                    v_value!(top),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("DrawRect (vips_draw_rect) failed"),
        )
    }

    /// VipsDrawSmudge (draw_smudge), blur a rectangle on an image
    ///
    /// left: `i32` -> Rect to fill
    ///
    /// top: `i32` -> Rect to fill
    ///
    /// width: `i32` -> Rect to fill
    ///
    /// height: `i32` -> Rect to fill
    pub fn draw_smudge(&self, left: i32, top: i32, width: i32, height: i32) -> Result<()> {
        let vips_op_response = call(
            "draw_smudge",
            VOption::new()
                .with(
                    "image",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "left",
                    v_value!(left),
                )
                .with(
                    "top",
                    v_value!(top),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("DrawSmudge (vips_draw_smudge) failed"),
        )
    }

    /// VipsForeignSaveDzFile (dzsave), save image to deepzoom file (.dz, .szi), priority=0, any
    ///
    /// filename: `&str` -> Filename to save to
    pub fn dzsave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "dzsave",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Dzsave (vips_dzsave) failed"),
        )
    }

    /// VipsForeignSaveDzFile (dzsave), save image to deepzoom file (.dz, .szi), priority=0, any
    ///
    /// filename: `&str` -> Filename to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// imagename: `&str` -> Image name
    ///
    /// layout: `ForeignDzLayout` -> Directory layout
    ///
    /// suffix: `&str` -> Filename suffix for tiles
    ///
    /// overlap: `i32` -> Tile overlap in pixels
    ///
    /// tile_size: `i32` -> Tile size in pixels
    ///
    /// centre: `bool` -> Center image in tile
    ///
    /// depth: `ForeignDzDepth` -> Pyramid depth
    ///
    /// angle: `Angle` -> Rotate image during save
    ///
    /// container: `ForeignDzContainer` -> Pyramid container type
    ///
    /// compression: `i32` -> ZIP deflate compression level
    ///
    /// region_shrink: `RegionShrink` -> Method to shrink regions
    ///
    /// skip_blanks: `i32` -> Skip tiles which are nearly equal to the background
    ///
    /// id: `&str` -> Resource ID
    ///
    /// Q: `i32` -> Q factor
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn dzsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "dzsave",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Dzsave (vips_dzsave) failed"),
        )
    }

    /// VipsForeignSaveDzBuffer (dzsave_buffer), save image to dz buffer (.dz, .szi), priority=0, any
    /// returns `Vec<u8>` - Buffer to save to
    pub fn dzsave_buffer(&self) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "dzsave_buffer",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("DzsaveBuffer (vips_dzsave_buffer) failed"),
        )
    }

    /// VipsForeignSaveDzBuffer (dzsave_buffer), save image to dz buffer (.dz, .szi), priority=0, any
    /// returns `Vec<u8>` - Buffer to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// imagename: `&str` -> Image name
    ///
    /// layout: `ForeignDzLayout` -> Directory layout
    ///
    /// suffix: `&str` -> Filename suffix for tiles
    ///
    /// overlap: `i32` -> Tile overlap in pixels
    ///
    /// tile_size: `i32` -> Tile size in pixels
    ///
    /// centre: `bool` -> Center image in tile
    ///
    /// depth: `ForeignDzDepth` -> Pyramid depth
    ///
    /// angle: `Angle` -> Rotate image during save
    ///
    /// container: `ForeignDzContainer` -> Pyramid container type
    ///
    /// compression: `i32` -> ZIP deflate compression level
    ///
    /// region_shrink: `RegionShrink` -> Method to shrink regions
    ///
    /// skip_blanks: `i32` -> Skip tiles which are nearly equal to the background
    ///
    /// id: `&str` -> Resource ID
    ///
    /// Q: `i32` -> Q factor
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn dzsave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "dzsave_buffer",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("DzsaveBuffer (vips_dzsave_buffer) failed"),
        )
    }

    /// VipsForeignSaveDzTarget (dzsave_target), save image to deepzoom target (.dz, .szi), priority=0, any
    ///
    /// target: `&VipsTarget` -> Target to save to
    pub fn dzsave_target(&self, target: &VipsTarget) -> Result<()> {
        let vips_op_response = call(
            "dzsave_target",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("DzsaveTarget (vips_dzsave_target) failed"),
        )
    }

    /// VipsForeignSaveDzTarget (dzsave_target), save image to deepzoom target (.dz, .szi), priority=0, any
    ///
    /// target: `&VipsTarget` -> Target to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// imagename: `&str` -> Image name
    ///
    /// layout: `ForeignDzLayout` -> Directory layout
    ///
    /// suffix: `&str` -> Filename suffix for tiles
    ///
    /// overlap: `i32` -> Tile overlap in pixels
    ///
    /// tile_size: `i32` -> Tile size in pixels
    ///
    /// centre: `bool` -> Center image in tile
    ///
    /// depth: `ForeignDzDepth` -> Pyramid depth
    ///
    /// angle: `Angle` -> Rotate image during save
    ///
    /// container: `ForeignDzContainer` -> Pyramid container type
    ///
    /// compression: `i32` -> ZIP deflate compression level
    ///
    /// region_shrink: `RegionShrink` -> Method to shrink regions
    ///
    /// skip_blanks: `i32` -> Skip tiles which are nearly equal to the background
    ///
    /// id: `&str` -> Resource ID
    ///
    /// Q: `i32` -> Q factor
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn dzsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "dzsave_target",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("DzsaveTarget (vips_dzsave_target) failed"),
        )
    }

    /// VipsEmbed (embed), embed an image in a larger image
    /// returns `VipsImage` - Output image
    ///
    /// x: `i32` -> Left edge of input in output
    ///
    /// y: `i32` -> Top edge of input in output
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    pub fn embed(&self, x: i32, y: i32, width: i32, height: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "embed",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "x",
                    v_value!(x),
                )
                .with(
                    "y",
                    v_value!(y),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Embed (vips_embed) failed"),
        )
    }

    /// VipsEmbed (embed), embed an image in a larger image
    /// returns `VipsImage` - Output image
    ///
    /// x: `i32` -> Left edge of input in output
    ///
    /// y: `i32` -> Top edge of input in output
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// extend: `Extend` -> How to generate the extra pixels
    ///
    /// background: `&[f64]` -> Color for background pixels
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "x",
                    v_value!(x),
                )
                .with(
                    "y",
                    v_value!(y),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Embed (vips_embed) failed"),
        )
    }

    /// VipsExtractArea (extract_area), extract an area from an image
    /// returns `VipsImage` - Output image
    ///
    /// left: `i32` -> Left edge of extract area
    ///
    /// top: `i32` -> Top edge of extract area
    ///
    /// width: `i32` -> Width of extract area
    ///
    /// height: `i32` -> Height of extract area
    pub fn extract_area(&self, left: i32, top: i32, width: i32, height: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "extract_area",
            VOption::new()
                .with(
                    "input",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "left",
                    v_value!(left),
                )
                .with(
                    "top",
                    v_value!(top),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ExtractArea (vips_extract_area) failed"),
        )
    }

    /// crop (extract_area), extract an area from an image
    /// returns `VipsImage` - Output image
    ///
    /// left: `i32` -> Left edge of extract area
    ///
    /// top: `i32` -> Top edge of extract area
    ///
    /// width: `i32` -> Width of extract area
    ///
    /// height: `i32` -> Height of extract area
    pub fn crop(&self, left: i32, top: i32, width: i32, height: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "crop",
            VOption::new()
                .with(
                    "input",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "left",
                    v_value!(left),
                )
                .with(
                    "top",
                    v_value!(top),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Crop (vips_crop) failed"),
        )
    }

    /// VipsExtractBand (extract_band), extract band from an image
    /// returns `VipsImage` - Output image
    ///
    /// band: `i32` -> Band to extract
    pub fn extract_band(&self, band: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "extract_band",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "band",
                    v_value!(band),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ExtractBand (vips_extract_band) failed"),
        )
    }

    /// VipsExtractBand (extract_band), extract band from an image
    /// returns `VipsImage` - Output image
    ///
    /// band: `i32` -> Band to extract
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// n: `i32` -> Number of bands to extract
    pub fn extract_band_with_opts(&self, band: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "extract_band",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "band",
                    v_value!(band),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ExtractBand (vips_extract_band) failed"),
        )
    }

    /// VipsEye (eye), make an image showing the eye's spatial response
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    pub fn eye(width: i32, height: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "eye",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Eye (vips_eye) failed"),
        )
    }

    /// VipsEye (eye), make an image showing the eye's spatial response
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// uchar: `bool` -> Output an unsigned char image
    ///
    /// factor: `f64` -> Maximum spatial frequency
    pub fn eye_with_opts(width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "eye",
            option
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Eye (vips_eye) failed"),
        )
    }

    /// VipsFalsecolour (falsecolour), false-color an image
    /// returns `VipsImage` - Output image
    pub fn falsecolour(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "falsecolour",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Falsecolour (vips_falsecolour) failed"),
        )
    }

    /// VipsFastcor (fastcor), fast correlation
    /// returns `VipsImage` - Output image
    ///
    /// refp: `&VipsImage` -> Input reference image
    pub fn fastcor(&self, refp: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "fastcor",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "ref",
                    v_value!(refp),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Fastcor (vips_fastcor) failed"),
        )
    }

    /// VipsFillNearest (fill_nearest), fill image zeros with nearest non-zero pixel
    /// returns `VipsImage` - Value of nearest non-zero pixel
    pub fn fill_nearest(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "fill_nearest",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("FillNearest (vips_fill_nearest) failed"),
        )
    }

    /// VipsFillNearest (fill_nearest), fill image zeros with nearest non-zero pixel
    /// returns `VipsImage` - Value of nearest non-zero pixel
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// distance: `&mut VipsImage` -> Distance to nearest non-zero pixel
    pub fn fill_nearest_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "fill_nearest",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("FillNearest (vips_fill_nearest) failed"),
        )
    }

    /// VipsFindTrim (find_trim), search an image for non-edge areas
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
        let mut left_out: i32 = 1;
        let mut top_out: i32 = 0;
        let mut width_out: i32 = 1;
        let mut height_out: i32 = 1;
        let vips_op_response = call(
            "find_trim",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "left",
                    v_value!(&mut left_out),
                )
                .with(
                    "top",
                    v_value!(&mut top_out),
                )
                .with(
                    "width",
                    v_value!(&mut width_out),
                )
                .with(
                    "height",
                    v_value!(&mut height_out),
                ),
        );
        utils::result(
            vips_op_response,
            (
                left_out,
                top_out,
                width_out,
                height_out,
            ),
            Error::OperationError("FindTrim (vips_find_trim) failed"),
        )
    }

    /// VipsFindTrim (find_trim), search an image for non-edge areas
    /// Tuple (
    /// i32 - Left edge of image
    /// i32 - Top edge of extract area
    /// i32 - Width of extract area
    /// i32 - Height of extract area
    ///)
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// threshold: `f64` -> Object threshold
    ///
    /// background: `&[f64]` -> Color for background pixels
    ///
    /// line_art: `bool` -> Enable line art mode
    pub fn find_trim_with_opts(
        &self,
        option: VOption,
    ) -> Result<(
        i32,
        i32,
        i32,
        i32,
    )> {
        let mut left_out: i32 = 1;
        let mut top_out: i32 = 0;
        let mut width_out: i32 = 1;
        let mut height_out: i32 = 1;
        let vips_op_response = call(
            "find_trim",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "left",
                    v_value!(&mut left_out),
                )
                .with(
                    "top",
                    v_value!(&mut top_out),
                )
                .with(
                    "width",
                    v_value!(&mut width_out),
                )
                .with(
                    "height",
                    v_value!(&mut height_out),
                ),
        );
        utils::result(
            vips_op_response,
            (
                left_out,
                top_out,
                width_out,
                height_out,
            ),
            Error::OperationError("FindTrim (vips_find_trim) failed"),
        )
    }

    /// VipsForeignLoadFitsFile (fitsload), load a FITS image (.fits, .fit, .fts), priority=-50, untrusted, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn fitsload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "fitsload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Fitsload (vips_fitsload) failed"),
        )
    }

    /// VipsForeignLoadFitsFile (fitsload), load a FITS image (.fits, .fit, .fts), priority=-50, untrusted, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn fitsload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "fitsload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Fitsload (vips_fitsload) failed"),
        )
    }

    /// VipsForeignLoadFitsSource (fitsload_source), load FITS from a source, priority=-50, untrusted, is_a, is_a_source, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    pub fn fitsload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "fitsload_source",
            VOption::new()
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("FitsloadSource (vips_fitsload_source) failed"),
        )
    }

    /// VipsForeignLoadFitsSource (fitsload_source), load FITS from a source, priority=-50, untrusted, is_a, is_a_source, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn fitsload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "fitsload_source",
            option
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("FitsloadSource (vips_fitsload_source) failed"),
        )
    }

    /// VipsForeignSaveFits (fitssave), save image to fits file (.fits, .fit, .fts), priority=0, untrusted, any
    ///
    /// filename: `&str` -> Filename to save to
    pub fn fitssave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "fitssave",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Fitssave (vips_fitssave) failed"),
        )
    }

    /// VipsForeignSaveFits (fitssave), save image to fits file (.fits, .fit, .fts), priority=0, untrusted, any
    ///
    /// filename: `&str` -> Filename to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn fitssave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "fitssave",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Fitssave (vips_fitssave) failed"),
        )
    }

    /// VipsFlatten (flatten), flatten alpha out of an image
    /// returns `VipsImage` - Output image
    pub fn flatten(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "flatten",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Flatten (vips_flatten) failed"),
        )
    }

    /// VipsFlatten (flatten), flatten alpha out of an image
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// max_alpha: `f64` -> Maximum value of alpha channel
    pub fn flatten_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "flatten",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Flatten (vips_flatten) failed"),
        )
    }

    /// VipsFlip (flip), flip an image
    /// returns `VipsImage` - Output image
    ///
    /// direction: `Direction` -> Direction to flip image
    pub fn flip(&self, direction: Direction) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "flip",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "direction",
                    v_value!(direction as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Flip (vips_flip) failed"),
        )
    }

    /// VipsFloat2rad (float2rad), transform float RGB to Radiance coding
    /// returns `VipsImage` - Output image
    pub fn float2rad(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "float2rad",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Float2Rad (vips_float2rad) failed"),
        )
    }

    /// VipsFractsurf (fractsurf), make a fractal surface
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// fractal_dimension: `f64` -> Fractal dimension
    pub fn fractsurf(width: i32, height: i32, fractal_dimension: f64) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "fractsurf",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "fractal-dimension",
                    v_value!(fractal_dimension),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Fractsurf (vips_fractsurf) failed"),
        )
    }

    /// VipsFreqmult (freqmult), frequency-domain filtering
    /// returns `VipsImage` - Output image
    ///
    /// mask: `&VipsImage` -> Input mask image
    pub fn freqmult(&self, mask: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "freqmult",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "mask",
                    v_value!(mask),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Freqmult (vips_freqmult) failed"),
        )
    }

    /// VipsFwfft (fwfft), forward FFT
    /// returns `VipsImage` - Output image
    pub fn fwfft(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "fwfft",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Fwfft (vips_fwfft) failed"),
        )
    }

    /// VipsGamma (gamma), gamma an image
    /// returns `VipsImage` - Output image
    pub fn gamma(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "gamma",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Gamma (vips_gamma) failed"),
        )
    }

    /// VipsGamma (gamma), gamma an image
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// exponent: `f64` -> Gamma factor
    pub fn gamma_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "gamma",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Gamma (vips_gamma) failed"),
        )
    }

    /// VipsGaussblur (gaussblur), gaussian blur
    /// returns `VipsImage` - Output image
    ///
    /// sigma: `f64` -> Sigma of Gaussian
    pub fn gaussblur(&self, sigma: f64) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "gaussblur",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "sigma",
                    v_value!(sigma),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Gaussblur (vips_gaussblur) failed"),
        )
    }

    /// VipsGaussblur (gaussblur), gaussian blur
    /// returns `VipsImage` - Output image
    ///
    /// sigma: `f64` -> Sigma of Gaussian
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// min_ampl: `f64` -> Minimum amplitude of Gaussian
    ///
    /// precision: `Precision` -> Convolve with this precision
    pub fn gaussblur_with_opts(&self, sigma: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "gaussblur",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "sigma",
                    v_value!(sigma),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Gaussblur (vips_gaussblur) failed"),
        )
    }

    /// VipsGaussmat (gaussmat), make a gaussian image
    /// returns `VipsImage` - Output image
    ///
    /// sigma: `f64` -> Sigma of Gaussian
    ///
    /// min_ampl: `f64` -> Minimum amplitude of Gaussian
    pub fn gaussmat(sigma: f64, min_ampl: f64) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "gaussmat",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "sigma",
                    v_value!(sigma),
                )
                .with(
                    "min-ampl",
                    v_value!(min_ampl),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Gaussmat (vips_gaussmat) failed"),
        )
    }

    /// VipsGaussmat (gaussmat), make a gaussian image
    /// returns `VipsImage` - Output image
    ///
    /// sigma: `f64` -> Sigma of Gaussian
    ///
    /// min_ampl: `f64` -> Minimum amplitude of Gaussian
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// separable: `bool` -> Generate separable Gaussian
    ///
    /// precision: `Precision` -> Generate with this precision
    pub fn gaussmat_with_opts(sigma: f64, min_ampl: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "gaussmat",
            option
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "sigma",
                    v_value!(sigma),
                )
                .with(
                    "min-ampl",
                    v_value!(min_ampl),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Gaussmat (vips_gaussmat) failed"),
        )
    }

    /// VipsGaussnoise (gaussnoise), make a gaussnoise image
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    pub fn gaussnoise(width: i32, height: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "gaussnoise",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Gaussnoise (vips_gaussnoise) failed"),
        )
    }

    /// VipsGaussnoise (gaussnoise), make a gaussnoise image
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// sigma: `f64` -> Standard deviation of pixels in generated image
    ///
    /// mean: `f64` -> Mean of pixels in generated image
    ///
    /// seed: `i32` -> Random number seed
    pub fn gaussnoise_with_opts(width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "gaussnoise",
            option
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Gaussnoise (vips_gaussnoise) failed"),
        )
    }

    /// VipsGetpoint (getpoint), read a point from an image
    /// returns `Vec<f64>` - Array of output values
    ///
    /// x: `i32` -> Point to read
    ///
    /// y: `i32` -> Point to read
    pub fn getpoint(&self, x: i32, y: i32) -> Result<Vec<f64>> {
        let mut out_array_out: Vec<f64> = Vec::new();
        let vips_op_response = call(
            "getpoint",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out-array",
                    v_value!(&mut out_array_out),
                )
                .with(
                    "x",
                    v_value!(x),
                )
                .with(
                    "y",
                    v_value!(y),
                ),
        );
        utils::result(
            vips_op_response,
            out_array_out,
            Error::OperationError("Getpoint (vips_getpoint) failed"),
        )
    }

    /// VipsGetpoint (getpoint), read a point from an image
    /// returns `Vec<f64>` - Array of output values
    ///
    /// x: `i32` -> Point to read
    ///
    /// y: `i32` -> Point to read
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// unpack_complex: `bool` -> Complex pixels should be unpacked
    pub fn getpoint_with_opts(&self, x: i32, y: i32, option: VOption) -> Result<Vec<f64>> {
        let mut out_array_out: Vec<f64> = Vec::new();
        let vips_op_response = call(
            "getpoint",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out-array",
                    v_value!(&mut out_array_out),
                )
                .with(
                    "x",
                    v_value!(x),
                )
                .with(
                    "y",
                    v_value!(y),
                ),
        );
        utils::result(
            vips_op_response,
            out_array_out,
            Error::OperationError("Getpoint (vips_getpoint) failed"),
        )
    }

    /// VipsForeignLoadNsgifFile (gifload), load GIF with libnsgif (.gif), priority=50, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn gifload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "gifload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Gifload (vips_gifload) failed"),
        )
    }

    /// VipsForeignLoadNsgifFile (gifload), load GIF with libnsgif (.gif), priority=50, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// n: `i32` -> Number of pages to load, -1 for all
    ///
    /// page: `i32` -> First page to load
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn gifload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "gifload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Gifload (vips_gifload) failed"),
        )
    }

    /// VipsForeignLoadNsgifBuffer (gifload_buffer), load GIF with libnsgif, priority=50, is_a_buffer, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    pub fn gifload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "gifload_buffer",
            VOption::new()
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("GifloadBuffer (vips_gifload_buffer) failed"),
        )
    }

    /// VipsForeignLoadNsgifBuffer (gifload_buffer), load GIF with libnsgif, priority=50, is_a_buffer, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// n: `i32` -> Number of pages to load, -1 for all
    ///
    /// page: `i32` -> First page to load
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn gifload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "gifload_buffer",
            option
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("GifloadBuffer (vips_gifload_buffer) failed"),
        )
    }

    /// VipsForeignLoadNsgifSource (gifload_source), load gif from source, priority=50, is_a_source, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    pub fn gifload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "gifload_source",
            VOption::new()
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("GifloadSource (vips_gifload_source) failed"),
        )
    }

    /// VipsForeignLoadNsgifSource (gifload_source), load gif from source, priority=50, is_a_source, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// n: `i32` -> Number of pages to load, -1 for all
    ///
    /// page: `i32` -> First page to load
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn gifload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "gifload_source",
            option
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("GifloadSource (vips_gifload_source) failed"),
        )
    }

    /// VipsForeignSaveCgifFile (gifsave), save as gif (.gif), priority=0, rgba-only
    ///
    /// filename: `&str` -> Filename to save to
    pub fn gifsave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "gifsave",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Gifsave (vips_gifsave) failed"),
        )
    }

    /// VipsForeignSaveCgifFile (gifsave), save as gif (.gif), priority=0, rgba-only
    ///
    /// filename: `&str` -> Filename to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// dither: `f64` -> Amount of dithering
    ///
    /// effort: `i32` -> Quantisation effort
    ///
    /// bitdepth: `i32` -> Number of bits per pixel
    ///
    /// interframe_maxerror: `f64` -> Maximum inter-frame error for transparency
    ///
    /// reuse: `bool` -> Reuse palette from input
    ///
    /// interpalette_maxerror: `f64` -> Maximum inter-palette error for palette reusage
    ///
    /// interlace: `bool` -> Generate an interlaced (progressive) GIF
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn gifsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "gifsave",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Gifsave (vips_gifsave) failed"),
        )
    }

    /// VipsForeignSaveCgifBuffer (gifsave_buffer), save as gif (.gif), priority=0, rgba-only
    /// returns `Vec<u8>` - Buffer to save to
    pub fn gifsave_buffer(&self) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "gifsave_buffer",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("GifsaveBuffer (vips_gifsave_buffer) failed"),
        )
    }

    /// VipsForeignSaveCgifBuffer (gifsave_buffer), save as gif (.gif), priority=0, rgba-only
    /// returns `Vec<u8>` - Buffer to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// dither: `f64` -> Amount of dithering
    ///
    /// effort: `i32` -> Quantisation effort
    ///
    /// bitdepth: `i32` -> Number of bits per pixel
    ///
    /// interframe_maxerror: `f64` -> Maximum inter-frame error for transparency
    ///
    /// reuse: `bool` -> Reuse palette from input
    ///
    /// interpalette_maxerror: `f64` -> Maximum inter-palette error for palette reusage
    ///
    /// interlace: `bool` -> Generate an interlaced (progressive) GIF
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn gifsave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "gifsave_buffer",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("GifsaveBuffer (vips_gifsave_buffer) failed"),
        )
    }

    /// VipsForeignSaveCgifTarget (gifsave_target), save as gif (.gif), priority=0, rgba-only
    ///
    /// target: `&VipsTarget` -> Target to save to
    pub fn gifsave_target(&self, target: &VipsTarget) -> Result<()> {
        let vips_op_response = call(
            "gifsave_target",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("GifsaveTarget (vips_gifsave_target) failed"),
        )
    }

    /// VipsForeignSaveCgifTarget (gifsave_target), save as gif (.gif), priority=0, rgba-only
    ///
    /// target: `&VipsTarget` -> Target to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// dither: `f64` -> Amount of dithering
    ///
    /// effort: `i32` -> Quantisation effort
    ///
    /// bitdepth: `i32` -> Number of bits per pixel
    ///
    /// interframe_maxerror: `f64` -> Maximum inter-frame error for transparency
    ///
    /// reuse: `bool` -> Reuse palette from input
    ///
    /// interpalette_maxerror: `f64` -> Maximum inter-palette error for palette reusage
    ///
    /// interlace: `bool` -> Generate an interlaced (progressive) GIF
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn gifsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "gifsave_target",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("GifsaveTarget (vips_gifsave_target) failed"),
        )
    }

    /// VipsGlobalbalance (globalbalance), global balance an image mosaic
    /// returns `VipsImage` - Output image
    pub fn globalbalance(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "globalbalance",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Globalbalance (vips_globalbalance) failed"),
        )
    }

    /// VipsGlobalbalance (globalbalance), global balance an image mosaic
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// gamma: `f64` -> Image gamma
    ///
    /// int_output: `bool` -> Integer output
    pub fn globalbalance_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "globalbalance",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Globalbalance (vips_globalbalance) failed"),
        )
    }

    /// VipsGravity (gravity), place an image within a larger image with a certain gravity
    /// returns `VipsImage` - Output image
    ///
    /// direction: `CompassDirection` -> Direction to place image within width/height
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    pub fn gravity(
        &self,
        direction: CompassDirection,
        width: i32,
        height: i32,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "gravity",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "direction",
                    v_value!(direction as i32),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Gravity (vips_gravity) failed"),
        )
    }

    /// VipsGravity (gravity), place an image within a larger image with a certain gravity
    /// returns `VipsImage` - Output image
    ///
    /// direction: `CompassDirection` -> Direction to place image within width/height
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// extend: `Extend` -> How to generate the extra pixels
    ///
    /// background: `&[f64]` -> Color for background pixels
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "direction",
                    v_value!(direction as i32),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Gravity (vips_gravity) failed"),
        )
    }

    /// VipsGrey (grey), make a grey ramp image
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    pub fn grey(width: i32, height: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "grey",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Grey (vips_grey) failed"),
        )
    }

    /// VipsGrey (grey), make a grey ramp image
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// uchar: `bool` -> Output an unsigned char image
    pub fn grey_with_opts(width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "grey",
            option
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Grey (vips_grey) failed"),
        )
    }

    /// VipsGrid (grid), grid an image
    /// returns `VipsImage` - Output image
    ///
    /// tile_height: `i32` -> Chop into tiles this high
    ///
    /// across: `i32` -> Number of tiles across
    ///
    /// down: `i32` -> Number of tiles down
    pub fn grid(&self, tile_height: i32, across: i32, down: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "grid",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "tile-height",
                    v_value!(tile_height),
                )
                .with(
                    "across",
                    v_value!(across),
                )
                .with(
                    "down",
                    v_value!(down),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Grid (vips_grid) failed"),
        )
    }

    /// VipsForeignLoadHeifFile (heifload), load a HEIF image (.heic, .heif, .avif), priority=0, is_a, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn heifload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "heifload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Heifload (vips_heifload) failed"),
        )
    }

    /// VipsForeignLoadHeifFile (heifload), load a HEIF image (.heic, .heif, .avif), priority=0, is_a, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// page: `i32` -> First page to load
    ///
    /// n: `i32` -> Number of pages to load, -1 for all
    ///
    /// thumbnail: `bool` -> Fetch thumbnail image
    ///
    /// unlimited: `bool` -> Remove all denial of service limits
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn heifload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "heifload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Heifload (vips_heifload) failed"),
        )
    }

    /// VipsForeignLoadHeifBuffer (heifload_buffer), load a HEIF image, priority=0, is_a_buffer, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    pub fn heifload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "heifload_buffer",
            VOption::new()
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HeifloadBuffer (vips_heifload_buffer) failed"),
        )
    }

    /// VipsForeignLoadHeifBuffer (heifload_buffer), load a HEIF image, priority=0, is_a_buffer, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// page: `i32` -> First page to load
    ///
    /// n: `i32` -> Number of pages to load, -1 for all
    ///
    /// thumbnail: `bool` -> Fetch thumbnail image
    ///
    /// unlimited: `bool` -> Remove all denial of service limits
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn heifload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "heifload_buffer",
            option
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HeifloadBuffer (vips_heifload_buffer) failed"),
        )
    }

    /// VipsForeignLoadHeifSource (heifload_source), load a HEIF image, priority=0, is_a_source, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    pub fn heifload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "heifload_source",
            VOption::new()
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HeifloadSource (vips_heifload_source) failed"),
        )
    }

    /// VipsForeignLoadHeifSource (heifload_source), load a HEIF image, priority=0, is_a_source, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// page: `i32` -> First page to load
    ///
    /// n: `i32` -> Number of pages to load, -1 for all
    ///
    /// thumbnail: `bool` -> Fetch thumbnail image
    ///
    /// unlimited: `bool` -> Remove all denial of service limits
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn heifload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "heifload_source",
            option
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HeifloadSource (vips_heifload_source) failed"),
        )
    }

    /// VipsForeignSaveHeifFile (heifsave), save image in HEIF format (.heic, .heif, .avif), priority=0, rgba-only
    ///
    /// filename: `&str` -> Filename to save to
    pub fn heifsave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "heifsave",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Heifsave (vips_heifsave) failed"),
        )
    }

    /// VipsForeignSaveHeifFile (heifsave), save image in HEIF format (.heic, .heif, .avif), priority=0, rgba-only
    ///
    /// filename: `&str` -> Filename to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// Q: `i32` -> Q factor
    ///
    /// bitdepth: `i32` -> Number of bits per pixel
    ///
    /// lossless: `bool` -> Enable lossless compression
    ///
    /// compression: `ForeignHeifCompression` -> Compression format
    ///
    /// effort: `i32` -> CPU effort
    ///
    /// subsample_mode: `ForeignSubsample` -> Select chroma subsample operation mode
    ///
    /// encoder: `ForeignHeifEncoder` -> Select encoder to use
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn heifsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "heifsave",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Heifsave (vips_heifsave) failed"),
        )
    }

    /// VipsForeignSaveHeifBuffer (heifsave_buffer), save image in HEIF format (.heic, .heif), priority=0, rgba-only
    /// returns `Vec<u8>` - Buffer to save to
    pub fn heifsave_buffer(&self) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "heifsave_buffer",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("HeifsaveBuffer (vips_heifsave_buffer) failed"),
        )
    }

    /// VipsForeignSaveHeifBuffer (heifsave_buffer), save image in HEIF format (.heic, .heif), priority=0, rgba-only
    /// returns `Vec<u8>` - Buffer to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// Q: `i32` -> Q factor
    ///
    /// bitdepth: `i32` -> Number of bits per pixel
    ///
    /// lossless: `bool` -> Enable lossless compression
    ///
    /// compression: `ForeignHeifCompression` -> Compression format
    ///
    /// effort: `i32` -> CPU effort
    ///
    /// subsample_mode: `ForeignSubsample` -> Select chroma subsample operation mode
    ///
    /// encoder: `ForeignHeifEncoder` -> Select encoder to use
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn heifsave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "heifsave_buffer",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("HeifsaveBuffer (vips_heifsave_buffer) failed"),
        )
    }

    /// VipsForeignSaveHeifTarget (heifsave_target), save image in HEIF format (.heic, .heif), priority=0, rgba-only
    ///
    /// target: `&VipsTarget` -> Target to save to
    pub fn heifsave_target(&self, target: &VipsTarget) -> Result<()> {
        let vips_op_response = call(
            "heifsave_target",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("HeifsaveTarget (vips_heifsave_target) failed"),
        )
    }

    /// VipsForeignSaveHeifTarget (heifsave_target), save image in HEIF format (.heic, .heif), priority=0, rgba-only
    ///
    /// target: `&VipsTarget` -> Target to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// Q: `i32` -> Q factor
    ///
    /// bitdepth: `i32` -> Number of bits per pixel
    ///
    /// lossless: `bool` -> Enable lossless compression
    ///
    /// compression: `ForeignHeifCompression` -> Compression format
    ///
    /// effort: `i32` -> CPU effort
    ///
    /// subsample_mode: `ForeignSubsample` -> Select chroma subsample operation mode
    ///
    /// encoder: `ForeignHeifEncoder` -> Select encoder to use
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn heifsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "heifsave_target",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("HeifsaveTarget (vips_heifsave_target) failed"),
        )
    }

    /// VipsHistCum (hist_cum), form cumulative histogram
    /// returns `VipsImage` - Output image
    pub fn hist_cum(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "hist_cum",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HistCum (vips_hist_cum) failed"),
        )
    }

    /// VipsHistEntropy (hist_entropy), estimate image entropy
    /// returns `f64` - Output value
    pub fn hist_entropy(&self) -> Result<f64> {
        let mut out_out: f64 = 0.0;
        let vips_op_response = call(
            "hist_entropy",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HistEntropy (vips_hist_entropy) failed"),
        )
    }

    /// VipsHistEqual (hist_equal), histogram equalisation
    /// returns `VipsImage` - Output image
    pub fn hist_equal(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "hist_equal",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HistEqual (vips_hist_equal) failed"),
        )
    }

    /// VipsHistEqual (hist_equal), histogram equalisation
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// band: `i32` -> Equalise with this band
    pub fn hist_equal_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "hist_equal",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HistEqual (vips_hist_equal) failed"),
        )
    }

    /// VipsHistFind (hist_find), find image histogram
    /// returns `VipsImage` - Output histogram
    pub fn hist_find(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "hist_find",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HistFind (vips_hist_find) failed"),
        )
    }

    /// VipsHistFind (hist_find), find image histogram
    /// returns `VipsImage` - Output histogram
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// band: `i32` -> Find histogram of band
    pub fn hist_find_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "hist_find",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HistFind (vips_hist_find) failed"),
        )
    }

    /// VipsHistFindIndexed (hist_find_indexed), find indexed image histogram
    /// returns `VipsImage` - Output histogram
    ///
    /// index: `&VipsImage` -> Index image
    pub fn hist_find_indexed(&self, index: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "hist_find_indexed",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "index",
                    v_value!(index),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HistFindIndexed (vips_hist_find_indexed) failed"),
        )
    }

    /// VipsHistFindIndexed (hist_find_indexed), find indexed image histogram
    /// returns `VipsImage` - Output histogram
    ///
    /// index: `&VipsImage` -> Index image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// combine: `Combine` -> Combine bins like this
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "index",
                    v_value!(index),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HistFindIndexed (vips_hist_find_indexed) failed"),
        )
    }

    /// VipsHistFindNDim (hist_find_ndim), find n-dimensional image histogram
    /// returns `VipsImage` - Output histogram
    pub fn hist_find_ndim(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "hist_find_ndim",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HistFindNdim (vips_hist_find_ndim) failed"),
        )
    }

    /// VipsHistFindNDim (hist_find_ndim), find n-dimensional image histogram
    /// returns `VipsImage` - Output histogram
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// bins: `i32` -> Number of bins in each dimension
    pub fn hist_find_ndim_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "hist_find_ndim",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HistFindNdim (vips_hist_find_ndim) failed"),
        )
    }

    /// VipsHistIsmonotonic (hist_ismonotonic), test for monotonicity
    /// returns `bool` - true if in is monotonic
    pub fn hist_ismonotonic(&self) -> Result<bool> {
        let mut monotonic_out: bool = false;
        let vips_op_response = call(
            "hist_ismonotonic",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "monotonic",
                    v_value!(&mut monotonic_out),
                ),
        );
        utils::result(
            vips_op_response,
            monotonic_out,
            Error::OperationError("HistIsmonotonic (vips_hist_ismonotonic) failed"),
        )
    }

    /// VipsHistLocal (hist_local), local histogram equalisation
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Window width in pixels
    ///
    /// height: `i32` -> Window height in pixels
    pub fn hist_local(&self, width: i32, height: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "hist_local",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HistLocal (vips_hist_local) failed"),
        )
    }

    /// VipsHistLocal (hist_local), local histogram equalisation
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Window width in pixels
    ///
    /// height: `i32` -> Window height in pixels
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// max_slope: `i32` -> Maximum slope (CLAHE)
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HistLocal (vips_hist_local) failed"),
        )
    }

    /// VipsHistMatch (hist_match), match two histograms
    /// returns `VipsImage` - Output image
    ///
    /// refp: `&VipsImage` -> Reference histogram
    pub fn hist_match(&self, refp: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "hist_match",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "ref",
                    v_value!(refp),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HistMatch (vips_hist_match) failed"),
        )
    }

    /// VipsHistNorm (hist_norm), normalise histogram
    /// returns `VipsImage` - Output image
    pub fn hist_norm(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "hist_norm",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HistNorm (vips_hist_norm) failed"),
        )
    }

    /// VipsHistPlot (hist_plot), plot histogram
    /// returns `VipsImage` - Output image
    pub fn hist_plot(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "hist_plot",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HistPlot (vips_hist_plot) failed"),
        )
    }

    /// VipsHoughCircle (hough_circle), find hough circle transform
    /// returns `VipsImage` - Output image
    pub fn hough_circle(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "hough_circle",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HoughCircle (vips_hough_circle) failed"),
        )
    }

    /// VipsHoughCircle (hough_circle), find hough circle transform
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// scale: `i32` -> Scale down dimensions by this factor
    ///
    /// min_radius: `i32` -> Smallest radius to search for
    ///
    /// max_radius: `i32` -> Largest radius to search for
    pub fn hough_circle_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "hough_circle",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HoughCircle (vips_hough_circle) failed"),
        )
    }

    /// VipsHoughLine (hough_line), find hough line transform
    /// returns `VipsImage` - Output image
    pub fn hough_line(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "hough_line",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HoughLine (vips_hough_line) failed"),
        )
    }

    /// VipsHoughLine (hough_line), find hough line transform
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// width: `i32` -> Horizontal size of parameter space
    ///
    /// height: `i32` -> Vertical size of parameter space
    pub fn hough_line_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "hough_line",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("HoughLine (vips_hough_line) failed"),
        )
    }

    /// VipsIccExport (icc_export), output to device with ICC profile
    /// returns `VipsImage` - Output image
    pub fn icc_export(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "icc_export",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("IccExport (vips_icc_export) failed"),
        )
    }

    /// VipsIccExport (icc_export), output to device with ICC profile
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// pcs: `PCS` -> Set Profile Connection Space
    ///
    /// intent: `Intent` -> Rendering intent
    ///
    /// black_point_compensation: `bool` -> Enable black point compensation
    ///
    /// output_profile: `&str` -> Filename to load output profile from
    ///
    /// depth: `i32` -> Output device space depth in bits
    pub fn icc_export_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "icc_export",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("IccExport (vips_icc_export) failed"),
        )
    }

    /// VipsIccImport (icc_import), import from device with ICC profile
    /// returns `VipsImage` - Output image
    pub fn icc_import(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "icc_import",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("IccImport (vips_icc_import) failed"),
        )
    }

    /// VipsIccImport (icc_import), import from device with ICC profile
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// pcs: `PCS` -> Set Profile Connection Space
    ///
    /// intent: `Intent` -> Rendering intent
    ///
    /// black_point_compensation: `bool` -> Enable black point compensation
    ///
    /// embedded: `bool` -> Use embedded input profile, if available
    ///
    /// input_profile: `&str` -> Filename to load input profile from
    pub fn icc_import_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "icc_import",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("IccImport (vips_icc_import) failed"),
        )
    }

    /// VipsIccTransform (icc_transform), transform between devices with ICC profiles
    /// returns `VipsImage` - Output image
    ///
    /// output_profile: `&str` -> Filename to load output profile from
    pub fn icc_transform(&self, output_profile: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "icc_transform",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "output-profile",
                    v_value!(output_profile),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("IccTransform (vips_icc_transform) failed"),
        )
    }

    /// VipsIccTransform (icc_transform), transform between devices with ICC profiles
    /// returns `VipsImage` - Output image
    ///
    /// output_profile: `&str` -> Filename to load output profile from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// pcs: `PCS` -> Set Profile Connection Space
    ///
    /// intent: `Intent` -> Rendering intent
    ///
    /// black_point_compensation: `bool` -> Enable black point compensation
    ///
    /// embedded: `bool` -> Use embedded input profile, if available
    ///
    /// input_profile: `&str` -> Filename to load input profile from
    ///
    /// depth: `i32` -> Output device space depth in bits
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "output-profile",
                    v_value!(output_profile),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("IccTransform (vips_icc_transform) failed"),
        )
    }

    /// VipsIdentity (identity), make a 1D image where pixel values are indexes
    /// returns `VipsImage` - Output image
    pub fn identity() -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "identity",
            VOption::new().with(
                "out",
                v_value!(&mut out_out),
            ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Identity (vips_identity) failed"),
        )
    }

    /// VipsIdentity (identity), make a 1D image where pixel values are indexes
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// bands: `i32` -> Number of bands in LUT
    ///
    /// ushort: `bool` -> Create a 16-bit LUT
    ///
    /// size: `i32` -> Size of 16-bit LUT
    pub fn identity_with_opts(option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "identity",
            option.with(
                "out",
                v_value!(&mut out_out),
            ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Identity (vips_identity) failed"),
        )
    }

    /// VipsIfthenelse (ifthenelse), ifthenelse an image
    /// returns `VipsImage` - Output image
    ///
    /// in1: `&VipsImage` -> Source for TRUE pixels
    ///
    /// in2: `&VipsImage` -> Source for FALSE pixels
    pub fn ifthenelse(&self, in1: &VipsImage, in2: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "ifthenelse",
            VOption::new()
                .with(
                    "cond",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "in1",
                    v_value!(in1),
                )
                .with(
                    "in2",
                    v_value!(in2),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Ifthenelse (vips_ifthenelse) failed"),
        )
    }

    /// VipsIfthenelse (ifthenelse), ifthenelse an image
    /// returns `VipsImage` - Output image
    ///
    /// in1: `&VipsImage` -> Source for TRUE pixels
    ///
    /// in2: `&VipsImage` -> Source for FALSE pixels
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// blend: `bool` -> Blend smoothly between then and else parts
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "in1",
                    v_value!(in1),
                )
                .with(
                    "in2",
                    v_value!(in2),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Ifthenelse (vips_ifthenelse) failed"),
        )
    }

    /// VipsInsert (insert), insert image @sub into @main at @x, @y
    /// returns `VipsImage` - Output image
    ///
    /// sub: `&VipsImage` -> Sub-image to insert into main image
    ///
    /// x: `i32` -> Left edge of sub in main
    ///
    /// y: `i32` -> Top edge of sub in main
    pub fn insert(&self, sub: &VipsImage, x: i32, y: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "insert",
            VOption::new()
                .with(
                    "main",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "sub",
                    v_value!(sub),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "x",
                    v_value!(x),
                )
                .with(
                    "y",
                    v_value!(y),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Insert (vips_insert) failed"),
        )
    }

    /// VipsInsert (insert), insert image @sub into @main at @x, @y
    /// returns `VipsImage` - Output image
    ///
    /// sub: `&VipsImage` -> Sub-image to insert into main image
    ///
    /// x: `i32` -> Left edge of sub in main
    ///
    /// y: `i32` -> Top edge of sub in main
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// expand: `bool` -> Expand output to hold all of both inputs
    ///
    /// background: `&[f64]` -> Color for new pixels
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "sub",
                    v_value!(sub),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "x",
                    v_value!(x),
                )
                .with(
                    "y",
                    v_value!(y),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Insert (vips_insert) failed"),
        )
    }

    /// VipsInvert (invert), invert an image
    /// returns `VipsImage` - Output image
    pub fn invert(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "invert",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Invert (vips_invert) failed"),
        )
    }

    /// VipsInvertlut (invertlut), build an inverted look-up table
    /// returns `VipsImage` - Output image
    pub fn invertlut(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "invertlut",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Invertlut (vips_invertlut) failed"),
        )
    }

    /// VipsInvertlut (invertlut), build an inverted look-up table
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// size: `i32` -> LUT size to generate
    pub fn invertlut_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "invertlut",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Invertlut (vips_invertlut) failed"),
        )
    }

    /// VipsInvfft (invfft), inverse FFT
    /// returns `VipsImage` - Output image
    pub fn invfft(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "invfft",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Invfft (vips_invfft) failed"),
        )
    }

    /// VipsInvfft (invfft), inverse FFT
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// real: `bool` -> Output only the real part of the transform
    pub fn invfft_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "invfft",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Invfft (vips_invfft) failed"),
        )
    }

    /// VipsJoin (join), join a pair of images
    /// returns `VipsImage` - Output image
    ///
    /// in2: `&VipsImage` -> Second input image
    ///
    /// direction: `Direction` -> Join left-right or up-down
    pub fn join(&self, in2: &VipsImage, direction: Direction) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "join",
            VOption::new()
                .with(
                    "in1",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "in2",
                    v_value!(in2),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "direction",
                    v_value!(direction as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Join (vips_join) failed"),
        )
    }

    /// VipsJoin (join), join a pair of images
    /// returns `VipsImage` - Output image
    ///
    /// in2: `&VipsImage` -> Second input image
    ///
    /// direction: `Direction` -> Join left-right or up-down
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// expand: `bool` -> Expand output to hold all of both inputs
    ///
    /// shim: `i32` -> Pixels between images
    ///
    /// background: `&[f64]` -> Colour for new pixels
    ///
    /// align: `Align` -> Align on the low, centre or high coordinate edge
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "in2",
                    v_value!(in2),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "direction",
                    v_value!(direction as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Join (vips_join) failed"),
        )
    }

    /// VipsForeignLoadJp2kFile (jp2kload), load JPEG2000 image (.j2k, .jp2, .jpt, .j2c, .jpc), priority=0, untrusted, is_a, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn jp2kload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "jp2kload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Jp2Kload (vips_jp2kload) failed"),
        )
    }

    /// VipsForeignLoadJp2kFile (jp2kload), load JPEG2000 image (.j2k, .jp2, .jpt, .j2c, .jpc), priority=0, untrusted, is_a, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// page: `i32` -> Load this page from the image
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn jp2kload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "jp2kload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Jp2Kload (vips_jp2kload) failed"),
        )
    }

    /// VipsForeignLoadJp2kBuffer (jp2kload_buffer), load JPEG2000 image, priority=0, untrusted, is_a_buffer, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    pub fn jp2kload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "jp2kload_buffer",
            VOption::new()
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Jp2KloadBuffer (vips_jp2kload_buffer) failed"),
        )
    }

    /// VipsForeignLoadJp2kBuffer (jp2kload_buffer), load JPEG2000 image, priority=0, untrusted, is_a_buffer, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// page: `i32` -> Load this page from the image
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn jp2kload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "jp2kload_buffer",
            option
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Jp2KloadBuffer (vips_jp2kload_buffer) failed"),
        )
    }

    /// VipsForeignLoadJp2kSource (jp2kload_source), load JPEG2000 image, priority=0, untrusted, is_a_source, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    pub fn jp2kload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "jp2kload_source",
            VOption::new()
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Jp2KloadSource (vips_jp2kload_source) failed"),
        )
    }

    /// VipsForeignLoadJp2kSource (jp2kload_source), load JPEG2000 image, priority=0, untrusted, is_a_source, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// page: `i32` -> Load this page from the image
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn jp2kload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "jp2kload_source",
            option
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Jp2KloadSource (vips_jp2kload_source) failed"),
        )
    }

    /// VipsForeignSaveJp2kFile (jp2ksave), save image in JPEG2000 format (.j2k, .jp2, .jpt, .j2c, .jpc), priority=0, any
    ///
    /// filename: `&str` -> Filename to save to
    pub fn jp2ksave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "jp2ksave",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Jp2Ksave (vips_jp2ksave) failed"),
        )
    }

    /// VipsForeignSaveJp2kFile (jp2ksave), save image in JPEG2000 format (.j2k, .jp2, .jpt, .j2c, .jpc), priority=0, any
    ///
    /// filename: `&str` -> Filename to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// tile_width: `i32` -> Tile width in pixels
    ///
    /// tile_height: `i32` -> Tile height in pixels
    ///
    /// lossless: `bool` -> Enable lossless compression
    ///
    /// Q: `i32` -> Q factor
    ///
    /// subsample_mode: `ForeignSubsample` -> Select chroma subsample operation mode
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn jp2ksave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jp2ksave",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Jp2Ksave (vips_jp2ksave) failed"),
        )
    }

    /// VipsForeignSaveJp2kBuffer (jp2ksave_buffer), save image in JPEG2000 format (.j2k, .jp2, .jpt, .j2c, .jpc), priority=0, any
    /// returns `Vec<u8>` - Buffer to save to
    pub fn jp2ksave_buffer(&self) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "jp2ksave_buffer",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("Jp2KsaveBuffer (vips_jp2ksave_buffer) failed"),
        )
    }

    /// VipsForeignSaveJp2kBuffer (jp2ksave_buffer), save image in JPEG2000 format (.j2k, .jp2, .jpt, .j2c, .jpc), priority=0, any
    /// returns `Vec<u8>` - Buffer to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// tile_width: `i32` -> Tile width in pixels
    ///
    /// tile_height: `i32` -> Tile height in pixels
    ///
    /// lossless: `bool` -> Enable lossless compression
    ///
    /// Q: `i32` -> Q factor
    ///
    /// subsample_mode: `ForeignSubsample` -> Select chroma subsample operation mode
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn jp2ksave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "jp2ksave_buffer",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("Jp2KsaveBuffer (vips_jp2ksave_buffer) failed"),
        )
    }

    /// VipsForeignSaveJp2kTarget (jp2ksave_target), save image in JPEG2000 format (.j2k, .jp2, .jpt, .j2c, .jpc), priority=0, any
    ///
    /// target: `&VipsTarget` -> Target to save to
    pub fn jp2ksave_target(&self, target: &VipsTarget) -> Result<()> {
        let vips_op_response = call(
            "jp2ksave_target",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Jp2KsaveTarget (vips_jp2ksave_target) failed"),
        )
    }

    /// VipsForeignSaveJp2kTarget (jp2ksave_target), save image in JPEG2000 format (.j2k, .jp2, .jpt, .j2c, .jpc), priority=0, any
    ///
    /// target: `&VipsTarget` -> Target to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// tile_width: `i32` -> Tile width in pixels
    ///
    /// tile_height: `i32` -> Tile height in pixels
    ///
    /// lossless: `bool` -> Enable lossless compression
    ///
    /// Q: `i32` -> Q factor
    ///
    /// subsample_mode: `ForeignSubsample` -> Select chroma subsample operation mode
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn jp2ksave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jp2ksave_target",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Jp2KsaveTarget (vips_jp2ksave_target) failed"),
        )
    }

    /// VipsForeignLoadJpegFile (jpegload), load jpeg from file (.jpg, .jpeg, .jpe, .jfif), priority=50, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn jpegload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "jpegload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Jpegload (vips_jpegload) failed"),
        )
    }

    /// VipsForeignLoadJpegFile (jpegload), load jpeg from file (.jpg, .jpeg, .jpe, .jfif), priority=50, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// shrink: `i32` -> Shrink factor on load
    ///
    /// autorotate: `bool` -> Rotate image using exif orientation
    ///
    /// unlimited: `bool` -> Remove all denial of service limits
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn jpegload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "jpegload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Jpegload (vips_jpegload) failed"),
        )
    }

    /// VipsForeignLoadJpegBuffer (jpegload_buffer), load jpeg from buffer, priority=50, is_a_buffer, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    pub fn jpegload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "jpegload_buffer",
            VOption::new()
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("JpegloadBuffer (vips_jpegload_buffer) failed"),
        )
    }

    /// VipsForeignLoadJpegBuffer (jpegload_buffer), load jpeg from buffer, priority=50, is_a_buffer, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// shrink: `i32` -> Shrink factor on load
    ///
    /// autorotate: `bool` -> Rotate image using exif orientation
    ///
    /// unlimited: `bool` -> Remove all denial of service limits
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn jpegload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "jpegload_buffer",
            option
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("JpegloadBuffer (vips_jpegload_buffer) failed"),
        )
    }

    /// VipsForeignLoadJpegSource (jpegload_source), load image from jpeg source, priority=50, is_a_source, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    pub fn jpegload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "jpegload_source",
            VOption::new()
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("JpegloadSource (vips_jpegload_source) failed"),
        )
    }

    /// VipsForeignLoadJpegSource (jpegload_source), load image from jpeg source, priority=50, is_a_source, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// shrink: `i32` -> Shrink factor on load
    ///
    /// autorotate: `bool` -> Rotate image using exif orientation
    ///
    /// unlimited: `bool` -> Remove all denial of service limits
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn jpegload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "jpegload_source",
            option
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("JpegloadSource (vips_jpegload_source) failed"),
        )
    }

    /// VipsForeignSaveJpegFile (jpegsave), save image to jpeg file (.jpg, .jpeg, .jpe, .jfif), priority=0, rgb-cmyk
    ///
    /// filename: `&str` -> Filename to save to
    pub fn jpegsave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "jpegsave",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Jpegsave (vips_jpegsave) failed"),
        )
    }

    /// VipsForeignSaveJpegFile (jpegsave), save image to jpeg file (.jpg, .jpeg, .jpe, .jfif), priority=0, rgb-cmyk
    ///
    /// filename: `&str` -> Filename to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// Q: `i32` -> Q factor
    ///
    /// optimize_coding: `bool` -> Compute optimal Huffman coding tables
    ///
    /// interlace: `bool` -> Generate an interlaced (progressive) jpeg
    ///
    /// trellis_quant: `bool` -> Apply trellis quantisation to each 8x8 block
    ///
    /// overshoot_deringing: `bool` -> Apply overshooting to samples with extreme values
    ///
    /// optimize_scans: `bool` -> Split spectrum of DCT coefficients into separate scans
    ///
    /// quant_table: `i32` -> Use predefined quantization table with given index
    ///
    /// subsample_mode: `ForeignSubsample` -> Select chroma subsample operation mode
    ///
    /// restart_interval: `i32` -> Add restart markers every specified number of mcu
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn jpegsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jpegsave",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Jpegsave (vips_jpegsave) failed"),
        )
    }

    /// VipsForeignSaveJpegBuffer (jpegsave_buffer), save image to jpeg buffer (.jpg, .jpeg, .jpe, .jfif), priority=0, rgb-cmyk
    /// returns `Vec<u8>` - Buffer to save to
    pub fn jpegsave_buffer(&self) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "jpegsave_buffer",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("JpegsaveBuffer (vips_jpegsave_buffer) failed"),
        )
    }

    /// VipsForeignSaveJpegBuffer (jpegsave_buffer), save image to jpeg buffer (.jpg, .jpeg, .jpe, .jfif), priority=0, rgb-cmyk
    /// returns `Vec<u8>` - Buffer to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// Q: `i32` -> Q factor
    ///
    /// optimize_coding: `bool` -> Compute optimal Huffman coding tables
    ///
    /// interlace: `bool` -> Generate an interlaced (progressive) jpeg
    ///
    /// trellis_quant: `bool` -> Apply trellis quantisation to each 8x8 block
    ///
    /// overshoot_deringing: `bool` -> Apply overshooting to samples with extreme values
    ///
    /// optimize_scans: `bool` -> Split spectrum of DCT coefficients into separate scans
    ///
    /// quant_table: `i32` -> Use predefined quantization table with given index
    ///
    /// subsample_mode: `ForeignSubsample` -> Select chroma subsample operation mode
    ///
    /// restart_interval: `i32` -> Add restart markers every specified number of mcu
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn jpegsave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "jpegsave_buffer",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("JpegsaveBuffer (vips_jpegsave_buffer) failed"),
        )
    }

    /// VipsForeignSaveJpegMime (jpegsave_mime), save image to jpeg mime (.jpg, .jpeg, .jpe, .jfif), priority=0, rgb-cmyk
    pub fn jpegsave_mime(&self) -> Result<()> {
        let vips_op_response = call(
            "jpegsave_mime",
            VOption::new().with(
                "in",
                v_value!(&VipsImage::from(self.ctx)),
            ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("JpegsaveMime (vips_jpegsave_mime) failed"),
        )
    }

    /// VipsForeignSaveJpegMime (jpegsave_mime), save image to jpeg mime (.jpg, .jpeg, .jpe, .jfif), priority=0, rgb-cmyk
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// Q: `i32` -> Q factor
    ///
    /// optimize_coding: `bool` -> Compute optimal Huffman coding tables
    ///
    /// interlace: `bool` -> Generate an interlaced (progressive) jpeg
    ///
    /// trellis_quant: `bool` -> Apply trellis quantisation to each 8x8 block
    ///
    /// overshoot_deringing: `bool` -> Apply overshooting to samples with extreme values
    ///
    /// optimize_scans: `bool` -> Split spectrum of DCT coefficients into separate scans
    ///
    /// quant_table: `i32` -> Use predefined quantization table with given index
    ///
    /// subsample_mode: `ForeignSubsample` -> Select chroma subsample operation mode
    ///
    /// restart_interval: `i32` -> Add restart markers every specified number of mcu
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn jpegsave_mime_with_opts(&self, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jpegsave_mime",
            option.with(
                "in",
                v_value!(&VipsImage::from(self.ctx)),
            ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("JpegsaveMime (vips_jpegsave_mime) failed"),
        )
    }

    /// VipsForeignSaveJpegTarget (jpegsave_target), save image to jpeg target (.jpg, .jpeg, .jpe, .jfif), priority=0, rgb-cmyk
    ///
    /// target: `&VipsTarget` -> Target to save to
    pub fn jpegsave_target(&self, target: &VipsTarget) -> Result<()> {
        let vips_op_response = call(
            "jpegsave_target",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("JpegsaveTarget (vips_jpegsave_target) failed"),
        )
    }

    /// VipsForeignSaveJpegTarget (jpegsave_target), save image to jpeg target (.jpg, .jpeg, .jpe, .jfif), priority=0, rgb-cmyk
    ///
    /// target: `&VipsTarget` -> Target to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// Q: `i32` -> Q factor
    ///
    /// optimize_coding: `bool` -> Compute optimal Huffman coding tables
    ///
    /// interlace: `bool` -> Generate an interlaced (progressive) jpeg
    ///
    /// trellis_quant: `bool` -> Apply trellis quantisation to each 8x8 block
    ///
    /// overshoot_deringing: `bool` -> Apply overshooting to samples with extreme values
    ///
    /// optimize_scans: `bool` -> Split spectrum of DCT coefficients into separate scans
    ///
    /// quant_table: `i32` -> Use predefined quantization table with given index
    ///
    /// subsample_mode: `ForeignSubsample` -> Select chroma subsample operation mode
    ///
    /// restart_interval: `i32` -> Add restart markers every specified number of mcu
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn jpegsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jpegsave_target",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("JpegsaveTarget (vips_jpegsave_target) failed"),
        )
    }

    /// VipsForeignLoadJxlFile (jxlload), load JPEG-XL image (.jxl), priority=0, untrusted, is_a, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn jxlload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "jxlload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Jxlload (vips_jxlload) failed"),
        )
    }

    /// VipsForeignLoadJxlFile (jxlload), load JPEG-XL image (.jxl), priority=0, untrusted, is_a, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// page: `i32` -> First page to load
    ///
    /// n: `i32` -> Number of pages to load, -1 for all
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn jxlload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "jxlload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Jxlload (vips_jxlload) failed"),
        )
    }

    /// VipsForeignLoadJxlBuffer (jxlload_buffer), load JPEG-XL image, priority=0, untrusted, is_a_buffer, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    pub fn jxlload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "jxlload_buffer",
            VOption::new()
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("JxlloadBuffer (vips_jxlload_buffer) failed"),
        )
    }

    /// VipsForeignLoadJxlBuffer (jxlload_buffer), load JPEG-XL image, priority=0, untrusted, is_a_buffer, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// page: `i32` -> First page to load
    ///
    /// n: `i32` -> Number of pages to load, -1 for all
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn jxlload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "jxlload_buffer",
            option
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("JxlloadBuffer (vips_jxlload_buffer) failed"),
        )
    }

    /// VipsForeignLoadJxlSource (jxlload_source), load JPEG-XL image, priority=0, untrusted, is_a_source, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    pub fn jxlload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "jxlload_source",
            VOption::new()
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("JxlloadSource (vips_jxlload_source) failed"),
        )
    }

    /// VipsForeignLoadJxlSource (jxlload_source), load JPEG-XL image, priority=0, untrusted, is_a_source, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// page: `i32` -> First page to load
    ///
    /// n: `i32` -> Number of pages to load, -1 for all
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn jxlload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "jxlload_source",
            option
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("JxlloadSource (vips_jxlload_source) failed"),
        )
    }

    /// VipsForeignSaveJxlFile (jxlsave), save image in JPEG-XL format (.jxl), priority=0, untrusted, any
    ///
    /// filename: `&str` -> Filename to save to
    pub fn jxlsave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "jxlsave",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Jxlsave (vips_jxlsave) failed"),
        )
    }

    /// VipsForeignSaveJxlFile (jxlsave), save image in JPEG-XL format (.jxl), priority=0, untrusted, any
    ///
    /// filename: `&str` -> Filename to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// tier: `i32` -> Decode speed tier
    ///
    /// distance: `f64` -> Target butteraugli distance
    ///
    /// effort: `i32` -> Encoding effort
    ///
    /// lossless: `bool` -> Enable lossless compression
    ///
    /// Q: `i32` -> Quality factor
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn jxlsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jxlsave",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Jxlsave (vips_jxlsave) failed"),
        )
    }

    /// VipsForeignSaveJxlBuffer (jxlsave_buffer), save image in JPEG-XL format (.jxl), priority=0, untrusted, any
    /// returns `Vec<u8>` - Buffer to save to
    pub fn jxlsave_buffer(&self) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "jxlsave_buffer",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("JxlsaveBuffer (vips_jxlsave_buffer) failed"),
        )
    }

    /// VipsForeignSaveJxlBuffer (jxlsave_buffer), save image in JPEG-XL format (.jxl), priority=0, untrusted, any
    /// returns `Vec<u8>` - Buffer to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// tier: `i32` -> Decode speed tier
    ///
    /// distance: `f64` -> Target butteraugli distance
    ///
    /// effort: `i32` -> Encoding effort
    ///
    /// lossless: `bool` -> Enable lossless compression
    ///
    /// Q: `i32` -> Quality factor
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn jxlsave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "jxlsave_buffer",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("JxlsaveBuffer (vips_jxlsave_buffer) failed"),
        )
    }

    /// VipsForeignSaveJxlTarget (jxlsave_target), save image in JPEG-XL format (.jxl), priority=0, untrusted, any
    ///
    /// target: `&VipsTarget` -> Target to save to
    pub fn jxlsave_target(&self, target: &VipsTarget) -> Result<()> {
        let vips_op_response = call(
            "jxlsave_target",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("JxlsaveTarget (vips_jxlsave_target) failed"),
        )
    }

    /// VipsForeignSaveJxlTarget (jxlsave_target), save image in JPEG-XL format (.jxl), priority=0, untrusted, any
    ///
    /// target: `&VipsTarget` -> Target to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// tier: `i32` -> Decode speed tier
    ///
    /// distance: `f64` -> Target butteraugli distance
    ///
    /// effort: `i32` -> Encoding effort
    ///
    /// lossless: `bool` -> Enable lossless compression
    ///
    /// Q: `i32` -> Quality factor
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn jxlsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "jxlsave_target",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("JxlsaveTarget (vips_jxlsave_target) failed"),
        )
    }

    /// VipsLabelregions (labelregions), label regions in an image
    /// returns `VipsImage` - Mask of region labels
    pub fn labelregions(&self) -> Result<VipsImage> {
        let mut mask_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "labelregions",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "mask",
                    v_value!(&mut mask_out),
                ),
        );
        utils::result(
            vips_op_response,
            mask_out,
            Error::OperationError("Labelregions (vips_labelregions) failed"),
        )
    }

    /// VipsLabelregions (labelregions), label regions in an image
    /// returns `VipsImage` - Mask of region labels
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// segments: `&mut i32` -> Number of discrete contiguous regions
    pub fn labelregions_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut mask_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "labelregions",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "mask",
                    v_value!(&mut mask_out),
                ),
        );
        utils::result(
            vips_op_response,
            mask_out,
            Error::OperationError("Labelregions (vips_labelregions) failed"),
        )
    }

    /// VipsLinear (linear), calculate (a * in + b)
    /// returns `VipsImage` - Output image
    ///
    /// a: `&[f64]` -> Multiply by this
    ///
    /// b: `&[f64]` -> Add this
    pub fn linear(&self, a: &[f64], b: &[f64]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "linear",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "a",
                    v_value!(a),
                )
                .with(
                    "b",
                    v_value!(b),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Linear (vips_linear) failed"),
        )
    }

    /// VipsLinear (linear), calculate (a * in + b)
    /// returns `VipsImage` - Output image
    ///
    /// a: `&[f64]` -> Multiply by this
    ///
    /// b: `&[f64]` -> Add this
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// uchar: `bool` -> Output should be uchar
    pub fn linear_with_opts(&self, a: &[f64], b: &[f64], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "linear",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "a",
                    v_value!(a),
                )
                .with(
                    "b",
                    v_value!(b),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Linear (vips_linear) failed"),
        )
    }

    /// VipsLineCache (linecache), cache an image as a set of lines
    /// returns `VipsImage` - Output image
    pub fn linecache(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "linecache",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Linecache (vips_linecache) failed"),
        )
    }

    /// VipsLineCache (linecache), cache an image as a set of lines
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// tile_height: `i32` -> Tile height in pixels
    ///
    /// access: `Access` -> Expected access pattern
    ///
    /// threaded: `bool` -> Allow threaded access
    ///
    /// persistent: `bool` -> Keep cache between evaluations
    pub fn linecache_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "linecache",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Linecache (vips_linecache) failed"),
        )
    }

    /// VipsLogmat (logmat), make a Laplacian of Gaussian image
    /// returns `VipsImage` - Output image
    ///
    /// sigma: `f64` -> Radius of Gaussian
    ///
    /// min_ampl: `f64` -> Minimum amplitude of Gaussian
    pub fn logmat(sigma: f64, min_ampl: f64) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "logmat",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "sigma",
                    v_value!(sigma),
                )
                .with(
                    "min-ampl",
                    v_value!(min_ampl),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Logmat (vips_logmat) failed"),
        )
    }

    /// VipsLogmat (logmat), make a Laplacian of Gaussian image
    /// returns `VipsImage` - Output image
    ///
    /// sigma: `f64` -> Radius of Gaussian
    ///
    /// min_ampl: `f64` -> Minimum amplitude of Gaussian
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// separable: `bool` -> Generate separable Gaussian
    ///
    /// precision: `Precision` -> Generate with this precision
    pub fn logmat_with_opts(sigma: f64, min_ampl: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "logmat",
            option
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "sigma",
                    v_value!(sigma),
                )
                .with(
                    "min-ampl",
                    v_value!(min_ampl),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Logmat (vips_logmat) failed"),
        )
    }

    /// VipsForeignLoadMagickFile (magickload), load file with ImageMagick, priority=-100, untrusted, is_a, get_flags, get_flags_filename, header
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn magickload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "magickload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Magickload (vips_magickload) failed"),
        )
    }

    /// VipsForeignLoadMagickFile (magickload), load file with ImageMagick, priority=-100, untrusted, is_a, get_flags, get_flags_filename, header
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// density: `&str` -> Canvas resolution for rendering vector formats like SVG
    ///
    /// page: `i32` -> First page to load
    ///
    /// n: `i32` -> Number of pages to load, -1 for all
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn magickload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "magickload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Magickload (vips_magickload) failed"),
        )
    }

    /// VipsForeignLoadMagickBuffer (magickload_buffer), load buffer with ImageMagick, priority=-100, untrusted, is_a_buffer, get_flags, get_flags_filename, header
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    pub fn magickload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "magickload_buffer",
            VOption::new()
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MagickloadBuffer (vips_magickload_buffer) failed"),
        )
    }

    /// VipsForeignLoadMagickBuffer (magickload_buffer), load buffer with ImageMagick, priority=-100, untrusted, is_a_buffer, get_flags, get_flags_filename, header
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// density: `&str` -> Canvas resolution for rendering vector formats like SVG
    ///
    /// page: `i32` -> First page to load
    ///
    /// n: `i32` -> Number of pages to load, -1 for all
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn magickload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "magickload_buffer",
            option
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MagickloadBuffer (vips_magickload_buffer) failed"),
        )
    }

    /// VipsForeignSaveMagickFile (magicksave), save file with ImageMagick (), priority=-100, untrusted, any
    ///
    /// filename: `&str` -> Filename to save to
    pub fn magicksave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "magicksave",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Magicksave (vips_magicksave) failed"),
        )
    }

    /// VipsForeignSaveMagickFile (magicksave), save file with ImageMagick (), priority=-100, untrusted, any
    ///
    /// filename: `&str` -> Filename to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// format: `&str` -> Format to save in
    ///
    /// quality: `i32` -> Quality to use
    ///
    /// optimize_gif_frames: `bool` -> Apply GIF frames optimization
    ///
    /// optimize_gif_transparency: `bool` -> Apply GIF transparency optimization
    ///
    /// bitdepth: `i32` -> Number of bits per pixel
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn magicksave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "magicksave",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Magicksave (vips_magicksave) failed"),
        )
    }

    /// VipsForeignSaveMagickBuffer (magicksave_buffer), save image to magick buffer (), priority=-100, untrusted, any
    /// returns `Vec<u8>` - Buffer to save to
    pub fn magicksave_buffer(&self) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "magicksave_buffer",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("MagicksaveBuffer (vips_magicksave_buffer) failed"),
        )
    }

    /// VipsForeignSaveMagickBuffer (magicksave_buffer), save image to magick buffer (), priority=-100, untrusted, any
    /// returns `Vec<u8>` - Buffer to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// format: `&str` -> Format to save in
    ///
    /// quality: `i32` -> Quality to use
    ///
    /// optimize_gif_frames: `bool` -> Apply GIF frames optimization
    ///
    /// optimize_gif_transparency: `bool` -> Apply GIF transparency optimization
    ///
    /// bitdepth: `i32` -> Number of bits per pixel
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn magicksave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "magicksave_buffer",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("MagicksaveBuffer (vips_magicksave_buffer) failed"),
        )
    }

    /// VipsMapim (mapim), resample with a map image
    /// returns `VipsImage` - Output image
    ///
    /// index: `&VipsImage` -> Index pixels with this
    pub fn mapim(&self, index: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "mapim",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "index",
                    v_value!(index),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Mapim (vips_mapim) failed"),
        )
    }

    /// VipsMapim (mapim), resample with a map image
    /// returns `VipsImage` - Output image
    ///
    /// index: `&VipsImage` -> Index pixels with this
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// interpolate: `&VipsInterpolate` -> Interpolate pixels with this
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// premultiplied: `bool` -> Images have premultiplied alpha
    ///
    /// extend: `Extend` -> How to generate the extra pixels
    pub fn mapim_with_opts(&self, index: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "mapim",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "index",
                    v_value!(index),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Mapim (vips_mapim) failed"),
        )
    }

    /// VipsMaplut (maplut), map an image though a lut
    /// returns `VipsImage` - Output image
    ///
    /// lut: `&VipsImage` -> Look-up table image
    pub fn maplut(&self, lut: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "maplut",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "lut",
                    v_value!(lut),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Maplut (vips_maplut) failed"),
        )
    }

    /// VipsMaplut (maplut), map an image though a lut
    /// returns `VipsImage` - Output image
    ///
    /// lut: `&VipsImage` -> Look-up table image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// band: `i32` -> Apply one-band lut to this band of in
    pub fn maplut_with_opts(&self, lut: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "maplut",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "lut",
                    v_value!(lut),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Maplut (vips_maplut) failed"),
        )
    }

    /// VipsMaskButterworth (mask_butterworth), make a butterworth filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// order: `f64` -> Filter order
    ///
    /// frequency_cutoff: `f64` -> Frequency cutoff
    ///
    /// amplitude_cutoff: `f64` -> Amplitude cutoff
    pub fn mask_butterworth(
        width: i32,
        height: i32,
        order: f64,
        frequency_cutoff: f64,
        amplitude_cutoff: f64,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "mask_butterworth",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "order",
                    v_value!(order),
                )
                .with(
                    "frequency-cutoff",
                    v_value!(frequency_cutoff),
                )
                .with(
                    "amplitude-cutoff",
                    v_value!(amplitude_cutoff),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskButterworth (vips_mask_butterworth) failed"),
        )
    }

    /// VipsMaskButterworth (mask_butterworth), make a butterworth filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// order: `f64` -> Filter order
    ///
    /// frequency_cutoff: `f64` -> Frequency cutoff
    ///
    /// amplitude_cutoff: `f64` -> Amplitude cutoff
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// uchar: `bool` -> Output an unsigned char image
    ///
    /// nodc: `bool` -> Remove DC component
    ///
    /// reject: `bool` -> Invert the sense of the filter
    ///
    /// optical: `bool` -> Rotate quadrants to optical space
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
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "order",
                    v_value!(order),
                )
                .with(
                    "frequency-cutoff",
                    v_value!(frequency_cutoff),
                )
                .with(
                    "amplitude-cutoff",
                    v_value!(amplitude_cutoff),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskButterworth (vips_mask_butterworth) failed"),
        )
    }

    /// VipsMaskButterworthBand (mask_butterworth_band), make a butterworth_band filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// order: `f64` -> Filter order
    ///
    /// frequency_cutoff_x: `f64` -> Frequency cutoff x
    ///
    /// frequency_cutoff_y: `f64` -> Frequency cutoff y
    ///
    /// radius: `f64` -> Radius of circle
    ///
    /// amplitude_cutoff: `f64` -> Amplitude cutoff
    pub fn mask_butterworth_band(
        width: i32,
        height: i32,
        order: f64,
        frequency_cutoff_x: f64,
        frequency_cutoff_y: f64,
        radius: f64,
        amplitude_cutoff: f64,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "mask_butterworth_band",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "order",
                    v_value!(order),
                )
                .with(
                    "frequency-cutoff-x",
                    v_value!(frequency_cutoff_x),
                )
                .with(
                    "frequency-cutoff-y",
                    v_value!(frequency_cutoff_y),
                )
                .with(
                    "radius",
                    v_value!(radius),
                )
                .with(
                    "amplitude-cutoff",
                    v_value!(amplitude_cutoff),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskButterworthBand (vips_mask_butterworth_band) failed"),
        )
    }

    /// VipsMaskButterworthBand (mask_butterworth_band), make a butterworth_band filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// order: `f64` -> Filter order
    ///
    /// frequency_cutoff_x: `f64` -> Frequency cutoff x
    ///
    /// frequency_cutoff_y: `f64` -> Frequency cutoff y
    ///
    /// radius: `f64` -> Radius of circle
    ///
    /// amplitude_cutoff: `f64` -> Amplitude cutoff
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// uchar: `bool` -> Output an unsigned char image
    ///
    /// nodc: `bool` -> Remove DC component
    ///
    /// reject: `bool` -> Invert the sense of the filter
    ///
    /// optical: `bool` -> Rotate quadrants to optical space
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
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "order",
                    v_value!(order),
                )
                .with(
                    "frequency-cutoff-x",
                    v_value!(frequency_cutoff_x),
                )
                .with(
                    "frequency-cutoff-y",
                    v_value!(frequency_cutoff_y),
                )
                .with(
                    "radius",
                    v_value!(radius),
                )
                .with(
                    "amplitude-cutoff",
                    v_value!(amplitude_cutoff),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskButterworthBand (vips_mask_butterworth_band) failed"),
        )
    }

    /// VipsMaskButterworthRing (mask_butterworth_ring), make a butterworth ring filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// order: `f64` -> Filter order
    ///
    /// frequency_cutoff: `f64` -> Frequency cutoff
    ///
    /// amplitude_cutoff: `f64` -> Amplitude cutoff
    ///
    /// ringwidth: `f64` -> Ringwidth
    pub fn mask_butterworth_ring(
        width: i32,
        height: i32,
        order: f64,
        frequency_cutoff: f64,
        amplitude_cutoff: f64,
        ringwidth: f64,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "mask_butterworth_ring",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "order",
                    v_value!(order),
                )
                .with(
                    "frequency-cutoff",
                    v_value!(frequency_cutoff),
                )
                .with(
                    "amplitude-cutoff",
                    v_value!(amplitude_cutoff),
                )
                .with(
                    "ringwidth",
                    v_value!(ringwidth),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskButterworthRing (vips_mask_butterworth_ring) failed"),
        )
    }

    /// VipsMaskButterworthRing (mask_butterworth_ring), make a butterworth ring filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// order: `f64` -> Filter order
    ///
    /// frequency_cutoff: `f64` -> Frequency cutoff
    ///
    /// amplitude_cutoff: `f64` -> Amplitude cutoff
    ///
    /// ringwidth: `f64` -> Ringwidth
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// uchar: `bool` -> Output an unsigned char image
    ///
    /// nodc: `bool` -> Remove DC component
    ///
    /// reject: `bool` -> Invert the sense of the filter
    ///
    /// optical: `bool` -> Rotate quadrants to optical space
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
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "order",
                    v_value!(order),
                )
                .with(
                    "frequency-cutoff",
                    v_value!(frequency_cutoff),
                )
                .with(
                    "amplitude-cutoff",
                    v_value!(amplitude_cutoff),
                )
                .with(
                    "ringwidth",
                    v_value!(ringwidth),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskButterworthRing (vips_mask_butterworth_ring) failed"),
        )
    }

    /// VipsMaskFractal (mask_fractal), make fractal filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// fractal_dimension: `f64` -> Fractal dimension
    pub fn mask_fractal(width: i32, height: i32, fractal_dimension: f64) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "mask_fractal",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "fractal-dimension",
                    v_value!(fractal_dimension),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskFractal (vips_mask_fractal) failed"),
        )
    }

    /// VipsMaskFractal (mask_fractal), make fractal filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// fractal_dimension: `f64` -> Fractal dimension
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// uchar: `bool` -> Output an unsigned char image
    ///
    /// nodc: `bool` -> Remove DC component
    ///
    /// reject: `bool` -> Invert the sense of the filter
    ///
    /// optical: `bool` -> Rotate quadrants to optical space
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
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "fractal-dimension",
                    v_value!(fractal_dimension),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskFractal (vips_mask_fractal) failed"),
        )
    }

    /// VipsMaskGaussian (mask_gaussian), make a gaussian filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// frequency_cutoff: `f64` -> Frequency cutoff
    ///
    /// amplitude_cutoff: `f64` -> Amplitude cutoff
    pub fn mask_gaussian(
        width: i32,
        height: i32,
        frequency_cutoff: f64,
        amplitude_cutoff: f64,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "mask_gaussian",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "frequency-cutoff",
                    v_value!(frequency_cutoff),
                )
                .with(
                    "amplitude-cutoff",
                    v_value!(amplitude_cutoff),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskGaussian (vips_mask_gaussian) failed"),
        )
    }

    /// VipsMaskGaussian (mask_gaussian), make a gaussian filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// frequency_cutoff: `f64` -> Frequency cutoff
    ///
    /// amplitude_cutoff: `f64` -> Amplitude cutoff
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// uchar: `bool` -> Output an unsigned char image
    ///
    /// nodc: `bool` -> Remove DC component
    ///
    /// reject: `bool` -> Invert the sense of the filter
    ///
    /// optical: `bool` -> Rotate quadrants to optical space
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
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "frequency-cutoff",
                    v_value!(frequency_cutoff),
                )
                .with(
                    "amplitude-cutoff",
                    v_value!(amplitude_cutoff),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskGaussian (vips_mask_gaussian) failed"),
        )
    }

    /// VipsMaskGaussianBand (mask_gaussian_band), make a gaussian filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// frequency_cutoff_x: `f64` -> Frequency cutoff x
    ///
    /// frequency_cutoff_y: `f64` -> Frequency cutoff y
    ///
    /// radius: `f64` -> Radius of circle
    ///
    /// amplitude_cutoff: `f64` -> Amplitude cutoff
    pub fn mask_gaussian_band(
        width: i32,
        height: i32,
        frequency_cutoff_x: f64,
        frequency_cutoff_y: f64,
        radius: f64,
        amplitude_cutoff: f64,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "mask_gaussian_band",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "frequency-cutoff-x",
                    v_value!(frequency_cutoff_x),
                )
                .with(
                    "frequency-cutoff-y",
                    v_value!(frequency_cutoff_y),
                )
                .with(
                    "radius",
                    v_value!(radius),
                )
                .with(
                    "amplitude-cutoff",
                    v_value!(amplitude_cutoff),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskGaussianBand (vips_mask_gaussian_band) failed"),
        )
    }

    /// VipsMaskGaussianBand (mask_gaussian_band), make a gaussian filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// frequency_cutoff_x: `f64` -> Frequency cutoff x
    ///
    /// frequency_cutoff_y: `f64` -> Frequency cutoff y
    ///
    /// radius: `f64` -> Radius of circle
    ///
    /// amplitude_cutoff: `f64` -> Amplitude cutoff
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// uchar: `bool` -> Output an unsigned char image
    ///
    /// nodc: `bool` -> Remove DC component
    ///
    /// reject: `bool` -> Invert the sense of the filter
    ///
    /// optical: `bool` -> Rotate quadrants to optical space
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
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "frequency-cutoff-x",
                    v_value!(frequency_cutoff_x),
                )
                .with(
                    "frequency-cutoff-y",
                    v_value!(frequency_cutoff_y),
                )
                .with(
                    "radius",
                    v_value!(radius),
                )
                .with(
                    "amplitude-cutoff",
                    v_value!(amplitude_cutoff),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskGaussianBand (vips_mask_gaussian_band) failed"),
        )
    }

    /// VipsMaskGaussianRing (mask_gaussian_ring), make a gaussian ring filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// frequency_cutoff: `f64` -> Frequency cutoff
    ///
    /// amplitude_cutoff: `f64` -> Amplitude cutoff
    ///
    /// ringwidth: `f64` -> Ringwidth
    pub fn mask_gaussian_ring(
        width: i32,
        height: i32,
        frequency_cutoff: f64,
        amplitude_cutoff: f64,
        ringwidth: f64,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "mask_gaussian_ring",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "frequency-cutoff",
                    v_value!(frequency_cutoff),
                )
                .with(
                    "amplitude-cutoff",
                    v_value!(amplitude_cutoff),
                )
                .with(
                    "ringwidth",
                    v_value!(ringwidth),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskGaussianRing (vips_mask_gaussian_ring) failed"),
        )
    }

    /// VipsMaskGaussianRing (mask_gaussian_ring), make a gaussian ring filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// frequency_cutoff: `f64` -> Frequency cutoff
    ///
    /// amplitude_cutoff: `f64` -> Amplitude cutoff
    ///
    /// ringwidth: `f64` -> Ringwidth
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// uchar: `bool` -> Output an unsigned char image
    ///
    /// nodc: `bool` -> Remove DC component
    ///
    /// reject: `bool` -> Invert the sense of the filter
    ///
    /// optical: `bool` -> Rotate quadrants to optical space
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
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "frequency-cutoff",
                    v_value!(frequency_cutoff),
                )
                .with(
                    "amplitude-cutoff",
                    v_value!(amplitude_cutoff),
                )
                .with(
                    "ringwidth",
                    v_value!(ringwidth),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskGaussianRing (vips_mask_gaussian_ring) failed"),
        )
    }

    /// VipsMaskIdeal (mask_ideal), make an ideal filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// frequency_cutoff: `f64` -> Frequency cutoff
    pub fn mask_ideal(width: i32, height: i32, frequency_cutoff: f64) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "mask_ideal",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "frequency-cutoff",
                    v_value!(frequency_cutoff),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskIdeal (vips_mask_ideal) failed"),
        )
    }

    /// VipsMaskIdeal (mask_ideal), make an ideal filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// frequency_cutoff: `f64` -> Frequency cutoff
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// uchar: `bool` -> Output an unsigned char image
    ///
    /// nodc: `bool` -> Remove DC component
    ///
    /// reject: `bool` -> Invert the sense of the filter
    ///
    /// optical: `bool` -> Rotate quadrants to optical space
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
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "frequency-cutoff",
                    v_value!(frequency_cutoff),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskIdeal (vips_mask_ideal) failed"),
        )
    }

    /// VipsMaskIdealBand (mask_ideal_band), make an ideal band filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// frequency_cutoff_x: `f64` -> Frequency cutoff x
    ///
    /// frequency_cutoff_y: `f64` -> Frequency cutoff y
    ///
    /// radius: `f64` -> Radius of circle
    pub fn mask_ideal_band(
        width: i32,
        height: i32,
        frequency_cutoff_x: f64,
        frequency_cutoff_y: f64,
        radius: f64,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "mask_ideal_band",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "frequency-cutoff-x",
                    v_value!(frequency_cutoff_x),
                )
                .with(
                    "frequency-cutoff-y",
                    v_value!(frequency_cutoff_y),
                )
                .with(
                    "radius",
                    v_value!(radius),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskIdealBand (vips_mask_ideal_band) failed"),
        )
    }

    /// VipsMaskIdealBand (mask_ideal_band), make an ideal band filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// frequency_cutoff_x: `f64` -> Frequency cutoff x
    ///
    /// frequency_cutoff_y: `f64` -> Frequency cutoff y
    ///
    /// radius: `f64` -> Radius of circle
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// uchar: `bool` -> Output an unsigned char image
    ///
    /// nodc: `bool` -> Remove DC component
    ///
    /// reject: `bool` -> Invert the sense of the filter
    ///
    /// optical: `bool` -> Rotate quadrants to optical space
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
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "frequency-cutoff-x",
                    v_value!(frequency_cutoff_x),
                )
                .with(
                    "frequency-cutoff-y",
                    v_value!(frequency_cutoff_y),
                )
                .with(
                    "radius",
                    v_value!(radius),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskIdealBand (vips_mask_ideal_band) failed"),
        )
    }

    /// VipsMaskIdealRing (mask_ideal_ring), make an ideal ring filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// frequency_cutoff: `f64` -> Frequency cutoff
    ///
    /// ringwidth: `f64` -> Ringwidth
    pub fn mask_ideal_ring(
        width: i32,
        height: i32,
        frequency_cutoff: f64,
        ringwidth: f64,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "mask_ideal_ring",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "frequency-cutoff",
                    v_value!(frequency_cutoff),
                )
                .with(
                    "ringwidth",
                    v_value!(ringwidth),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskIdealRing (vips_mask_ideal_ring) failed"),
        )
    }

    /// VipsMaskIdealRing (mask_ideal_ring), make an ideal ring filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// frequency_cutoff: `f64` -> Frequency cutoff
    ///
    /// ringwidth: `f64` -> Ringwidth
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// uchar: `bool` -> Output an unsigned char image
    ///
    /// nodc: `bool` -> Remove DC component
    ///
    /// reject: `bool` -> Invert the sense of the filter
    ///
    /// optical: `bool` -> Rotate quadrants to optical space
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
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "frequency-cutoff",
                    v_value!(frequency_cutoff),
                )
                .with(
                    "ringwidth",
                    v_value!(ringwidth),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MaskIdealRing (vips_mask_ideal_ring) failed"),
        )
    }

    /// VipsMatch (match), first-order match of two images
    /// returns `VipsImage` - Output image
    ///
    /// sec: `&VipsImage` -> Secondary image
    ///
    /// xr1: `i32` -> Position of first reference tie-point
    ///
    /// yr1: `i32` -> Position of first reference tie-point
    ///
    /// xs1: `i32` -> Position of first secondary tie-point
    ///
    /// ys1: `i32` -> Position of first secondary tie-point
    ///
    /// xr2: `i32` -> Position of second reference tie-point
    ///
    /// yr2: `i32` -> Position of second reference tie-point
    ///
    /// xs2: `i32` -> Position of second secondary tie-point
    ///
    /// ys2: `i32` -> Position of second secondary tie-point
    pub fn matches(
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "sec",
                    v_value!(sec),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "xr1",
                    v_value!(xr1),
                )
                .with(
                    "yr1",
                    v_value!(yr1),
                )
                .with(
                    "xs1",
                    v_value!(xs1),
                )
                .with(
                    "ys1",
                    v_value!(ys1),
                )
                .with(
                    "xr2",
                    v_value!(xr2),
                )
                .with(
                    "yr2",
                    v_value!(yr2),
                )
                .with(
                    "xs2",
                    v_value!(xs2),
                )
                .with(
                    "ys2",
                    v_value!(ys2),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Matchs (vips_match) failed"),
        )
    }

    /// VipsMatch (match), first-order match of two images
    /// returns `VipsImage` - Output image
    ///
    /// sec: `&VipsImage` -> Secondary image
    ///
    /// xr1: `i32` -> Position of first reference tie-point
    ///
    /// yr1: `i32` -> Position of first reference tie-point
    ///
    /// xs1: `i32` -> Position of first secondary tie-point
    ///
    /// ys1: `i32` -> Position of first secondary tie-point
    ///
    /// xr2: `i32` -> Position of second reference tie-point
    ///
    /// yr2: `i32` -> Position of second reference tie-point
    ///
    /// xs2: `i32` -> Position of second secondary tie-point
    ///
    /// ys2: `i32` -> Position of second secondary tie-point
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// hwindow: `i32` -> Half window size
    ///
    /// harea: `i32` -> Half area size
    ///
    /// search: `bool` -> Search to improve tie-points
    ///
    /// interpolate: `&VipsInterpolate` -> Interpolate pixels with this
    pub fn matches_with_opts(
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "sec",
                    v_value!(sec),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "xr1",
                    v_value!(xr1),
                )
                .with(
                    "yr1",
                    v_value!(yr1),
                )
                .with(
                    "xs1",
                    v_value!(xs1),
                )
                .with(
                    "ys1",
                    v_value!(ys1),
                )
                .with(
                    "xr2",
                    v_value!(xr2),
                )
                .with(
                    "yr2",
                    v_value!(yr2),
                )
                .with(
                    "xs2",
                    v_value!(xs2),
                )
                .with(
                    "ys2",
                    v_value!(ys2),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Matchs (vips_match) failed"),
        )
    }

    /// VipsMath2 (math2), binary math operations
    /// returns `VipsImage` - Output image
    ///
    /// right: `&VipsImage` -> Right-hand image argument
    ///
    /// math2: `OperationMath2` -> Math to perform
    pub fn math2(&self, right: &VipsImage, math2: OperationMath2) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "math2",
            VOption::new()
                .with(
                    "left",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "right",
                    v_value!(right),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "math2",
                    v_value!(math2 as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Math2 (vips_math2) failed"),
        )
    }

    /// VipsMath2Const (math2_const), binary math operations with a constant
    /// returns `VipsImage` - Output image
    ///
    /// math2: `OperationMath2` -> Math to perform
    ///
    /// c: `&[f64]` -> Array of constants
    pub fn math2_const(&self, math2: OperationMath2, c: &[f64]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "math2_const",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "math2",
                    v_value!(math2 as i32),
                )
                .with(
                    "c",
                    v_value!(c),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Math2Const (vips_math2_const) failed"),
        )
    }

    /// VipsMath (math), apply a math operation to an image
    /// returns `VipsImage` - Output image
    ///
    /// math: `OperationMath` -> Math to perform
    pub fn math(&self, math: OperationMath) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "math",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "math",
                    v_value!(math as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Math (vips_math) failed"),
        )
    }

    /// VipsForeignLoadMat (matload), load mat from file (.mat), priority=0, untrusted, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn matload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "matload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Matload (vips_matload) failed"),
        )
    }

    /// VipsForeignLoadMat (matload), load mat from file (.mat), priority=0, untrusted, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn matload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "matload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Matload (vips_matload) failed"),
        )
    }

    /// VipsMatrixinvert (matrixinvert), invert an matrix
    /// returns `VipsImage` - Output matrix
    pub fn matrixinvert(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "matrixinvert",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Matrixinvert (vips_matrixinvert) failed"),
        )
    }

    /// VipsForeignLoadMatrixFile (matrixload), load matrix (.mat), priority=0, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn matrixload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "matrixload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Matrixload (vips_matrixload) failed"),
        )
    }

    /// VipsForeignLoadMatrixFile (matrixload), load matrix (.mat), priority=0, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn matrixload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "matrixload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Matrixload (vips_matrixload) failed"),
        )
    }

    /// VipsForeignLoadMatrixSource (matrixload_source), load matrix, priority=0, is_a_source, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    pub fn matrixload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "matrixload_source",
            VOption::new()
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MatrixloadSource (vips_matrixload_source) failed"),
        )
    }

    /// VipsForeignLoadMatrixSource (matrixload_source), load matrix, priority=0, is_a_source, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn matrixload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "matrixload_source",
            option
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("MatrixloadSource (vips_matrixload_source) failed"),
        )
    }

    /// VipsForeignPrintMatrix (matrixprint), print matrix (.mat), priority=0, mono
    pub fn matrixprint(&self) -> Result<()> {
        let vips_op_response = call(
            "matrixprint",
            VOption::new().with(
                "in",
                v_value!(&VipsImage::from(self.ctx)),
            ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Matrixprint (vips_matrixprint) failed"),
        )
    }

    /// VipsForeignPrintMatrix (matrixprint), print matrix (.mat), priority=0, mono
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn matrixprint_with_opts(&self, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "matrixprint",
            option.with(
                "in",
                v_value!(&VipsImage::from(self.ctx)),
            ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Matrixprint (vips_matrixprint) failed"),
        )
    }

    /// VipsForeignSaveMatrixFile (matrixsave), save image to matrix (.mat), priority=0, mono
    ///
    /// filename: `&str` -> Filename to save to
    pub fn matrixsave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "matrixsave",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Matrixsave (vips_matrixsave) failed"),
        )
    }

    /// VipsForeignSaveMatrixFile (matrixsave), save image to matrix (.mat), priority=0, mono
    ///
    /// filename: `&str` -> Filename to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn matrixsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "matrixsave",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Matrixsave (vips_matrixsave) failed"),
        )
    }

    /// VipsForeignSaveMatrixTarget (matrixsave_target), save image to matrix (.mat), priority=0, mono
    ///
    /// target: `&VipsTarget` -> Target to save to
    pub fn matrixsave_target(&self, target: &VipsTarget) -> Result<()> {
        let vips_op_response = call(
            "matrixsave_target",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("MatrixsaveTarget (vips_matrixsave_target) failed"),
        )
    }

    /// VipsForeignSaveMatrixTarget (matrixsave_target), save image to matrix (.mat), priority=0, mono
    ///
    /// target: `&VipsTarget` -> Target to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn matrixsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "matrixsave_target",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("MatrixsaveTarget (vips_matrixsave_target) failed"),
        )
    }

    /// VipsMax (max), find image maximum
    /// returns `f64` - Output value
    pub fn max(&self) -> Result<f64> {
        let mut out_out: f64 = 0.0;
        let vips_op_response = call(
            "max",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Max (vips_max) failed"),
        )
    }

    /// VipsMax (max), find image maximum
    /// returns `f64` - Output value
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// x: `&mut i32` -> Horizontal position of maximum
    ///
    /// y: `&mut i32` -> Vertical position of maximum
    ///
    /// size: `i32` -> Number of maximum values to find
    ///
    /// out_array: `&mut Vec<f64>` -> Array of output values
    ///
    /// x_array: `&[i32]` -> Array of horizontal positions
    ///
    /// y_array: `&[i32]` -> Array of vertical positions
    pub fn max_with_opts(&self, option: VOption) -> Result<f64> {
        let mut out_out: f64 = 0.0;
        let vips_op_response = call(
            "max",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Max (vips_max) failed"),
        )
    }

    /// VipsMaxpair (maxpair), maximum of a pair of images
    /// returns `VipsImage` - Output image
    ///
    /// right: `&VipsImage` -> Right-hand image argument
    pub fn maxpair(&self, right: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "maxpair",
            VOption::new()
                .with(
                    "left",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "right",
                    v_value!(right),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Maxpair (vips_maxpair) failed"),
        )
    }

    /// VipsMeasure (measure), measure a set of patches on a color chart
    /// returns `VipsImage` - Output array of statistics
    ///
    /// h: `i32` -> Number of patches across chart
    ///
    /// v: `i32` -> Number of patches down chart
    pub fn measure(&self, h: i32, v: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "measure",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "h",
                    v_value!(h),
                )
                .with(
                    "v",
                    v_value!(v),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Measure (vips_measure) failed"),
        )
    }

    /// VipsMeasure (measure), measure a set of patches on a color chart
    /// returns `VipsImage` - Output array of statistics
    ///
    /// h: `i32` -> Number of patches across chart
    ///
    /// v: `i32` -> Number of patches down chart
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// left: `i32` -> Left edge of extract area
    ///
    /// top: `i32` -> Top edge of extract area
    ///
    /// width: `i32` -> Width of extract area
    ///
    /// height: `i32` -> Height of extract area
    pub fn measure_with_opts(&self, h: i32, v: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "measure",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "h",
                    v_value!(h),
                )
                .with(
                    "v",
                    v_value!(v),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Measure (vips_measure) failed"),
        )
    }

    /// VipsMerge (merge), merge two images
    /// returns `VipsImage` - Output image
    ///
    /// sec: `&VipsImage` -> Secondary image
    ///
    /// direction: `Direction` -> Horizontal or vertical merge
    ///
    /// dx: `i32` -> Horizontal displacement from sec to ref
    ///
    /// dy: `i32` -> Vertical displacement from sec to ref
    pub fn merge(
        &self,
        sec: &VipsImage,
        direction: Direction,
        dx: i32,
        dy: i32,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "merge",
            VOption::new()
                .with(
                    "ref",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "sec",
                    v_value!(sec),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "direction",
                    v_value!(direction as i32),
                )
                .with(
                    "dx",
                    v_value!(dx),
                )
                .with(
                    "dy",
                    v_value!(dy),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Merge (vips_merge) failed"),
        )
    }

    /// VipsMerge (merge), merge two images
    /// returns `VipsImage` - Output image
    ///
    /// sec: `&VipsImage` -> Secondary image
    ///
    /// direction: `Direction` -> Horizontal or vertical merge
    ///
    /// dx: `i32` -> Horizontal displacement from sec to ref
    ///
    /// dy: `i32` -> Vertical displacement from sec to ref
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// mblend: `i32` -> Maximum blend size
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "sec",
                    v_value!(sec),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "direction",
                    v_value!(direction as i32),
                )
                .with(
                    "dx",
                    v_value!(dx),
                )
                .with(
                    "dy",
                    v_value!(dy),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Merge (vips_merge) failed"),
        )
    }

    /// VipsMin (min), find image minimum
    /// returns `f64` - Output value
    pub fn min(&self) -> Result<f64> {
        let mut out_out: f64 = 0.0;
        let vips_op_response = call(
            "min",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Min (vips_min) failed"),
        )
    }

    /// VipsMin (min), find image minimum
    /// returns `f64` - Output value
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// x: `&mut i32` -> Horizontal position of minimum
    ///
    /// y: `&mut i32` -> Vertical position of minimum
    ///
    /// size: `i32` -> Number of minimum values to find
    ///
    /// out_array: `&mut Vec<f64>` -> Array of output values
    ///
    /// x_array: `&[i32]` -> Array of horizontal positions
    ///
    /// y_array: `&[i32]` -> Array of vertical positions
    pub fn min_with_opts(&self, option: VOption) -> Result<f64> {
        let mut out_out: f64 = 0.0;
        let vips_op_response = call(
            "min",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Min (vips_min) failed"),
        )
    }

    /// VipsMinpair (minpair), minimum of a pair of images
    /// returns `VipsImage` - Output image
    ///
    /// right: `&VipsImage` -> Right-hand image argument
    pub fn minpair(&self, right: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "minpair",
            VOption::new()
                .with(
                    "left",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "right",
                    v_value!(right),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Minpair (vips_minpair) failed"),
        )
    }

    /// VipsMorph (morph), morphology operation
    /// returns `VipsImage` - Output image
    ///
    /// mask: `&VipsImage` -> Input matrix image
    ///
    /// morph: `OperationMorphology` -> Morphological operation to perform
    pub fn morph(&self, mask: &VipsImage, morph: OperationMorphology) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "morph",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "mask",
                    v_value!(mask),
                )
                .with(
                    "morph",
                    v_value!(morph as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Morph (vips_morph) failed"),
        )
    }

    /// VipsMosaic1 (mosaic1), first-order mosaic of two images
    /// returns `VipsImage` - Output image
    ///
    /// sec: `&VipsImage` -> Secondary image
    ///
    /// direction: `Direction` -> Horizontal or vertical mosaic
    ///
    /// xr1: `i32` -> Position of first reference tie-point
    ///
    /// yr1: `i32` -> Position of first reference tie-point
    ///
    /// xs1: `i32` -> Position of first secondary tie-point
    ///
    /// ys1: `i32` -> Position of first secondary tie-point
    ///
    /// xr2: `i32` -> Position of second reference tie-point
    ///
    /// yr2: `i32` -> Position of second reference tie-point
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
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "mosaic1",
            VOption::new()
                .with(
                    "ref",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "sec",
                    v_value!(sec),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "direction",
                    v_value!(direction as i32),
                )
                .with(
                    "xr1",
                    v_value!(xr1),
                )
                .with(
                    "yr1",
                    v_value!(yr1),
                )
                .with(
                    "xs1",
                    v_value!(xs1),
                )
                .with(
                    "ys1",
                    v_value!(ys1),
                )
                .with(
                    "xr2",
                    v_value!(xr2),
                )
                .with(
                    "yr2",
                    v_value!(yr2),
                )
                .with(
                    "xs2",
                    v_value!(xs2),
                )
                .with(
                    "ys2",
                    v_value!(ys2),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Mosaic1 (vips_mosaic1) failed"),
        )
    }

    /// VipsMosaic1 (mosaic1), first-order mosaic of two images
    /// returns `VipsImage` - Output image
    ///
    /// sec: `&VipsImage` -> Secondary image
    ///
    /// direction: `Direction` -> Horizontal or vertical mosaic
    ///
    /// xr1: `i32` -> Position of first reference tie-point
    ///
    /// yr1: `i32` -> Position of first reference tie-point
    ///
    /// xs1: `i32` -> Position of first secondary tie-point
    ///
    /// ys1: `i32` -> Position of first secondary tie-point
    ///
    /// xr2: `i32` -> Position of second reference tie-point
    ///
    /// yr2: `i32` -> Position of second reference tie-point
    ///
    /// xs2: `i32` -> Position of second secondary tie-point
    ///
    /// ys2: `i32` -> Position of second secondary tie-point
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// hwindow: `i32` -> Half window size
    ///
    /// harea: `i32` -> Half area size
    ///
    /// search: `bool` -> Search to improve tie-points
    ///
    /// interpolate: `&VipsInterpolate` -> Interpolate pixels with this
    ///
    /// mblend: `i32` -> Maximum blend size
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "sec",
                    v_value!(sec),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "direction",
                    v_value!(direction as i32),
                )
                .with(
                    "xr1",
                    v_value!(xr1),
                )
                .with(
                    "yr1",
                    v_value!(yr1),
                )
                .with(
                    "xs1",
                    v_value!(xs1),
                )
                .with(
                    "ys1",
                    v_value!(ys1),
                )
                .with(
                    "xr2",
                    v_value!(xr2),
                )
                .with(
                    "yr2",
                    v_value!(yr2),
                )
                .with(
                    "xs2",
                    v_value!(xs2),
                )
                .with(
                    "ys2",
                    v_value!(ys2),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Mosaic1 (vips_mosaic1) failed"),
        )
    }

    /// VipsMosaic (mosaic), mosaic two images
    /// returns `VipsImage` - Output image
    ///
    /// sec: `&VipsImage` -> Secondary image
    ///
    /// direction: `Direction` -> Horizontal or vertical mosaic
    ///
    /// xref: `i32` -> Position of reference tie-point
    ///
    /// yref: `i32` -> Position of reference tie-point
    ///
    /// xsec: `i32` -> Position of secondary tie-point
    ///
    /// ysec: `i32` -> Position of secondary tie-point
    pub fn mosaic(
        &self,
        sec: &VipsImage,
        direction: Direction,
        xref: i32,
        yref: i32,
        xsec: i32,
        ysec: i32,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "mosaic",
            VOption::new()
                .with(
                    "ref",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "sec",
                    v_value!(sec),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "direction",
                    v_value!(direction as i32),
                )
                .with(
                    "xref",
                    v_value!(xref),
                )
                .with(
                    "yref",
                    v_value!(yref),
                )
                .with(
                    "xsec",
                    v_value!(xsec),
                )
                .with(
                    "ysec",
                    v_value!(ysec),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Mosaic (vips_mosaic) failed"),
        )
    }

    /// VipsMosaic (mosaic), mosaic two images
    /// returns `VipsImage` - Output image
    ///
    /// sec: `&VipsImage` -> Secondary image
    ///
    /// direction: `Direction` -> Horizontal or vertical mosaic
    ///
    /// xref: `i32` -> Position of reference tie-point
    ///
    /// yref: `i32` -> Position of reference tie-point
    ///
    /// xsec: `i32` -> Position of secondary tie-point
    ///
    /// ysec: `i32` -> Position of secondary tie-point
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// hwindow: `i32` -> Half window size
    ///
    /// harea: `i32` -> Half area size
    ///
    /// mblend: `i32` -> Maximum blend size
    ///
    /// bandno: `i32` -> Band to search for features on
    ///
    /// dx0: `&mut i32` -> Detected integer offset
    ///
    /// dy0: `&mut i32` -> Detected integer offset
    ///
    /// scale1: `&mut f64` -> Detected scale
    ///
    /// angle1: `&mut f64` -> Detected rotation
    ///
    /// dy1: `&mut f64` -> Detected first-order displacement
    ///
    /// dx1: `&mut f64` -> Detected first-order displacement
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "sec",
                    v_value!(sec),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "direction",
                    v_value!(direction as i32),
                )
                .with(
                    "xref",
                    v_value!(xref),
                )
                .with(
                    "yref",
                    v_value!(yref),
                )
                .with(
                    "xsec",
                    v_value!(xsec),
                )
                .with(
                    "ysec",
                    v_value!(ysec),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Mosaic (vips_mosaic) failed"),
        )
    }

    /// VipsMsb (msb), pick most-significant byte from an image
    /// returns `VipsImage` - Output image
    pub fn msb(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "msb",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Msb (vips_msb) failed"),
        )
    }

    /// VipsMsb (msb), pick most-significant byte from an image
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// band: `i32` -> Band to msb
    pub fn msb_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "msb",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Msb (vips_msb) failed"),
        )
    }

    /// VipsMultiply (multiply), multiply two images
    /// returns `VipsImage` - Output image
    ///
    /// right: `&VipsImage` -> Right-hand image argument
    pub fn multiply(&self, right: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "multiply",
            VOption::new()
                .with(
                    "left",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "right",
                    v_value!(right),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Multiply (vips_multiply) failed"),
        )
    }

    /// VipsForeignLoadOpenexr (openexrload), load an OpenEXR image (.exr), priority=200, untrusted, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn openexrload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "openexrload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Openexrload (vips_openexrload) failed"),
        )
    }

    /// VipsForeignLoadOpenexr (openexrload), load an OpenEXR image (.exr), priority=200, untrusted, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn openexrload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "openexrload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Openexrload (vips_openexrload) failed"),
        )
    }

    /// VipsForeignLoadOpenslideFile (openslideload), load file with OpenSlide (.svs, .vms, .vmu, .ndpi, .scn, .mrxs, .svslide, .tif, .bif), priority=100, untrusted, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn openslideload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "openslideload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Openslideload (vips_openslideload) failed"),
        )
    }

    /// VipsForeignLoadOpenslideFile (openslideload), load file with OpenSlide (.svs, .vms, .vmu, .ndpi, .scn, .mrxs, .svslide, .tif, .bif), priority=100, untrusted, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// level: `i32` -> Load this level from the file
    ///
    /// autocrop: `bool` -> Crop to image bounds
    ///
    /// associated: `&str` -> Load this associated image
    ///
    /// attach_associated: `bool` -> Attach all associated images
    ///
    /// rgb: `bool` -> Output RGB (not RGBA)
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn openslideload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "openslideload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Openslideload (vips_openslideload) failed"),
        )
    }

    /// VipsForeignLoadOpenslideSource (openslideload_source), load source with OpenSlide, priority=100, untrusted, is_a_source, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    pub fn openslideload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "openslideload_source",
            VOption::new()
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("OpenslideloadSource (vips_openslideload_source) failed"),
        )
    }

    /// VipsForeignLoadOpenslideSource (openslideload_source), load source with OpenSlide, priority=100, untrusted, is_a_source, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// level: `i32` -> Load this level from the file
    ///
    /// autocrop: `bool` -> Crop to image bounds
    ///
    /// associated: `&str` -> Load this associated image
    ///
    /// attach_associated: `bool` -> Attach all associated images
    ///
    /// rgb: `bool` -> Output RGB (not RGBA)
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn openslideload_source_with_opts(
        source: &VipsSource,
        option: VOption,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "openslideload_source",
            option
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("OpenslideloadSource (vips_openslideload_source) failed"),
        )
    }

    /// VipsForeignLoadPdfFile (pdfload), load PDF from file (.pdf), priority=0, untrusted, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn pdfload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "pdfload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Pdfload (vips_pdfload) failed"),
        )
    }

    /// VipsForeignLoadPdfFile (pdfload), load PDF from file (.pdf), priority=0, untrusted, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// page: `i32` -> First page to load
    ///
    /// n: `i32` -> Number of pages to load, -1 for all
    ///
    /// dpi: `f64` -> DPI to render at
    ///
    /// scale: `f64` -> Factor to scale by
    ///
    /// background: `&[f64]` -> Background colour
    ///
    /// password: `&str` -> Password to decrypt with
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn pdfload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "pdfload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Pdfload (vips_pdfload) failed"),
        )
    }

    /// VipsForeignLoadPdfBuffer (pdfload_buffer), load PDF from buffer, priority=0, untrusted, is_a_buffer, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    pub fn pdfload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "pdfload_buffer",
            VOption::new()
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("PdfloadBuffer (vips_pdfload_buffer) failed"),
        )
    }

    /// VipsForeignLoadPdfBuffer (pdfload_buffer), load PDF from buffer, priority=0, untrusted, is_a_buffer, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// page: `i32` -> First page to load
    ///
    /// n: `i32` -> Number of pages to load, -1 for all
    ///
    /// dpi: `f64` -> DPI to render at
    ///
    /// scale: `f64` -> Factor to scale by
    ///
    /// background: `&[f64]` -> Background colour
    ///
    /// password: `&str` -> Password to decrypt with
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn pdfload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "pdfload_buffer",
            option
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("PdfloadBuffer (vips_pdfload_buffer) failed"),
        )
    }

    /// VipsForeignLoadPdfSource (pdfload_source), load PDF from source, priority=0, untrusted, is_a_source, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    pub fn pdfload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "pdfload_source",
            VOption::new()
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("PdfloadSource (vips_pdfload_source) failed"),
        )
    }

    /// VipsForeignLoadPdfSource (pdfload_source), load PDF from source, priority=0, untrusted, is_a_source, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// page: `i32` -> First page to load
    ///
    /// n: `i32` -> Number of pages to load, -1 for all
    ///
    /// dpi: `f64` -> DPI to render at
    ///
    /// scale: `f64` -> Factor to scale by
    ///
    /// background: `&[f64]` -> Background colour
    ///
    /// password: `&str` -> Password to decrypt with
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn pdfload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "pdfload_source",
            option
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("PdfloadSource (vips_pdfload_source) failed"),
        )
    }

    /// VipsPercent (percent), find threshold for percent of pixels
    /// returns `i32` - Threshold above which lie percent of pixels
    ///
    /// percent: `f64` -> Percent of pixels
    pub fn percent(&self, percent: f64) -> Result<i32> {
        let mut threshold_out: i32 = 0;
        let vips_op_response = call(
            "percent",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "percent",
                    v_value!(percent),
                )
                .with(
                    "threshold",
                    v_value!(&mut threshold_out),
                ),
        );
        utils::result(
            vips_op_response,
            threshold_out,
            Error::OperationError("Percent (vips_percent) failed"),
        )
    }

    /// VipsPerlin (perlin), make a perlin noise image
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    pub fn perlin(width: i32, height: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "perlin",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Perlin (vips_perlin) failed"),
        )
    }

    /// VipsPerlin (perlin), make a perlin noise image
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// cell_size: `i32` -> Size of Perlin cells
    ///
    /// uchar: `bool` -> Output an unsigned char image
    ///
    /// seed: `i32` -> Random number seed
    pub fn perlin_with_opts(width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "perlin",
            option
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Perlin (vips_perlin) failed"),
        )
    }

    /// VipsPhasecor (phasecor), calculate phase correlation
    /// returns `VipsImage` - Output image
    ///
    /// in2: `&VipsImage` -> Second input image
    pub fn phasecor(&self, in2: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "phasecor",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "in2",
                    v_value!(in2),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Phasecor (vips_phasecor) failed"),
        )
    }

    /// VipsForeignLoadPngFile (pngload), load png from file (.png), priority=200, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn pngload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "pngload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Pngload (vips_pngload) failed"),
        )
    }

    /// VipsForeignLoadPngFile (pngload), load png from file (.png), priority=200, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// unlimited: `bool` -> Remove all denial of service limits
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn pngload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "pngload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Pngload (vips_pngload) failed"),
        )
    }

    /// VipsForeignLoadPngBuffer (pngload_buffer), load png from buffer, priority=200, is_a_buffer, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    pub fn pngload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "pngload_buffer",
            VOption::new()
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("PngloadBuffer (vips_pngload_buffer) failed"),
        )
    }

    /// VipsForeignLoadPngBuffer (pngload_buffer), load png from buffer, priority=200, is_a_buffer, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// unlimited: `bool` -> Remove all denial of service limits
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn pngload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "pngload_buffer",
            option
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("PngloadBuffer (vips_pngload_buffer) failed"),
        )
    }

    /// VipsForeignLoadPngSource (pngload_source), load png from source, priority=200, is_a_source, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    pub fn pngload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "pngload_source",
            VOption::new()
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("PngloadSource (vips_pngload_source) failed"),
        )
    }

    /// VipsForeignLoadPngSource (pngload_source), load png from source, priority=200, is_a_source, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// unlimited: `bool` -> Remove all denial of service limits
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn pngload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "pngload_source",
            option
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("PngloadSource (vips_pngload_source) failed"),
        )
    }

    /// VipsForeignSaveSpngFile (pngsave), save image to file as PNG (.png), priority=0, rgba
    ///
    /// filename: `&str` -> Filename to save to
    pub fn pngsave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "pngsave",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Pngsave (vips_pngsave) failed"),
        )
    }

    /// VipsForeignSaveSpngFile (pngsave), save image to file as PNG (.png), priority=0, rgba
    ///
    /// filename: `&str` -> Filename to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// compression: `i32` -> Compression factor
    ///
    /// interlace: `bool` -> Interlace image
    ///
    /// filter: `ForeignPngFilter` -> libspng row filter flag(s)
    ///
    /// palette: `bool` -> Quantise to 8bpp palette
    ///
    /// Q: `i32` -> Quantisation quality
    ///
    /// dither: `f64` -> Amount of dithering
    ///
    /// bitdepth: `i32` -> Write as a 1, 2, 4, 8 or 16 bit image
    ///
    /// effort: `i32` -> Quantisation CPU effort
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn pngsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "pngsave",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Pngsave (vips_pngsave) failed"),
        )
    }

    /// VipsForeignSaveSpngBuffer (pngsave_buffer), save image to buffer as PNG (.png), priority=0, rgba
    /// returns `Vec<u8>` - Buffer to save to
    pub fn pngsave_buffer(&self) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "pngsave_buffer",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("PngsaveBuffer (vips_pngsave_buffer) failed"),
        )
    }

    /// VipsForeignSaveSpngBuffer (pngsave_buffer), save image to buffer as PNG (.png), priority=0, rgba
    /// returns `Vec<u8>` - Buffer to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// compression: `i32` -> Compression factor
    ///
    /// interlace: `bool` -> Interlace image
    ///
    /// filter: `ForeignPngFilter` -> libspng row filter flag(s)
    ///
    /// palette: `bool` -> Quantise to 8bpp palette
    ///
    /// Q: `i32` -> Quantisation quality
    ///
    /// dither: `f64` -> Amount of dithering
    ///
    /// bitdepth: `i32` -> Write as a 1, 2, 4, 8 or 16 bit image
    ///
    /// effort: `i32` -> Quantisation CPU effort
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn pngsave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "pngsave_buffer",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("PngsaveBuffer (vips_pngsave_buffer) failed"),
        )
    }

    /// VipsForeignSaveSpngTarget (pngsave_target), save image to target as PNG (.png), priority=0, rgba
    ///
    /// target: `&VipsTarget` -> Target to save to
    pub fn pngsave_target(&self, target: &VipsTarget) -> Result<()> {
        let vips_op_response = call(
            "pngsave_target",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("PngsaveTarget (vips_pngsave_target) failed"),
        )
    }

    /// VipsForeignSaveSpngTarget (pngsave_target), save image to target as PNG (.png), priority=0, rgba
    ///
    /// target: `&VipsTarget` -> Target to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// compression: `i32` -> Compression factor
    ///
    /// interlace: `bool` -> Interlace image
    ///
    /// filter: `ForeignPngFilter` -> libspng row filter flag(s)
    ///
    /// palette: `bool` -> Quantise to 8bpp palette
    ///
    /// Q: `i32` -> Quantisation quality
    ///
    /// dither: `f64` -> Amount of dithering
    ///
    /// bitdepth: `i32` -> Write as a 1, 2, 4, 8 or 16 bit image
    ///
    /// effort: `i32` -> Quantisation CPU effort
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn pngsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "pngsave_target",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("PngsaveTarget (vips_pngsave_target) failed"),
        )
    }

    /// VipsForeignLoadPpmFile (ppmload), load ppm from file (.pbm, .pgm, .ppm, .pfm, .pnm), priority=200, untrusted, is_a, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn ppmload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "ppmload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Ppmload (vips_ppmload) failed"),
        )
    }

    /// VipsForeignLoadPpmFile (ppmload), load ppm from file (.pbm, .pgm, .ppm, .pfm, .pnm), priority=200, untrusted, is_a, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn ppmload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "ppmload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Ppmload (vips_ppmload) failed"),
        )
    }

    /// VipsForeignLoadPpmSource (ppmload_source), load ppm base class (.pbm, .pgm, .ppm, .pfm, .pnm), priority=200, untrusted, is_a_source, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    pub fn ppmload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "ppmload_source",
            VOption::new()
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("PpmloadSource (vips_ppmload_source) failed"),
        )
    }

    /// VipsForeignLoadPpmSource (ppmload_source), load ppm base class (.pbm, .pgm, .ppm, .pfm, .pnm), priority=200, untrusted, is_a_source, get_flags, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn ppmload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "ppmload_source",
            option
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("PpmloadSource (vips_ppmload_source) failed"),
        )
    }

    /// VipsForeignSavePpmFile (ppmsave), save image to ppm file (.pbm, .pgm, .ppm, .pfm, .pnm), priority=0, any
    ///
    /// filename: `&str` -> Filename to save to
    pub fn ppmsave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "ppmsave",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Ppmsave (vips_ppmsave) failed"),
        )
    }

    /// VipsForeignSavePpmFile (ppmsave), save image to ppm file (.pbm, .pgm, .ppm, .pfm, .pnm), priority=0, any
    ///
    /// filename: `&str` -> Filename to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// format: `ForeignPpmFormat` -> Format to save in
    ///
    /// ascii: `bool` -> Save as ascii
    ///
    /// bitdepth: `i32` -> Set to 1 to write as a 1 bit image
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn ppmsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "ppmsave",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Ppmsave (vips_ppmsave) failed"),
        )
    }

    /// VipsForeignSavePpmTarget (ppmsave_target), save to ppm (.ppm), priority=0, any
    ///
    /// target: `&VipsTarget` -> Target to save to
    pub fn ppmsave_target(&self, target: &VipsTarget) -> Result<()> {
        let vips_op_response = call(
            "ppmsave_target",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("PpmsaveTarget (vips_ppmsave_target) failed"),
        )
    }

    /// VipsForeignSavePpmTarget (ppmsave_target), save to ppm (.ppm), priority=0, any
    ///
    /// target: `&VipsTarget` -> Target to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// format: `ForeignPpmFormat` -> Format to save in
    ///
    /// ascii: `bool` -> Save as ascii
    ///
    /// bitdepth: `i32` -> Set to 1 to write as a 1 bit image
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn ppmsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "ppmsave_target",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("PpmsaveTarget (vips_ppmsave_target) failed"),
        )
    }

    /// VipsPremultiply (premultiply), premultiply image alpha
    /// returns `VipsImage` - Output image
    pub fn premultiply(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "premultiply",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Premultiply (vips_premultiply) failed"),
        )
    }

    /// VipsPremultiply (premultiply), premultiply image alpha
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// max_alpha: `f64` -> Maximum value of alpha channel
    pub fn premultiply_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "premultiply",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Premultiply (vips_premultiply) failed"),
        )
    }

    /// VipsPrewitt (prewitt), Prewitt edge detector
    /// returns `VipsImage` - Output image
    pub fn prewitt(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "prewitt",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Prewitt (vips_prewitt) failed"),
        )
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
        let mut columns_out = VipsImage::from(null_mut());
        let mut rows_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "profile",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "columns",
                    v_value!(&mut columns_out),
                )
                .with(
                    "rows",
                    v_value!(&mut rows_out),
                ),
        );
        utils::result(
            vips_op_response,
            (
                columns_out,
                rows_out,
            ),
            Error::OperationError("Profile (vips_profile) failed"),
        )
    }

    /// VipsProfileLoad (profile_load), load named ICC profile
    /// returns `Vec<u8>` - Loaded profile
    ///
    /// name: `&str` -> Profile name
    pub fn profile_load(name: &str) -> Result<Vec<u8>> {
        let mut profile_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "profile_load",
            VOption::new()
                .with(
                    "name",
                    v_value!(name),
                )
                .with(
                    "profile",
                    v_value!(&mut profile_out),
                ),
        );
        utils::result(
            vips_op_response,
            profile_out.into(),
            Error::OperationError("ProfileLoad (vips_profile_load) failed"),
        )
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
        let mut columns_out = VipsImage::from(null_mut());
        let mut rows_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "project",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "columns",
                    v_value!(&mut columns_out),
                )
                .with(
                    "rows",
                    v_value!(&mut rows_out),
                ),
        );
        utils::result(
            vips_op_response,
            (
                columns_out,
                rows_out,
            ),
            Error::OperationError("Project (vips_project) failed"),
        )
    }

    /// VipsQuadratic (quadratic), resample an image with a quadratic transform
    /// returns `VipsImage` - Output image
    ///
    /// coeff: `&VipsImage` -> Coefficient matrix
    pub fn quadratic(&self, coeff: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "quadratic",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "coeff",
                    v_value!(coeff),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Quadratic (vips_quadratic) failed"),
        )
    }

    /// VipsQuadratic (quadratic), resample an image with a quadratic transform
    /// returns `VipsImage` - Output image
    ///
    /// coeff: `&VipsImage` -> Coefficient matrix
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// interpolate: `&VipsInterpolate` -> Interpolate values with this
    pub fn quadratic_with_opts(&self, coeff: &VipsImage, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "quadratic",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "coeff",
                    v_value!(coeff),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Quadratic (vips_quadratic) failed"),
        )
    }

    /// VipsRad2float (rad2float), unpack Radiance coding to float RGB
    /// returns `VipsImage` - Output image
    pub fn rad2float(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "rad2float",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Rad2Float (vips_rad2float) failed"),
        )
    }

    /// VipsForeignLoadRadFile (radload), load a Radiance image from a file (.hdr), priority=-50, untrusted, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn radload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "radload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Radload (vips_radload) failed"),
        )
    }

    /// VipsForeignLoadRadFile (radload), load a Radiance image from a file (.hdr), priority=-50, untrusted, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn radload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "radload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Radload (vips_radload) failed"),
        )
    }

    /// VipsForeignLoadRadBuffer (radload_buffer), load rad from buffer, priority=-50, untrusted, is_a_buffer, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    pub fn radload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "radload_buffer",
            VOption::new()
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("RadloadBuffer (vips_radload_buffer) failed"),
        )
    }

    /// VipsForeignLoadRadBuffer (radload_buffer), load rad from buffer, priority=-50, untrusted, is_a_buffer, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn radload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "radload_buffer",
            option
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("RadloadBuffer (vips_radload_buffer) failed"),
        )
    }

    /// VipsForeignLoadRadSource (radload_source), load rad from source, priority=-50, untrusted, is_a_source, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    pub fn radload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "radload_source",
            VOption::new()
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("RadloadSource (vips_radload_source) failed"),
        )
    }

    /// VipsForeignLoadRadSource (radload_source), load rad from source, priority=-50, untrusted, is_a_source, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn radload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "radload_source",
            option
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("RadloadSource (vips_radload_source) failed"),
        )
    }

    /// VipsForeignSaveRadFile (radsave), save image to Radiance file (.hdr), priority=0, rgb
    ///
    /// filename: `&str` -> Filename to save to
    pub fn radsave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "radsave",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Radsave (vips_radsave) failed"),
        )
    }

    /// VipsForeignSaveRadFile (radsave), save image to Radiance file (.hdr), priority=0, rgb
    ///
    /// filename: `&str` -> Filename to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn radsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "radsave",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Radsave (vips_radsave) failed"),
        )
    }

    /// VipsForeignSaveRadBuffer (radsave_buffer), save image to Radiance buffer (.hdr), priority=0, rgb
    /// returns `Vec<u8>` - Buffer to save to
    pub fn radsave_buffer(&self) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "radsave_buffer",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("RadsaveBuffer (vips_radsave_buffer) failed"),
        )
    }

    /// VipsForeignSaveRadBuffer (radsave_buffer), save image to Radiance buffer (.hdr), priority=0, rgb
    /// returns `Vec<u8>` - Buffer to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn radsave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "radsave_buffer",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("RadsaveBuffer (vips_radsave_buffer) failed"),
        )
    }

    /// VipsForeignSaveRadTarget (radsave_target), save image to Radiance target (.hdr), priority=0, rgb
    ///
    /// target: `&VipsTarget` -> Target to save to
    pub fn radsave_target(&self, target: &VipsTarget) -> Result<()> {
        let vips_op_response = call(
            "radsave_target",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("RadsaveTarget (vips_radsave_target) failed"),
        )
    }

    /// VipsForeignSaveRadTarget (radsave_target), save image to Radiance target (.hdr), priority=0, rgb
    ///
    /// target: `&VipsTarget` -> Target to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn radsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "radsave_target",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("RadsaveTarget (vips_radsave_target) failed"),
        )
    }

    /// VipsRank (rank), rank filter
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Window width in pixels
    ///
    /// height: `i32` -> Window height in pixels
    ///
    /// index: `i32` -> Select pixel at index
    pub fn rank(&self, width: i32, height: i32, index: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "rank",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "index",
                    v_value!(index),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Rank (vips_rank) failed"),
        )
    }

    /// VipsForeignLoadRaw (rawload), load raw data from a file, priority=0, untrusted, get_flags, get_flags_filename, header
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// bands: `i32` -> Number of bands in image
    pub fn rawload(filename: &str, width: i32, height: i32, bands: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "rawload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "bands",
                    v_value!(bands),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Rawload (vips_rawload) failed"),
        )
    }

    /// VipsForeignLoadRaw (rawload), load raw data from a file, priority=0, untrusted, get_flags, get_flags_filename, header
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// bands: `i32` -> Number of bands in image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// offset: `u64` -> Offset in bytes from start of file
    ///
    /// format: `BandFormat` -> Pixel format in image
    ///
    /// interpretation: `Interpretation` -> Pixel interpretation
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
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
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "bands",
                    v_value!(bands),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Rawload (vips_rawload) failed"),
        )
    }

    /// VipsForeignSaveRawFile (rawsave), save image to raw file (.raw), priority=0, any
    ///
    /// filename: `&str` -> Filename to save to
    pub fn rawsave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "rawsave",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Rawsave (vips_rawsave) failed"),
        )
    }

    /// VipsForeignSaveRawFile (rawsave), save image to raw file (.raw), priority=0, any
    ///
    /// filename: `&str` -> Filename to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn rawsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "rawsave",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Rawsave (vips_rawsave) failed"),
        )
    }

    /// VipsForeignSaveRawBuffer (rawsave_buffer), write raw image to buffer (.raw), priority=0, any
    /// returns `Vec<u8>` - Buffer to save to
    pub fn rawsave_buffer(&self) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "rawsave_buffer",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("RawsaveBuffer (vips_rawsave_buffer) failed"),
        )
    }

    /// VipsForeignSaveRawBuffer (rawsave_buffer), write raw image to buffer (.raw), priority=0, any
    /// returns `Vec<u8>` - Buffer to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn rawsave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "rawsave_buffer",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("RawsaveBuffer (vips_rawsave_buffer) failed"),
        )
    }

    /// VipsForeignSaveRawTarget (rawsave_target), write raw image to target (.raw), priority=0, any
    ///
    /// target: `&VipsTarget` -> Target to save to
    pub fn rawsave_target(&self, target: &VipsTarget) -> Result<()> {
        let vips_op_response = call(
            "rawsave_target",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("RawsaveTarget (vips_rawsave_target) failed"),
        )
    }

    /// VipsForeignSaveRawTarget (rawsave_target), write raw image to target (.raw), priority=0, any
    ///
    /// target: `&VipsTarget` -> Target to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn rawsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "rawsave_target",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("RawsaveTarget (vips_rawsave_target) failed"),
        )
    }

    /// VipsRecomb (recomb), linear recombination with matrix
    /// returns `VipsImage` - Output image
    ///
    /// m: `&VipsImage` -> Matrix of coefficients
    pub fn recomb(&self, m: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "recomb",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "m",
                    v_value!(m),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Recomb (vips_recomb) failed"),
        )
    }

    /// VipsReduce (reduce), reduce an image
    /// returns `VipsImage` - Output image
    ///
    /// hshrink: `f64` -> Horizontal shrink factor
    ///
    /// vshrink: `f64` -> Vertical shrink factor
    pub fn reduce(&self, hshrink: f64, vshrink: f64) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "reduce",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "hshrink",
                    v_value!(hshrink),
                )
                .with(
                    "vshrink",
                    v_value!(vshrink),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Reduce (vips_reduce) failed"),
        )
    }

    /// VipsReduce (reduce), reduce an image
    /// returns `VipsImage` - Output image
    ///
    /// hshrink: `f64` -> Horizontal shrink factor
    ///
    /// vshrink: `f64` -> Vertical shrink factor
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// kernel: `Kernel` -> Resampling kernel
    ///
    /// gap: `f64` -> Reducing gap
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "hshrink",
                    v_value!(hshrink),
                )
                .with(
                    "vshrink",
                    v_value!(vshrink),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Reduce (vips_reduce) failed"),
        )
    }

    /// VipsReduceh (reduceh), shrink an image horizontally
    /// returns `VipsImage` - Output image
    ///
    /// hshrink: `f64` -> Horizontal shrink factor
    pub fn reduceh(&self, hshrink: f64) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "reduceh",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "hshrink",
                    v_value!(hshrink),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Reduceh (vips_reduceh) failed"),
        )
    }

    /// VipsReduceh (reduceh), shrink an image horizontally
    /// returns `VipsImage` - Output image
    ///
    /// hshrink: `f64` -> Horizontal shrink factor
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// kernel: `Kernel` -> Resampling kernel
    ///
    /// gap: `f64` -> Reducing gap
    pub fn reduceh_with_opts(&self, hshrink: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "reduceh",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "hshrink",
                    v_value!(hshrink),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Reduceh (vips_reduceh) failed"),
        )
    }

    /// VipsReducev (reducev), shrink an image vertically
    /// returns `VipsImage` - Output image
    ///
    /// vshrink: `f64` -> Vertical shrink factor
    pub fn reducev(&self, vshrink: f64) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "reducev",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "vshrink",
                    v_value!(vshrink),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Reducev (vips_reducev) failed"),
        )
    }

    /// VipsReducev (reducev), shrink an image vertically
    /// returns `VipsImage` - Output image
    ///
    /// vshrink: `f64` -> Vertical shrink factor
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// kernel: `Kernel` -> Resampling kernel
    ///
    /// gap: `f64` -> Reducing gap
    pub fn reducev_with_opts(&self, vshrink: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "reducev",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "vshrink",
                    v_value!(vshrink),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Reducev (vips_reducev) failed"),
        )
    }

    /// VipsRelational (relational), relational operation on two images
    /// returns `VipsImage` - Output image
    ///
    /// right: `&VipsImage` -> Right-hand image argument
    ///
    /// relational: `OperationRelational` -> Relational to perform
    pub fn relational(
        &self,
        right: &VipsImage,
        relational: OperationRelational,
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "relational",
            VOption::new()
                .with(
                    "left",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "right",
                    v_value!(right),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "relational",
                    v_value!(relational as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Relational (vips_relational) failed"),
        )
    }

    /// VipsRelationalConst (relational_const), relational operations against a constant
    /// returns `VipsImage` - Output image
    ///
    /// relational: `OperationRelational` -> Relational to perform
    ///
    /// c: `&[f64]` -> Array of constants
    pub fn relational_const(
        &self,
        relational: OperationRelational,
        c: &[f64],
    ) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "relational_const",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "relational",
                    v_value!(relational as i32),
                )
                .with(
                    "c",
                    v_value!(c),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("RelationalConst (vips_relational_const) failed"),
        )
    }

    /// VipsRemainder (remainder), remainder after integer division of two images
    /// returns `VipsImage` - Output image
    ///
    /// right: `&VipsImage` -> Right-hand image argument
    pub fn remainder(&self, right: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "remainder",
            VOption::new()
                .with(
                    "left",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "right",
                    v_value!(right),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Remainder (vips_remainder) failed"),
        )
    }

    /// VipsRemainderConst (remainder_const), remainder after integer division of an image and a constant
    /// returns `VipsImage` - Output image
    ///
    /// c: `&[f64]` -> Array of constants
    pub fn remainder_const(&self, c: &[f64]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "remainder_const",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "c",
                    v_value!(c),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("RemainderConst (vips_remainder_const) failed"),
        )
    }

    /// VipsReplicate (replicate), replicate an image
    /// returns `VipsImage` - Output image
    ///
    /// across: `i32` -> Repeat this many times horizontally
    ///
    /// down: `i32` -> Repeat this many times vertically
    pub fn replicate(&self, across: i32, down: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "replicate",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "across",
                    v_value!(across),
                )
                .with(
                    "down",
                    v_value!(down),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Replicate (vips_replicate) failed"),
        )
    }

    /// VipsResize (resize), resize an image
    /// returns `VipsImage` - Output image
    ///
    /// scale: `f64` -> Scale image by this factor
    pub fn resize(&self, scale: f64) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "resize",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "scale",
                    v_value!(scale),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Resize (vips_resize) failed"),
        )
    }

    /// VipsResize (resize), resize an image
    /// returns `VipsImage` - Output image
    ///
    /// scale: `f64` -> Scale image by this factor
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// kernel: `Kernel` -> Resampling kernel
    ///
    /// gap: `f64` -> Reducing gap
    ///
    /// vscale: `f64` -> Vertical scale image by this factor
    pub fn resize_with_opts(&self, scale: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "resize",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "scale",
                    v_value!(scale),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Resize (vips_resize) failed"),
        )
    }

    /// VipsRot45 (rot45), rotate an image
    /// returns `VipsImage` - Output image
    pub fn rot45(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "rot45",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Rot45 (vips_rot45) failed"),
        )
    }

    /// VipsRot45 (rot45), rotate an image
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// angle: `Angle45` -> Angle to rotate image
    pub fn rot45_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "rot45",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Rot45 (vips_rot45) failed"),
        )
    }

    /// VipsRot (rot), rotate an image
    /// returns `VipsImage` - Output image
    ///
    /// angle: `Angle` -> Angle to rotate image
    pub fn rot(&self, angle: Angle) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "rot",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "angle",
                    v_value!(angle as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Rot (vips_rot) failed"),
        )
    }

    /// VipsRotate (rotate), rotate an image by a number of degrees
    /// returns `VipsImage` - Output image
    ///
    /// angle: `f64` -> Rotate clockwise by this many degrees
    pub fn rotate(&self, angle: f64) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "rotate",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "angle",
                    v_value!(angle),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Rotate (vips_rotate) failed"),
        )
    }

    /// VipsRotate (rotate), rotate an image by a number of degrees
    /// returns `VipsImage` - Output image
    ///
    /// angle: `f64` -> Rotate clockwise by this many degrees
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// interpolate: `&VipsInterpolate` -> Interpolate pixels with this
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// odx: `f64` -> Horizontal output displacement
    ///
    /// ody: `f64` -> Vertical output displacement
    ///
    /// idx: `f64` -> Horizontal input displacement
    ///
    /// idy: `f64` -> Vertical input displacement
    pub fn rotate_with_opts(&self, angle: f64, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "rotate",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "angle",
                    v_value!(angle),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Rotate (vips_rotate) failed"),
        )
    }

    /// VipsRound (round), perform a round function on an image
    /// returns `VipsImage` - Output image
    ///
    /// round: `OperationRound` -> Rounding operation to perform
    pub fn round(&self, round: OperationRound) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "round",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "round",
                    v_value!(round as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Round (vips_round) failed"),
        )
    }

    /// VipssRGB2HSV (sRGB2HSV), transform sRGB to HSV
    /// returns `VipsImage` - Output image
    pub fn sRGB2HSV(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "sRGB2HSV",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("SRgb2Hsv (vips_sRGB2HSV) failed"),
        )
    }

    /// VipssRGB2scRGB (sRGB2scRGB), convert an sRGB image to scRGB
    /// returns `VipsImage` - Output image
    pub fn sRGB2scRGB(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "sRGB2scRGB",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("SRgb2ScRgb (vips_sRGB2scRGB) failed"),
        )
    }

    /// VipsscRGB2BW (scRGB2BW), convert scRGB to BW
    /// returns `VipsImage` - Output image
    pub fn scRGB2BW(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "scRGB2BW",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ScRgb2Bw (vips_scRGB2BW) failed"),
        )
    }

    /// VipsscRGB2BW (scRGB2BW), convert scRGB to BW
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// depth: `i32` -> Output device space depth in bits
    pub fn scRGB2BW_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "scRGB2BW",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ScRgb2Bw (vips_scRGB2BW) failed"),
        )
    }

    /// VipsscRGB2XYZ (scRGB2XYZ), transform scRGB to XYZ
    /// returns `VipsImage` - Output image
    pub fn scRGB2XYZ(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "scRGB2XYZ",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ScRgb2Xyz (vips_scRGB2XYZ) failed"),
        )
    }

    /// VipsscRGB2sRGB (scRGB2sRGB), convert an scRGB image to sRGB
    /// returns `VipsImage` - Output image
    pub fn scRGB2sRGB(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "scRGB2sRGB",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ScRgb2SRgb (vips_scRGB2sRGB) failed"),
        )
    }

    /// VipsscRGB2sRGB (scRGB2sRGB), convert an scRGB image to sRGB
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// depth: `i32` -> Output device space depth in bits
    pub fn scRGB2sRGB_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "scRGB2sRGB",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ScRgb2SRgb (vips_scRGB2sRGB) failed"),
        )
    }

    /// VipsScale (scale), scale an image to uchar
    /// returns `VipsImage` - Output image
    pub fn scale(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "scale",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Scale (vips_scale) failed"),
        )
    }

    /// VipsScale (scale), scale an image to uchar
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// exp: `f64` -> Exponent for log scale
    ///
    /// log: `bool` -> Log scale
    pub fn scale_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "scale",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Scale (vips_scale) failed"),
        )
    }

    /// VipsScharr (scharr), Scharr edge detector
    /// returns `VipsImage` - Output image
    pub fn scharr(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "scharr",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Scharr (vips_scharr) failed"),
        )
    }

    /// VipsSdf (sdf), create an SDF image
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// shape: `SdfShape` -> SDF shape to create
    pub fn sdf(width: i32, height: i32, shape: SdfShape) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "sdf",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "shape",
                    v_value!(shape as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Sdf (vips_sdf) failed"),
        )
    }

    /// VipsSdf (sdf), create an SDF image
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// shape: `SdfShape` -> SDF shape to create
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// r: `f64` -> Radius
    ///
    /// a: `&[f64]` -> Point a
    ///
    /// b: `&[f64]` -> Point b
    ///
    /// corners: `&[f64]` -> Corner radii
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
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                )
                .with(
                    "shape",
                    v_value!(shape as i32),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Sdf (vips_sdf) failed"),
        )
    }

    /// VipsSequential (sequential), check sequential access
    /// returns `VipsImage` - Output image
    pub fn sequential(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "sequential",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Sequential (vips_sequential) failed"),
        )
    }

    /// VipsSequential (sequential), check sequential access
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// tile_height: `i32` -> Tile height in pixels
    pub fn sequential_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "sequential",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Sequential (vips_sequential) failed"),
        )
    }

    /// VipsSharpen (sharpen), unsharp masking for print
    /// returns `VipsImage` - Output image
    pub fn sharpen(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "sharpen",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Sharpen (vips_sharpen) failed"),
        )
    }

    /// VipsSharpen (sharpen), unsharp masking for print
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// sigma: `f64` -> Sigma of Gaussian
    ///
    /// x1: `f64` -> Flat/jaggy threshold
    ///
    /// y2: `f64` -> Maximum brightening
    ///
    /// y3: `f64` -> Maximum darkening
    ///
    /// m1: `f64` -> Slope for flat areas
    ///
    /// m2: `f64` -> Slope for jaggy areas
    pub fn sharpen_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "sharpen",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Sharpen (vips_sharpen) failed"),
        )
    }

    /// VipsShrink (shrink), shrink an image
    /// returns `VipsImage` - Output image
    ///
    /// hshrink: `f64` -> Horizontal shrink factor
    ///
    /// vshrink: `f64` -> Vertical shrink factor
    pub fn shrink(&self, hshrink: f64, vshrink: f64) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "shrink",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "hshrink",
                    v_value!(hshrink),
                )
                .with(
                    "vshrink",
                    v_value!(vshrink),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Shrink (vips_shrink) failed"),
        )
    }

    /// VipsShrink (shrink), shrink an image
    /// returns `VipsImage` - Output image
    ///
    /// hshrink: `f64` -> Horizontal shrink factor
    ///
    /// vshrink: `f64` -> Vertical shrink factor
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// ceil: `bool` -> Round-up output dimensions
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "hshrink",
                    v_value!(hshrink),
                )
                .with(
                    "vshrink",
                    v_value!(vshrink),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Shrink (vips_shrink) failed"),
        )
    }

    /// VipsShrinkh (shrinkh), shrink an image horizontally
    /// returns `VipsImage` - Output image
    ///
    /// hshrink: `i32` -> Horizontal shrink factor
    pub fn shrinkh(&self, hshrink: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "shrinkh",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "hshrink",
                    v_value!(hshrink),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Shrinkh (vips_shrinkh) failed"),
        )
    }

    /// VipsShrinkh (shrinkh), shrink an image horizontally
    /// returns `VipsImage` - Output image
    ///
    /// hshrink: `i32` -> Horizontal shrink factor
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// ceil: `bool` -> Round-up output dimensions
    pub fn shrinkh_with_opts(&self, hshrink: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "shrinkh",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "hshrink",
                    v_value!(hshrink),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Shrinkh (vips_shrinkh) failed"),
        )
    }

    /// VipsShrinkv (shrinkv), shrink an image vertically
    /// returns `VipsImage` - Output image
    ///
    /// vshrink: `i32` -> Vertical shrink factor
    pub fn shrinkv(&self, vshrink: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "shrinkv",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "vshrink",
                    v_value!(vshrink),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Shrinkv (vips_shrinkv) failed"),
        )
    }

    /// VipsShrinkv (shrinkv), shrink an image vertically
    /// returns `VipsImage` - Output image
    ///
    /// vshrink: `i32` -> Vertical shrink factor
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// ceil: `bool` -> Round-up output dimensions
    pub fn shrinkv_with_opts(&self, vshrink: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "shrinkv",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "vshrink",
                    v_value!(vshrink),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Shrinkv (vips_shrinkv) failed"),
        )
    }

    /// VipsSign (sign), unit vector of pixel
    /// returns `VipsImage` - Output image
    pub fn sign(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "sign",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Sign (vips_sign) failed"),
        )
    }

    /// VipsSimilarity (similarity), similarity transform of an image
    /// returns `VipsImage` - Output image
    pub fn similarity(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "similarity",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Similarity (vips_similarity) failed"),
        )
    }

    /// VipsSimilarity (similarity), similarity transform of an image
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// scale: `f64` -> Scale by this factor
    ///
    /// angle: `f64` -> Rotate clockwise by this many degrees
    ///
    /// interpolate: `&VipsInterpolate` -> Interpolate pixels with this
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// odx: `f64` -> Horizontal output displacement
    ///
    /// ody: `f64` -> Vertical output displacement
    ///
    /// idx: `f64` -> Horizontal input displacement
    ///
    /// idy: `f64` -> Vertical input displacement
    pub fn similarity_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "similarity",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Similarity (vips_similarity) failed"),
        )
    }

    /// VipsSines (sines), make a 2D sine wave
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    pub fn sines(width: i32, height: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "sines",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Sines (vips_sines) failed"),
        )
    }

    /// VipsSines (sines), make a 2D sine wave
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// uchar: `bool` -> Output an unsigned char image
    ///
    /// hfreq: `f64` -> Horizontal spatial frequency
    ///
    /// vfreq: `f64` -> Vertical spatial frequency
    pub fn sines_with_opts(width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "sines",
            option
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Sines (vips_sines) failed"),
        )
    }

    /// VipsSmartcrop (smartcrop), extract an area from an image
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Width of extract area
    ///
    /// height: `i32` -> Height of extract area
    pub fn smartcrop(&self, width: i32, height: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "smartcrop",
            VOption::new()
                .with(
                    "input",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Smartcrop (vips_smartcrop) failed"),
        )
    }

    /// VipsSmartcrop (smartcrop), extract an area from an image
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Width of extract area
    ///
    /// height: `i32` -> Height of extract area
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// attention_x: `&mut i32` -> Horizontal position of attention centre
    ///
    /// attention_y: `&mut i32` -> Vertical position of attention centre
    ///
    /// interesting: `Interesting` -> How to measure interestingness
    ///
    /// premultiplied: `bool` -> Input image already has premultiplied alpha
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
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Smartcrop (vips_smartcrop) failed"),
        )
    }

    /// VipsSobel (sobel), Sobel edge detector
    /// returns `VipsImage` - Output image
    pub fn sobel(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "sobel",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Sobel (vips_sobel) failed"),
        )
    }

    /// VipsSpcor (spcor), spatial correlation
    /// returns `VipsImage` - Output image
    ///
    /// refp: `&VipsImage` -> Input reference image
    pub fn spcor(&self, refp: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "spcor",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "ref",
                    v_value!(refp),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Spcor (vips_spcor) failed"),
        )
    }

    /// VipsSpectrum (spectrum), make displayable power spectrum
    /// returns `VipsImage` - Output image
    pub fn spectrum(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "spectrum",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Spectrum (vips_spectrum) failed"),
        )
    }

    /// VipsStats (stats), find many image stats
    /// returns `VipsImage` - Output array of statistics
    pub fn stats(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "stats",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Stats (vips_stats) failed"),
        )
    }

    /// VipsStdif (stdif), statistical difference
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Window width in pixels
    ///
    /// height: `i32` -> Window height in pixels
    pub fn stdif(&self, width: i32, height: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "stdif",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Stdif (vips_stdif) failed"),
        )
    }

    /// VipsStdif (stdif), statistical difference
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Window width in pixels
    ///
    /// height: `i32` -> Window height in pixels
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// s0: `f64` -> New deviation
    ///
    /// b: `f64` -> Weight of new deviation
    ///
    /// m0: `f64` -> New mean
    ///
    /// a: `f64` -> Weight of new mean
    pub fn stdif_with_opts(&self, width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "stdif",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Stdif (vips_stdif) failed"),
        )
    }

    /// VipsSubsample (subsample), subsample an image
    /// returns `VipsImage` - Output image
    ///
    /// xfac: `i32` -> Horizontal subsample factor
    ///
    /// yfac: `i32` -> Vertical subsample factor
    pub fn subsample(&self, xfac: i32, yfac: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "subsample",
            VOption::new()
                .with(
                    "input",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "xfac",
                    v_value!(xfac),
                )
                .with(
                    "yfac",
                    v_value!(yfac),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Subsample (vips_subsample) failed"),
        )
    }

    /// VipsSubsample (subsample), subsample an image
    /// returns `VipsImage` - Output image
    ///
    /// xfac: `i32` -> Horizontal subsample factor
    ///
    /// yfac: `i32` -> Vertical subsample factor
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// point: `bool` -> Point sample
    pub fn subsample_with_opts(&self, xfac: i32, yfac: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "subsample",
            option
                .with(
                    "input",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "xfac",
                    v_value!(xfac),
                )
                .with(
                    "yfac",
                    v_value!(yfac),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Subsample (vips_subsample) failed"),
        )
    }

    /// VipsSubtract (subtract), subtract two images
    /// returns `VipsImage` - Output image
    ///
    /// right: `&VipsImage` -> Right-hand image argument
    pub fn subtract(&self, right: &VipsImage) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "subtract",
            VOption::new()
                .with(
                    "left",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "right",
                    v_value!(right),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Subtract (vips_subtract) failed"),
        )
    }

    /// VipsSum (sum), sum an array of images
    /// returns `VipsImage` - Output image
    ///
    /// inp: `&[VipsImage]` -> Array of input images
    pub fn sum(inp: &[VipsImage]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "sum",
            VOption::new()
                .with(
                    "in",
                    v_value!(inp),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Sum (vips_sum) failed"),
        )
    }

    /// VipsForeignLoadSvgFile (svgload), load SVG with rsvg (.svg, .svgz, .svg.gz), priority=-5, untrusted, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn svgload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "svgload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Svgload (vips_svgload) failed"),
        )
    }

    /// VipsForeignLoadSvgFile (svgload), load SVG with rsvg (.svg, .svgz, .svg.gz), priority=-5, untrusted, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// dpi: `f64` -> Render at this DPI
    ///
    /// scale: `f64` -> Scale output by this factor
    ///
    /// unlimited: `bool` -> Allow SVG of any size
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn svgload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "svgload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Svgload (vips_svgload) failed"),
        )
    }

    /// VipsForeignLoadSvgBuffer (svgload_buffer), load SVG with rsvg, priority=-5, untrusted, is_a_buffer, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    pub fn svgload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "svgload_buffer",
            VOption::new()
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("SvgloadBuffer (vips_svgload_buffer) failed"),
        )
    }

    /// VipsForeignLoadSvgBuffer (svgload_buffer), load SVG with rsvg, priority=-5, untrusted, is_a_buffer, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// dpi: `f64` -> Render at this DPI
    ///
    /// scale: `f64` -> Scale output by this factor
    ///
    /// unlimited: `bool` -> Allow SVG of any size
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn svgload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "svgload_buffer",
            option
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("SvgloadBuffer (vips_svgload_buffer) failed"),
        )
    }

    /// VipsForeignLoadSvgSource (svgload_source), load svg from source, priority=-5, untrusted, is_a_source, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    pub fn svgload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "svgload_source",
            VOption::new()
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("SvgloadSource (vips_svgload_source) failed"),
        )
    }

    /// VipsForeignLoadSvgSource (svgload_source), load svg from source, priority=-5, untrusted, is_a_source, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// dpi: `f64` -> Render at this DPI
    ///
    /// scale: `f64` -> Scale output by this factor
    ///
    /// unlimited: `bool` -> Allow SVG of any size
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn svgload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "svgload_source",
            option
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("SvgloadSource (vips_svgload_source) failed"),
        )
    }

    /// VipsSwitch (switch), find the index of the first non-zero pixel in tests
    /// returns `VipsImage` - Output image
    ///
    /// tests: `&[VipsImage]` -> Table of images to test
    pub fn switch(tests: &[VipsImage]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "switch",
            VOption::new()
                .with(
                    "tests",
                    v_value!(tests),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Switch (vips_switch) failed"),
        )
    }

    /// VipsSystem (system), run an external command
    ///
    /// cmd_format: `&str` -> Command to run
    pub fn system(cmd_format: &str) -> Result<()> {
        let vips_op_response = call(
            "system",
            VOption::new().with(
                "cmd-format",
                v_value!(cmd_format),
            ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("System (vips_system) failed"),
        )
    }

    /// VipsSystem (system), run an external command
    ///
    /// cmd_format: `&str` -> Command to run
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// inp: `&[VipsImage]` -> Array of input images
    ///
    /// out: `&mut VipsImage` -> Output image
    ///
    /// log: `&str` -> Command log
    ///
    /// out_format: `&str` -> Format for output filename
    ///
    /// in_format: `&str` -> Format for input filename
    pub fn system_with_opts(cmd_format: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "system",
            option.with(
                "cmd-format",
                v_value!(cmd_format),
            ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("System (vips_system) failed"),
        )
    }

    /// VipsText (text), make a text image
    /// returns `VipsImage` - Output image
    ///
    /// text: `&str` -> Text to render
    pub fn text(text: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "text",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "text",
                    v_value!(text),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Text (vips_text) failed"),
        )
    }

    /// VipsText (text), make a text image
    /// returns `VipsImage` - Output image
    ///
    /// text: `&str` -> Text to render
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// font: `&str` -> Font to render with
    ///
    /// width: `i32` -> Maximum image width in pixels
    ///
    /// height: `i32` -> Maximum image height in pixels
    ///
    /// align: `Align` -> Align on the low, centre or high edge
    ///
    /// justify: `bool` -> Justify lines
    ///
    /// dpi: `i32` -> DPI to render at
    ///
    /// autofit_dpi: `&mut i32` -> DPI selected by autofit
    ///
    /// spacing: `i32` -> Line spacing
    ///
    /// fontfile: `&str` -> Load this font file
    ///
    /// rgba: `bool` -> Enable RGBA output
    ///
    /// wrap: `TextWrap` -> Wrap lines on word or character boundaries
    pub fn text_with_opts(text: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "text",
            option
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "text",
                    v_value!(text),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Text (vips_text) failed"),
        )
    }

    /// VipsThumbnailFile (thumbnail), generate thumbnail from file
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to read from
    ///
    /// width: `i32` -> Size to this width
    pub fn thumbnail(filename: &str, width: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "thumbnail",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Thumbnail (vips_thumbnail) failed"),
        )
    }

    /// VipsThumbnailFile (thumbnail), generate thumbnail from file
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to read from
    ///
    /// width: `i32` -> Size to this width
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// height: `i32` -> Size to this height
    ///
    /// size: `Size` -> Only upsize, only downsize, or both
    ///
    /// no_rotate: `bool` -> Don't use orientation tags to rotate image upright
    ///
    /// crop: `Interesting` -> Reduce to fill target rectangle, then crop
    ///
    /// linear: `bool` -> Reduce in linear light
    ///
    /// import_profile: `&str` -> Fallback import profile
    ///
    /// export_profile: `&str` -> Fallback export profile
    ///
    /// intent: `Intent` -> Rendering intent
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    pub fn thumbnail_with_opts(filename: &str, width: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "thumbnail",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Thumbnail (vips_thumbnail) failed"),
        )
    }

    /// VipsThumbnailBuffer (thumbnail_buffer), generate thumbnail from buffer
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    ///
    /// width: `i32` -> Size to this width
    pub fn thumbnail_buffer(buffer: &[u8], width: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "thumbnail_buffer",
            VOption::new()
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ThumbnailBuffer (vips_thumbnail_buffer) failed"),
        )
    }

    /// VipsThumbnailBuffer (thumbnail_buffer), generate thumbnail from buffer
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    ///
    /// width: `i32` -> Size to this width
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// option_string: `&str` -> Options that are passed on to the underlying loader
    ///
    /// height: `i32` -> Size to this height
    ///
    /// size: `Size` -> Only upsize, only downsize, or both
    ///
    /// no_rotate: `bool` -> Don't use orientation tags to rotate image upright
    ///
    /// crop: `Interesting` -> Reduce to fill target rectangle, then crop
    ///
    /// linear: `bool` -> Reduce in linear light
    ///
    /// import_profile: `&str` -> Fallback import profile
    ///
    /// export_profile: `&str` -> Fallback export profile
    ///
    /// intent: `Intent` -> Rendering intent
    ///
    /// fail_on: `FailOn` -> Error level to fail on
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
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ThumbnailBuffer (vips_thumbnail_buffer) failed"),
        )
    }

    /// VipsThumbnailImage (thumbnail_image), generate thumbnail from image
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Size to this width
    pub fn thumbnail_image(&self, width: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "thumbnail_image",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ThumbnailImage (vips_thumbnail_image) failed"),
        )
    }

    /// VipsThumbnailImage (thumbnail_image), generate thumbnail from image
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Size to this width
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// height: `i32` -> Size to this height
    ///
    /// size: `Size` -> Only upsize, only downsize, or both
    ///
    /// no_rotate: `bool` -> Don't use orientation tags to rotate image upright
    ///
    /// crop: `Interesting` -> Reduce to fill target rectangle, then crop
    ///
    /// linear: `bool` -> Reduce in linear light
    ///
    /// import_profile: `&str` -> Fallback import profile
    ///
    /// export_profile: `&str` -> Fallback export profile
    ///
    /// intent: `Intent` -> Rendering intent
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    pub fn thumbnail_image_with_opts(&self, width: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "thumbnail_image",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ThumbnailImage (vips_thumbnail_image) failed"),
        )
    }

    /// VipsThumbnailSource (thumbnail_source), generate thumbnail from source
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    ///
    /// width: `i32` -> Size to this width
    pub fn thumbnail_source(source: &VipsSource, width: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "thumbnail_source",
            VOption::new()
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ThumbnailSource (vips_thumbnail_source) failed"),
        )
    }

    /// VipsThumbnailSource (thumbnail_source), generate thumbnail from source
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    ///
    /// width: `i32` -> Size to this width
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// option_string: `&str` -> Options that are passed on to the underlying loader
    ///
    /// height: `i32` -> Size to this height
    ///
    /// size: `Size` -> Only upsize, only downsize, or both
    ///
    /// no_rotate: `bool` -> Don't use orientation tags to rotate image upright
    ///
    /// crop: `Interesting` -> Reduce to fill target rectangle, then crop
    ///
    /// linear: `bool` -> Reduce in linear light
    ///
    /// import_profile: `&str` -> Fallback import profile
    ///
    /// export_profile: `&str` -> Fallback export profile
    ///
    /// intent: `Intent` -> Rendering intent
    ///
    /// fail_on: `FailOn` -> Error level to fail on
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
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("ThumbnailSource (vips_thumbnail_source) failed"),
        )
    }

    /// VipsForeignLoadTiffFile (tiffload), load tiff from file (.tif, .tiff), priority=50, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn tiffload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "tiffload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Tiffload (vips_tiffload) failed"),
        )
    }

    /// VipsForeignLoadTiffFile (tiffload), load tiff from file (.tif, .tiff), priority=50, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// page: `i32` -> First page to load
    ///
    /// subifd: `i32` -> Subifd index
    ///
    /// n: `i32` -> Number of pages to load, -1 for all
    ///
    /// autorotate: `bool` -> Rotate image using orientation tag
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn tiffload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "tiffload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Tiffload (vips_tiffload) failed"),
        )
    }

    /// VipsForeignLoadTiffBuffer (tiffload_buffer), load tiff from buffer, priority=50, is_a_buffer, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    pub fn tiffload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "tiffload_buffer",
            VOption::new()
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("TiffloadBuffer (vips_tiffload_buffer) failed"),
        )
    }

    /// VipsForeignLoadTiffBuffer (tiffload_buffer), load tiff from buffer, priority=50, is_a_buffer, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// page: `i32` -> First page to load
    ///
    /// subifd: `i32` -> Subifd index
    ///
    /// n: `i32` -> Number of pages to load, -1 for all
    ///
    /// autorotate: `bool` -> Rotate image using orientation tag
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn tiffload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "tiffload_buffer",
            option
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("TiffloadBuffer (vips_tiffload_buffer) failed"),
        )
    }

    /// VipsForeignLoadTiffSource (tiffload_source), load tiff from source, priority=50, is_a_source, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    pub fn tiffload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "tiffload_source",
            VOption::new()
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("TiffloadSource (vips_tiffload_source) failed"),
        )
    }

    /// VipsForeignLoadTiffSource (tiffload_source), load tiff from source, priority=50, is_a_source, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// page: `i32` -> First page to load
    ///
    /// subifd: `i32` -> Subifd index
    ///
    /// n: `i32` -> Number of pages to load, -1 for all
    ///
    /// autorotate: `bool` -> Rotate image using orientation tag
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn tiffload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "tiffload_source",
            option
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("TiffloadSource (vips_tiffload_source) failed"),
        )
    }

    /// VipsForeignSaveTiffFile (tiffsave), save image to tiff file (.tif, .tiff), priority=0, any
    ///
    /// filename: `&str` -> Filename to save to
    pub fn tiffsave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "tiffsave",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Tiffsave (vips_tiffsave) failed"),
        )
    }

    /// VipsForeignSaveTiffFile (tiffsave), save image to tiff file (.tif, .tiff), priority=0, any
    ///
    /// filename: `&str` -> Filename to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// compression: `ForeignTiffCompression` -> Compression for this file
    ///
    /// Q: `i32` -> Q factor
    ///
    /// predictor: `ForeignTiffPredictor` -> Compression prediction
    ///
    /// tile: `bool` -> Write a tiled tiff
    ///
    /// tile_width: `i32` -> Tile width in pixels
    ///
    /// tile_height: `i32` -> Tile height in pixels
    ///
    /// pyramid: `bool` -> Write a pyramidal tiff
    ///
    /// miniswhite: `bool` -> Use 0 for white in 1-bit images
    ///
    /// bitdepth: `i32` -> Write as a 1, 2, 4 or 8 bit image
    ///
    /// resunit: `ForeignTiffResunit` -> Resolution unit
    ///
    /// xres: `f64` -> Horizontal resolution in pixels/mm
    ///
    /// yres: `f64` -> Vertical resolution in pixels/mm
    ///
    /// bigtiff: `bool` -> Write a bigtiff image
    ///
    /// properties: `bool` -> Write a properties document to IMAGEDESCRIPTION
    ///
    /// region_shrink: `RegionShrink` -> Method to shrink regions
    ///
    /// level: `i32` -> Deflate (1-9, default 6) or ZSTD (1-22, default 9) compression level
    ///
    /// lossless: `bool` -> Enable WEBP lossless mode
    ///
    /// depth: `ForeignDzDepth` -> Pyramid depth
    ///
    /// subifd: `bool` -> Save pyr layers as sub-IFDs
    ///
    /// premultiply: `bool` -> Save with premultiplied alpha
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn tiffsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "tiffsave",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Tiffsave (vips_tiffsave) failed"),
        )
    }

    /// VipsForeignSaveTiffBuffer (tiffsave_buffer), save image to tiff buffer (.tif, .tiff), priority=0, any
    /// returns `Vec<u8>` - Buffer to save to
    pub fn tiffsave_buffer(&self) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "tiffsave_buffer",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("TiffsaveBuffer (vips_tiffsave_buffer) failed"),
        )
    }

    /// VipsForeignSaveTiffBuffer (tiffsave_buffer), save image to tiff buffer (.tif, .tiff), priority=0, any
    /// returns `Vec<u8>` - Buffer to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// compression: `ForeignTiffCompression` -> Compression for this file
    ///
    /// Q: `i32` -> Q factor
    ///
    /// predictor: `ForeignTiffPredictor` -> Compression prediction
    ///
    /// tile: `bool` -> Write a tiled tiff
    ///
    /// tile_width: `i32` -> Tile width in pixels
    ///
    /// tile_height: `i32` -> Tile height in pixels
    ///
    /// pyramid: `bool` -> Write a pyramidal tiff
    ///
    /// miniswhite: `bool` -> Use 0 for white in 1-bit images
    ///
    /// bitdepth: `i32` -> Write as a 1, 2, 4 or 8 bit image
    ///
    /// resunit: `ForeignTiffResunit` -> Resolution unit
    ///
    /// xres: `f64` -> Horizontal resolution in pixels/mm
    ///
    /// yres: `f64` -> Vertical resolution in pixels/mm
    ///
    /// bigtiff: `bool` -> Write a bigtiff image
    ///
    /// properties: `bool` -> Write a properties document to IMAGEDESCRIPTION
    ///
    /// region_shrink: `RegionShrink` -> Method to shrink regions
    ///
    /// level: `i32` -> Deflate (1-9, default 6) or ZSTD (1-22, default 9) compression level
    ///
    /// lossless: `bool` -> Enable WEBP lossless mode
    ///
    /// depth: `ForeignDzDepth` -> Pyramid depth
    ///
    /// subifd: `bool` -> Save pyr layers as sub-IFDs
    ///
    /// premultiply: `bool` -> Save with premultiplied alpha
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn tiffsave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "tiffsave_buffer",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("TiffsaveBuffer (vips_tiffsave_buffer) failed"),
        )
    }

    /// VipsForeignSaveTiffTarget (tiffsave_target), save image to tiff target (.tif, .tiff), priority=0, any
    ///
    /// target: `&VipsTarget` -> Target to save to
    pub fn tiffsave_target(&self, target: &VipsTarget) -> Result<()> {
        let vips_op_response = call(
            "tiffsave_target",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("TiffsaveTarget (vips_tiffsave_target) failed"),
        )
    }

    /// VipsForeignSaveTiffTarget (tiffsave_target), save image to tiff target (.tif, .tiff), priority=0, any
    ///
    /// target: `&VipsTarget` -> Target to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// compression: `ForeignTiffCompression` -> Compression for this file
    ///
    /// Q: `i32` -> Q factor
    ///
    /// predictor: `ForeignTiffPredictor` -> Compression prediction
    ///
    /// tile: `bool` -> Write a tiled tiff
    ///
    /// tile_width: `i32` -> Tile width in pixels
    ///
    /// tile_height: `i32` -> Tile height in pixels
    ///
    /// pyramid: `bool` -> Write a pyramidal tiff
    ///
    /// miniswhite: `bool` -> Use 0 for white in 1-bit images
    ///
    /// bitdepth: `i32` -> Write as a 1, 2, 4 or 8 bit image
    ///
    /// resunit: `ForeignTiffResunit` -> Resolution unit
    ///
    /// xres: `f64` -> Horizontal resolution in pixels/mm
    ///
    /// yres: `f64` -> Vertical resolution in pixels/mm
    ///
    /// bigtiff: `bool` -> Write a bigtiff image
    ///
    /// properties: `bool` -> Write a properties document to IMAGEDESCRIPTION
    ///
    /// region_shrink: `RegionShrink` -> Method to shrink regions
    ///
    /// level: `i32` -> Deflate (1-9, default 6) or ZSTD (1-22, default 9) compression level
    ///
    /// lossless: `bool` -> Enable WEBP lossless mode
    ///
    /// depth: `ForeignDzDepth` -> Pyramid depth
    ///
    /// subifd: `bool` -> Save pyr layers as sub-IFDs
    ///
    /// premultiply: `bool` -> Save with premultiplied alpha
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn tiffsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "tiffsave_target",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("TiffsaveTarget (vips_tiffsave_target) failed"),
        )
    }

    /// VipsTileCache (tilecache), cache an image as a set of tiles
    /// returns `VipsImage` - Output image
    pub fn tilecache(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "tilecache",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Tilecache (vips_tilecache) failed"),
        )
    }

    /// VipsTileCache (tilecache), cache an image as a set of tiles
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// tile_width: `i32` -> Tile width in pixels
    ///
    /// tile_height: `i32` -> Tile height in pixels
    ///
    /// max_tiles: `i32` -> Maximum number of tiles to cache
    ///
    /// access: `Access` -> Expected access pattern
    ///
    /// threaded: `bool` -> Allow threaded access
    ///
    /// persistent: `bool` -> Keep cache between evaluations
    pub fn tilecache_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "tilecache",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Tilecache (vips_tilecache) failed"),
        )
    }

    /// VipsTonelut (tonelut), build a look-up table
    /// returns `VipsImage` - Output image
    pub fn tonelut() -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "tonelut",
            VOption::new().with(
                "out",
                v_value!(&mut out_out),
            ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Tonelut (vips_tonelut) failed"),
        )
    }

    /// VipsTonelut (tonelut), build a look-up table
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// in_max: `i32` -> Size of LUT to build
    ///
    /// out_max: `i32` -> Maximum value in output LUT
    ///
    /// Lb: `f64` -> Lowest value in output
    ///
    /// Lw: `f64` -> Highest value in output
    ///
    /// Ps: `f64` -> Position of shadow
    ///
    /// Pm: `f64` -> Position of mid-tones
    ///
    /// Ph: `f64` -> Position of highlights
    ///
    /// S: `f64` -> Adjust shadows by this much
    ///
    /// M: `f64` -> Adjust mid-tones by this much
    ///
    /// H: `f64` -> Adjust highlights by this much
    pub fn tonelut_with_opts(option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "tonelut",
            option.with(
                "out",
                v_value!(&mut out_out),
            ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Tonelut (vips_tonelut) failed"),
        )
    }

    /// VipsTranspose3d (transpose3d), transpose3d an image
    /// returns `VipsImage` - Output image
    pub fn transpose3d(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "transpose3d",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Transpose3D (vips_transpose3d) failed"),
        )
    }

    /// VipsTranspose3d (transpose3d), transpose3d an image
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// page_height: `i32` -> Height of each input page
    pub fn transpose3d_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "transpose3d",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Transpose3D (vips_transpose3d) failed"),
        )
    }

    /// VipsUnpremultiply (unpremultiply), unpremultiply image alpha
    /// returns `VipsImage` - Output image
    pub fn unpremultiply(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "unpremultiply",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Unpremultiply (vips_unpremultiply) failed"),
        )
    }

    /// VipsUnpremultiply (unpremultiply), unpremultiply image alpha
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// max_alpha: `f64` -> Maximum value of alpha channel
    ///
    /// alpha_band: `i32` -> Unpremultiply with this alpha
    pub fn unpremultiply_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "unpremultiply",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Unpremultiply (vips_unpremultiply) failed"),
        )
    }

    /// VipsForeignLoadVipsFile (vipsload), load vips from file (.v, .vips), priority=200, untrusted, is_a, get_flags, get_flags_filename, header
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn vipsload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "vipsload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Vipsload (vips_vipsload) failed"),
        )
    }

    /// VipsForeignLoadVipsFile (vipsload), load vips from file (.v, .vips), priority=200, untrusted, is_a, get_flags, get_flags_filename, header
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn vipsload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "vipsload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Vipsload (vips_vipsload) failed"),
        )
    }

    /// VipsForeignLoadVipsSource (vipsload_source), load vips from source, priority=200, untrusted, is_a_source, get_flags, get_flags_filename, header
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    pub fn vipsload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "vipsload_source",
            VOption::new()
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("VipsloadSource (vips_vipsload_source) failed"),
        )
    }

    /// VipsForeignLoadVipsSource (vipsload_source), load vips from source, priority=200, untrusted, is_a_source, get_flags, get_flags_filename, header
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn vipsload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "vipsload_source",
            option
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("VipsloadSource (vips_vipsload_source) failed"),
        )
    }

    /// VipsForeignSaveVipsFile (vipssave), save image to file in vips format (.v, .vips), priority=0, any
    ///
    /// filename: `&str` -> Filename to save to
    pub fn vipssave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "vipssave",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Vipssave (vips_vipssave) failed"),
        )
    }

    /// VipsForeignSaveVipsFile (vipssave), save image to file in vips format (.v, .vips), priority=0, any
    ///
    /// filename: `&str` -> Filename to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn vipssave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "vipssave",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Vipssave (vips_vipssave) failed"),
        )
    }

    /// VipsForeignSaveVipsTarget (vipssave_target), save image to target in vips format (.v, .vips), priority=0, any
    ///
    /// target: `&VipsTarget` -> Target to save to
    pub fn vipssave_target(&self, target: &VipsTarget) -> Result<()> {
        let vips_op_response = call(
            "vipssave_target",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("VipssaveTarget (vips_vipssave_target) failed"),
        )
    }

    /// VipsForeignSaveVipsTarget (vipssave_target), save image to target in vips format (.v, .vips), priority=0, any
    ///
    /// target: `&VipsTarget` -> Target to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn vipssave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "vipssave_target",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("VipssaveTarget (vips_vipssave_target) failed"),
        )
    }

    /// VipsForeignLoadWebpFile (webpload), load webp from file (.webp), priority=200, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    pub fn webpload(filename: &str) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "webpload",
            VOption::new()
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Webpload (vips_webpload) failed"),
        )
    }

    /// VipsForeignLoadWebpFile (webpload), load webp from file (.webp), priority=200, is_a, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// filename: `&str` -> Filename to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// page: `i32` -> First page to load
    ///
    /// n: `i32` -> Number of pages to load, -1 for all
    ///
    /// scale: `f64` -> Factor to scale by
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn webpload_with_opts(filename: &str, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "webpload",
            option
                .with(
                    "filename",
                    v_value!(filename),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Webpload (vips_webpload) failed"),
        )
    }

    /// VipsForeignLoadWebpBuffer (webpload_buffer), load webp from buffer, priority=200, is_a_buffer, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    pub fn webpload_buffer(buffer: &[u8]) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "webpload_buffer",
            VOption::new()
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("WebploadBuffer (vips_webpload_buffer) failed"),
        )
    }

    /// VipsForeignLoadWebpBuffer (webpload_buffer), load webp from buffer, priority=200, is_a_buffer, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// buffer: `&[u8]` -> Buffer to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// page: `i32` -> First page to load
    ///
    /// n: `i32` -> Number of pages to load, -1 for all
    ///
    /// scale: `f64` -> Factor to scale by
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn webpload_buffer_with_opts(buffer: &[u8], option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "webpload_buffer",
            option
                .with(
                    "buffer",
                    v_value!(buffer),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("WebploadBuffer (vips_webpload_buffer) failed"),
        )
    }

    /// VipsForeignLoadWebpSource (webpload_source), load webp from source, priority=200, is_a_source, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    pub fn webpload_source(source: &VipsSource) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "webpload_source",
            VOption::new()
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("WebploadSource (vips_webpload_source) failed"),
        )
    }

    /// VipsForeignLoadWebpSource (webpload_source), load webp from source, priority=200, is_a_source, get_flags, get_flags_filename, header, load
    /// returns `VipsImage` - Output image
    ///
    /// source: `&VipsSource` -> Source to load from
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// page: `i32` -> First page to load
    ///
    /// n: `i32` -> Number of pages to load, -1 for all
    ///
    /// scale: `f64` -> Factor to scale by
    ///
    /// flags: `ForeignFlags` -> Flags for this file
    ///
    /// memory: `bool` -> Force open via memory
    ///
    /// access: `Access` -> Required access pattern for this file
    ///
    /// fail_on: `FailOn` -> Error level to fail on
    ///
    /// revalidate: `bool` -> Don't use a cached result for this operation
    pub fn webpload_source_with_opts(source: &VipsSource, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "webpload_source",
            option
                .with(
                    "source",
                    v_value!(source),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("WebploadSource (vips_webpload_source) failed"),
        )
    }

    /// VipsForeignSaveWebpFile (webpsave), save as WebP (.webp), priority=0, rgba-only
    ///
    /// filename: `&str` -> Filename to save to
    pub fn webpsave(&self, filename: &str) -> Result<()> {
        let vips_op_response = call(
            "webpsave",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Webpsave (vips_webpsave) failed"),
        )
    }

    /// VipsForeignSaveWebpFile (webpsave), save as WebP (.webp), priority=0, rgba-only
    ///
    /// filename: `&str` -> Filename to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// Q: `i32` -> Q factor
    ///
    /// lossless: `bool` -> Enable lossless compression
    ///
    /// preset: `ForeignWebpPreset` -> Preset for lossy compression
    ///
    /// smart_subsample: `bool` -> Enable high quality chroma subsampling
    ///
    /// near_lossless: `bool` -> Enable preprocessing in lossless mode (uses Q)
    ///
    /// alpha_q: `i32` -> Change alpha plane fidelity for lossy compression
    ///
    /// min_size: `bool` -> Optimise for minimum size
    ///
    /// kmin: `i32` -> Minimum number of frames between key frames
    ///
    /// kmax: `i32` -> Maximum number of frames between key frames
    ///
    /// effort: `i32` -> Level of CPU effort to reduce file size
    ///
    /// target_size: `i32` -> Desired target size in bytes
    ///
    /// mixed: `bool` -> Allow mixed encoding (might reduce file size)
    ///
    /// smart_deblock: `bool` -> Enable auto-adjusting of the deblocking filter
    ///
    /// passes: `i32` -> Number of entropy-analysis passes (in [1..10])
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn webpsave_with_opts(&self, filename: &str, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "webpsave",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "filename",
                    v_value!(filename),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("Webpsave (vips_webpsave) failed"),
        )
    }

    /// VipsForeignSaveWebpBuffer (webpsave_buffer), save as WebP (.webp), priority=0, rgba-only
    /// returns `Vec<u8>` - Buffer to save to
    pub fn webpsave_buffer(&self) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "webpsave_buffer",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("WebpsaveBuffer (vips_webpsave_buffer) failed"),
        )
    }

    /// VipsForeignSaveWebpBuffer (webpsave_buffer), save as WebP (.webp), priority=0, rgba-only
    /// returns `Vec<u8>` - Buffer to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// Q: `i32` -> Q factor
    ///
    /// lossless: `bool` -> Enable lossless compression
    ///
    /// preset: `ForeignWebpPreset` -> Preset for lossy compression
    ///
    /// smart_subsample: `bool` -> Enable high quality chroma subsampling
    ///
    /// near_lossless: `bool` -> Enable preprocessing in lossless mode (uses Q)
    ///
    /// alpha_q: `i32` -> Change alpha plane fidelity for lossy compression
    ///
    /// min_size: `bool` -> Optimise for minimum size
    ///
    /// kmin: `i32` -> Minimum number of frames between key frames
    ///
    /// kmax: `i32` -> Maximum number of frames between key frames
    ///
    /// effort: `i32` -> Level of CPU effort to reduce file size
    ///
    /// target_size: `i32` -> Desired target size in bytes
    ///
    /// mixed: `bool` -> Allow mixed encoding (might reduce file size)
    ///
    /// smart_deblock: `bool` -> Enable auto-adjusting of the deblocking filter
    ///
    /// passes: `i32` -> Number of entropy-analysis passes (in [1..10])
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn webpsave_buffer_with_opts(&self, option: VOption) -> Result<Vec<u8>> {
        let mut buffer_out = VipsBlob::from(null_mut());
        let vips_op_response = call(
            "webpsave_buffer",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "buffer",
                    v_value!(&mut buffer_out),
                ),
        );
        utils::result(
            vips_op_response,
            buffer_out.into(),
            Error::OperationError("WebpsaveBuffer (vips_webpsave_buffer) failed"),
        )
    }

    /// VipsForeignSaveWebpMime (webpsave_mime), save image to webp mime (.webp), priority=0, rgba-only
    pub fn webpsave_mime(&self) -> Result<()> {
        let vips_op_response = call(
            "webpsave_mime",
            VOption::new().with(
                "in",
                v_value!(&VipsImage::from(self.ctx)),
            ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("WebpsaveMime (vips_webpsave_mime) failed"),
        )
    }

    /// VipsForeignSaveWebpMime (webpsave_mime), save image to webp mime (.webp), priority=0, rgba-only
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// Q: `i32` -> Q factor
    ///
    /// lossless: `bool` -> Enable lossless compression
    ///
    /// preset: `ForeignWebpPreset` -> Preset for lossy compression
    ///
    /// smart_subsample: `bool` -> Enable high quality chroma subsampling
    ///
    /// near_lossless: `bool` -> Enable preprocessing in lossless mode (uses Q)
    ///
    /// alpha_q: `i32` -> Change alpha plane fidelity for lossy compression
    ///
    /// min_size: `bool` -> Optimise for minimum size
    ///
    /// kmin: `i32` -> Minimum number of frames between key frames
    ///
    /// kmax: `i32` -> Maximum number of frames between key frames
    ///
    /// effort: `i32` -> Level of CPU effort to reduce file size
    ///
    /// target_size: `i32` -> Desired target size in bytes
    ///
    /// mixed: `bool` -> Allow mixed encoding (might reduce file size)
    ///
    /// smart_deblock: `bool` -> Enable auto-adjusting of the deblocking filter
    ///
    /// passes: `i32` -> Number of entropy-analysis passes (in [1..10])
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn webpsave_mime_with_opts(&self, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "webpsave_mime",
            option.with(
                "in",
                v_value!(&VipsImage::from(self.ctx)),
            ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("WebpsaveMime (vips_webpsave_mime) failed"),
        )
    }

    /// VipsForeignSaveWebpTarget (webpsave_target), save as WebP (.webp), priority=0, rgba-only
    ///
    /// target: `&VipsTarget` -> Target to save to
    pub fn webpsave_target(&self, target: &VipsTarget) -> Result<()> {
        let vips_op_response = call(
            "webpsave_target",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("WebpsaveTarget (vips_webpsave_target) failed"),
        )
    }

    /// VipsForeignSaveWebpTarget (webpsave_target), save as WebP (.webp), priority=0, rgba-only
    ///
    /// target: `&VipsTarget` -> Target to save to
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// Q: `i32` -> Q factor
    ///
    /// lossless: `bool` -> Enable lossless compression
    ///
    /// preset: `ForeignWebpPreset` -> Preset for lossy compression
    ///
    /// smart_subsample: `bool` -> Enable high quality chroma subsampling
    ///
    /// near_lossless: `bool` -> Enable preprocessing in lossless mode (uses Q)
    ///
    /// alpha_q: `i32` -> Change alpha plane fidelity for lossy compression
    ///
    /// min_size: `bool` -> Optimise for minimum size
    ///
    /// kmin: `i32` -> Minimum number of frames between key frames
    ///
    /// kmax: `i32` -> Maximum number of frames between key frames
    ///
    /// effort: `i32` -> Level of CPU effort to reduce file size
    ///
    /// target_size: `i32` -> Desired target size in bytes
    ///
    /// mixed: `bool` -> Allow mixed encoding (might reduce file size)
    ///
    /// smart_deblock: `bool` -> Enable auto-adjusting of the deblocking filter
    ///
    /// passes: `i32` -> Number of entropy-analysis passes (in [1..10])
    ///
    /// keep: `ForeignKeep` -> Which metadata to retain
    ///
    /// background: `&[f64]` -> Background value
    ///
    /// page_height: `i32` -> Set page height for multipage save
    ///
    /// profile: `&str` -> Filename of ICC profile to embed
    pub fn webpsave_target_with_opts(&self, target: &VipsTarget, option: VOption) -> Result<()> {
        let vips_op_response = call(
            "webpsave_target",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "target",
                    v_value!(target),
                ),
        );
        utils::result(
            vips_op_response,
            (),
            Error::OperationError("WebpsaveTarget (vips_webpsave_target) failed"),
        )
    }

    /// VipsWorley (worley), make a worley noise image
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    pub fn worley(width: i32, height: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "worley",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Worley (vips_worley) failed"),
        )
    }

    /// VipsWorley (worley), make a worley noise image
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// cell_size: `i32` -> Size of Worley cells
    ///
    /// seed: `i32` -> Random number seed
    pub fn worley_with_opts(width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "worley",
            option
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Worley (vips_worley) failed"),
        )
    }

    /// VipsWrap (wrap), wrap image origin
    /// returns `VipsImage` - Output image
    pub fn wrap(&self) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "wrap",
            VOption::new()
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Wrap (vips_wrap) failed"),
        )
    }

    /// VipsWrap (wrap), wrap image origin
    /// returns `VipsImage` - Output image
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// x: `i32` -> Left edge of input in output
    ///
    /// y: `i32` -> Top edge of input in output
    pub fn wrap_with_opts(&self, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "wrap",
            option
                .with(
                    "in",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Wrap (vips_wrap) failed"),
        )
    }

    /// VipsXyz (xyz), make an image where pixel values are coordinates
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    pub fn xyz(width: i32, height: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "xyz",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Xyz (vips_xyz) failed"),
        )
    }

    /// VipsXyz (xyz), make an image where pixel values are coordinates
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// csize: `i32` -> Size of third dimension
    ///
    /// dsize: `i32` -> Size of fourth dimension
    ///
    /// esize: `i32` -> Size of fifth dimension
    pub fn xyz_with_opts(width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "xyz",
            option
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Xyz (vips_xyz) failed"),
        )
    }

    /// VipsZone (zone), make a zone plate
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    pub fn zone(width: i32, height: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "zone",
            VOption::new()
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Zone (vips_zone) failed"),
        )
    }

    /// VipsZone (zone), make a zone plate
    /// returns `VipsImage` - Output image
    ///
    /// width: `i32` -> Image width in pixels
    ///
    /// height: `i32` -> Image height in pixels
    ///
    /// <ins>Optional arguments</ins>
    ///
    /// uchar: `bool` -> Output an unsigned char image
    pub fn zone_with_opts(width: i32, height: i32, option: VOption) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "zone",
            option
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "width",
                    v_value!(width),
                )
                .with(
                    "height",
                    v_value!(height),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Zone (vips_zone) failed"),
        )
    }

    /// VipsZoom (zoom), zoom an image
    /// returns `VipsImage` - Output image
    ///
    /// xfac: `i32` -> Horizontal zoom factor
    ///
    /// yfac: `i32` -> Vertical zoom factor
    pub fn zoom(&self, xfac: i32, yfac: i32) -> Result<VipsImage> {
        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "zoom",
            VOption::new()
                .with(
                    "input",
                    v_value!(&VipsImage::from(self.ctx)),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                )
                .with(
                    "xfac",
                    v_value!(xfac),
                )
                .with(
                    "yfac",
                    v_value!(yfac),
                ),
        );
        utils::result(
            vips_op_response,
            out_out,
            Error::OperationError("Zoom (vips_zoom) failed"),
        )
    }

    // Alias for operator overload
    pub(crate) fn add_image(&self, right: &VipsImage) -> Result<VipsImage> {
        self.add(right)
    }

    /// VipsBandjoin (bandjoin), bandwise join a set of images
    /// returns `VipsImage` - Output image
    ///
    /// others: `&[VipsImage]` -> Array of input images
    pub fn bandjoin_with(&self, others: &[VipsImage]) -> Result<VipsImage> {
        let mut inp_in = Vec::new();
        inp_in.push(
            VipsImage {
                ctx: self.ctx,
            },
        );
        for img in others {
            inp_in.push(
                VipsImage {
                    ctx: img.ctx,
                },
            )
        }

        let mut out_out = VipsImage::from(null_mut());
        let vips_op_response = call(
            "bandjoin",
            VOption::new()
                .with(
                    "in",
                    v_value!(inp_in.as_slice()),
                )
                .with(
                    "out",
                    v_value!(&mut out_out),
                ),
        );
        utils::result(
            vips_op_response,
            VipsImage {
                ctx: out_out.ctx,
            },
            Error::OperationError("Bandjoin (vips_bandjoin) failed"),
        )
    }

    /// VipsMedian (median), median filter of the specified size.
    pub fn median(&self, size: i32) -> Result<VipsImage> {
        self.rank(
            size,
            size,
            (size * size) / 2,
        )
    }
}
