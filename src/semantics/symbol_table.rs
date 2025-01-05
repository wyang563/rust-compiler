use std::collections::HashMap;

pub enum GlobalEntry {
    MethodEntry(MethodEntry),
    ImportEntry(ImportEntry),
    VarEntry(VarEntry),
    ArrayEntry(ArrayEntry),
}

pub enum LocalEntry {
    VarEntry(VarEntry),
    ArrayEntry(ArrayEntry),
}

enum Type {
    Void,
    Int,
    Bool,
    IntArray,
    BoolArray,
}

pub struct VarEntry {
    pub name: String,
    pub var_type: Type,
}

pub struct ArrayEntry {
    pub name: String,
    pub var_type: Type,
    pub var_length: u32,
    pub var_elements: Vec<MethodEntry>,
}

pub struct MethodEntry {
    pub name: String,
    pub return_type: Type,
    pub scope: MethodTable,
    pub param_list: Vec<VarEntry>,
    pub param_count: u32,
}

pub struct ImportEntry {
    pub name: String,
    pub return_type: Type, // int by default
}

pub struct GlobalTable {
    pub entries: HashMap<String, GlobalEntry>
}

pub struct MethodTable {
    pub parent: Option<Box<MethodTable>>,
    pub entries: HashMap<String, LocalEntry>,
}
