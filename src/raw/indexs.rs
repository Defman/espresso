macro_rules! index {
    ($index:ident) => {

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $index(u16);

        impl std::ops::Deref for $index {
            type Target = u16;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

index!(StringIndex);
index!(FieldIndex);
index!(ClassIndex);
index!(NameAndTypeIndex);
index!(MethodIndex);
index!(MethodOrInterfaceMethodIndex);
index!(InterfaceMethodIndex);