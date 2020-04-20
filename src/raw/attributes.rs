
pub enum Attribute {
    ConstantValue(ConstantValueAttribute),
    Code(CodeAttribute),
    StackMapTable(StackMapTableAttribute),
    Exceptions(ExceptionsAttribute),
    InnerClasses(InnerClassesAttribute),
    EnclosingMethod(EnclosingMethodAttribute),
    Synthetic(SyntheticAttribute),
    Signature(SignatureAttribute),
    SourceFile(SourceFileAttribute),
    SourceDebugExtension(SourceDebugExtensionAttribute),
    LineNumberTable(LineNumberTableAttribute),
    LocalVariableTable(LocalVariableTableAttribute),
    LocalVariableTypeTable(LocalVariableTypeTableAttribute),
    Deprecated(DeprecatedAttribute),
    RuntimeVisibleAnnotations(RuntimeVisibleAnnotations),
    RuntimeInvisibleAnnotations(RuntimeInvisibleAnnotationsAttribute),
    RuntimeVisibleParameterAnnotations(RuntimeVisibleParameterAnnotations),
    RuntimeInvisibleParameterAnnotations(RuntimeInvisibleParameterAnnotationsAttribute),
    RuntimeVisibleTypeAnnotations(RuntimeVisibleTypeAnnotationsAttribute),
    BootstrapMethods(BootstrapMethodsAttribute),
    Other(String),
}

pub struct ConstantValueAttribute {
    value: Primitive,
}



pub struct CodeAttribute {
    max_stack: MaxStack,
    max_locals: MaxLocals,
    code: Vec<Code>,
    exception_table: Vec<Exception>,
    attributes: Vec<Attribute>,
}

pub struct Exception {
    start: Pc,
    end: Pc,
    handler: Pc,
    catch_type: u16,
}

pub struct StackMapTableAttribute {
    entries: Vec<u8>,
}

// struct StackMapFrame {}

pub struct ExceptionsAttribute {
    exceptions: Vec<ClassInfo>,
}


macro_rules! wrap {
    {
        $(#[$meta:meta])*
        pub struct $outer:ident($inner:ident);
    } => {

        pub struct $outer($inner);

        impl std::ops::Deref for $outer {
            type Target = $inner;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

wrap! {
   pub struct Pc(u16); 
}

wrap! {
    pub struct MaxStack(u16);
}

wrap! {
    pub struct MaxLocals(u16);
}

wrap! {
    pub struct Code(u8);
}