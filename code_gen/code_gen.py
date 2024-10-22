import json

def types_to_enums(types):
    code = ""
    for type in types:
        code += "#[derive(Clone, Copy, Serialize, Deserialize)]\n"
        code += "pub enum " + type["name"] + " {"
        for e in type["enums"]:
            parts = ""
            for key in e["key"].split("_"):
                parts += key.lower().capitalize()

            code += parts + " = " + e["value"] + "," + " /* " + e["description"] + " */\n"

        code += "}"
    return code

def extract_types(params):
    types = ""
    for p in params:
        match p["name"]:
            case "loop":
                name = "selectedloop"
            case n:
                name = n
        typ = name + ":"
        match p["type"]:
            case "bool":
                typ += "bool"
            case "uint8":
                typ += "u8"
            case "sint8":
                typ += "i8"
            case "uint16":
                typ += "u16"
            case "sint16":
                typ += "i16"
            case "uint32":
                typ += "u32"
            case "sint32":
                typ += "i32"
            case "uint64":
                typ += "u64"
            case "sint64":
                typ += "i64"
            case t:
                typ += t

        types += typ + ", "

    return types[0:-2]

def extract_names(params):
    names = " /* "

    for p in params:
        names += p["name"] + ", "

    return names + " */\n"


def methods_to_structs(methods, params):
    code = "#[derive(Clone, Copy, Serialize, Deserialize)]\n"
    code += " pub enum " + params + " { \n"
    for m in methods:
        code += m["family"] + m["command"] + "{" + extract_types(m[params]) + "},"
        # code += extract_names(m[params])
    code += "}"
    return code

def gen_match_statement(methods, params):
    match = "pub fn get_msgtype(param: inParams) -> (u16, u8) {"
    match += "match param { \n"
    for m in methods:
        match += params + "::" + m["family"] + m["command"]
        if len(m[params]) != 0:
            match += "{..}"
        else:
            match += "{}"
        match += " => (" + m["protocol"][0]["value"] + ", " + m["protocol"][1]["value"] + "),\n"

    return match + "}}"

if __name__ == "__main__":
    with open("code_gen/automower_hrp.json", "r") as contents:
        json_data = json.load(contents)

    out = """#![allow(non_camel_case_types)] // I should proably change the python code
    #![allow(non_snake_case)]
    #![allow(unused)] // for now
    pub mod types {
    use serde::{Serialize,Deserialize};
    """
    ###### Enums ######
    types = json_data["types"]

    enums = types_to_enums(types)

    out += enums
    ##### Methods #####
    methods = json_data["methods"]

    out += methods_to_structs(methods, "inParams") + "\n"
    out += methods_to_structs(methods, "outParams") + "\n"
    out += gen_match_statement(methods, "inParams") + "\n"
    #out += gen_match_statement(methods, "outParams")

    out += "\n}"

    with open("code_gen/gen_types.rs", "w") as write_file:
        write_file.write(out)
