use std::task::Poll;

use super::util::Shared;

pub struct Pollable {
    trigger: Shared<bool>,
}

impl Pollable {
    pub fn new(trigger: Shared<bool>) -> Self {
        Pollable { trigger }
    }
}