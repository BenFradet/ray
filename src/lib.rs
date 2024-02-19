#[macro_export]
macro_rules! enum_variants_as_structs {
    (
        enum $Name:ident {
            $($Variant:ident { $($f:ident: $ty:ty),* }),* $(,)?
        }
    ) => {
        enum $Name {
            $($Variant($Variant),)*
        }

        $(
            struct $Variant { $($f: $ty)+ }

            impl TryFrom<$Name> for $Variant {
                type Error = $Name;

                fn try_from(other: $Name) -> Result<Self, Self::Error> {
                    match other {
                        $Name::$Variant(v) => Ok(v),
                        o => Err(o),
                    }
                }
            }
        )*
    };
    (
        enum $Name:ident {
            $($Variant:ident($($f:ident),+)),* $(,)?
        }
    ) => {
        enum $Name {
            $($Variant($Variant),)*
        }

        $(
            struct $Variant($($f)+);

            impl TryFrom<$Name> for $Variant {
                type Error = $Name;

                fn try_from(other: $Name) -> Result<Self, Self::Error> {
                    match other {
                        $Name::$Variant(v) => Ok(v),
                        o => Err(o),
                    }
                }
            }
        )*
    };
}