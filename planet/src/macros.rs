macro_rules! iterable_enum {
    ($Name:ident { $($Variant:ident),*$(,)? }) =>
    {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize)]
        pub enum $Name {
            $($Variant),*,
        }
        impl $Name {
            pub const ITEMS: &'static [$Name] = &[$($Name::$Variant),*];
            pub const ITEM_COUNT: usize = $Name::ITEMS.len();

            pub fn iterator() -> core::slice::Iter<'static, $Name> {
                $Name::ITEMS.iter()
            }
        }
        impl From<$Name> for &'static str {
            fn from(value: $Name) -> &'static str {
                match value {
                    $($Name::$Variant => stringify!($Variant),)*
                }
            }
        }
        impl From<&$Name> for &'static str {
            fn from(value: &$Name) -> &'static str {
                match value {
                    $($Name::$Variant => stringify!($Variant),)*
                }
            }
        }
    }
}
pub(crate) use iterable_enum;
