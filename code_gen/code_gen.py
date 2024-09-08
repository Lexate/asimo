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

def methods_to_structs(methods):
    code = ""


if __name__ == "__main__":
    with open("code_gen/automower_hrp.json", "r") as contents:
        json_data = json.load(contents)

    ###### Enums ######
    types = json_data["types"]

    enums = types_to_enums(types)

    with open("enums.rs", "w") as write_file:
        write_file.write(enums)

    ##### Methods #####
    methods = json_data["methods"]

    methods_to_structs(methods)


