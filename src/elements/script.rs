use super::job::Job;
use crate::{Feeder, ShellCore};

#[derive(Debug)]
pub struct Script {
    pub jobs: Vec<Job>,
    pub text: String,
}

impl Script {
    fn new() -> Script {
        Script {
            jobs: vec![],
            text: String::new(),
        }
    }
    pub fn exec(&mut self, core: &mut ShellCore) {
        for job in self.jobs.iter_mut() {
            job.exec(core);
        }
    }

    fn eat_job(feeder: &mut Feeder, core: &mut ShellCore, ans: &mut Script) -> bool {
        if let Some(job) = Job::parse(feeder, core) {
            ans.text += &job.text.clone();
            ans.jobs.push(job);
            true
        } else {
            false
        }
    }

    fn eat_job_end(feeder: &mut Feeder, ans: &mut Script) -> bool {
        let len = feeder.scanner_job_end();
        if len > 0 {
            ans.text += &feeder.consume(len);
            true
        } else {
            false
        }
    }

    pub fn parse(feeder: &mut Feeder, core: &mut ShellCore) -> Option<Script> {
        let mut ans = Self::new();

        while Self::eat_job_end(feeder, &mut ans) {}
        while Self::eat_job(feeder, core, &mut ans) {
            while Self::eat_job_end(feeder, &mut ans) {}
        }

        if feeder.remaining.len() == 0 {
            // eprintln!("{:?}", &ans);
            Some(ans)
        } else {
            eprintln!("error");
            None
        }
    }
}
