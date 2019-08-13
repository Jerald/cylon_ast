use crate::ast::*;

#[macro_export]
macro_rules! boxed_conversion_impl {
    ( $(From<Box<$t:ty>> for Box<$g:ty>);+ ) => {
        $(
            impl From<Box<$t>> for Box<$g>
            where
                $t: Into<$g>
            {
                fn from(boxed: Box<$t>) -> Box<$g>
                {
                    Box::new((*boxed).into())
                }
            }
        )+
    };
}