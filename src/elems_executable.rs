//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use std::env;

use crate::{ShellCore,CommandPart};
use crate::utils::blue_string;


pub trait Executable {
    fn eval(&mut self, _conf: &mut ShellCore) -> Vec<String> { vec!() }
    fn exec(&mut self, _conf: &mut ShellCore) -> String { "".to_string() }
}

pub struct BlankPart {
    pub elems: Vec<Box<dyn CommandPart>>,
    text: String,
}

impl Executable for BlankPart {
}

impl BlankPart {
    pub fn new() -> BlankPart{
        BlankPart {
            elems: vec!(),
            text: "".to_string(),
        }
    }

    pub fn push(&mut self, s: Box<dyn CommandPart>){
        self.text += &s.text();
        self.elems.push(s);
    }

    pub fn return_if_valid(ans: BlankPart) -> Option<BlankPart> {
        if ans.elems.len() > 0 {
            Some(ans)
        }else{
            None
        }
    }
}

pub struct Substitutions {
    pub elems: Vec<Box<dyn CommandPart>>,
    text: String,
}

impl Substitutions {
    pub fn new() -> Substitutions{
        Substitutions {
            elems: vec!(),
            text: "".to_string(),
        }
    }

    pub fn return_if_valid(ans: Substitutions) -> Option<Substitutions> {
        if ans.elems.len() > 0 {
            Some(ans)
        }else{
            None
        }
    }
}

impl Executable for Substitutions {
    fn exec(&mut self, conf: &mut ShellCore) -> String {
        if conf.flags.d {
            eprintln!("{}", self.parse_info().join("\n"));
        };

        for e in &mut self.elems {
            let sub = e.eval(conf);
            if sub.len() != 2{
                continue;
            };

            let (key, value) = (sub[0].clone(), sub[1].clone());
            if let Ok(_) = env::var(&key) {
                env::set_var(key, value);
            }else{
                conf.vars.insert(key, value);
            };
        };

        "".to_string()
    }
}

impl Substitutions {
    fn parse_info(&self) -> Vec<String> {
        let mut ans = vec!(format!("substitutions: '{}'", self.text));
        for elem in &self.elems {
            ans.append(&mut elem.parse_info());
        };
        
        blue_string(&ans)
    }

    pub fn push(&mut self, s: Box<dyn CommandPart>){
        self.text += &s.text();
        self.elems.push(s);
    }
}
