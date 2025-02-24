#[macro_export]
macro_rules! mk_tests {
    ( $($name:ident : $exp:expr ),+ $(,)? ) => {
    $(
        #[test]
        fn $name() {
            $exp
        }
    )*
    }
}
