use super::super::bindings;
use super::{AnalogConfigBuilder, AnalogOutPin};

impl AnalogConfigBuilder {
    pub fn new(label: String, pin_mode: bindings::wasi::gpio::general::PinMode) -> Self {
        Self {
            label,
            pin_mode,
            output_mode: None,
        }
    }

    pub fn add_flags(
        mut self,
        flags: wasmtime::component::__internal::Vec<bindings::wasi::gpio::analog::AnalogFlag>,
    ) -> Self {
        for flag in flags {
            if flag == bindings::wasi::gpio::analog::AnalogFlag::PWM {
                self.output_mode = Some(bindings::wasi::gpio::analog::OutputMode::Pwm)
            }
        }

        self
    }

    pub fn build(
        self,
    ) -> Result<bindings::wasi::gpio::analog::AnalogConfig, bindings::wasi::gpio::general::GpioError>
    {
        match self.pin_mode {
            bindings::wasi::gpio::general::PinMode::Out => {
                if self.output_mode.is_none() {
                    return Err(bindings::wasi::gpio::general::GpioError::InvalidFlag);
                }
            }
            bindings::wasi::gpio::general::PinMode::In => {
                return Err(bindings::wasi::gpio::general::GpioError::PinModeNotAvailable)
            }
        }

        Ok(bindings::wasi::gpio::analog::AnalogConfig {
            label: self.label,
            pin_mode: self.pin_mode,
            output_mode: self.output_mode,
        })
    }
}

impl AnalogOutPin {
    pub fn new(
        pin: rppal::gpio::OutputPin,
        config: bindings::wasi::gpio::analog::AnalogConfig,
    ) -> Self {
        Self { pin, config }
    }

    pub fn get_config(&self) -> bindings::wasi::gpio::analog::AnalogConfig {
        self.config.clone()
    }

    pub fn set_value(&mut self, value: f32) -> Result<(), String> {
        self.pin
            .set_pwm_frequency(1000., value as f64)
            .map_err(|err| err.to_string())
    }
}

pub fn check_invalid_flags(
    flags: &Vec<bindings::wasi::gpio::analog::AnalogFlag>,
    disallowed_flags: Vec<bindings::wasi::gpio::analog::AnalogFlag>,
) -> Result<(), ()> {
    for flag in flags {
        if disallowed_flags.contains(flag) {
            return Err(());
        }
    }

    Ok(())
}
