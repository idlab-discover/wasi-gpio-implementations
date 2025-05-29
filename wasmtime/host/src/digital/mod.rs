use super::{bindings};
pub mod implementation;
use super::util;
use super::host_component;
use super::policies;
use super::poll;
use super::watch_event;

pub struct DigitalConfigBuilder {
    label: String,
    pin_mode: bindings::wasi::gpio::general::PinMode,
    active_level: Option<bindings::wasi::gpio::general::ActiveLevel>,
    pull_resistor: Option<bindings::wasi::gpio::general::PullResistor>,
}

impl bindings::wasi::gpio::digital::Host for host_component::HostComponent {}

pub struct DigitalInPin {
    pin: util::Shared<rppal::gpio::InputPin>,
    config: bindings::wasi::gpio::digital::DigitalConfig,
}

impl bindings::wasi::gpio::digital::HostDigitalInPin for host_component::HostComponent {
    fn get(
        &mut self,
        pin_label: wasmtime::component::__internal::String,
        mut flags: wasmtime::component::__internal::Vec<bindings::wasi::gpio::digital::DigitalFlag>,
    ) -> Result<wasmtime::component::Resource<DigitalInPin>, bindings::wasi::gpio::general::GpioError> {
        if !self.policies.is_mode_allowed(&pin_label, policies::Mode::DigitalInput) {
            return Err(bindings::wasi::gpio::general::GpioError::PinModeNotAllowed);
        }

        for flag in flags.iter() {
            if *flag == bindings::wasi::gpio::digital::DigitalFlag::ACTIVE || *flag == bindings::wasi::gpio::digital::DigitalFlag::INACTIVE || *flag == bindings::wasi::gpio::digital::DigitalFlag::OUTPUT {
                return Err(bindings::wasi::gpio::general::GpioError::InvalidFlag);
            }
        }

        let pin = self.get_digital_pin(&pin_label)?;

        let config = DigitalConfigBuilder::new(pin_label, bindings::wasi::gpio::general::PinMode::In).add_flags(flags).build().map_err(|_| bindings::wasi::gpio::general::GpioError::InvalidFlag)?;

        self.table.push(DigitalInPin::new(pin, config)).map_err(|err| bindings::wasi::gpio::general::GpioError::Other(err.to_string()))
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
    ) -> Result<wasmtime::component::Resource<poll::Pollable>, bindings::wasi::gpio::general::GpioError> {
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

        self.table.push(poll::Pollable::new(trigger)).map_err(|err| bindings::wasi::gpio::general::GpioError::Other(err.to_string()))
    }

    fn watch_active(
        &mut self,
        self_: wasmtime::component::Resource<DigitalInPin>,
    ) -> Result<wasmtime::component::Resource<poll::Pollable>, bindings::wasi::gpio::general::GpioError>
    {
        self.watch_state(self_, bindings::wasi::gpio::digital::PinState::Active)
    }

    fn watch_inactive(
        &mut self,
        self_: wasmtime::component::Resource<DigitalInPin>,
    ) -> Result<wasmtime::component::Resource<poll::Pollable>, bindings::wasi::gpio::general::GpioError>
    {
        self.watch_state(self_, bindings::wasi::gpio::digital::PinState::Inactive)
    }

    fn watch_falling_edge(
        &mut self,
        self_: wasmtime::component::Resource<DigitalInPin>,
    ) -> Result<wasmtime::component::Resource<poll::Pollable>, bindings::wasi::gpio::general::GpioError>
    {
        let pin = self.table.get(&self_).map_err(|_| bindings::wasi::gpio::general::GpioError::ResourceInvalidated)?;

        let watch_event = match &pin.get_config().active_level {
            bindings::wasi::gpio::general::ActiveLevel::ActiveHigh => watch_event::WatchType::Falling,
            bindings::wasi::gpio::general::ActiveLevel::ActiveLow => watch_event::WatchType::Rising,
        };
        
        let trigger = self.watcher.watch_event(pin, watch_event);

        self.table.push(poll::Pollable::new(trigger)).map_err(|err| bindings::wasi::gpio::general::GpioError::Other(err.to_string()))
    }

    fn watch_rising_edge(
        &mut self,
        self_: wasmtime::component::Resource<DigitalInPin>,
    ) -> Result<wasmtime::component::Resource<poll::Pollable>, bindings::wasi::gpio::general::GpioError>
    {
        let pin = self.table.get(&self_).map_err(|_| bindings::wasi::gpio::general::GpioError::ResourceInvalidated)?;

        let watch_event = match &pin.get_config().active_level {
            bindings::wasi::gpio::general::ActiveLevel::ActiveHigh => watch_event::WatchType::Rising,
            bindings::wasi::gpio::general::ActiveLevel::ActiveLow => watch_event::WatchType::Falling,
        };
        
        let trigger = self.watcher.watch_event(pin, watch_event);

        self.table.push(poll::Pollable::new(trigger)).map_err(|err| bindings::wasi::gpio::general::GpioError::Other(err.to_string()))
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<DigitalInPin>) -> wasmtime::Result<()> {
        self.table.delete(rep).expect("failed to delete resource");
        Ok(())
    }
}

