pub use super::indexs::{
    ClassIndex, FieldIndex, InterfaceMethodIndex, MethodIndex, MethodOrInterfaceMethodIndex,
    NameAndTypeIndex, StringIndex,
};

#[derive(Debug, PartialEq)]
pub enum Constant {
    String(String),
    I32(i32),
    F32(f32),
    I64(i64),
    F64(f64),
    Class(ClassConstant),
    StringRef(StringRefConstant),
    Field(FieldConstant),
    Method(MethodConstant),
    InterfaceMethod(InterfaceMethodConstant),
    NameAndType(NameAndTypeConstant),
    MethodHandle(MethodHandleConstant),
    MethodType(MethodTypeConstant),
    InvokeDynamic(InvokeDynamicConstant),
}

#[derive(Debug, PartialEq)]
pub struct ClassConstant {
    name: StringIndex,
}

#[derive(Debug, PartialEq)]
pub struct NameAndTypeConstant {
    name: StringIndex,
    descriptor: StringIndex,
}

#[derive(Debug, PartialEq)]
pub struct FieldConstant {
    class: ClassIndex,
    name_and_type: NameAndTypeIndex,
}

#[derive(Debug, PartialEq)]
pub struct MethodConstant {
    class: ClassIndex,
    name_and_type: NameAndTypeIndex,
}

#[derive(Debug, PartialEq)]
pub struct InterfaceMethodConstant {
    class: ClassIndex,
    name_and_type: NameAndTypeIndex,
}

#[derive(Debug, PartialEq)]
pub enum MethodHandleConstant {
    GetField(FieldIndex),
    GetStatic(FieldIndex),
    PutField(FieldIndex),
    PutStatic(FieldIndex),
    InvokeVirtual(MethodIndex),
    NewInvokeSpecial(MethodIndex),
    InvokeSpecial(MethodOrInterfaceMethodIndex),
    InvokeStatic(MethodOrInterfaceMethodIndex),
    InvokeInterface(InterfaceMethodIndex),
}

#[derive(Debug, PartialEq)]
pub struct InvokeDynamicConstant {
    bootstrap_method_attr: u16,
    name_and_type: NameAndTypeIndex,
}

#[derive(Debug, PartialEq)]
pub struct MethodTypeConstant {
    descriptor: StringIndex,
}

#[derive(Debug, PartialEq)]
pub struct StringRefConstant {
    string: StringIndex,
}

pub struct ConstantPool(Vec<Constant>);

impl std::ops::Index<StringIndex> for ConstantPool {
    type Output = str;

    fn index(&self, index: StringIndex) -> &Self::Output {
        match self.0.get(*index as usize) {
            Some(Constant::String(s)) => s,
            _ => panic!("constant at index {} is not a String", *index),
        }
    }
}

impl std::ops::Index<ClassIndex> for ConstantPool {
    type Output = ClassConstant;

    fn index(&self, index: ClassIndex) -> &Self::Output {
        match self.0.get(*index as usize) {
            Some(Constant::Class(class)) => class,
            _ => panic!("constant at index {} is not a ClassConstant", *index),
        }
    }
}

impl std::ops::Index<FieldIndex> for ConstantPool {
    type Output = FieldConstant;

    fn index(&self, index: FieldIndex) -> &Self::Output {
        match self.0.get(*index as usize) {
            Some(Constant::Field(field)) => field,
            _ => panic!("constant at index {} is not a FieldConstant", *index),
        }
    }
}

impl std::ops::Index<NameAndTypeIndex> for ConstantPool {
    type Output = NameAndTypeConstant;

    fn index(&self, index: NameAndTypeIndex) -> &Self::Output {
        match self.0.get(*index as usize) {
            Some(Constant::NameAndType(named_and_typed)) => named_and_typed,
            _ => panic!("constant at index {} is not a NameAndType", *index),
        }
    }
}

impl std::ops::Index<MethodIndex> for ConstantPool {
    type Output = MethodConstant;

    fn index(&self, index: MethodIndex) -> &Self::Output {
        match self.0.get(*index as usize) {
            Some(Constant::Method(method)) => method,
            _ => panic!("constant at index {} is not a NameAndType", *index),
        }
    }
}

impl std::ops::Index<MethodOrInterfaceMethodIndex> for ConstantPool {
    type Output = Constant;

    fn index(&self, index: MethodOrInterfaceMethodIndex) -> &Self::Output {
        match self.0.get(*index as usize) {
            Some(c @ Constant::Method(_)) => c,
            Some(c @ Constant::InterfaceMethod(_)) => c,
            _ => panic!("constant at index {} is not a MethodOrInterface", *index),
        }
    }
}

impl std::ops::Index<InterfaceMethodIndex> for ConstantPool {
    type Output = InterfaceMethodConstant;

    fn index(&self, index: InterfaceMethodIndex) -> &Self::Output {
        match self.0.get(*index as usize) {
            Some(Constant::InterfaceMethod(method)) => method,
            _ => panic!("constant at index {} is not a InterfaceMethod", *index),
        }
    }
}
