//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::debuginfo::DebugInfo;
use crate::ShellCore;
use crate::Feeder;
use crate::elem_pipeline::{Pipeline};
use crate::scanner::*;

use crate::abst_arg_elem::ArgElem;
use crate::abst_script_elem::ScriptElem;

pub struct SubArgCommandExp {
    pub text: String,
    pub pos: DebugInfo,
    pub com: Pipeline, 
}

impl ArgElem for SubArgCommandExp {
    fn eval(&mut self, conf: &mut ShellCore) -> Vec<String> {
        self.com.expansion = true;
        let _ = self.com.exec(conf);
        vec!(self.com.expansion_str.replace("\n", " "))
    }

    fn text(&self) -> String {
        self.text.clone()
    }
}

impl SubArgCommandExp {
    pub fn parse(text: &mut Feeder, conf: &mut ShellCore) -> Option<SubArgCommandExp> {
        if !(text.nth(0) == '$' && text.nth(1) == '(') {
            return None;
        }
    
        let pos = scanner_end_of_bracket(text, 2, ')');
        let mut sub_feeder = Feeder::new_with(text.from_to(2, pos));
    
        if let Some(e) = Pipeline::parse(&mut sub_feeder, conf){
            let ans = Some (SubArgCommandExp {
                text: text.consume(pos+1),
                pos: DebugInfo::init(text),
                com: e }
            );
    
            return ans;
        };
        None
    }
}
