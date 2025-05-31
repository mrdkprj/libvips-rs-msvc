import os
import re

processed_files = set()
function_pattern = re.compile(r"^\s*(VIPS_API\s+)?([a-zA-Z_][a-zA-Z0-9_*\s]*?)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\((.*?)\)\s*;")
deprecated_functions = ["vips_path_filename7","vips_path_mode7","vips_object_set_required","vips_region_dump_all","vips_format_get_type","vips_format_map","vips_format_for_file","vips_format_for_name","vips_format_read","vips_format_write","vips_operation_call_valist"]
temp_names = ["a","b","c","d","e","f","g"]

def collect_headers(filepath, include_paths):
    headers = []
    queue = [filepath]

    while queue:
        path = queue.pop()
        if path in processed_files or not os.path.exists(path):
            continue
        processed_files.add(path)

        with open(path, "r", encoding="utf-8", errors="ignore") as f:
            for line in f:
                if line.strip().startswith("#include <vips/"):
                    header = line.strip().split("<vips/")[1].split(">")[0]
                    for inc in include_paths:
                        full = os.path.join(inc, header)
                        if os.path.exists(full):
                            queue.append(full)
                            break
        headers.append(path)
    return headers

def parse_arguments(args_str):
    bridge_args = []
    call_args = []

    args_str = args_str.strip()

    for i, arg in enumerate(args_str.split(",")):
        arg = arg.strip()
        if not arg:
            continue

        if arg == "void":
            bridge_args.append("void")
            continue

        parts = arg.split()
        name = parts[-1]
        typ = " ".join(parts[:-1])
        if not typ:
            typ = name
            name = temp_names[i]

        if name == "*":
            name = f"* {temp_names[i]}"

        bridge_args.append(f"{typ} {name}")

        if "[]" in name:
            name = name.replace("[]","")

        if "*" in name:
            name = name.replace("*", "")

        name = name.strip()
        call_args.append(name)

    return ", ".join(bridge_args), ", ".join(call_args)

def parse_functions(header_path):
    with open(header_path, "r", encoding="utf-8", errors="ignore") as f:
        lines = f.readlines()

    bridges = []
    defs = []
    i = 0
    while i < len(lines):
        line = lines[i].strip()

        if line == "VIPS_API":
            # Look ahead for the actual function declaration
            j = i + 1
            while j < len(lines) and lines[j].strip() == "":
                j += 1

            if j < len(lines):
                func_line = lines[j].strip()
                # Match a function like: return_type name(args...);
                match = re.match( r"^\s*([a-zA-Z_][\w\s\*\d]*\*?)\s+(\*?[a-zA-Z_]\w*)\s*\(([^)]*)\)\s*;", func_line)
                if match:
                    return_type = match.group(1).strip()
                    function_name = match.group(2).strip()
                    bridge_function_name = function_name

                    not_void = return_type == "void" and function_name.startswith("*")
                    if function_name.startswith("*"):
                        function_name = function_name.replace("*", "")

                    if not function_name in deprecated_functions:
                        args_str = match.group(3).strip()

                        if not "..." in args_str:
                            bridge_args, call_args = parse_arguments(args_str)

                            body = f"return {function_name}({call_args});" if return_type != "void" or not_void else f"{function_name}({call_args});"
                            bridge = f"""__declspec(dllexport) {return_type} {bridge_function_name}_bridge({bridge_args}) {{{body}}}"""
                            bridges.append(bridge)
                            defs.append(f"{function_name},{function_name}_bridge")
            i = j
        i += 1

    return (bridges, defs)

def generate_bridge_code(vips_h_path, vips_include_path):
    all_headers = collect_headers(vips_h_path, [vips_include_path])
    bridges = []
    defs = []

    for header in all_headers:
        print(f"🔍 Parsing: {header}")
        (bridge_funcs, def_lines) = parse_functions(header)
        bridges.extend(bridge_funcs)
        defs.extend(def_lines)

    return ("\n".join(bridges), defs)

def write_def_file(def_file, functions):
    with open(def_file, "w", encoding="utf-8") as f:
        f.write("LIBRARY vips_bridge\nEXPORTS\n")
        for _, orig, bridge, _ in functions:
            f.write(f"    {orig} = {bridge}\n")

if __name__ == "__main__":
    vips_cache_fixed_bridge = '\n__declspec(dllexport) int vips_cache_fixed_bridge(VipsImage *in, VipsImage **out, const char* max_tiles_in_name, int max_tiles_in, const char* tile_height_in_name, int tile_height_in, const char* tile_width_in_name, int tile_width_in) {return vips_call_split("cache", max_tiles_in_name, max_tiles_in, tile_height_in_name, tile_height_in, tile_width_in_name, tile_width_in, NULL, in, out);}'
    vips_cache_bridge = '\n__declspec(dllexport) int vips_cache_bridge(VipsImage *in, VipsImage **out) {return vips_call_split("cache", NULL, in, out);}'
    missing_def = ["vips_cache", "vips_cache_bridge"]
    vips_include_dir = os.path.join(os.getcwd(), "include", "vips")
    vips_h_path = os.path.join(vips_include_dir, "vips.h")

    (bridge_code, def_codes) = generate_bridge_code(vips_h_path, vips_include_dir)

    with open("vips.c", "w", encoding="utf-8") as f:
        f.write("#include <vips/vips.h>\n\n")
        f.write(bridge_code)
        f.write(vips_cache_fixed_bridge)
        f.write(vips_cache_bridge)

    with open("vips.def", "w", encoding="utf-8") as f:
        f.write("LIBRARY vips\nEXPORTS\n")
        for def_code in def_codes:
            def_code = def_code.split(",")
            orig = def_code[0]
            bridge = def_code[1]
            f.write(f"    {orig} = {bridge}\n")
            f.write("    vips_cache = vips_cache_bridge\n")
            f.write("    vips_cache_fixed = vips_cache_fixed_bridge\n")

    print("✅ Bridge file generated: vips.c")
