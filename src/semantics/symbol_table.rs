use std::collections::HashMap;

#[derive(Clone)]
pub enum Entry {
    Var(VarEntry),
    Array(ArrayEntry),
    Method(MethodEntry),
    Import(ImportEntry),
}

impl Entry {
    pub fn get_is_const(&self) -> bool {
        match self {
            Entry::Var(v) => v.is_const,
            Entry::Array(a) => a.is_const,
            Entry::Method(m) => m.is_const,
            Entry::Import(i) => i.is_const,
        }
    }
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Type {
    Void,
    Int,
    Bool,
    IntArray,
    BoolArray,
    None, // default value for error propogation
}

#[derive(Clone)]
pub struct VarEntry {
    pub name: String,
    pub var_type: Type,
    pub is_const: bool,
}

#[derive(Clone)]
pub struct ArrayEntry {
    pub name: String,
    pub var_type: Type,
    pub is_const: bool,
}

#[derive(Clone)]
pub struct MethodEntry {
    pub name: String,
    pub return_type: Type,
    pub is_const: bool,
    pub param_list: Vec<VarEntry>,
    pub param_count: u32,
}

#[derive(Clone)]
pub struct ImportEntry {
    pub name: String,
    pub is_const: bool,
    pub return_type: Type, // int by default
}

#[derive(Clone)]
pub struct GlobalTable {
    pub entries: HashMap<String, Entry>
}

#[derive(Clone)]
pub struct MethodTable {
    pub parent: Option<Box<MethodTable>>,
    pub method_return_type: Type,
    pub entries: HashMap<String, Entry>,
}
