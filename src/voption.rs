#![allow(dead_code)]
use crate::bindings::*;
use std::{
    ffi::{c_int, c_void, CString},
    mem::MaybeUninit,
};

extern "C" {
    fn vo_set_bool(gvalue: *mut GValue, value: bool);
    fn vo_set_int(gvalue: *mut GValue, value: i32);
    fn vo_set_guint64(gvalue: *mut GValue, value: u64);
    fn vo_set_double(gvalue: *mut GValue, value: f64);
    fn vo_set_string(gvalue: *mut GValue, value: *const std::os::raw::c_char);
    fn vo_set_image(gvalue: *mut GValue, value: *mut VipsImage);
    fn vo_set_target(gvalue: *mut GValue, value: *mut VipsTarget);
    fn vo_set_source(gvalue: *mut GValue, value: *mut VipsSource);
    fn vo_set_int_array(gvalue: *mut GValue, value: *const i32, size: i32);
    fn vo_set_double_array(gvalue: *mut GValue, value: *const f64, size: i32);
    fn vo_set_images(gvalue: *mut GValue, value: *const *mut VipsImage, size: i32);
    fn vo_set_blob(gvalue: *mut GValue, value: *mut VipsBlob);
    fn vo_get_bool(op: *mut VipsOperation, name: *const std::os::raw::c_char, out: &mut bool);
    fn vo_get_int(op: *mut VipsOperation, name: *const std::os::raw::c_char, out: &mut i32);
    fn vo_get_double(op: *mut VipsOperation, name: *const std::os::raw::c_char, out: &mut f64);
    fn vo_get_double_array(
        op: *mut VipsOperation,
        name: *const std::os::raw::c_char,
        out: *mut f64,
        size: i32,
    );
    fn vo_get_blob(
        op: *mut VipsOperation,
        name: *const std::os::raw::c_char,
        out: *mut *mut VipsBlob,
    );
    fn vo_get_image(
        op: *mut VipsOperation,
        name: *const std::os::raw::c_char,
        out: *mut *mut VipsImage,
    );
    fn vo_set_property(object: *mut GObject, property_name: *const gchar, value: *const GValue);
}

pub enum VipsValue<'a> {
    Bool(bool),
    MutBool(&'a mut bool),
    Int(i32),
    MutInt(&'a mut i32),
    Uint(u64),
    Double(f64),
    MutDouble(&'a mut f64),
    String(&'a str),
    Image(*mut VipsImage),
    MutImage(&'a mut *mut VipsImage),
    IntArray(&'a [i32]),
    DoubleArray(&'a [f64]),
    MutDoubleArray(&'a mut [f64]),
    ImageArray(&'a [*mut VipsImage]),
    Blob(&'a [u8]),
    MutBlob(&'a mut *mut VipsBlob),
    Target(*mut VipsTarget),
    Source(*mut VipsSource),
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

    pub fn set(mut self, name: &str, vips_value: VipsValue<'a>) -> Self {
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
pub fn call(operation: &str, option: VOption) -> std::os::raw::c_int {
    unsafe {
        let operation_name = new_c_string(operation).unwrap();
        let mut vips_operation = vips_operation_new(operation_name.as_ptr());

        set_opreration(
            vips_operation,
            &option,
        );

        // vips_cache_operation_build(vips_operation);
        let result = vips_cache_operation_buildp(&mut vips_operation);

        for opt in option.options {
            if opt.input {
                continue;
            }

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
                        out,
                    );
                }
                VipsValue::MutImage(out) => vo_get_image(
                    vips_operation,
                    name.as_ptr(),
                    out,
                ),
                _ => {}
            }
        }
        g_object_unref(vips_operation as _);

        result
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
                    value,
                ),
                VipsValue::ImageArray(value) => vo_set_images(
                    gvalue_ptr,
                    value.as_ptr(),
                    value.len() as _,
                ),
                VipsValue::IntArray(value) => vo_set_int_array(
                    gvalue_ptr,
                    value.as_ptr(),
                    value.len() as _,
                ),
                VipsValue::String(value) => {
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
                    let blob = vips_blob_new(
                        Some(vips_area_free_cb_wrapper),
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
                    value,
                ),
                VipsValue::Target(value) => vo_set_target(
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

fn new_c_string(string: &str) -> Result<CString, String> {
    CString::new(string).map_err(|_| "Error initializing C string".to_string())
}

unsafe extern "C" fn vips_area_free_cb_wrapper(mem: *mut c_void, area: *mut c_void) -> c_int {
    // Reinterpret the second argument as *mut VipsArea
    let area = area as *mut VipsArea;
    vips_area_free_cb(mem, area)
}
const NULL: *const std::ffi::c_void = std::ptr::null_mut();
/*
pub fn copy3() {
    println!("new");

    unsafe {
        let progname = CString::new("sharp-rs").unwrap();

        bindings::vips_init(progname.as_ptr());

        // let inp = libvips::bindings::vips_image_new_matrix(300, 300);
        let name = CString::new(r#"D:\@Download\test\img.jpg"#).unwrap();
        let inp = bindings::vips_image_new_from_file(
            name.as_ptr(),
            NULL,
        );
        let mut out_out: *mut VipsImage = std::ptr::null_mut();
        let mut attention_x = 0;
        let mut attention_y = 0;
        // let mut ops = VOption::new();
        let ops = VOption::new()
            .set(
                "width",
                VipsValue::Int(20),
            )
            .set(
                "height",
                VipsValue::Int(20),
            )
            .set(
                "attention_x",
                VipsValue::MutInt(&mut attention_x),
            )
            .set(
                "attention_y",
                VipsValue::MutInt(&mut attention_y),
            )
            .set(
                "input",
                VipsValue::Image(inp),
            )
            .set(
                "out",
                VipsValue::MutImage(&mut out_out),
            );

        // let mut value = MaybeUninit::<GValue>::zeroed();
        // let value_ptr = value.as_mut_ptr();
        // let g_type_int: usize = 10 << 2;
        // println!("type:{:?}", g_type_int);
        // let a = CString::new("gint").unwrap();
        // let g_type_int = g_type_from_name(a.as_ptr());
        // println!("type:{:?}", g_type_int);
        // // let a = CString::new("gint64").unwrap();
        // // let g_type_int = g_type_from_name(a.as_ptr());
        // // println!("type:{:?}", g_type_int);
        // g_value_init(value_ptr, g_type_int as _);
        // println!("init");
        // g_value_set_int(value_ptr, 1 as std::os::raw::c_int);
        // println!("set_int");
        // let x = g_value_get_int(value_ptr);
        // println!("get_int:{:?}", x);
        // g_value_unset(value_ptr);
        // println!("done");
        // println!("type:{:?}", g_type_int);
        ops.call("smartcrop");
        let img = crate::VipsImage::from_raw(out_out);
        println!(
            "img:{:?}",
            img.get_width()
        );
        println!(
            "img:{:?}",
            img.get_height()
        );
        println!(
            "img:{:?}",
            attention_x
        );
        println!(
            "img:{:?}",
            attention_y
        );
    }
}
 */
