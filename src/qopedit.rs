mod qe_asserts;
mod qe_gut_methods;
mod qe_kcs_methods;
mod qe_misc_btns;
mod qe_set_methods;
mod qe_trnsp_methods;

macro_rules! edit_mode_only {
    ($self:ident, $body:block) => {
        match $self.qop_mode {
            crate::QopMode::Edit => {
                $body;
                $self.check_gutdelta_lengths();
                $self.check_digitalref_invariants();
            },
            crate::QopMode::Play => {
                panic!("called Qop::Edit method in Qop::Play mode!")
            }
        }
    };
}

use edit_mode_only;