pub struct DigitalOutPin {
    pin: rppal::gpio::OutputPin,
    config: bindings::wasi::gpio::digital::DigitalConfig,
}

impl bindings::wasi::gpio::digital::HostDigitalOutPin for host_component::HostComponent {
    fn get(
        &mut self,
        pin_label: wasmtime::component::__internal::String,
        mut flags: wasmtime::component::__internal::Vec<bindings::wasi::gpio::digital::DigitalFlag>,
    ) -> Result<wasmtime::component::Resource<DigitalOutPin>, bindings::wasi::gpio::general::GpioError> {
        if !self.policies.is_mode_allowed(&pin_label, policies::Mode::DigitalInput) {
            return Err(bindings::wasi::gpio::general::GpioError::PinModeNotAllowed);
        }

        let mut pin_state = None;
        for flag in flags.iter() {
            if *flag == bindings::wasi::gpio::digital::DigitalFlag::INPUT || *flag == bindings::wasi::gpio::digital::DigitalFlag::PULL_UP || *flag == bindings::wasi::gpio::digital::DigitalFlag::PULL_DOWN {
                return Err(bindings::wasi::gpio::general::GpioError::InvalidFlag);
            } else if *flag == bindings::wasi::gpio::digital::DigitalFlag::ACTIVE {
                pin_state = Some(bindings::wasi::gpio::digital::PinState::Active);
            } else if *flag == bindings::wasi::gpio::digital::DigitalFlag::INACTIVE {
                pin_state = Some(bindings::wasi::gpio::digital::PinState::Active);
            }
        }

        let pin = self.get_digital_pin(&pin_label)?;

        let config = DigitalConfigBuilder::new(pin_label, bindings::wasi::gpio::digital::PinMode::Out).add_flags(flags).build().map_err(|_| bindings::wasi::gpio::general::GpioError::InvalidFlag)?;

        self.table.push(DigitalOutPin::new(pin, config, pin_state)).map_err(|err| bindings::wasi::gpio::general::GpioError::Other(err.to_string()))
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
        self.table.delete(rep).expect("failed to delete resource");
        Ok(())
    }
}

pub struct DigitalInOutPin {
    pin: rppal::gpio::IoPin,
    config: bindings::wasi::gpio::digital::DigitalConfig,
}


impl bindings::wasi::gpio::digital::HostDigitalInOutPin for host_component::HostComponent {
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
        self.table.delete(rep).expect("failed to delete resource");
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

        let pin = self.get_digital_pin(&pin_label)?;

        let mut pin_mode = None;

        for flag in flags.iter() {
            if *flag == bindings::wasi::gpio::digital::DigitalFlag::INPUT {
                match pin_mode {
                    Some(_) => return Err(bindings::wasi::gpio::general::GpioError::InvalidFlag),
                    None => pin_mode = Some(bindings::wasi::gpio::digital::PinMode::In),
                }
            } else if *flag == bindings::wasi::gpio::digital::DigitalFlag::OUTPUT {
                match pin_mode {
                    Some(_) => return Err(bindings::wasi::gpio::general::GpioError::InvalidFlag),
                    None => pin_mode = Some(bindings::wasi::gpio::digital::PinMode::Out),
                }
            } else if *flag == bindings::wasi::gpio::digital::DigitalFlag::PULL_UP || *flag == bindings::wasi::gpio::digital::DigitalFlag::PULL_DOWN || *flag == bindings::wasi::gpio::digital::DigitalFlag::ACTIVE || *flag == bindings::wasi::gpio::digital::DigitalFlag::INACTIVE {
                return Err(bindings::wasi::gpio::general::GpioError::InvalidFlag);
            }
        }

        let pin_mode = match pin_mode {
            Some(pin_mode) => pin_mode,
            None => return Err(bindings::wasi::gpio::general::GpioError::InvalidFlag),
        };

        let config = DigitalConfigBuilder::new(pin_label, pin_mode).add_flags(flags).build().map_err(|_| bindings::wasi::gpio::general::GpioError::InvalidFlag)?;

        self.table.push(DigitalInOutPin::new(pin, config, pin_mode)).map_err(|err| bindings::wasi::gpio::general::GpioError::Other(err.to_string()))
    }
    
    fn set_pin_mode(&mut self,self_:wasmtime::component::Resource<DigitalInOutPin>,pin_mode:bindings::wasi::gpio::general::PinMode,) -> Result<(),bindings::wasi::gpio::general::GpioError> {
        Ok(self.table.get_mut(&self_).map_err(|_| bindings::wasi::gpio::general::GpioError::ResourceInvalidated)?.set_pin_mode(pin_mode))
    }
}

pub struct StatefulDigitalOutPin {}

impl bindings::wasi::gpio::digital::HostStatefulDigitalOutPin for host_component::HostComponent {
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
        self.table.delete(rep).expect("failed to delete resource");
        Ok(())
    }
}