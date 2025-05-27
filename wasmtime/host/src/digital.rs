use std::env::var;

use super::bindings;
use super::util::Shared;
use super::watch_event;

pub struct DigitalOutPin {
    pin: rppal::gpio::OutputPin,
    config: bindings::wasi::gpio::digital::DigitalConfig,
}

impl DigitalOutPin {
    pub fn new(
        pin: rppal::gpio::Pin,
        config: bindings::wasi::gpio::digital::DigitalConfig,
    ) -> Self {
        Self {
            pin: pin.into_output(),
            config,
        }
    }

    pub fn get_config(&self) -> &bindings::wasi::gpio::digital::DigitalConfig {
        &self.config
    }

    pub fn write(&mut self, pin_state: bindings::wasi::gpio::digital::PinState) {
        let pin_state = match &self.config.active_level {
            bindings::wasi::gpio::general::ActiveLevel::ActiveHigh => pin_state,
            bindings::wasi::gpio::general::ActiveLevel::ActiveLow => !pin_state,
        };

        let pin_state: rppal::gpio::Level = pin_state.into();

        self.pin.write(pin_state);
    }
}

impl From<bindings::wasi::gpio::digital::PinState> for rppal::gpio::Level {
    fn from(value: bindings::wasi::gpio::digital::PinState) -> Self {
        match value {
            bindings::wasi::gpio::digital::PinState::Active => Self::High,
            bindings::wasi::gpio::digital::PinState::Inactive => Self::Low,
        }
    }
}

pub struct DigitalInPin {
    pin: Shared<rppal::gpio::InputPin>,
    config: bindings::wasi::gpio::digital::DigitalConfig,
}

impl DigitalInPin {
    pub fn new(
        pin: rppal::gpio::Pin,
        config: bindings::wasi::gpio::digital::DigitalConfig,
    ) -> Self {
        let pin = match &config.pull_resistor {
            Some(bindings::wasi::gpio::general::PullResistor::PullDown) => {
                pin.into_input_pulldown()
            }
            Some(bindings::wasi::gpio::general::PullResistor::PullUp) => pin.into_input_pullup(),
            None => pin.into_input(),
        };

        Self {
            pin: std::sync::Arc::new(std::sync::Mutex::new(pin)),
            config,
        }
    }

    pub fn get_config(&self) -> &bindings::wasi::gpio::digital::DigitalConfig {
        &self.config
    }

    pub fn read(&self) -> bindings::wasi::gpio::digital::PinState {
        let pin_state = match (*self.pin.lock().unwrap()).read() {
            rppal::gpio::Level::Low => bindings::wasi::gpio::digital::PinState::Inactive,
            rppal::gpio::Level::High => bindings::wasi::gpio::digital::PinState::Active,
        };

        match self.config.active_level {
            bindings::wasi::gpio::general::ActiveLevel::ActiveHigh => pin_state,
            bindings::wasi::gpio::general::ActiveLevel::ActiveLow => !pin_state,
        }
    }

    pub fn clone_pin(&self) -> Shared<rppal::gpio::InputPin> {
        self.pin.clone()
    }
}

pub struct DigitalInOutPin {
    pin: Shared<rppal::gpio::IoPin>,
    config: bindings::wasi::gpio::digital::DigitalConfig,
}

impl DigitalInOutPin {
    pub fn new(
        pin: rppal::gpio::Pin,
        config: bindings::wasi::gpio::digital::DigitalConfig,
    ) -> Self {
        Self {
            pin: std::sync::Arc::new(std::sync::Mutex::new(pin.into_io(rppal::gpio::Mode::Input))),
            config,
        }
    }

    pub fn get_config(&self) -> &bindings::wasi::gpio::digital::DigitalConfig {
        &self.config
    }

    pub fn write(&mut self, pin_state: bindings::wasi::gpio::digital::PinState) {
        let pin_state = match &self.config.active_level {
            bindings::wasi::gpio::general::ActiveLevel::ActiveHigh => pin_state,
            bindings::wasi::gpio::general::ActiveLevel::ActiveLow => !pin_state,
        };

        let pin_state: rppal::gpio::Level = pin_state.into();

        (*self.pin.lock().unwrap()).write(pin_state);
    }

