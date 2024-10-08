use super::job::Job;
use crate::{Feeder, ShellCore};

pub struct Script {
    pub jobs: Vec<Job>,
    pub text: String,
}

impl Script {
    pub fn parse(text: &mut Feeder, core: &mut ShellCore) -> Option<Script> {
        if let Some(job) = Job::parse(text, core) {
            return Some( Script{text: job.text.clone(), jobs: vec!(job)} );
        }
        None
    }
}