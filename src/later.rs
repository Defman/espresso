#[derive(Debug)]
pub struct ClassConstant {
    name: String,
}

#[derive(Debug)]
pub struct DescriptorConstant(String);

#[derive(Debug)]
pub struct NameAndTypeConstant {
    name: String,
    descriptor: DescriptorConstant,
}

#[derive(Debug)]
pub struct FieldConstant {
    class: ClassConstant,
    name_and_type: NameAndTypeConstant,
}

#[derive(Debug)]
pub struct MethodConstant {
    class: ClassConstant,
    name_and_type: NameAndTypeConstant,
}

#[derive(Debug)]
pub struct InterfaceMethodConstant {
    class: ClassConstant,
    name_and_type: NameAndTypeConstant,
}

#[derive(Debug)]
pub enum MethodHandleConstant {
    GetField(FieldConstant),
    GetStatic(FieldConstant),
    PutField(FieldConstant),
    PutStatic(FieldConstant),
    InvokeVirtual(MethodConstant),
    NewInvokeSpecial(MethodConstant),
    InvokeSpecial(MethodHandleInvokeSpecialConstant),
    InvokeStatic(MethodHandleInvokeStaticConstant),
    InvokeInterface(InterfaceMethodConstant),
}

#[derive(Debug)]
pub enum MethodHandleInvokeSpecialConstant {
    Method(MethodConstant),
    InterfaceMethod(InterfaceMethodConstant),
}

#[derive(Debug)]
pub enum MethodHandleInvokeStaticConstant {
    Method(MethodConstant),
    InterfaceMethod(InterfaceMethodConstant),
}

#[derive(Debug)]
pub struct InvokeDynamicConstant {
    bootstrap_method_attr: u16,
    name_and_type: NameAndTypeConstant
}

#[derive(Debug)]
pub enum Constant {
    String(String),
    I32(i32),
    F32(f32),
    I64(i64),
    F64(f64),
    Class(ClassConstant),
    Field(FieldConstant),
    Method(MethodConstant),
    InterfaceMethod(InterfaceMethodConstant),
    NameAndType(NameAndTypeConstant),
    MethodHandle(MethodHandleConstant),
    MethodType(DescriptorConstant),
    InvokeDynamic(InvokeDynamicConstant),
}