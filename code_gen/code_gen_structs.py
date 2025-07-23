import json
import subprocess

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
            typ += "Types::" + t
    return typ

def gen_types(types: dict) -> str:
    contents = []
    for (name, type) in types.items():
        contents.append("#[derive(Clone, Copy, Debug)]\n")
        contents.append(f"pub enum {name} {"{\n"}")
        for variant in type["enums"]:
            contents.append(f"{variant["key"]}, //{variant["description"]}\n")
        contents.append("}\n")

        contents.append(f"impl {name} {"{"} \n")
        contents.append("fn u8_to_variant(value: u8) -> Result<Self> {\n")
        contents.append("match value {\n")
        for variant in type["enums"]:
            contents.append(f"{variant["value"]} => Ok(Self::{variant["key"]}),")
        contents.append(f"v => Err(Error::DoesNotCorespondToVariant(format!(\"Value {"{v}"} does not corespond to a variant in {name}\"))),")
        contents.append("}}")

        contents.append("fn to_u8(&self) -> u8 {\n")
        contents.append("match *self {\n")
        for variant in type["enums"]:
            contents.append(f"Self::{variant["key"]} => {variant["value"]},")
        contents.append("}}}\n")

        contents.append(f"impl Serialize for {name}" + "{")
        contents.append("fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: Serializer, {")
        contents.append("serializer.serialize_u8(self.to_u8())")
        contents.append("}}")

        visitor = f"""
struct {name}Visitor;

impl<'de> Visitor<'de> for {name}Visitor {"{"}
    type Value = {name};

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {"{"}
        formatter.write_str("an unsigned integer coresponding to a enum variant")
    {"}"}

    fn visit_u8<E>(self, value: u8) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {"{"}
        match Self::Value::u8_to_variant(value) {"{"}
            Ok(v) => std::result::Result::Ok(v),
            Err(e) => std::result::Result::Err(de::Error::custom(format!("{"{"}value{"}"} does not corespond to an enum variant"))),
        {"}"}
    {"}"}
{"}"}

impl<'de> Deserialize<'de> for {name} {"{"}
    fn deserialize<D>(deserializer: D) -> std::result::Result<{name}, D::Error>
    where
        D: Deserializer<'de>,
    {"{"}
        deserializer.deserialize_u8({name}Visitor)
    {"}"}
{"}"}
        """
        contents.append(visitor)
        
    return "".join(contents)

def clean_name(name: str) -> str:
    out = ""
    if name == "loop":
        out = "selectedloop"
    else:
        out = name
    return out

def gen_method_enum(method: dict) -> str:
    contents = []
    contents.append("#[derive(Clone, Copy, Debug, Serialize, Deserialize)]\n")
    contents.append("#[serde(untagged)]")
    contents.append("pub enum ")
    contents.append(method["command"]) # Add name
    contents.append(" {")

    contents.append("\noutParams")
    contents.append(" {\n")
    for param in method["outParams"]:
        contents.append(f"{param["name"]}: {convert_type_name(param["type"])},\n")
    contents.append("},")

    contents.append("\ninParams")
    contents.append(" {\n")
    for param in method["inParams"]:
        contents.append(f"{clean_name(param["name"])}: {convert_type_name(param["type"])},\n")
    contents.append("},")

    contents.append("\n}")

    contents.append(f"impl {method["command"]}" + "{")
    contents.append("fn new(")
    contents.append(", ".join([f"{clean_name(a["name"])}: {convert_type_name(a["type"])}" for a in method["inParams"]]) + ")")
    contents.append(" -> Self {")
    contents.append(f"Self::inParams{"{"}{", ".join([clean_name(a["name"]) for a in method["inParams"]])}{"}"}")
    contents.append("}")
    contents.append("}")

    contents.append(f"impl Hcp for {method["command"]} {"{\n"}")
    contents.append("fn get_msgtype_subcmd() -> Msgtype {\n")
    contents.append(f"Msgtype::new({method["protocol"][0]["value"]}, {method["protocol"][1]["value"]}) {"}"}\n")

    #contents.append("fn get_outparams() -> Self {")
    #contents.append("Self::outParams{}")
    #contents.append("}")

    contents.append("}")
    return "".join(contents)

def gen_method_enums(methods: dict) -> str:
    contents = []
    for (family, commands) in methods.items():
        contents.append(f"pub mod {family} {"{"}\n use super::super::{"{Hcp, Msgtype, Types, Serialize, Deserialize}"};")
        for (_, command) in commands.items():
            contents.append(gen_method_enum(command))
        contents.append("\n}\n")
    return "".join(contents)

def test(data: dict):
    print(gen_method_enums(data["Methods"]))

def main(data):
    contents = []
    contents.append("""#![allow(non_camel_case_types)] // I should proably change the python code
#![allow(non_snake_case)]
#![allow(unused)] // for now
use std::fmt;
use serde::{Serialize,Deserialize};
use serde::ser::Serializer;
use serde::de::{self, Visitor, Deserializer};

use crate::error::{Error, Result};
use crate::type_methods::{Msgtype, Hcp, HcpType};

""")
    contents.append("pub mod Types {\n use super::*;")
    contents.append(gen_types(data["Types"]))
    contents.append("}\n")

    contents.append("pub mod Commands {\n")
    contents.append(gen_method_enums(data["Methods"]))
    contents.append("}")
    with open("./gen_types.rs", "w") as file:
        file.write("".join(contents))

    subprocess.run(["rustfmt", "./gen_types.rs"])

if __name__ == "__main__":
    with open("./hrp_commands.json") as json_data:
        data = json.loads(json_data.read())
    main(data)
    #test(data)
