import json

def convert_type_name(name: str) -> str:
    typ = ""    
    match name:
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
    return typ

def gen_types(types: dict) -> str:
    contents = []
    for (name, type) in types.items():
        contents.append(f"enum {name} {"{\n"}")
        for variant in type["enums"]:
            contents.append(f"{variant["key"]}, //{variant["description"]}\n")
        contents.append("}\n")
    return "".join(contents)

def gen_method_enum(method: dict) -> str:
    contents = ["enum ", ]
    contents.append(method["command"]) # Add name
    contents.append(" {")
    
    contents.append("\ninParams")
    contents.append(" {\n")
    for param in method["inParams"]:
        if param["name"] == "loop":
            name = "selectedloop"
        else:
            name = param["name"]
        contents.append(f"{name}: {convert_type_name(param["type"])},\n")
    contents.append("},")
    
    contents.append("\noutParams")
    contents.append(" {\n")
    for param in method["outParams"]:
        contents.append(f"{param["name"]}: {convert_type_name(param["type"])},\n")
    contents.append("},")

    contents.append("\n}")
    return "".join(contents)

def gen_method_enums(methods: dict) -> str:
    contents = []
    for (family, commands) in methods.items():
        contents.append(f"mod {family} {"{"}\n")
        for (_, command) in commands.items():
            contents.append(gen_method_enum(command))
        contents.append("\n}\n")
    return "".join(contents)

def test(data: dict):
    print(gen_method_enums(data["Methods"]))    

def main(data):
    contents = []
    contents.append("mod Types {\n")
    contents.append(gen_types(data["Types"]))
    contents.append("}\n")

    contents.append("mod Commands {\n")
    contents.append(gen_method_enums(data["Methods"]))
    contents.append("}")
    with open("./gen_types.rs", "w") as file:
        file.write("".join(contents))

if __name__ == "__main__":
    with open("./hrp_commands.json") as json_data:
        data = json.loads(json_data.read())
    main(data)
    test(data)
