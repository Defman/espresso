use bitflags::bitflags;

mod bytes;
pub mod raw;

#[derive(Debug, PartialEq)]
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

bitflags! {
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