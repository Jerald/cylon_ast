#[macro_export]
macro_rules! boxed_from_impl {
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

#[macro_export]
macro_rules! boxed_try_from_impl {
    ( $(TryFrom<Box<$t:ty>> for Box<$g:ty>);+ ) => {
        $(
            impl TryFrom<Box<$t>> for Box<$g>
            where
                $t: TryInto<$g>
            {
                type Error = <$t as TryInto<$g>>::Error;

                fn try_from(boxed: Box<$t>) -> Result<Box<$g>, Self::Error>
                {
                    Ok(Box::new((*boxed).try_into()?))
                }
            }
        )+
    };
}