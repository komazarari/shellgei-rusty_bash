//SPDX-FileCopyrightText: 2023 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

#[derive(Debug)]
pub struct Word {
    pub text: String,
    pub subwords: Vec<Word>,
}

impl Word {
    pub fn new(s: String) -> Word {
        Word {
            text: s,
            subwords: vec![],
        }
    }
}
