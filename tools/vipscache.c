#include <vips/vips.h>

__declspec(dllexport) int vips_cache_fixed_bridge(VipsImage *in, VipsImage **out, const char* max_tiles_in_name, int max_tiles_in, const char* tile_height_in_name, int tile_height_in, const char* tile_width_in_name, int tile_width_in) {return vips_call_split("cache", max_tiles_in_name, max_tiles_in, tile_height_in_name, tile_height_in, tile_width_in_name, tile_width_in, NULL, in, out);}
__declspec(dllexport) int vips_cache_bridge(VipsImage *in, VipsImage **out) {return vips_call_split("cache", NULL, in, out);}