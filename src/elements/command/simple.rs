//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use crate::{ShellCore, Feeder};
use super::{Command, Pipe, Redirect};
use crate::elements::command;
use crate::elements::word::Word;
use nix::unistd;
use std::ffi::CString;
use std::process;

use nix::unistd::Pid;
use nix::errno::Errno;

fn reserved(w: &str) -> bool {
    match w {
        "{" | "}" | "while" | "do" | "done" => true,
        _ => false,
    }
}

#[derive(Debug)]
pub struct SimpleCommand {
    text: String,
    words: Vec<Word>,
    args: Vec<String>,
    redirects: Vec<Redirect>,
    force_fork: bool,
}

impl Command for SimpleCommand {
    fn exec(&mut self, core: &mut ShellCore, pipe: &mut Pipe) -> Option<Pid> {
        if self.words.len() == 0 {
            return None;
        }

        for a in &mut self.words {
            self.args.extend(a.get_args());
        }

        if self.force_fork 
        || pipe.is_connected() 
        || ! core.builtins.contains_key(&self.args[0]) {
            self.fork_exec(core, pipe)
        }else{
            self.nofork_exec(core);
            None
        }
    }

    fn run_command(&mut self, core: &mut ShellCore, fork: bool) {
        if ! fork {
            core.run_builtin(&mut self.args);
            return;
        }

        if core.run_builtin(&mut self.args) {
            core.exit()
        }else{
            Self::exec_external_command(&mut self.args)
        }
    }

    fn get_text(&self) -> String { self.text.clone() }
    fn get_redirects(&mut self) -> &mut Vec<Redirect> { &mut self.redirects }
    fn set_force_fork(&mut self) { self.force_fork = true; }
}

impl SimpleCommand {
    fn exec_external_command(words: &mut Vec<String>) -> ! {
        let cwords = Self::to_cwords(words);
        match unistd::execvp(&cwords[0], &cwords) {
            Err(Errno::EACCES) => {
                println!("sush: {}: Permission denied", &words[0]);
                process::exit(126)
            },
            Err(Errno::ENOENT) => {
                println!("{}: command not found", &words[0]);
                process::exit(127)
            },
            Err(err) => {
                println!("Failed to execute. {:?}", err);
                process::exit(127)
            }
            _ => panic!("SUSH INTERNAL ERROR (never come here)")
        }
    }

    /*
    fn eval(&self) -> Vec<String> {
        let mut ans = vec![];
        for a in self.words {
            ans.append(a.eval());
        }

        ans
    }*/

    fn to_cwords(words: &mut Vec<String>) -> Vec<CString> {
        words.iter()
            .map(|a| CString::new(a.to_string()).unwrap())
            .collect()
    }

    fn new() -> SimpleCommand {
        SimpleCommand {
            text: String::new(),
            words: vec![],
            args: vec![],
            redirects: vec![],
            force_fork: false,
        }
    }
 
    fn eat_words(feeder: &mut Feeder, ans: &mut SimpleCommand, core: &mut ShellCore) -> bool {
        let w = match Word::parse(feeder, core) {
            Some(w) => w,
            _       => return false,
        };

        if ans.words.len() == 0 && reserved(&w.text) {
            return false;
        }
 
        ans.text += &w.text;
        ans.words.push(w);
        true
    }

    pub fn parse(feeder: &mut Feeder, core: &mut ShellCore) -> Option<SimpleCommand> {
        let mut ans = Self::new();
        feeder.set_backup();

        loop {
            command::eat_blank_with_comment(feeder, core, &mut ans.text);
            if ! command::eat_redirect(feeder, core, &mut ans.redirects, &mut ans.text)
                && ! Self::eat_words(feeder, &mut ans, core) {
                break;
            }
        }

        if ans.words.len() + ans.redirects.len() > 0 {
            feeder.pop_backup();
            Some(ans)
        }else{
            feeder.rewind();
            None
        }
    }
}
