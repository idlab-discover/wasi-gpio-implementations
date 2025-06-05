use super::{bindings, policies, watch_event};

pub struct HostComponent {
    pub policies: policies::Policies,
    gpio: rppal::gpio::Gpio,
    pub table: wasmtime::component::ResourceTable,
    pub watcher: watch_event::Watcher,
}

impl HostComponent {
    pub fn new(policies: policies::Policies, gpio: rppal::gpio::Gpio) -> Self {
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
        let pin_nr = self
            .policies
            .get_plabel(vlabel)
            .and_then(|plabel| Self::label_to_u8(&plabel))
            .ok_or(bindings::wasi::gpio::general::GpioError::UndefinedPinLabel)?;

        self.gpio
            .get(pin_nr)
            .map_err(|_| bindings::wasi::gpio::general::GpioError::AlreadyInUse)
    }
}
