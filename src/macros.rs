#[cfg(feature = "render")]
macro_rules! iterable_enum {
    ($Name:ident { $($Variant:ident),*$(,)? }) =>
    {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Component)]
        pub enum $Name {
            $($Variant),*,
        }
        impl $Name {
            const ITEMS: &'static [$Name] = &[$($Name::$Variant),*];

            pub fn iterator() -> core::slice::Iter<'static, $Name> {
                $Name::ITEMS.iter()
            }
        }
    }
}
#[cfg(feature = "render")]
pub(crate) use iterable_enum;
