use crate::{
    bindings::*,
    utils::{
        get_g_type, new_c_string, G_TYPE_BOOLEAN, G_TYPE_DOUBLE, G_TYPE_INT, G_TYPE_STRING,
        G_TYPE_UINT64,
    },
};
use std::{mem::MaybeUninit, os::raw::c_void};

/// Runs the vips operation with options
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

impl<'a> V_Value<'a> for &'a mut Vec<f64> {
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
    MutImage(&'a mut crate::VipsImage),
    IntArray(&'a [i32]),
    DoubleArray(&'a [f64]),
    MutDoubleArray(&'a mut Vec<f64>),
    ImageArray(&'a [crate::VipsImage]),
    Blob(&'a crate::VipsBlob),
    Buffer(&'a [u8]),
    MutBlob(&'a mut crate::VipsBlob),
    Target(&'a crate::VipsTarget),
    Source(&'a crate::VipsSource),
    Interpolate(&'a crate::VipsInterpolate),
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
            | VipsValue::MutDoubleArray(_)
            | VipsValue::MutBlob(_) => {
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
            | VipsValue::MutDoubleArray(_)
            | VipsValue::MutBlob(_) => {
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
                continue;
            }

            let mut gvalue = MaybeUninit::<GValue>::zeroed();
            let value = gvalue.as_mut_ptr();
            let name = new_c_string(&opt.name).unwrap();
            match opt.value {
                VipsValue::MutBool(out) => {
                    g_value_init(
                        value,
                        get_g_type(G_TYPE_BOOLEAN),
                    );
                    g_object_get_property(
                        vips_operation.cast(),
                        name.as_ptr(),
                        value,
                    );
                    *out = g_value_get_boolean(value) != 0;
                }
                VipsValue::MutInt(out) => {
                    g_value_init(
                        value,
                        get_g_type(G_TYPE_INT),
                    );
                    g_object_get_property(
                        vips_operation.cast(),
                        name.as_ptr(),
                        value,
                    );
                    *out = g_value_get_int(value);
                }
                VipsValue::MutDouble(out) => {
                    g_value_init(
                        value,
                        get_g_type(G_TYPE_DOUBLE),
                    );
                    g_object_get_property(
                        vips_operation.cast(),
                        name.as_ptr(),
                        value,
                    );
                    *out = g_value_get_double(value);
                }
                VipsValue::MutDoubleArray(out) => {
                    g_value_init(
                        value,
                        vips_array_double_get_type(),
                    );
                    g_object_get_property(
                        vips_operation.cast(),
                        name.as_ptr(),
                        value,
                    );
                    let mut len: i32 = 0;
                    let array = vips_value_get_array_double(
                        value,
                        &mut len,
                    );
                    let result = std::slice::from_raw_parts(
                        array,
                        len as usize,
                    );
                    out.extend(result);
                }
                VipsValue::MutBlob(out) => {
                    g_value_init(
                        value,
                        vips_blob_get_type(),
                    );
                    g_object_get_property(
                        vips_operation.cast(),
                        name.as_ptr(),
                        value,
                    );
                    let out_blob: *mut VipsBlob = g_value_dup_boxed(value).cast();
                    out.ctx = out_blob;
                }
                VipsValue::MutImage(out) => {
                    g_value_init(
                        value,
                        vips_image_get_type(),
                    );
                    g_object_get_property(
                        vips_operation.cast(),
                        name.as_ptr(),
                        value,
                    );
                    let out_image: *mut VipsImage = g_value_get_object(value).cast();
                    out.ctx = out_image;
                }
                _ => {}
            }
            g_value_unset(value);
        }
    }
}

fn set_opreration(operation: *mut VipsOperation, option: &VOption) {
    unsafe {
        for pair in &option.options {
            if !pair.input {
                continue;
            }

            let mut gvalue = MaybeUninit::<GValue>::zeroed();
            let gvalue_ptr = gvalue.as_mut_ptr();
            match pair.value {
                VipsValue::Bool(value) => {
                    g_value_init(
                        gvalue_ptr,
                        get_g_type(G_TYPE_BOOLEAN),
                    );
                    g_value_set_boolean(
                        gvalue_ptr,
                        value.into(),
                    );
                }
                VipsValue::Int(value) => {
                    g_value_init(
                        gvalue_ptr,
                        get_g_type(G_TYPE_INT),
                    );
                    g_value_set_int(
                        gvalue_ptr,
                        value,
                    );
                }
                VipsValue::Uint(value) => {
                    g_value_init(
                        gvalue_ptr,
                        get_g_type(G_TYPE_UINT64),
                    );
                    g_value_set_uint64(
                        gvalue_ptr,
                        value,
                    );
                }
                VipsValue::Double(value) => {
                    g_value_init(
                        gvalue_ptr,
                        get_g_type(G_TYPE_DOUBLE),
                    );
                    g_value_set_double(
                        gvalue_ptr,
                        value,
                    );
                }
                VipsValue::Str(value) => {
                    let str = new_c_string(value).unwrap();
                    g_value_init(
                        gvalue_ptr,
                        get_g_type(G_TYPE_STRING),
                    );
                    g_value_set_string(
                        gvalue_ptr,
                        str.as_ptr(),
                    );
                }
                VipsValue::IntArray(value) => {
                    g_value_init(
                        gvalue_ptr,
                        vips_array_int_get_type(),
                    );
                    vips_value_set_array_int(
                        gvalue_ptr,
                        value.as_ptr(),
                        value.len() as _,
                    );
                }
                VipsValue::DoubleArray(value) => {
                    g_value_init(
                        gvalue_ptr,
                        vips_array_double_get_type(),
                    );
                    vips_value_set_array_double(
                        gvalue_ptr,
                        value.as_ptr(),
                        value.len() as _,
                    );
                }
                VipsValue::Image(value) => {
                    g_value_init(
                        gvalue_ptr,
                        vips_image_get_type(),
                    );
                    g_value_set_object(
                        gvalue_ptr,
                        value.ctx as *mut c_void,
                    );
                }
                VipsValue::ImageArray(value) => {
                    g_value_init(
                        gvalue_ptr,
                        vips_array_image_get_type(),
                    );
                    vips_value_set_array_image(
                        gvalue_ptr,
                        value.len() as _,
                    );
                    let array = vips_value_get_array_image(
                        gvalue_ptr,
                        &mut 0,
                    );
                    let array = std::slice::from_raw_parts_mut(
                        array,
                        value.len() as _,
                    );
                    for i in 0..value.len() {
                        array[i] = value[i].ctx;
                        g_object_ref(value[i].ctx as _);
                    }
                }
                VipsValue::Blob(value) => {
                    g_value_init(
                        gvalue_ptr,
                        vips_blob_get_type(),
                    );
                    g_value_set_boxed(
                        gvalue_ptr,
                        value.ctx as *const c_void,
                    );
                    // vips_area_unref(value.ctx as _);
                }
                VipsValue::Buffer(value) => {
                    let blob = vips_blob_new(
                        None,
                        value.as_ptr() as _,
                        value.len() as _,
                    );
                    g_value_init(
                        gvalue_ptr,
                        vips_blob_get_type(),
                    );
                    g_value_set_boxed(
                        gvalue_ptr,
                        blob as *const c_void,
                    );
                }
                VipsValue::Source(value) => {
                    g_value_init(
                        gvalue_ptr,
                        vips_source_get_type(),
                    );
                    g_value_set_object(
                        gvalue_ptr,
                        value.ctx as *mut c_void,
                    );
                }
                VipsValue::Target(value) => {
                    g_value_init(
                        gvalue_ptr,
                        vips_target_get_type(),
                    );
                    g_value_set_object(
                        gvalue_ptr,
                        value.ctx as *mut c_void,
                    );
                }
                VipsValue::Interpolate(value) => {
                    g_value_init(
                        gvalue_ptr,
                        vips_interpolate_get_type(),
                    );
                    g_value_set_object(
                        gvalue_ptr,
                        value.ctx as *mut c_void,
                    );
                }
                _ => {}
            }

            set_property(
                operation,
                &pair.name,
                gvalue_ptr,
            );
        }
    }
}

fn set_property(operation: *mut VipsOperation, name: &str, value: *mut GValue) {
    unsafe {
        let object: *mut VipsObject = operation.cast();
        let name = new_c_string(name).unwrap();

        let mut pspec: *mut GParamSpec = std::ptr::null_mut();
        let mut argument_class: *mut VipsArgumentClass = std::ptr::null_mut();
        let mut argument_instance: *mut VipsArgumentInstance = std::ptr::null_mut();
        if vips_object_get_argument(
            object,
            name.as_ptr(),
            &mut pspec,
            &mut argument_class,
            &mut argument_instance,
        ) < 0
        {
            g_warning();
            vips_error_clear();
            return;
        }

        let is_param_spec_enum = g_type_check_instance_is_a(
            pspec as *mut GTypeInstance,
            get_g_type("GParamEnum"),
        ) != 0;

        if is_param_spec_enum && (*value).g_type == get_g_type(G_TYPE_STRING) {
            let pspec_type = (*pspec).value_type;
            let enum_value = vips_enum_from_nick(
                (*object).nickname,
                pspec_type,
                g_value_get_string(value),
            );
            if enum_value < 0 {
                g_warning();
                vips_error_clear();
                return;
            }

            let mut gvalue = MaybeUninit::<GValue>::zeroed();
            let value2 = gvalue.as_mut_ptr();
            g_value_init(
                value2,
                pspec_type,
            );
            g_value_set_enum(
                value2,
                enum_value,
            );
            g_object_set_property(
                object.cast(),
                name.as_ptr(),
                value2,
            );
            g_value_unset(value2);
        } else {
            g_object_set_property(
                object.cast(),
                name.as_ptr(),
                value,
            );
        }
    }
}

fn g_warning() {
    let domain = new_c_string("GLib-GObject").unwrap();
    let format = new_c_string("%s").unwrap();
    unsafe {
        g_log(
            domain.as_ptr(),
            GLogLevelFlags_G_LOG_LEVEL_WARNING,
            format.as_ptr(),
            vips_error_buffer(),
        )
    };
}
