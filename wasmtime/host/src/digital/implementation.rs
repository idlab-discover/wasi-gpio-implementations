use super::bindings;
use super::super::Shared;

use super::{DigitalOutPin, DigitalInPin, DigitalInOutPin};

impl DigitalOutPin {
    pub fn new(
        pin: rppal::gpio::Pin,
        config: bindings::wasi::gpio::digital::DigitalConfig,
        pin_state: Option<bindings::wasi::gpio::digital::PinState>
    ) -> Self {
        let pin = if let None = pin_state {
            pin.into_output()
        } else if let Some(bindings::wasi::gpio::digital::PinState::Inactive) = pin_state {
            match &config.active_level {
                bindings::wasi::gpio::general::ActiveLevel::ActiveHigh => pin.into_output_low(),
                bindings::wasi::gpio::general::ActiveLevel::ActiveLow => pin.into_output_high(),
            }
        } else {
            match &config.active_level {
                bindings::wasi::gpio::general::ActiveLevel::ActiveHigh => pin.into_output_high(),
                bindings::wasi::gpio::general::ActiveLevel::ActiveLow => pin.into_output_low(),
            }
        };

        Self {
            pin,
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

impl DigitalInOutPin {
    pub fn new(
        pin: rppal::gpio::Pin,
        config: bindings::wasi::gpio::digital::DigitalConfig,
        pin_mode: bindings::wasi::gpio::digital::PinMode
    ) -> Self {
        match pin_mode {
            bindings::wasi::gpio::general::PinMode::In => Self { pin: pin.into_io(rppal::gpio::Mode::Input), config },
            bindings::wasi::gpio::general::PinMode::Out => Self { pin: pin.into_io(rppal::gpio::Mode::Output), config },
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

    pub fn read(&self) -> bindings::wasi::gpio::digital::PinState {
        let pin_state = match self.pin.read() {
            rppal::gpio::Level::Low => bindings::wasi::gpio::digital::PinState::Inactive,
            rppal::gpio::Level::High => bindings::wasi::gpio::digital::PinState::Active,
        };

        match self.config.active_level {
            bindings::wasi::gpio::general::ActiveLevel::ActiveHigh => pin_state,
            bindings::wasi::gpio::general::ActiveLevel::ActiveLow => !pin_state,
        }
    }

    pub fn set_pin_mode(&mut self, mode: bindings::wasi::gpio::general::PinMode) {
        self.pin.set_mode(mode.into());
    }
}

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

impl From<bindings::wasi::gpio::general::PinMode> for rppal::gpio::Mode {
    fn from(value: bindings::wasi::gpio::general::PinMode) -> Self {
        match value {
            bindings::wasi::gpio::general::PinMode::In => Self::Input,
            bindings::wasi::gpio::general::PinMode::Out => Self::Output,
        }
    }
}

use super::DigitalConfigBuilder;

impl DigitalConfigBuilder {
    pub fn new(label: String, pin_mode: bindings::wasi::gpio::general::PinMode) -> Self {
        Self {
            label,
            pin_mode,
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

    pub fn add_flags(
        mut self,
        flags: wasmtime::component::__internal::Vec<bindings::wasi::gpio::digital::DigitalFlag>,
    ) -> Self {
        for flag in flags {
            if flag == bindings::wasi::gpio::digital::DigitalFlag::ACTIVE_HIGH {
                self.add_active_level(bindings::wasi::gpio::general::ActiveLevel::ActiveHigh);
            } else if flag == bindings::wasi::gpio::digital::DigitalFlag::ACTIVE_LOW {
                self.add_active_level(bindings::wasi::gpio::general::ActiveLevel::ActiveLow);
            } else if flag == bindings::wasi::gpio::digital::DigitalFlag::PULL_UP {
                self.add_pull_resistor(bindings::wasi::gpio::general::PullResistor::PullUp);
            } else if flag == bindings::wasi::gpio::digital::DigitalFlag::PULL_DOWN {
                self.add_pull_resistor(bindings::wasi::gpio::general::PullResistor::PullDown);
            }
        }

        self
    }

    pub fn build(self) -> Result<bindings::wasi::gpio::digital::DigitalConfig, bindings::wasi::gpio::general::GpioError> {
        let active_level = match self.active_level {
            Some(active_level) => active_level,
            None => return Err(bindings::wasi::gpio::general::GpioError::InvalidFlag),
        };

        match self.pin_mode {
            bindings::wasi::gpio::general::PinMode::In => {},
            bindings::wasi::gpio::general::PinMode::Out => {
                if self.pull_resistor.is_some() { return Err(bindings::wasi::gpio::general::GpioError::InvalidFlag)}
            },
        }

        Ok(bindings::wasi::gpio::digital::DigitalConfig {
            label: self.label.clone(),
            pin_mode: self.pin_mode,
            active_level,
            pull_resistor: self.pull_resistor,
        })
    }
}