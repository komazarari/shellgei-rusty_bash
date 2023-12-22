//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

// pub mod UnquotedSubword;
// pub mod SingleQuotedSubword;
// pub mod DoubleQuotedSubword;
// pub mod Variable;

use crate::ShellCore;
use crate::elements::word::Word;
use std::fmt;
use std::fmt::Debug;

impl Debug for dyn Subword {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("SUBWORD").finish()
    }
}

pub trait Subword {
    fn eval(&mut self, _: &mut ShellCore) -> Word;
    fn brace_expansion(&mut self, _: &mut ShellCore) -> Vec<Word>;
    fn expansion(&mut self, _: &mut ShellCore) -> Vec<Word>;
}
