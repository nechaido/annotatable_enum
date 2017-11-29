#[macro_export]
macro_rules! implement_annotatable_enum {
    ($name:ident {
        $($item: ident = $code: expr,)*
    }) => {
        impl $name {
            pub fn try_from(code: isize) -> ::std::option::Option<$name> {
                match code {
                    $($code => ::std::option::Option::Some($name::$item),)*
                    _ => ::std::option::Option::None
                }
            }
        }

        impl ::std::convert::Into<isize> for $name {
            fn into(self) -> isize {
                self as isize
            }
        }
    }
}

#[macro_export]
macro_rules! annotatable_enum {
    ($name:ident {
        $($item: ident = $code: expr,)*
    }) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        enum $name {
            $($item = $code,)*
        }

        implement_annotatable_enum!($name {
            $($item = $code,)*
        });
    };

    (pub $name:ident {
        $($item: ident = $code: expr,)*
    }) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum $name {
            $($item = $code,)*
        }

        implement_annotatable_enum!($name {
            $($item = $code,)*
        });

    };
}

#[cfg(test)]
mod tests {
    annotatable_enum!(Test {
        Value = 0,
        AnotherValue = 1,
    });

    #[test]
    fn it_creates_enum_from_valid_code() {
        assert_eq!(Test::try_from(0), Some(Test::Value));
        assert_eq!(Test::try_from(1), Some(Test::AnotherValue));
    }

    #[test]
    fn it_does_not_create_enum_from_invalid_code() {
        assert_eq!(Test::try_from(2), None);
    }

    #[test]
    fn it_converts_enum_into_isize() {
        let zero: isize = Test::Value.into();
        assert_eq!(zero, 0);
    }
}
