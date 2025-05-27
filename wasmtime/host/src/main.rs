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

use clap::Parser;
use util::Shared;

mod host_component;
use host_component::HostComponent;

pub mod digital;
mod watch_event;
pub use digital::{DigitalInOutPin, DigitalInPin, DigitalOutPin, StatefulDigitalOutPin};

pub struct AnalogInPin {}
pub struct AnalogOutPin {}
pub struct AnalogInOutPin {}
pub struct Pollable {
    trigger: Shared<bool>,
}

pub struct Delay {}

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

impl bindings::wasi::gpio::general::Host for HostComponent {}

impl bindings::wasi::gpio::digital::Host for HostComponent {}

impl bindings::wasi::gpio::digital::HostDigitalInPin for HostComponent {
    fn get(
        &mut self,
        pin_label: wasmtime::component::__internal::String,
        mut flags: wasmtime::component::__internal::Vec<bindings::wasi::gpio::digital::DigitalFlag>,
    ) -> Result<wasmtime::component::Resource<DigitalInPin>, bindings::wasi::gpio::general::GpioError> {
        if !self.policies.is_mode_allowed(&pin_label, policies::Mode::DigitalInput) {
            return Err(bindings::wasi::gpio::general::GpioError::PinModeNotAllowed);
        }

        let pin = match self.get_pin(&pin_label) {
            Ok(pin) => pin,
            Err(err) => {
                return Err(err);
            }
        };

        flags.push(bindings::wasi::gpio::digital::DigitalFlag::INPUT);

        let mut config = digital::DigitalConfigBuilder::new(pin_label);
        config.add_flags(flags);
        let config = match config.build() {
            Ok(config) => config,
            Err(_) => return Err(bindings::wasi::gpio::general::GpioError::InvalidFlag),
        };

        match self.table.push(DigitalInPin::new(pin, config)) {
            Ok(pin) => Ok(pin),
            Err(err) => Err(bindings::wasi::gpio::general::GpioError::Other(
                err.to_string(),
            )),
        }
    }

    fn get_config(
        &mut self,
        self_: wasmtime::component::Resource<DigitalInPin>,
    ) -> Result<bindings::wasi::gpio::digital::DigitalConfig, bindings::wasi::gpio::general::GpioError> {
        Ok(self.table.get(&self_).map_err(|_| bindings::wasi::gpio::general::GpioError::ResourceInvalidated)?.get_config().clone())
    }

