#![allow(unused)]

pub mod bindings {
    wasmtime::component::bindgen!({
        path: "../../wit",
        world: "rpi",
        with: {
            "wasi:gpio/digital/digital-out-pin": crate::DigitalOutPin,
            "wasi:gpio/digital/digital-in-pin": crate::DigitalInPin,
            "wasi:gpio/digital/digital-in-out-pin": crate::DigitalInOutPin,
            "wasi:gpio/digital/stateful-digital-out-pin": crate::StatefulDigitalOutPin,
            "wasi:gpio/analog/analog-in-pin": crate::AnalogInPin,
            "wasi:gpio/analog/analog-out-pin": crate::AnalogOutPin,
            "wasi:gpio/analog/analog-in-out-pin": crate::AnalogInOutPin,
            "wasi:io/poll/pollable": crate::Pollable,
            "wasi:gpio/delay/delay": crate::Delay
        }
    });
}

mod util;
mod policies;
mod delay;
mod analog;
mod digital;
mod general;
mod poll;

pub use digital::{DigitalInOutPin, DigitalInPin, DigitalOutPin, StatefulDigitalOutPin};
pub use delay::Delay;
pub use analog::{AnalogInOutPin, AnalogInPin, AnalogOutPin};
pub use poll::Pollable;

use clap::Parser;
use util::Shared;

mod host_component;
use host_component::HostComponent;

mod watch_event;

struct State {
    ctx: wasmtime_wasi::WasiCtx,
    host: HostComponent,
}

impl wasmtime_wasi::IoView for State {
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        &mut self.host.table
    }
}

impl wasmtime_wasi::WasiView for State {
    fn ctx(&mut self) -> &mut wasmtime_wasi::WasiCtx {
        &mut self.ctx
    }
}

fn main() {
    let config = policies::Config::parse();
    
    println!("{:?}", config);
    let policies = config.get_policies();
    
    let engine = wasmtime::Engine::new(wasmtime::Config::new().wasm_component_model(true)).unwrap();
    let component = wasmtime::component::Component::from_file(&engine, config.get_component_path()).unwrap();

    let mut linker = wasmtime::component::Linker::new(&engine);

    bindings::wasi::gpio::general::add_to_linker(&mut linker, |state: &mut State| &mut state.host).unwrap();
    bindings::wasi::gpio::digital::add_to_linker(&mut linker, |state: &mut State| &mut state.host).unwrap();
    bindings::wasi::gpio::analog::add_to_linker(&mut linker, |state: &mut State| &mut state.host).unwrap();
    bindings::wasi::gpio::delay::add_to_linker(&mut linker, |state: &mut State| &mut state.host).unwrap();
    
    wasmtime_wasi::add_to_linker_sync(&mut linker).unwrap();
    
    let mut state = State {
        ctx: wasmtime_wasi::WasiCtxBuilder::new().inherit_stdio().build(),
        host: HostComponent::new(policies, rppal::gpio::Gpio::new().unwrap(), config.get_pi_type())
    };
    
    let delay = state.host.table.push(Delay{}).unwrap();
    
    let mut store = wasmtime::Store::new(&engine, state);
    
    let instance = bindings::Rpi::instantiate(&mut store, &component, &linker).unwrap();
    
    instance.call_start(&mut store, delay).unwrap();
}
