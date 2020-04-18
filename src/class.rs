use bytes::{Buf, buf::BufExt};
use std::io::Read;
use bitflags::bitflags;
use std::rc::Rc;
use std::cell::{RefCell, Ref};
use crate::buf_ext::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MajorVersion {
    JavaSE14,
    JavaSE13,
    JavaSE12,
    JavaSE11,
    JavaSE10,
    JavaSE9,
    JavaSE8,
    JavaSE7,
    JavaSE6_0,
    JavaSE5_0,
    JDK1_4,
    JDK1_3,
    JDK1_2,
    JDK1_1,
    Other(u16),
}

impl TryReadFrom for MajorVersion {
    fn try_read(buf: &mut impl Buf) -> Result<Self, TryError> {
        let version = buf.get_u16();
        Ok(match version {
            58 => MajorVersion::JavaSE14,
            57 => MajorVersion::JavaSE13,
            56 => MajorVersion::JavaSE12,
            55 => MajorVersion::JavaSE11,
            54 => MajorVersion::JavaSE10,
            53 => MajorVersion::JavaSE9,
            52 => MajorVersion::JavaSE8,
            51 => MajorVersion::JavaSE7,
            50 => MajorVersion::JavaSE6_0,
            49 => MajorVersion::JavaSE5_0,
            48 => MajorVersion::JDK1_4,
            47 => MajorVersion::JDK1_3,
            46 => MajorVersion::JDK1_2,
            45 => MajorVersion::JDK1_1,
            n => MajorVersion::Other(n),
        })
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ZeroOffset;
impl Offset for ZeroOffset {
    const VALUE: usize = 0;
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OneOffset;
impl Offset for OneOffset {
    const VALUE: usize = 1;
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassFile {
    magic_number: u32,
    minor_version: u16,
    major_version: MajorVersion,
    constant_pool: PrefixedVec<u16, Constant, OneOffset>,
    access_flags: AccessFlags,
    identifier_this: u16,
    identifier_super: u16,
    interfaces: PrefixedVec<u16, u16, ZeroOffset>,
    fields: PrefixedVec<u16, FieldInfo, ZeroOffset>,
    methods: PrefixedVec<u16, MethodInfo, ZeroOffset>,
    attributes: PrefixedVec<u16, AttributeInfo, ZeroOffset>,
}

impl<'a> TryReadFrom for ClassFile {
    fn try_read(buf: &mut impl Buf) -> Result<Self, TryError> {
        let magic_number = buf.try_read()?;
        let minor_version = buf.try_read()?;
        let major_version = buf.try_read()?;

        let constant_pool = buf.try_read()?;

        let access_flags = AccessFlags::from_bits_truncate(buf.try_read()?);
        let identifier_this = buf.try_read()?;
        let identifier_super = buf.try_read()?;
        
        let interfaces = buf.try_read()?;

        let fields = buf.try_read()?;

        let methods = buf.try_read()?;

        let attributes = buf.try_read()?;

        Ok(ClassFile {
            magic_number,
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            identifier_this,
            identifier_super,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: PrefixedVec<u16, AttributeInfo, ZeroOffset>,
}

impl TryReadFrom for FieldInfo {
    fn try_read(buf: &mut impl Buf) -> Result<Self, TryError> {
        let access_flags = buf.try_read()?;
        let name_index = buf.try_read()?;
        let descriptor_index = buf.try_read()?;

        let attributes = buf.try_read()?;

        Ok(Self {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        })
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: PrefixedVec<u16, AttributeInfo, ZeroOffset>,
}

impl TryReadFrom for MethodInfo {
    fn try_read(buf: &mut impl Buf) -> Result<Self, TryError> {
        let access_flags = buf.try_read()?;
        let name_index = buf.try_read()?;
        let descriptor_index = buf.try_read()?;

        let attributes = buf.try_read()?;

        Ok(Self {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        })
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AttributeInfo {
    attribute_name_index: u16,
    info: PrefixedVec<u32, u8, ZeroOffset>,
}

impl TryReadFrom for AttributeInfo {
    fn try_read(buf: &mut impl Buf) -> Result<Self, TryError> {
        let attribute_name_index = buf.try_read()?;
        let info = buf.try_read()?;

        Ok(Self {
            attribute_name_index,
            info,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StringIndex(u16);
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FieldIndex(u16);
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ClassIndex(u16);
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NameAndTypeIndex(u16);
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MethodIndex(u16);
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MethodOrInterfaceMethodIndex(u16);
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct InterfaceMethodIndex(u16);

macro_rules! try_read_index {
    ($index:ident) => {
        impl TryReadFrom for $index {
            fn try_read(buf: &mut impl Buf) -> Result<Self, TryError> {
                Ok(Self(buf.try_read()?))
            }
        }
    };
}

try_read_index!(StringIndex);
try_read_index!(FieldIndex);
try_read_index!(ClassIndex);
try_read_index!(NameAndTypeIndex);
try_read_index!(MethodIndex);
try_read_index!(MethodOrInterfaceMethodIndex);
try_read_index!(InterfaceMethodIndex);

pub struct ConstantPool(Vec<Constant>);

impl std::ops::Index<StringIndex> for ConstantPool {
    type Output = str;

    fn index(&self, index: StringIndex) -> &Self::Output {
        match self.0.get(index.0 as usize) {
            Some(Constant::String(s)) => s,
            _ => panic!("constant at index {} is not a String", index.0)
        }
    }
}

impl std::ops::Index<ClassIndex> for ConstantPool {
    type Output = ClassConstant;

    fn index(&self, index: ClassIndex) -> &Self::Output {
        match self.0.get(index.0 as usize) {
            Some(Constant::Class(class)) => class,
            _ => panic!("constant at index {} is not a ClassConstant", index.0)
        }
    }
}

impl std::ops::Index<FieldIndex> for ConstantPool {
    type Output = FieldConstant;

    fn index(&self, index: FieldIndex) -> &Self::Output {
        match self.0.get(index.0 as usize) {
            Some(Constant::Field(field)) => field,
            _ => panic!("constant at index {} is not a FieldConstant", index.0)
        }
    }
}

impl std::ops::Index<NameAndTypeIndex> for ConstantPool {
    type Output = NameAndTypeConstant;

    fn index(&self, index: NameAndTypeIndex) -> &Self::Output {
        match self.0.get(index.0 as usize) {
            Some(Constant::NameAndType(named_and_typed)) => named_and_typed,
            _ => panic!("constant at index {} is not a NameAndType", index.0)
        }
    }
}

impl std::ops::Index<MethodIndex> for ConstantPool {
    type Output = MethodConstant;

    fn index(&self, index: MethodIndex) -> &Self::Output {
        match self.0.get(index.0 as usize) {
            Some(Constant::Method(method)) => method,
            _ => panic!("constant at index {} is not a NameAndType", index.0)
        }
    }
}

impl std::ops::Index<MethodOrInterfaceMethodIndex> for ConstantPool {
    type Output = Constant;

    fn index(&self, index: MethodOrInterfaceMethodIndex) -> &Self::Output {
        match self.0.get(index.0 as usize) {
            Some(c @ Constant::Method(_)) => c,
            Some(c @ Constant::InterfaceMethod(_)) => c,
            _ => panic!("constant at index {} is not a MethodOrInterface", index.0)
        }
    }
}

impl std::ops::Index<InterfaceMethodIndex> for ConstantPool {
    type Output = InterfaceMethodConstant;

    fn index(&self, index: InterfaceMethodIndex) -> &Self::Output {
        match self.0.get(index.0 as usize) {
            Some(Constant::InterfaceMethod(method)) => method,
            _ => panic!("constant at index {} is not a InterfaceMethod", index.0)
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassConstant {
    name: StringIndex,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NameAndTypeConstant {
    name: StringIndex,
    descriptor: StringIndex,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldConstant {
    class: ClassIndex,
    name_and_type: NameAndTypeIndex,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MethodConstant {
    class: ClassIndex,
    name_and_type: NameAndTypeIndex,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InterfaceMethodConstant {
    class: ClassIndex,
    name_and_type: NameAndTypeIndex,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InvokeDynamicConstant {
    bootstrap_method_attr: u16,
    name_and_type: NameAndTypeIndex
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MethodTypeConstant {
    descriptor: StringIndex,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StringRefConstant {
    string: StringIndex,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

impl TryReadFrom for Constant {
    fn try_read(buf: &mut impl Buf) -> Result<Self, TryError> {        
        let tag = buf.get_u8();
        let constant_ref = match tag {
            1 => {
                let length = u16::try_read(buf)? as usize;
                let mut string = String::with_capacity(length);
                let read = buf.reader()
                    .take(length as u64)
                    .read_to_string(&mut string)
                    .map_err(|_| TryError::Malformed)?;
                if read < length {
                    return Err(TryError::NotEnoughBytes);
                }
                Constant::String(string)
            }
            3 => Constant::I32(buf.try_read()?),
            4 => Constant::F32(buf.try_read()?),
            5 => Constant::I64(buf.try_read()?),
            6 => Constant::F64(buf.try_read()?),
            7 => Constant::Class(ClassConstant {
                name: buf.try_read()?,
            }),
            8 => Constant::StringRef(StringRefConstant{
                string: buf.try_read()?
            }),
            9 => Constant::Field(FieldConstant {
                class: buf.try_read()?,
                name_and_type: buf.try_read()?,
            }),
            10 => Constant::Method(MethodConstant {
                class: buf.try_read()?,
                name_and_type: buf.try_read()?,
            }),
            11 => Constant::InterfaceMethod(InterfaceMethodConstant {
                class: buf.try_read()?,
                name_and_type: buf.try_read()?,
            }),
            12 => Constant::NameAndType(NameAndTypeConstant {
                name: buf.try_read()?,
                descriptor: buf.try_read()?,
            }),
            15 => {
                let kind: u8 = buf.try_read()?;
                Constant::MethodHandle(match kind {
                    1 => MethodHandleConstant::GetField(buf.try_read()?),
                    2 => MethodHandleConstant::GetStatic(buf.try_read()?),
                    3 => MethodHandleConstant::PutField(buf.try_read()?),
                    4 => MethodHandleConstant::PutStatic(buf.try_read()?),
                    5 => MethodHandleConstant::InvokeVirtual(buf.try_read()?),
                    6 => MethodHandleConstant::NewInvokeSpecial(buf.try_read()?),
                    7 => MethodHandleConstant::InvokeSpecial(buf.try_read()?),
                    8 => MethodHandleConstant::InvokeStatic(buf.try_read()?),
                    9 => MethodHandleConstant::InvokeInterface(buf.try_read()?),
                    _ => return Err(TryError::Malformed),
                })
            },
            16 => Constant::MethodType(MethodTypeConstant {
                descriptor: buf.try_read()?
            }),
            18 => Constant::InvokeDynamic(InvokeDynamicConstant {
                bootstrap_method_attr: buf.try_read()?,
                name_and_type: buf.try_read()?,
            }),
            _ => return Err(TryError::Malformed),
        };
        Ok(constant_ref)
    }
}

#[derive(Debug)]
pub struct Field {
    access_flags: AccessFlags,
    name_index: u16,
    descriptor_index: u16,
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    struct AccessFlags: u16 {
        /// Declared public; may be accessed from outside its package. 
        const ACC_PUBLIC = 0x0001;
        /// Declared private; usable only within the defining class. 
        const ACC_PRIVATE = 0x0002;
        /// Declared protected; may be accessed within subclasses. 
        const ACC_PROTECTED = 0x0004;
        /// Declared static. 
        const ACC_STATIC = 0x0008;
        /// Declared final; never directly assigned to after object construction (JLS §17.5). 
        const ACC_FINAL = 0x0010;
        /// Declared volatile; cannot be cached. 
        const ACC_VOLATILE = 0x0040;
        /// Declared transient; not written or read by a persistent object manager. 
        const ACC_TRANSIENT = 0x0080;
        /// Declared synthetic; not present in the source code. 
        const ACC_SYNTHETIC = 0x1000;
        /// Declared as an element of an enum. 
        const ACC_ENUM = 0x4000;
    }
}