    fn is_ready(&mut self, self_: wasmtime::component::Resource<DigitalInPin>) -> bool {
        match self.table.get(&self_) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn read(
        &mut self,
        self_: wasmtime::component::Resource<DigitalInPin>,
    ) -> Result<bindings::wasi::gpio::digital::PinState, bindings::wasi::gpio::general::GpioError>
    {
        let pin = self.table.get(&self_).map_err(|_| bindings::wasi::gpio::general::GpioError::ResourceInvalidated)?;

        Ok(pin.read())
    }

    fn is_active(
        &mut self,
        self_: wasmtime::component::Resource<DigitalInPin>,
    ) -> Result<bool, bindings::wasi::gpio::general::GpioError> {
        Ok(self.read(self_)? == bindings::wasi::gpio::digital::PinState::Active)
    }

    fn is_inactive(
        &mut self,
        self_: wasmtime::component::Resource<DigitalInPin>,
    ) -> Result<bool, bindings::wasi::gpio::general::GpioError> {
        Ok(self.read(self_)? == bindings::wasi::gpio::digital::PinState::Inactive)
    }

    fn watch_state(
        &mut self,
        self_: wasmtime::component::Resource<DigitalInPin>,
        state: bindings::wasi::gpio::digital::PinState,
    ) -> Result<wasmtime::component::Resource<Pollable>, bindings::wasi::gpio::general::GpioError> {
        let pin = self.table.get(&self_).map_err(|_| bindings::wasi::gpio::general::GpioError::ResourceInvalidated)?;

        let watch_type = match state {
            bindings::wasi::gpio::digital::PinState::Active => watch_event::WatchType::High,
            bindings::wasi::gpio::digital::PinState::Inactive => watch_event::WatchType::Low,
        };

        let watch_type = match &pin.get_config().active_level {
            bindings::wasi::gpio::general::ActiveLevel::ActiveHigh => watch_type,
            bindings::wasi::gpio::general::ActiveLevel::ActiveLow => !watch_type,
        };

        let trigger = self.watcher.watch_event(pin, watch_type);

        match self.table.push(Pollable { trigger }) {
            Ok(pol) => Ok(pol),
            Err(err) => Err(bindings::wasi::gpio::general::GpioError::Other(
                err.to_string(),
            )),
        }
    }

    fn watch_active(
        &mut self,
        self_: wasmtime::component::Resource<DigitalInPin>,
    ) -> Result<wasmtime::component::Resource<Pollable>, bindings::wasi::gpio::general::GpioError>
    {
        self.watch_state(self_, bindings::wasi::gpio::digital::PinState::Active)
    }

    fn watch_inactive(
        &mut self,
        self_: wasmtime::component::Resource<DigitalInPin>,
    ) -> Result<wasmtime::component::Resource<Pollable>, bindings::wasi::gpio::general::GpioError>
    {
        self.watch_state(self_, bindings::wasi::gpio::digital::PinState::Inactive)
    }

    fn watch_falling_edge(
        &mut self,
        self_: wasmtime::component::Resource<DigitalInPin>,
    ) -> Result<wasmtime::component::Resource<Pollable>, bindings::wasi::gpio::general::GpioError>
    {
        let pin = self.table.get(&self_).map_err(|_| bindings::wasi::gpio::general::GpioError::ResourceInvalidated)?;

        let watch_event = match &pin.get_config().active_level {
            bindings::wasi::gpio::general::ActiveLevel::ActiveHigh => watch_event::WatchType::Falling,
            bindings::wasi::gpio::general::ActiveLevel::ActiveLow => watch_event::WatchType::Rising,
        };
        
        let trigger = self.watcher.watch_event(pin, watch_event);

        match self.table.push(Pollable { trigger }) {
            Ok(pol) => Ok(pol),
            Err(err) => Err(bindings::wasi::gpio::general::GpioError::Other(
                err.to_string(),
            )),
        }
    }

    fn watch_rising_edge(
        &mut self,
        self_: wasmtime::component::Resource<DigitalInPin>,
    ) -> Result<wasmtime::component::Resource<Pollable>, bindings::wasi::gpio::general::GpioError>
    {
        let pin = self.table.get(&self_).map_err(|_| bindings::wasi::gpio::general::GpioError::ResourceInvalidated)?;

        let watch_event = match &pin.get_config().active_level {
            bindings::wasi::gpio::general::ActiveLevel::ActiveHigh => watch_event::WatchType::Rising,
            bindings::wasi::gpio::general::ActiveLevel::ActiveLow => watch_event::WatchType::Falling,
        };
        
        let trigger = self.watcher.watch_event(pin, watch_event);

        match self.table.push(Pollable { trigger }) {
            Ok(pol) => Ok(pol),
            Err(err) => Err(bindings::wasi::gpio::general::GpioError::Other(
                err.to_string(),
            )),
        }
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<DigitalInPin>) -> wasmtime::Result<()> {
        self.table.delete(rep).expect("failed to delete resource");
        Ok(())
    }
}

impl bindings::wasi::gpio::digital::HostDigitalOutPin for HostComponent {
    fn get(
        &mut self,
        pin_label: wasmtime::component::__internal::String,
        mut flags: wasmtime::component::__internal::Vec<bindings::wasi::gpio::digital::DigitalFlag>,
    ) -> Result<wasmtime::component::Resource<DigitalOutPin>, bindings::wasi::gpio::general::GpioError> {
        if !self.policies.is_mode_allowed(&pin_label, policies::Mode::DigitalInput) {
            return Err(bindings::wasi::gpio::general::GpioError::PinModeNotAllowed);
        }

        let pin = match self.get_pin(&pin_label) {
            Ok(pin) => pin,
            Err(err) => {
                return Err(err);
            }
        };

        flags.push(bindings::wasi::gpio::digital::DigitalFlag::INPUT);

        let mut config = digital::DigitalConfigBuilder::new(pin_label);
        config.add_flags(flags);
        let config = match config.build() {
            Ok(config) => config,
            Err(_) => return Err(bindings::wasi::gpio::general::GpioError::InvalidFlag),
        };

        match self.table.push(DigitalOutPin::new(pin, config)) {
            Ok(pin) => Ok(pin),
            Err(err) => Err(bindings::wasi::gpio::general::GpioError::Other(
                err.to_string(),
            )),
        }
    }

    fn get_config(
        &mut self,
        self_: wasmtime::component::Resource<DigitalOutPin>,
    ) -> Result<bindings::wasi::gpio::digital::DigitalConfig, bindings::wasi::gpio::general::GpioError> {
        Ok(self.table.get(&self_).map_err(|_| bindings::wasi::gpio::general::GpioError::ResourceInvalidated)?.get_config().clone())
    }

