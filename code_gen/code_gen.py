import json

def types_to_enums(types):
    code = ""
    for type in types:
        code += "enum " + type["name"] + " {"
        for e in type["enums"]:
            parts = ""
            for key in e["key"].split("_"):
                parts += key.lower().capitalize();

            code += parts + " = " + e["value"] + "," + " /* " + e["description"] + " */\n"

        code += "}"
    return code

def extract_types(params):
    types = ""
    for p in params:
        typ = ""
        match p["type"]:
            case "bool":
                typ = "bool"
            case "uint8":
                typ = "u8"
            case "sint8":
                typ = "i8"
            case "uint16":
                typ = "u16"
            case "sint16":
                typ = "i16"
            case "uint32":
                typ = "u32"
            case "sint32":
                typ = "i32"
            case "uint64":
                typ = "u64"
            case "sint64":
                typ = "i64"
            case t:
                typ = t

        types += typ + ", "

    return types[0:-2]

def extract_names(params):
    names = " /* "

    for p in params:
        names += p["name"] + ", "

    return names + " */\n"


def methods_to_structs(methods, params):
    code = "enum " + params + " { \n"
    for m in methods:
        code += m["family"] + m["command"] + "(" + extract_types(m[params]) + "),"
        code += extract_names(m[params])
    code += "}"
    return code

def gen_match_statement(methods, params):
    match = "match params { \n"
    for m in methods:
        match += params + "::" + m["family"] + m["command"]
        if len(m[params]) != 0:
            match += "(..)"
        else:
            match += "()"
        match += " => (" + m["protocol"][0]["value"] + ", " + m["protocol"][1]["value"] + "),\n"

    return match + "}"

if __name__ == "__main__":
    with open("code_gen/automower_hrp.json", "r") as contents:
        json_data = json.load(contents)

    ###### Enums ######
    types = json_data["types"]

    enums = types_to_enums(types)

    #with open("enums.rs", "w") as write_file:
    #    write_file.write(enums)

    ##### Methods #####
    methods = json_data["methods"]

    out = methods_to_structs(methods, "inParams") + "\n"
    out += methods_to_structs(methods, "outParams") + "\n"
    out += gen_match_statement(methods, "inParams") + "\n"
    out += gen_match_statement(methods, "outParams")

    with open("methods.rs", "w") as write_file:
        write_file.write(out)