    pub fn read(&self) -> bindings::wasi::gpio::digital::PinState {
        let pin_state = match (*self.pin.lock().unwrap()).read() {
            rppal::gpio::Level::Low => bindings::wasi::gpio::digital::PinState::Inactive,
            rppal::gpio::Level::High => bindings::wasi::gpio::digital::PinState::Active,
        };

        match self.config.active_level {
            bindings::wasi::gpio::general::ActiveLevel::ActiveHigh => pin_state,
            bindings::wasi::gpio::general::ActiveLevel::ActiveLow => !pin_state,
        }
    }
}

pub struct StatefulDigitalOutPin {}

impl From<rppal::gpio::Level> for bindings::wasi::gpio::digital::PinState {
    fn from(value: rppal::gpio::Level) -> Self {
        match value {
            rppal::gpio::Level::Low => Self::Inactive,
            rppal::gpio::Level::High => Self::Active,
        }
    }
}

impl std::ops::Not for bindings::wasi::gpio::digital::PinState {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            bindings::wasi::gpio::digital::PinState::Active => Self::Inactive,
            bindings::wasi::gpio::digital::PinState::Inactive => Self::Active,
        }
    }
}

pub struct DigitalConfigBuilder {
    label: String,
    pin_mode: Option<bindings::wasi::gpio::general::PinMode>,
    active_level: Option<bindings::wasi::gpio::general::ActiveLevel>,
    pull_resistor: Option<bindings::wasi::gpio::general::PullResistor>,
}

impl DigitalConfigBuilder {
    pub fn new(label: String) -> Self {
        Self {
            label,
            pin_mode: None,
            active_level: None,
            pull_resistor: None,
        }
    }

    fn add_active_level(&mut self, active_level: bindings::wasi::gpio::general::ActiveLevel) {
        self.active_level = Some(active_level);
    }

    fn add_pull_resistor(&mut self, pull_resistor: bindings::wasi::gpio::general::PullResistor) {
        self.pull_resistor = Some(pull_resistor)
    }

    fn add_pin_mode(&mut self, pin_mode: bindings::wasi::gpio::general::PinMode) {
        self.pin_mode = Some(pin_mode)
    }

    pub fn add_flags(
        &mut self,
        flags: wasmtime::component::__internal::Vec<bindings::wasi::gpio::digital::DigitalFlag>,
    ) {
        for flag in flags {
            if flag == bindings::wasi::gpio::digital::DigitalFlag::ACTIVE_HIGH {
                self.add_active_level(bindings::wasi::gpio::general::ActiveLevel::ActiveHigh);
            } else if flag == bindings::wasi::gpio::digital::DigitalFlag::ACTIVE_LOW {
                self.add_active_level(bindings::wasi::gpio::general::ActiveLevel::ActiveLow);
            } else if flag == bindings::wasi::gpio::digital::DigitalFlag::PULL_UP {
                self.add_pull_resistor(bindings::wasi::gpio::general::PullResistor::PullUp);
            } else if flag == bindings::wasi::gpio::digital::DigitalFlag::PULL_DOWN {
                self.add_pull_resistor(bindings::wasi::gpio::general::PullResistor::PullDown);
            } else if flag == bindings::wasi::gpio::digital::DigitalFlag::INPUT {
                self.add_pin_mode(bindings::wasi::gpio::general::PinMode::In);
            } else if flag == bindings::wasi::gpio::digital::DigitalFlag::OUTPUT {
                self.add_pin_mode(bindings::wasi::gpio::general::PinMode::Out);
            }
        }
    }

    pub fn build(&self) -> Result<bindings::wasi::gpio::digital::DigitalConfig, ()> {
        if self.pin_mode.is_none() {
            return Err(());
        }
        if self.active_level.is_none() {
            return Err(());
        }

        Ok(bindings::wasi::gpio::digital::DigitalConfig {
            label: self.label.clone(),
            pin_mode: self.pin_mode.unwrap(),
            active_level: self.active_level.unwrap(),
            pull_resistor: self.pull_resistor,
        })
    }
}