    fn is_ready(&mut self, self_: wasmtime::component::Resource<DigitalOutPin>) -> bool {
        todo!()
    }

    fn set_state(
        &mut self,
        self_: wasmtime::component::Resource<DigitalOutPin>,
        state: bindings::wasi::gpio::digital::PinState,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        self.table.get_mut(&self_).unwrap().write(state);
        Ok(())
    }

    fn set_active(
        &mut self,
        self_: wasmtime::component::Resource<DigitalOutPin>,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        self.set_state(self_, bindings::wasi::gpio::digital::PinState::Active)
    }

    fn set_inactive(
        &mut self,
        self_: wasmtime::component::Resource<DigitalOutPin>,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        self.set_state(self_, bindings::wasi::gpio::digital::PinState::Inactive)
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<DigitalOutPin>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}

impl bindings::wasi::gpio::digital::HostDigitalInOutPin for HostComponent {
    fn get_config(
        &mut self,
        self_: wasmtime::component::Resource<DigitalInOutPin>,
    ) -> Result<bindings::wasi::gpio::digital::DigitalConfig, bindings::wasi::gpio::general::GpioError> {
        Ok(self.table.get(&self_).map_err(|_| bindings::wasi::gpio::general::GpioError::ResourceInvalidated)?.get_config().clone())
    }

    fn is_ready(&mut self, self_: wasmtime::component::Resource<DigitalInOutPin>) -> bool {
        match self.table.get(&self_) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn set_state(
        &mut self,
        self_: wasmtime::component::Resource<DigitalInOutPin>,
        state: bindings::wasi::gpio::digital::PinState,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        self.table.get_mut(&self_).map_err(|_| bindings::wasi::gpio::general::GpioError::ResourceInvalidated)?.write(state);
        Ok(())
    }

    fn set_active(
        &mut self,
        self_: wasmtime::component::Resource<DigitalInOutPin>,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        self.set_state(self_, bindings::wasi::gpio::digital::PinState::Active)
    }

    fn set_inactive(
        &mut self,
        self_: wasmtime::component::Resource<DigitalInOutPin>,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        self.set_state(self_, bindings::wasi::gpio::digital::PinState::Active)
    }

    fn read(
        &mut self,
        self_: wasmtime::component::Resource<DigitalInOutPin>,
    ) -> Result<bindings::wasi::gpio::digital::PinState, bindings::wasi::gpio::general::GpioError>
    {
        Ok(self.table.get(&self_).map_err(|_| bindings::wasi::gpio::general::GpioError::ResourceInvalidated)?.read())
    }

    fn is_active(
        &mut self,
        self_: wasmtime::component::Resource<DigitalInOutPin>,
    ) -> Result<bool, bindings::wasi::gpio::general::GpioError> {
        let state = self.read(self_)?;

        Ok(bindings::wasi::gpio::digital::PinState::Active == state)
    }

    fn is_inactive(
        &mut self,
        self_: wasmtime::component::Resource<DigitalInOutPin>,
    ) -> Result<bool, bindings::wasi::gpio::general::GpioError> {
        let state = self.read(self_)?;

        Ok(bindings::wasi::gpio::digital::PinState::Active == state)
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<DigitalInOutPin>,
    ) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }

    fn get(
        &mut self,
        pin_label: wasmtime::component::__internal::String,
        flags: wasmtime::component::__internal::Vec<bindings::wasi::gpio::digital::DigitalFlag>,
    ) -> Result<
        wasmtime::component::Resource<DigitalInOutPin>,
        bindings::wasi::gpio::general::GpioError,
    > {
        if !self.policies.is_mode_allowed(&pin_label, policies::Mode::DigitalInput) {
            return Err(bindings::wasi::gpio::general::GpioError::PinModeNotAllowed);
        }
        
        let pin = match self.get_pin(&pin_label) {
            Ok(pin) => pin,
            Err(err) => {
                return Err(err);
            }
        };

        let mut config = digital::DigitalConfigBuilder::new(pin_label);
        config.add_flags(flags);
        let config = match config.build() {
            Ok(config) => config,
            Err(_) => return Err(bindings::wasi::gpio::general::GpioError::InvalidFlag),
        };

        match self.table.push(DigitalInOutPin::new(pin, config)) {
            Ok(pin) => Ok(pin),
            Err(err) => Err(bindings::wasi::gpio::general::GpioError::Other(
                err.to_string(),
            )),
        }
    }
    
