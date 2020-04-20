mod constant_pool;
mod attributes;
mod indexs;

use constant_pool::*;
use crate::{MajorVersion, AccessFlags};
use attributes::Attribute;

#[derive(Debug, PartialEq)]
pub struct Class {
    magic_number: u32,
    minor_version: u16,
    major_version: MajorVersion,
    constant_pool: ConstantPool,
    access_flags: AccessFlags,
    this_class: ClassIndex,
    super_class: ClassIndex,
    interfaces: Vec<ClassIndex>,
    fields_table: Vec<Field>,
    methods_table: Vec<Method>,
    attributes: Vec<Attribute>
}

#[derive(Debug, PartialEq)]
pub struct Field {
    access_flags: AccessFlags,
    name_index: StringIndex,
    descriptor_index: StringIndex,
    attributes: Vec<Attribute>,
}

#[derive(Debug, PartialEq)]
pub struct Method {
    access_flags: AccessFlags,
    name_index: StringIndex,
    descriptor_index: StringIndex,
    attributes: Vec<Attribute>,
}