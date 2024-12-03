#[macro_export]
macro_rules! operate_on_all_multi {
    (
        $self:expr,
        [$($field:ident),+],
        $body:block
    ) => {
        $(
            for set in 0..$self.$field.len() {
                let field_name = stringify!($field);
                let field = &$self.$field;
                // The variables `field`, `field_name`, and `set` are available in `$body`
                $body
            }
        )+
    };
}

mod qe_asserts;
mod qe_gut_methods;
mod qe_kcs_methods;
mod qe_misc_btns;
mod qe_set_methods;
mod qe_trnsp_methods;
