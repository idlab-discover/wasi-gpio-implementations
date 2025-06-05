
use crate::host_component;

use super::bindings;
use super::util::Shared;

#[allow(dead_code)]
pub struct Pollable {
    trigger: Shared<bool>,
}

impl Pollable {
    pub fn new(trigger: Shared<bool>) -> Self {
        Pollable { trigger }
    }

    pub fn ready(&self) -> bool {
        *self.trigger.lock().unwrap()
    }
}

impl bindings::wasi::gpio::poll::Host for host_component::HostComponent {
    fn poll(&mut self,in_:wasmtime::component::__internal::Vec<wasmtime::component::Resource<Pollable>>,) -> wasmtime::component::__internal::Vec<u32> {
        todo!()
    }
}

impl bindings::wasi::gpio::poll::HostPollable for host_component::HostComponent {
    fn ready(&mut self,self_:wasmtime::component::Resource<Pollable>,) -> bool {
        let poll = self.table.get(&self_).unwrap();
        poll.ready()
    }

    fn block(&mut self,self_:wasmtime::component::Resource<Pollable>,) -> () {
        let poll = self.table.get(&self_).unwrap();
        while !poll.ready() {}
    }

    fn drop(&mut self,rep:wasmtime::component::Resource<Pollable>) -> wasmtime::Result<()> {
        self.table.delete(rep).expect("failed to delete resource");
        Ok(())
    }
}