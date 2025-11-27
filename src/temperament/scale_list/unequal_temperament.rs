use std::sync::LazyLock;
use crate::temperament::scale_list::UETScale;

pub static UET_SCALES: LazyLock<Vec<UETScale>> = LazyLock::new(|| vec![]);