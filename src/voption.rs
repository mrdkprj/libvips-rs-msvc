use crate::bindings::*;
use crate::utils::new_c_string;
use std::{mem::MaybeUninit, os::raw::c_char};

extern "C" {
    fn vo_set_bool(gvalue: *mut GValue, value: bool);
    fn vo_set_int(gvalue: *mut GValue, value: i32);
    fn vo_set_guint64(gvalue: *mut GValue, value: u64);
    fn vo_set_double(gvalue: *mut GValue, value: f64);
    fn vo_set_string(gvalue: *mut GValue, value: *const c_char);
    fn vo_set_image(gvalue: *mut GValue, value: *mut VipsImage);
    fn vo_set_target(gvalue: *mut GValue, value: *mut VipsTarget);
    fn vo_set_source(gvalue: *mut GValue, value: *mut VipsSource);
    fn vo_set_interpolate(gvalue: *mut GValue, value: *mut VipsInterpolate);
    fn vo_set_int_array(gvalue: *mut GValue, value: *const i32, size: i32);
    fn vo_set_double_array(gvalue: *mut GValue, value: *const f64, size: i32);
    fn vo_set_images(gvalue: *mut GValue, value: *const *mut VipsImage, size: i32);
    fn vo_set_blob(gvalue: *mut GValue, value: *mut VipsBlob);
    fn vo_set_object(gvalue: *mut GValue, value: *mut VipsObject);
    fn vo_get_bool(op: *mut VipsOperation, name: *const c_char, out: &mut bool);
    fn vo_get_int(op: *mut VipsOperation, name: *const c_char, out: &mut i32);
    fn vo_get_double(op: *mut VipsOperation, name: *const c_char, out: &mut f64);
    fn vo_get_double_array(op: *mut VipsOperation, name: *const c_char, out: *mut f64, size: i32);
    fn vo_get_blob(op: *mut VipsOperation, name: *const c_char, out: *mut *mut VipsBlob);
    fn vo_get_image(op: *mut VipsOperation, name: *const c_char, out: *mut *mut VipsImage);
    fn vo_set_property(object: *mut GObject, property_name: *const gchar, value: *const GValue);
}

#[macro_export]
macro_rules! v_value {
    ($value:expr) => {
        V_Value::value($value)
    };
}

pub use v_value;

pub trait V_Value<'a> {
    fn value(self) -> VipsValue<'a>;
}

impl<'a> V_Value<'a> for bool {
    fn value(self) -> VipsValue<'a> {
        VipsValue::Bool(self)
    }
}

impl<'a> V_Value<'a> for i32 {
    fn value(self) -> VipsValue<'a> {
        VipsValue::Int(self)
    }
}

impl<'a> V_Value<'a> for u64 {
    fn value(self) -> VipsValue<'a> {
        VipsValue::Uint(self)
    }
}

impl<'a> V_Value<'a> for f64 {
    fn value(self) -> VipsValue<'a> {
        VipsValue::Double(self)
    }
}

impl<'a> V_Value<'a> for &'a str {
    fn value(self) -> VipsValue<'a> {
        VipsValue::Str(self)
    }
}

impl<'a> V_Value<'a> for &'a String {
    fn value(self) -> VipsValue<'a> {
        VipsValue::Str(self)
    }
}

impl<'a> V_Value<'a> for &'a [i32] {
    fn value(self) -> VipsValue<'a> {
        VipsValue::IntArray(self)
    }
}

impl<'a> V_Value<'a> for &'a [f64] {
    fn value(self) -> VipsValue<'a> {
        VipsValue::DoubleArray(self)
    }
}

impl<'a> V_Value<'a> for &'a crate::VipsImage {
    fn value(self) -> VipsValue<'a> {
        VipsValue::Image(self)
    }
}

impl<'a> V_Value<'a> for &'a [crate::VipsImage] {
    fn value(self) -> VipsValue<'a> {
        VipsValue::ImageArray(self)
    }
}

impl<'a> V_Value<'a> for &'a crate::VipsTarget {
    fn value(self) -> VipsValue<'a> {
        VipsValue::Target(self)
    }
}

impl<'a> V_Value<'a> for &'a crate::VipsSource {
    fn value(self) -> VipsValue<'a> {
        VipsValue::Source(self)
    }
}

impl<'a> V_Value<'a> for &'a crate::VipsInterpolate {
    fn value(self) -> VipsValue<'a> {
        VipsValue::Interpolate(self)
    }
}

impl<'a> V_Value<'a> for &'a mut bool {
    fn value(self) -> VipsValue<'a> {
        VipsValue::MutBool(self)
    }
}

impl<'a> V_Value<'a> for &'a mut i32 {
    fn value(self) -> VipsValue<'a> {
        VipsValue::MutInt(self)
    }
}

impl<'a> V_Value<'a> for &'a mut f64 {
    fn value(self) -> VipsValue<'a> {
        VipsValue::MutDouble(self)
    }
}

impl<'a> V_Value<'a> for &'a mut crate::VipsImage {
    fn value(self) -> VipsValue<'a> {
        VipsValue::MutImage(self)
    }
}

