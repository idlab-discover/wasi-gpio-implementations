use std::thread::panicking;

use super::bindings;
use super::digital::DigitalInPin;
use super::util::Shared;
use super::watch_event::WatchType;
use super::policies;
use super::watch_event;

pub trait GpioLike {
    fn get(&self, pin_nr: u8) -> rppal::gpio::Result<rppal::gpio::Pin>;
}

pub struct MockGpio;

impl GpioLike for MockGpio {
    fn get(&self, pin_nr: u8) -> rppal::gpio::Result<rppal::gpio::Pin> {
        rppal::gpio::Result::Err(rppal::gpio::Error::UnknownModel)
    }
}

impl GpioLike for rppal::gpio::Gpio {
    fn get(&self, pin_nr: u8) -> rppal::gpio::Result<rppal::gpio::Pin> {
        self.get(pin_nr)
    }
}

pub struct HostComponent {
    pub policies: policies::Policies,
    gpio: rppal::gpio::Gpio,
    pub table: wasmtime::component::ResourceTable,
    pub watcher: watch_event::Watcher,
    pi_type: u8
}

impl HostComponent {
    pub fn new(policies: policies::Policies, gpio: rppal::gpio::Gpio, pi_type: u8) -> Self {
        Self {
            policies,
            gpio,
            table: wasmtime::component::ResourceTable::new(),
            watcher: watch_event::Watcher::new(),
            pi_type
        }
    }

    pub fn label_to_u8(label: &str) -> Option<u8> {
        label
            .strip_prefix("GPIO")
            .and_then(|num| num.parse::<u8>().ok())
    }

    pub fn get_pin(&self, vlabel: &str, mode: policies::Mode) {
        match mode {
            policies::Mode::DigitalInput | policies::Mode::DigitalOutput | policies::Mode::DigitalInputOutput => todo!(),
            policies::Mode::AnalogOutput => todo!(),
            _ => todo!()
        }
    }

    
    pub fn get_digital_pin(
        &self,
        vlabel: &str,
    ) -> Result<rppal::gpio::Pin, bindings::wasi::gpio::general::GpioError> {
        let pin_nr = self.policies.get_plabel(vlabel)
        .and_then(|plabel| Self::label_to_u8(&plabel))
        .ok_or(bindings::wasi::gpio::general::GpioError::UndefinedPinLabel)?;

        self.gpio.get(pin_nr).map_err(|_| bindings::wasi::gpio::general::GpioError::AlreadyInUse)
    }

    pub fn get_analog_pin(&self, vlabel: &str) -> Result<rppal::pwm::Pwm, bindings::wasi::gpio::general::GpioError> {
        let plabel = self.policies.get_plabel(vlabel).ok_or(bindings::wasi::gpio::general::GpioError::UndefinedPinLabel)?;

        let pwm_channel = if self.pi_type < 5 {
            match plabel.to_lowercase().as_str() {
                "gpio12" | "gpio18" => rppal::pwm::Channel::Pwm0,
                "gpio13" | "gpio19" => rppal::pwm::Channel::Pwm1,
                _ => return Err(bindings::wasi::gpio::general::GpioError::PinModeNotAvailable)
            }
        } else {
            match plabel.to_lowercase().as_str() {
                "gpio12" => rppal::pwm::Channel::Pwm0,
                "gpio13" => rppal::pwm::Channel::Pwm1,
                "gpio18" => rppal::pwm::Channel::Pwm2,
                "gpio19" => rppal::pwm::Channel::Pwm3,
                _ => return Err(bindings::wasi::gpio::general::GpioError::PinModeNotAvailable)
            }
        };

        rppal::pwm::Pwm::new(pwm_channel).map_err(|e| bindings::wasi::gpio::general::GpioError::Other(e.to_string()))
    }

    pub fn get_pin_resource<T: std::any::Any + Sized>(&self, key: &wasmtime::component::Resource<T>) -> Result<&T, bindings::wasi::gpio::general::GpioError> {
        self.table.get(&key).map_err(|_| bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }
}
