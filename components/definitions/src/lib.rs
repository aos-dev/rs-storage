use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;
use aos_specs::{PARSED_INFOS, PARSED_PAIRS};
use codegen::{Field, Scope};
use heck::{CamelCase, SnakeCase};
use lazy_static::lazy_static;
use std::fs::OpenOptions;
use std::io::Write;

lazy_static! {
    static ref TYPES_MAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        // Builtin types
        m.insert("string", "String");
        m.insert("int", "isize");
        m.insert("int64","i64");
        m.insert("bool", "bool");

        // Compose types
        m.insert("string_array", "&[String]");
        m.insert("string_string_map", "HashMap<String,String>");
        m.insert("time","chrono::DateTime<chrono::Utc>");
        m.insert("BlockIterator", "BlockIterator");
        m.insert("ListMode", "ListMode");
        m.insert("Interceptor", "Interceptor");
        m.insert("IoCallback", "IoCallback");
        m.insert("Object", "Object");
        m.insert("ObjectMode", "ObjectMode");
        m.insert("ObjectIterator", "ObjectIterator");
        m.insert("Pairs", "&[Pair]");
        m.insert("Parts", "&[Part]");
        m.insert("PartIterator", "PartIterator");
        m.insert("PairPolicy", "PairPolicy");
        m.insert("Reader", "&mut impl io::Read");
        m.insert("Storager", "Storager");
        m.insert("StoragerIterator", "StoragerIterator");
        m.insert("StorageMeta", "StorageMeta");
        m.insert("Writer", "&mut impl io::Write");

        m
    };
}

#[derive(Default)]
pub struct Builder {}

impl Builder {
    pub fn write_pairs(&self, path: &PathBuf) -> Result<()> {
        let p = self.generate_pairs();
        let mut f = OpenOptions::new().truncate(true).write(true).open(path)?;

        f.write_all(p.as_bytes())?;

        Ok(())
    }
    fn generate_pairs(&self) -> String {
        let mut scope = Scope::new();

        scope.import("super::super", "*");

        // Build enums
        let mut e = scope.new_enum("Pair").vis("pub");

        for v in PARSED_PAIRS.iter() {
            let mut variant = e.new_variant(&v.name.to_camel_case());
            variant.tuple(TYPES_MAP.get(&v.ty.as_str()).unwrap());
        }

        scope.to_string()
    }

    pub fn write_objects(&self, path: &PathBuf) -> Result<()> {
        let p = self.generate_object();
        let mut f = OpenOptions::new().truncate(true).write(true).open(path)?;

        f.write_all(p.as_bytes())?;

        Ok(())
    }
    fn generate_object(&self) -> String {
        let mut scope = Scope::new();

        scope.import("std::collections", "HashMap");
        scope.import("super::super", "*");

        let mut s = scope
            .new_struct("Object")
            .vis("pub")
            .derive("Debug")
            .derive("Clone")
            .derive("Default");

        for v in PARSED_INFOS.iter() {
            if v.scope != "object" || v.category != "meta" {
                continue;
            }

            let mut name = v.name.to_snake_case();
            if v.export {
                name = "pub ".to_string();
                name.push_str(&v.name.to_snake_case());
            }

            assert!(
                TYPES_MAP.contains_key(v.ty.as_str()),
                "type {} not found",
                v.ty.as_str()
            );

            let mut type_name = TYPES_MAP.get(v.ty.as_str()).unwrap().to_string();
            // Field could be Option if it's not export or a map.
            if !v.export && !type_name.starts_with("HashMap") {
                type_name = format!("Option<{}>", type_name);
            }

            let mut f = Field::new(&name, type_name);

            if v.description != "" {
                f.doc(vec![&v.description]);
            }

            s.push_field(f);
        }

        scope.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_pairs() {
        let s = generate_pairs();

        println!("{}", s)
    }

    #[test]
    fn test_generate_object() {
        let s = generate_object();

        println!("{}", s)
    }
}
