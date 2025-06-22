#include <vips/vips.h>

__declspec(dllexport)
void vo_set_bool(GValue *out_value, gboolean value){
    g_value_init(out_value, G_TYPE_BOOLEAN);
    g_value_set_boolean(out_value, value);
}

// input int ... this path is used for enums as well
__declspec(dllexport)
void vo_set_int(GValue *out_value, int value) {
    g_value_init(out_value, G_TYPE_INT);
    g_value_set_int(out_value, value);
}

// input guint64
__declspec(dllexport)
void vo_set_guint64(GValue *out_value, guint64 value) {
    g_value_init(out_value, G_TYPE_UINT64);
    g_value_set_uint64(out_value, value);
}

// input double
__declspec(dllexport)
void vo_set_double(GValue *out_value, double value) {
    g_value_init(out_value, G_TYPE_DOUBLE);
    g_value_set_double(out_value, value);
}

__declspec(dllexport)
void vo_set_string(GValue *out_value, const gchar *value) {
    g_value_init(out_value, G_TYPE_STRING);
    g_value_set_string(out_value, value);
}

// input int array
__declspec(dllexport)
void vo_set_int_array(GValue *out_value, const int *value, int size) {
    g_value_init(out_value, VIPS_TYPE_ARRAY_INT);
	vips_value_set_array_int(out_value, value, size);
}

// input double array
__declspec(dllexport)
void vo_set_double_array(GValue *out_value, const double *value, int size) {
    g_value_init(out_value, VIPS_TYPE_ARRAY_DOUBLE);
	vips_value_set_array_double(out_value, value, size);
}

// input image array
__declspec(dllexport)
void vo_set_images(GValue *out_value, VipsImage **value, int size){
	g_value_init(out_value, VIPS_TYPE_ARRAY_IMAGE);
	vips_value_set_array_image(out_value, size);
	VipsImage **array = vips_value_get_array_image(out_value, NULL);

	for (int i = 0; i < size; i++) {
		array[i] = value[i];
        g_object_ref(value[i]);
    }
}

// input blob
__declspec(dllexport)
void vo_set_blob(GValue *out_value, VipsBlob *value) {
    g_value_init(out_value, VIPS_TYPE_BLOB);
    g_value_set_boxed(out_value, value);
}

// input vips object (image, source, target, etc. etc.)
void vo_set_object(GValue *out_value, VipsObject *object) {
	GType type = G_OBJECT_TYPE(object);
    g_value_init(out_value, type);
    g_value_set_object(out_value, object);
}

__declspec(dllexport)
void vo_set_image(GValue *out_value, VipsImage *value) {
	vo_set_object(out_value, VIPS_OBJECT(value));
}

__declspec(dllexport)
void vo_set_target(GValue *out_value, VipsTarget *value) {
	vo_set_object(out_value, VIPS_OBJECT(value));
}

__declspec(dllexport)
void vo_set_source(GValue *out_value, VipsSource *value) {
	vo_set_object(out_value, VIPS_OBJECT(value));
}

__declspec(dllexport)
void vo_get_bool(VipsOperation *operation, const gchar *name, gboolean *out) {
	GValue value = G_VALUE_INIT;
    g_value_init(&value, G_TYPE_BOOLEAN);
    g_object_get_property(G_OBJECT(operation), name, &value);
    *out = g_value_get_boolean(&value);
}

__declspec(dllexport)
void vo_get_int(VipsOperation *operation, const gchar *name, int *out) {
	GValue value = G_VALUE_INIT;
    g_value_init(&value, G_TYPE_INT);
    g_object_get_property(G_OBJECT(operation), name, &value);
    *out = g_value_get_int(&value);
}

__declspec(dllexport)
void vo_get_double(VipsOperation *operation, const gchar *name, double *out) {
	GValue value = G_VALUE_INIT;
    g_value_init(&value, G_TYPE_DOUBLE);
    g_object_get_property(G_OBJECT(operation), name, &value);
    *out = g_value_get_double(&value);
}

__declspec(dllexport)
void vo_get_double_array(VipsOperation *operation, const gchar *name, double *out, int size) {
	GValue value = G_VALUE_INIT;
    g_value_init(&value, VIPS_TYPE_ARRAY_DOUBLE);
    g_object_get_property(G_OBJECT(operation), name, &value);
    int length;
    double *array = vips_value_get_array_double(&value, &length);
    for (int i = 0; i < size; i++) {
		out[i] = array[i];
    }
}

__declspec(dllexport)
void vo_get_blob(VipsOperation *operation, const gchar *name, VipsBlob **out) {
	GValue value = G_VALUE_INIT;
    g_value_init(&value, VIPS_TYPE_BLOB);
    g_object_get_property(G_OBJECT(operation), name, &value);
    *out = (VipsBlob *) g_value_dup_boxed(&value);
}

__declspec(dllexport)
void vo_get_image(VipsOperation *operation, const gchar *name, VipsImage **out) {
	GValue value = G_VALUE_INIT;
    g_value_init(&value, VIPS_TYPE_IMAGE);
    g_object_get_property(G_OBJECT(operation), name, &value);
    *out = VIPS_IMAGE(g_value_get_object(&value));
}

__declspec(dllexport)
void vo_set_property(VipsObject *object, const gchar *name, const GValue *value)
{
	VipsObjectClass *object_class = VIPS_OBJECT_GET_CLASS(object);
	GType type = G_VALUE_TYPE(value);

	GParamSpec *pspec;
	VipsArgumentClass *argument_class;
	VipsArgumentInstance *argument_instance;

	if (vips_object_get_argument(object, name,
			&pspec, &argument_class, &argument_instance)) {
		g_warning("%s", vips_error_buffer());
		vips_error_clear();
		return;
	}

	if (G_IS_PARAM_SPEC_ENUM(pspec) &&
		type == G_TYPE_STRING) {
		GType pspec_type = G_PARAM_SPEC_VALUE_TYPE(pspec);

		int enum_value;
		GValue value2 = G_VALUE_INIT;

		if ((enum_value = vips_enum_from_nick(object_class->nickname,
				 pspec_type, g_value_get_string(value))) < 0) {
			g_warning("%s", vips_error_buffer());
			vips_error_clear();
			return;
		}

		g_value_init(&value2, pspec_type);
		g_value_set_enum(&value2, enum_value);
		g_object_set_property(G_OBJECT(object), name, &value2);
		g_value_unset(&value2);
	}
	else
		g_object_set_property(G_OBJECT(object), name, value);
}