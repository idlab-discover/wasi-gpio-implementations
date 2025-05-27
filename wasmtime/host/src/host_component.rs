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
    gpio: Box<dyn GpioLike + Send>,
    pub table: wasmtime::component::ResourceTable,
    pub watcher: watch_event::Watcher,
}

impl HostComponent {
    pub fn new(policies: policies::Policies, gpio: Box<dyn GpioLike + Send>) -> Self {
        Self {
            policies,
            gpio,
            table: wasmtime::component::ResourceTable::new(),
            watcher: watch_event::Watcher::new(),
        }
    }

    pub fn label_to_u8(label: &str) -> Option<u8> {
        label
            .strip_prefix("GPIO")
            .and_then(|num| num.parse::<u8>().ok())
    }

    
    pub fn get_pin(
        &self,
        vlabel: &str,
    ) -> Result<rppal::gpio::Pin, bindings::wasi::gpio::general::GpioError> {
        let plabel = match self.policies.get_plabel(vlabel) {
            Some(plabel) => plabel,
            None => return Err(bindings::wasi::gpio::general::GpioError::UndefinedPinLabel),
        };
        
        let pin_nr = match Self::label_to_u8(&plabel) {
            Some(pin_nr) => pin_nr,
            None => {
                return Err(bindings::wasi::gpio::general::GpioError::UndefinedPinLabel);
            }
        };

        match self.gpio.get(pin_nr) {
            Ok(pin) => Ok(pin),
            Err(_) => {
                return Err(bindings::wasi::gpio::general::GpioError::AlreadyInUse);
            }
        }
    }

    pub fn get_pin_resource<T: std::any::Any + Sized>(&self, key: &wasmtime::component::Resource<T>) -> Result<&T, bindings::wasi::gpio::general::GpioError> {
        self.table.get(&key).map_err(|_| bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }
}
