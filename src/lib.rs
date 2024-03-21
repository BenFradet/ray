pub mod math;
pub mod model;
pub mod pattern;
pub mod shape;
pub mod viewer;

#[macro_export]
macro_rules! enum_variants_as_structs {
    (
        $(#[$meta_enum:meta])*
        enum $Name:ident {
            // haven't found a way to propagate enum meta's to variants because
            // "meta-variable x repeats N times, but y repeats M times"
            $($(#[$meta_variant:meta])* $Variant:ident { $($f:ident: $ty:ty),* }),* $(,)?
        }
    ) => {
        $(#[$meta_enum])*
        enum $Name {
            $($Variant($Variant),)*
        }

        $(
            $(#[$meta_variant])*
            struct $Variant { $($f: $ty),+ }

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
        $(#[$meta_enum:meta])*
        enum $Name:ident {
            $($(#[$meta_variant:meta])* $Variant:ident($($f:ident),+)),* $(,)?
        }
    ) => {
        $(#[$meta_enum])*
        enum $Name {
            $($Variant($Variant),)*
        }

        $(
            $(#[$meta_variant])*
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