impl<'a> V_Value<'a> for &'a mut [f64] {
    fn value(self) -> VipsValue<'a> {
        VipsValue::MutDoubleArray(self)
    }
}

impl<'a> V_Value<'a> for &'a mut crate::VipsBlob {
    fn value(self) -> VipsValue<'a> {
        VipsValue::MutBlob(self)
    }
}

pub enum VipsValue<'a> {
    Bool(bool),
    MutBool(&'a mut bool),
    Int(i32),
    MutInt(&'a mut i32),
    Uint(u64),
    Double(f64),
    MutDouble(&'a mut f64),
    Str(&'a str),
    Image(&'a crate::VipsImage),
    ImagePtr(*mut VipsImage),
    MutImage(&'a mut crate::VipsImage),
    MutImagePtr(&'a mut *mut VipsImage),
    IntArray(&'a [i32]),
    DoubleArray(&'a [f64]),
    MutDoubleArray(&'a mut [f64]),
    ImageArray(&'a [crate::VipsImage]),
    ImagePtrArray(&'a [*mut VipsImage]),
    Blob(&'a crate::VipsBlob),
    BlobPtr(*mut VipsBlob),
    Buffer(&'a [u8]),
    MutBlob(&'a mut crate::VipsBlob),
    MutBlobPtr(&'a mut *mut VipsBlob),
    Target(&'a crate::VipsTarget),
    TargetPtr(*mut VipsTarget),
    Source(&'a crate::VipsSource),
    SourcePtr(*mut VipsSource),
    Interpolate(&'a crate::VipsInterpolate),
    InterpolatePtr(*mut VipsInterpolate),
}

struct Pair<'a> {
    input: bool,
    name: String,
    value: VipsValue<'a>,
}

#[derive(Default)]
pub struct VOption<'a> {
    options: Vec<Pair<'a>>,
}

impl<'a> VOption<'a> {
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
        }
    }

    pub fn set(&mut self, name: &str, vips_value: VipsValue<'a>) {
        match vips_value {
            VipsValue::MutBool(_)
            | VipsValue::MutInt(_)
            | VipsValue::MutDouble(_)
            | VipsValue::MutImage(_)
            | VipsValue::MutImagePtr(_)
            | VipsValue::MutDoubleArray(_)
            | VipsValue::MutBlob(_)
            | VipsValue::MutBlobPtr(_) => {
                self.options
                    .push(Pair {
                        input: false,
                        name: name.to_string(),
                        value: vips_value,
                    });
            }
            _ => {
                self.options
                    .push(Pair {
                        input: true,
                        name: name.to_string(),
                        value: vips_value,
                    });
            }
        };
    }

    pub fn with(mut self, name: &str, vips_value: VipsValue<'a>) -> Self {
        match vips_value {
            VipsValue::MutBool(_)
            | VipsValue::MutInt(_)
            | VipsValue::MutDouble(_)
            | VipsValue::MutImage(_)
            | VipsValue::MutImagePtr(_)
            | VipsValue::MutDoubleArray(_)
            | VipsValue::MutBlob(_)
            | VipsValue::MutBlobPtr(_) => {
                self.options
                    .push(Pair {
                        input: false,
                        name: name.to_string(),
                        value: vips_value,
                    });
            }
            _ => {
                self.options
                    .push(Pair {
                        input: true,
                        name: name.to_string(),
                        value: vips_value,
                    });
            }
        };

        self
    }
}

fn get_operation(vips_operation: *mut VipsOperation, option: VOption) {
    unsafe {
        for opt in option.options {
            if opt.input {
                match opt.value {
                    // Unref input
                    VipsValue::Image(inp) => g_object_unref(inp.ctx as _),
                    VipsValue::Blob(inp) => g_object_unref(inp.ctx as _),
                    VipsValue::Target(inp) => g_object_unref(inp.ctx as _),
                    VipsValue::Source(inp) => g_object_unref(inp.ctx as _),
                    VipsValue::Interpolate(inp) => g_object_unref(inp.ctx as _),
                    VipsValue::ImagePtr(inp) => g_object_unref(inp as _),
                    VipsValue::BlobPtr(inp) => g_object_unref(inp as _),
                    VipsValue::TargetPtr(inp) => g_object_unref(inp as _),
                    VipsValue::SourcePtr(inp) => g_object_unref(inp as _),
                    VipsValue::InterpolatePtr(inp) => g_object_unref(inp as _),
                    VipsValue::ImageArray(inps) => {
                        for inp in inps {
                            g_object_unref(inp.ctx as _)
                        }
                    }
                    VipsValue::ImagePtrArray(inps) => {
                        for inp in inps {
                            g_object_unref(*inp as _)
                        }
                    }
                    _ => {}
                };
            } else {
                let name = new_c_string(&opt.name).unwrap();
                match opt.value {
                    VipsValue::MutInt(out) => vo_get_int(
                        vips_operation,
                        name.as_ptr(),
                        out,
                    ),
                    VipsValue::MutBool(out) => vo_get_bool(
                        vips_operation,
                        name.as_ptr(),
                        out,
                    ),
                    VipsValue::MutDouble(out) => vo_get_double(
                        vips_operation,
                        name.as_ptr(),
                        out,
                    ),
                    VipsValue::MutDoubleArray(out) => vo_get_double_array(
                        vips_operation,
                        name.as_ptr(),
                        out.as_mut_ptr(),
                        out.len() as _,
                    ),
                    VipsValue::MutBlob(out) => {
                        vo_get_blob(
                            vips_operation,
                            name.as_ptr(),
                            &mut out.ctx,
                        );
                    }
                    VipsValue::MutBlobPtr(out) => {
                        vo_get_blob(
                            vips_operation,
                            name.as_ptr(),
                            out,
                        );
                    }
                    VipsValue::MutImage(out) => vo_get_image(
                        vips_operation,
                        name.as_ptr(),
                        &mut out.ctx,
                    ),
                    VipsValue::MutImagePtr(out) => vo_get_image(
                        vips_operation,
                        name.as_ptr(),
                        out,
                    ),
                    _ => {}
                }
            }
        }
    }
}

fn set_opreration(operation: *mut VipsOperation, option: &VOption) {
    unsafe {
        let object: *mut GObject = operation.cast();
        for pair in &option.options {
            if !pair.input {
                continue;
            }
            let mut gvalue = MaybeUninit::<GValue>::zeroed();
            let gvalue_ptr = gvalue.as_mut_ptr();

            match pair.value {
                VipsValue::Bool(value) => vo_set_bool(
                    gvalue_ptr,
                    value,
                ),
                VipsValue::Double(value) => vo_set_double(
                    gvalue_ptr,
                    value,
                ),
                VipsValue::Int(value) => vo_set_int(
                    gvalue_ptr,
                    value,
                ),
                VipsValue::DoubleArray(value) => vo_set_double_array(
                    gvalue_ptr,
                    value.as_ptr(),
                    value.len() as _,
                ),
                VipsValue::Image(value) => vo_set_image(
                    gvalue_ptr,
                    value.ctx,
                ),
                VipsValue::ImagePtr(value) => vo_set_image(
                    gvalue_ptr,
                    value,
                ),
                VipsValue::ImageArray(value) => {
                    let images: Vec<*mut _VipsImage> = value
                        .iter()
                        .map(|i| i.ctx)
                        .collect();
                    vo_set_images(
                        gvalue_ptr,
                        images.as_ptr(),
                        value.len() as _,
                    );
                }
                VipsValue::ImagePtrArray(value) => {
                    vo_set_images(
                        gvalue_ptr,
                        value.as_ptr(),
                        value.len() as _,
                    );
                }
                VipsValue::IntArray(value) => vo_set_int_array(
                    gvalue_ptr,
                    value.as_ptr(),
                    value.len() as _,
                ),
                VipsValue::Str(value) => {
                    let str = new_c_string(value).unwrap();
                    vo_set_string(
                        gvalue_ptr,
                        str.as_ptr(),
                    );
                }
                VipsValue::Uint(value) => vo_set_guint64(
                    gvalue_ptr,
                    value,
                ),
                VipsValue::Blob(value) => {
                    vo_set_blob(
                        gvalue_ptr,
                        value.ctx,
                    );
                }
                VipsValue::BlobPtr(value) => {
                    vo_set_blob(
                        gvalue_ptr,
                        value,
                    );
                }
                VipsValue::Buffer(value) => {
                    let blob = vips_blob_new(
                        None,
                        value.as_ptr() as _,
                        value.len() as _,
                    );
                    vo_set_blob(
                        gvalue_ptr,
                        blob,
                    );
                }
                VipsValue::Source(value) => vo_set_source(
                    gvalue_ptr,
                    value.ctx,
                ),
                VipsValue::SourcePtr(value) => vo_set_source(
                    gvalue_ptr,
                    value,
                ),
                VipsValue::Target(value) => vo_set_target(
                    gvalue_ptr,
                    value.ctx,
                ),
                VipsValue::TargetPtr(value) => vo_set_target(
                    gvalue_ptr,
                    value,
                ),
                VipsValue::Interpolate(value) => vo_set_interpolate(
                    gvalue_ptr,
                    value.ctx,
                ),
                VipsValue::InterpolatePtr(value) => vo_set_interpolate(
                    gvalue_ptr,
                    value,
                ),
                _ => {}
            }
            let name = new_c_string(&pair.name).unwrap();
            vo_set_property(
                object,
                name.as_ptr(),
                gvalue_ptr,
            );
        }
    }
}

pub fn call(operation: &str, option: VOption) -> std::os::raw::c_int {
    unsafe {
        let operation_name = new_c_string(operation).unwrap();
        let mut vips_operation = vips_operation_new(operation_name.as_ptr());

        set_opreration(
            vips_operation,
            &option,
        );

        let result = vips_cache_operation_buildp(&mut vips_operation);
        if result == 1 {
            vips_object_unref_outputs(vips_operation as _);
            g_object_unref(vips_operation as _);
            return result;
        }

        get_operation(
            vips_operation,
            option,
        );

        g_object_unref(vips_operation as _);

        result
    }
}
