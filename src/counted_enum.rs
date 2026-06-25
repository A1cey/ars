/// Generate an enum with an associated constant `COUNT` equal to the number of variants.
///
/// # Example
///
/// ```
/// use ars::counted_enum;
///
/// counted_enum!(
///     #[doc = "Doc comment via attribute."]
///     #[derive(Debug, Clone, Copy, Default)]
///     pub enum Foo {
///         #[doc = "Variant doc comment."]
///         #[default]
///         Bar,
///         Baz(usize, usize),
///         Bat { foo: char }
///     }
/// );
///
/// assert_eq!(Foo::COUNT, 3);
/// ```
#[macro_export]
macro_rules! counted_enum {
    (
        $(#[$enum_meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident $({ $($variant_struct_fields:tt)* })? $(( $($variant_tuple_fields:tt)* ))?
            ),* $(,)?
        }
    ) => {
        $(#[$enum_meta])*
        $vis enum $name {
            $(
                $(#[$variant_meta])*
                $variant $({ $($variant_struct_fields)* })? $(( $($variant_tuple_fields)* ))?
            ),*
        }

        impl $name {
            $vis const COUNT: usize = [ $(stringify!($variant)),* ].len();
        }
    };
}
