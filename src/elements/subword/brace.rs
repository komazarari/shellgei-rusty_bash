//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::{ShellCore, Feeder};
use crate::elements::word::Word;
use super::Subword;

#[derive(Debug)]
pub struct BraceSubword {
    text: String,
    elements: Vec<Word>, 
}

impl Subword for BraceSubword {
    fn get_text(&self) -> String { self.text.clone() }
}

impl BraceSubword {
    fn new() -> BraceSubword {
        BraceSubword {
            text: String::new(),
            elements: vec![],
        }
    }

    pub fn parse(feeder: &mut Feeder, core: &mut ShellCore) -> Option<BraceSubword> {
        let mut ans = Self::new();

        let len = feeder.scanner_word(core);
        if len == 0 {
            return None;
        }
 
        ans.text = feeder.consume(len);
        Some(ans)
    }
}
