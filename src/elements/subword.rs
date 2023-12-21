//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

// pub mod UnquotedSubword;
// pub mod SingleQuotedSubword;
// pub mod DoubleQuotedSubword;
// pub mod Variable;

use crate::ShellCore;
use std::fmt;
use std::fmt::Debug;

impl Debug for dyn Subword {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("SUBWORD").finish()
    }
}

pub trait Subword {
    fn eval(&mut self, _: &mut ShellCore) -> Vec<Vec<String>>;
}