    fn set_pin_mode(&mut self,self_:wasmtime::component::Resource<DigitalInOutPin>,pin_mode:bindings::wasi::gpio::general::PinMode,) -> Result<(),bindings::wasi::gpio::general::GpioError> {
        Ok(self.table.get(&self_).map_err(|_| bindings::wasi::gpio::general::GpioError::ResourceInvalidated)?.set_pin_mode(pin_mode))
    }
}

impl bindings::wasi::gpio::digital::HostStatefulDigitalOutPin for HostComponent {
    fn get(
        &mut self,
        pin_label: wasmtime::component::__internal::String,
        flags: wasmtime::component::__internal::Vec<bindings::wasi::gpio::digital::DigitalFlag>,
    ) -> Result<
        wasmtime::component::Resource<StatefulDigitalOutPin>,
        bindings::wasi::gpio::general::GpioError,
    > {
        Err(bindings::wasi::gpio::general::GpioError::PinModeNotAvailable)
    }

    fn get_config(
        &mut self,
        self_: wasmtime::component::Resource<StatefulDigitalOutPin>,
    ) -> Result<bindings::wasi::gpio::digital::DigitalConfig, bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn is_ready(&mut self, self_: wasmtime::component::Resource<StatefulDigitalOutPin>) -> bool {
        match self.table.get(&self_) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn set_state(
        &mut self,
        self_: wasmtime::component::Resource<StatefulDigitalOutPin>,
        state: bindings::wasi::gpio::digital::PinState,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn set_active(
        &mut self,
        self_: wasmtime::component::Resource<StatefulDigitalOutPin>,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn set_inactive(
        &mut self,
        self_: wasmtime::component::Resource<StatefulDigitalOutPin>,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn toggle(
        &mut self,
        self_: wasmtime::component::Resource<StatefulDigitalOutPin>,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn is_set_active(
        &mut self,
        self_: wasmtime::component::Resource<StatefulDigitalOutPin>,
    ) -> Result<bool, bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn is_set_inactive(
        &mut self,
        self_: wasmtime::component::Resource<StatefulDigitalOutPin>,
    ) -> Result<bool, bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn get_state(
        &mut self,
        self_: wasmtime::component::Resource<StatefulDigitalOutPin>,
    ) -> Result<bindings::wasi::gpio::digital::PinState, bindings::wasi::gpio::general::GpioError>
    {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<StatefulDigitalOutPin>,
    ) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}

impl bindings::wasi::gpio::analog::Host for HostComponent {}

impl bindings::wasi::gpio::analog::HostAnalogOutPin for HostComponent {
    fn get(
        &mut self,
        pin_label: wasmtime::component::__internal::String,
        flags: wasmtime::component::__internal::Vec<bindings::wasi::gpio::analog::AnalogFlag>,
    ) -> Result<wasmtime::component::Resource<AnalogOutPin>, bindings::wasi::gpio::general::GpioError>
    {
        todo!()
    }

    fn get_config(
        &mut self,
        self_: wasmtime::component::Resource<AnalogOutPin>,
    ) -> Result<bindings::wasi::gpio::analog::AnalogConfig, bindings::wasi::gpio::general::GpioError>
    {
        todo!()
    }

    fn is_ready(&mut self, self_: wasmtime::component::Resource<AnalogOutPin>) -> bool {
        match self.table.get(&self_) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn set_value_raw(
        &mut self,
        self_: wasmtime::component::Resource<AnalogOutPin>,
        value: u32,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        todo!()
    }

    fn set_value(
        &mut self,
        self_: wasmtime::component::Resource<AnalogOutPin>,
        value: f32,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        todo!()
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<AnalogOutPin>) -> wasmtime::Result<()> {
        todo!()
    }
}

impl bindings::wasi::gpio::analog::HostAnalogInOutPin for HostComponent {
    fn get(
        &mut self,
        pin_label: wasmtime::component::__internal::String,
        flags: wasmtime::component::__internal::Vec<bindings::wasi::gpio::analog::AnalogFlag>,
    ) -> Result<
        wasmtime::component::Resource<AnalogInOutPin>,
        bindings::wasi::gpio::general::GpioError,
    > {
        todo!()
    }

    fn get_config(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInOutPin>,
    ) -> Result<bindings::wasi::gpio::analog::AnalogConfig, bindings::wasi::gpio::general::GpioError>
    {
        todo!()
    }

    fn is_ready(&mut self, self_: wasmtime::component::Resource<AnalogInOutPin>) -> bool {
        match self.table.get(&self_) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn set_value_raw(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInOutPin>,
        value: u32,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        todo!()
    }

    fn set_value(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInOutPin>,
        value: f32,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        todo!()
    }

    fn read_raw(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInOutPin>,
    ) -> Result<u32, bindings::wasi::gpio::general::GpioError> {
        todo!()
    }

    fn read(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInOutPin>,
    ) -> Result<f32, bindings::wasi::gpio::general::GpioError> {
        todo!()
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<AnalogInOutPin>) -> wasmtime::Result<()> {
        todo!()
    }
    
    #[doc = " Sets the pin mode"]
    fn set_pin_mode(&mut self,self_:wasmtime::component::Resource<AnalogInOutPin>,pin_mode:bindings::wasi::gpio::general::PinMode,) -> Result<(),bindings::wasi::gpio::general::GpioError> {
        todo!()
    }
}

impl bindings::wasi::gpio::analog::HostAnalogInPin for HostComponent {
    fn get(
        &mut self,
        pin_label: wasmtime::component::__internal::String,
        flags: wasmtime::component::__internal::Vec<bindings::wasi::gpio::analog::AnalogFlag>,
    ) -> Result<wasmtime::component::Resource<AnalogInPin>, bindings::wasi::gpio::general::GpioError>
    {
        todo!()
    }

    fn get_config(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInPin>,
    ) -> Result<bindings::wasi::gpio::analog::AnalogConfig, bindings::wasi::gpio::general::GpioError>
    {
        todo!()
    }

    fn is_ready(&mut self, self_: wasmtime::component::Resource<AnalogInPin>) -> bool {
        match self.table.get(&self_) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn read_raw(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInPin>,
    ) -> Result<u32, bindings::wasi::gpio::general::GpioError> {
        todo!()
    }

    fn read(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInPin>,
    ) -> Result<f32, bindings::wasi::gpio::general::GpioError> {
        todo!()
    }

    fn watch_above_raw(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInPin>,
        value: u32,
    ) -> Result<wasmtime::component::Resource<Pollable>, bindings::wasi::gpio::general::GpioError>
    {
        todo!()
    }

    fn watch_above(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInPin>,
        value: f32,
    ) -> Result<wasmtime::component::Resource<Pollable>, bindings::wasi::gpio::general::GpioError>
    {
        todo!()
    }

    fn watch_below_raw(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInPin>,
        value: u32,
    ) -> Result<wasmtime::component::Resource<Pollable>, bindings::wasi::gpio::general::GpioError>
    {
        todo!()
    }

    fn watch_below(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInPin>,
        value: f32,
    ) -> Result<wasmtime::component::Resource<Pollable>, bindings::wasi::gpio::general::GpioError>
    {
        todo!()
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<AnalogInPin>) -> wasmtime::Result<()> {
        todo!()
    }
}

impl bindings::wasi::gpio::delay::Host for HostComponent {}

impl bindings::wasi::gpio::delay::HostDelay for HostComponent {
    fn delay_ns(
        &mut self,
        self_: wasmtime::component::Resource<bindings::wasi::gpio::delay::Delay>,
        ns: u64,
    ) -> () {
        std::thread::sleep(std::time::Duration::from_nanos(ns));
    }

    fn delay_us(
        &mut self,
        self_: wasmtime::component::Resource<bindings::wasi::gpio::delay::Delay>,
        us: u64,
    ) -> () {
        std::thread::sleep(std::time::Duration::from_micros(us));
    }

    fn delay_ms(
        &mut self,
        self_: wasmtime::component::Resource<bindings::wasi::gpio::delay::Delay>,
        ms: u64,
    ) -> () {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }

    fn drop(
        &mut self,
        rep: wasmtime::component::Resource<bindings::wasi::gpio::delay::Delay>,
    ) -> wasmtime::Result<()> {
        todo!()
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
    
    let mock_gpio : Box<dyn host_component::GpioLike + Send> = Box::new(host_component::MockGpio);
    let real_gpio : Box<dyn host_component::GpioLike + Send> = Box::new(rppal::gpio::Gpio::new().unwrap());
    
    let mut state = State {
        ctx: wasmtime_wasi::WasiCtxBuilder::new().inherit_stdio().build(),
        host: HostComponent::new(policies, real_gpio)
    };
    
    let delay = state.host.table.push(Delay{}).unwrap();
    
    let mut store = wasmtime::Store::new(&engine, state);
    
    let instance = bindings::Rpi::instantiate(&mut store, &component, &linker).unwrap();
    
    instance.call_start(&mut store, delay).unwrap();
}
