use super::bindings;
use super::host_component;

pub struct Delay {}

impl bindings::wasi::gpio::delay::Host for host_component::HostComponent {}

impl bindings::wasi::gpio::delay::HostDelay for host_component::HostComponent {
    fn delay_ns(
        &mut self,
        _self_: wasmtime::component::Resource<bindings::wasi::gpio::delay::Delay>,
        ns: u64,
    ) -> () {
        std::thread::sleep(std::time::Duration::from_nanos(ns));
    }

    fn delay_us(
        &mut self,
        _self_: wasmtime::component::Resource<bindings::wasi::gpio::delay::Delay>,
        us: u64,
    ) -> () {
        std::thread::sleep(std::time::Duration::from_micros(us));
    }

    fn delay_ms(
        &mut self,
        _self_: wasmtime::component::Resource<bindings::wasi::gpio::delay::Delay>,
        ms: u64,
    ) -> () {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<bindings::wasi::gpio::delay::Delay>,
    ) -> wasmtime::Result<()> {
        self.table.delete(rep).expect("failed to delete resource");
        Ok(())
    }
}
