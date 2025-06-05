pub mod implementations;
use super::{bindings, host_component, policies, Pollable};

macro_rules! PWM_MAX {
    () => {
        (1 << 12) - 1
    };
}

pub struct AnalogConfigBuilder {
    label: String,
    pin_mode: bindings::wasi::gpio::general::PinMode,
    output_mode: Option<bindings::wasi::gpio::analog::OutputMode>,
}

pub struct AnalogInPin {}

pub struct AnalogInOutPin {}

impl bindings::wasi::gpio::analog::Host for host_component::HostComponent {}

pub struct AnalogOutPin {
    pin: rppal::gpio::OutputPin,
    config: bindings::wasi::gpio::analog::AnalogConfig,
}

impl bindings::wasi::gpio::analog::HostAnalogOutPin for host_component::HostComponent {
    fn get(
        &mut self,
        pin_label: wasmtime::component::__internal::String,
        flags: wasmtime::component::__internal::Vec<bindings::wasi::gpio::analog::AnalogFlag>,
    ) -> Result<wasmtime::component::Resource<AnalogOutPin>, bindings::wasi::gpio::general::GpioError>
    {
        if !self
            .policies
            .is_mode_allowed(&pin_label, policies::Mode::AnalogOutput)
        {
            return Err(bindings::wasi::gpio::general::GpioError::PinModeNotAllowed);
        }

        implementations::check_invalid_flags(
            &flags,
            vec![bindings::wasi::gpio::analog::AnalogFlag::DAC],
        )
        .map_err(|_| bindings::wasi::gpio::general::GpioError::InvalidFlag)?;

        let pin = self.get_pin(&pin_label)?.into_output_low();

        let config =
            AnalogConfigBuilder::new(pin_label, bindings::wasi::gpio::general::PinMode::Out)
                .add_flags(flags)
                .build()
                .map_err(|_| bindings::wasi::gpio::general::GpioError::InvalidFlag)?;
        self.table
            .push(AnalogOutPin::new(pin, config))
            .map_err(|err| bindings::wasi::gpio::general::GpioError::Other(err.to_string()))
    }

    fn get_config(
        &mut self,
        self_: wasmtime::component::Resource<AnalogOutPin>,
    ) -> Result<bindings::wasi::gpio::analog::AnalogConfig, bindings::wasi::gpio::general::GpioError>
    {
        Ok(self
            .table
            .get(&self_)
            .map_err(|_| bindings::wasi::gpio::general::GpioError::ResourceInvalidated)?
            .get_config()
            .clone())
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
        mut value: u32,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        if value > PWM_MAX!() {
            value = PWM_MAX!();
        }

        self.set_value(self_, value as f32 / (PWM_MAX!() as f32))
    }

    fn set_value(
        &mut self,
        self_: wasmtime::component::Resource<AnalogOutPin>,
        value: f32,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        let pin = self
            .table
            .get_mut(&self_)
            .map_err(|_| bindings::wasi::gpio::general::GpioError::ResourceInvalidated)?;
        Ok(pin
            .set_value(value)
            .map_err(|err| bindings::wasi::gpio::general::GpioError::Other(err))?)
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<AnalogOutPin>) -> wasmtime::Result<()> {
        self.table.delete(rep).expect("failed to delete resource");
        Ok(())
    }
}

impl bindings::wasi::gpio::analog::HostAnalogInOutPin for host_component::HostComponent {
    fn get(
        &mut self,
        _pin_label: wasmtime::component::__internal::String,
        _flags: wasmtime::component::__internal::Vec<bindings::wasi::gpio::analog::AnalogFlag>,
    ) -> Result<
        wasmtime::component::Resource<AnalogInOutPin>,
        bindings::wasi::gpio::general::GpioError,
    > {
        Err(bindings::wasi::gpio::general::GpioError::PinModeNotAvailable)
    }

    fn get_config(
        &mut self,
        _self_: wasmtime::component::Resource<AnalogInOutPin>,
    ) -> Result<bindings::wasi::gpio::analog::AnalogConfig, bindings::wasi::gpio::general::GpioError>
    {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn is_ready(&mut self, self_: wasmtime::component::Resource<AnalogInOutPin>) -> bool {
        match self.table.get(&self_) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn set_value_raw(
        &mut self,
        _self_: wasmtime::component::Resource<AnalogInOutPin>,
        _value: u32,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn set_value(
        &mut self,
        _self_: wasmtime::component::Resource<AnalogInOutPin>,
        _value: f32,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn read_raw(
        &mut self,
        _self_: wasmtime::component::Resource<AnalogInOutPin>,
    ) -> Result<u32, bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn read(
        &mut self,
        _self_: wasmtime::component::Resource<AnalogInOutPin>,
    ) -> Result<f32, bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<AnalogInOutPin>) -> wasmtime::Result<()> {
        self.table.delete(rep).expect("failed to delete resource");
        Ok(())
    }

    fn set_pin_mode(
        &mut self,
        _self_: wasmtime::component::Resource<AnalogInOutPin>,
        _pin_mode: bindings::wasi::gpio::general::PinMode,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }
}

impl bindings::wasi::gpio::analog::HostAnalogInPin for host_component::HostComponent {
    fn get(
        &mut self,
        _pin_label: wasmtime::component::__internal::String,
        _flags: wasmtime::component::__internal::Vec<bindings::wasi::gpio::analog::AnalogFlag>,
    ) -> Result<wasmtime::component::Resource<AnalogInPin>, bindings::wasi::gpio::general::GpioError>
    {
        Err(bindings::wasi::gpio::general::GpioError::PinModeNotAvailable)
    }

    fn get_config(
        &mut self,
        _self_: wasmtime::component::Resource<AnalogInPin>,
    ) -> Result<bindings::wasi::gpio::analog::AnalogConfig, bindings::wasi::gpio::general::GpioError>
    {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn is_ready(&mut self, self_: wasmtime::component::Resource<AnalogInPin>) -> bool {
        match self.table.get(&self_) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn read_raw(
        &mut self,
        _self_: wasmtime::component::Resource<AnalogInPin>,
    ) -> Result<u32, bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn read(
        &mut self,
        _self_: wasmtime::component::Resource<AnalogInPin>,
    ) -> Result<f32, bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn watch_above_raw(
        &mut self,
        _self_: wasmtime::component::Resource<AnalogInPin>,
        _value: u32,
    ) -> Result<wasmtime::component::Resource<Pollable>, bindings::wasi::gpio::general::GpioError>
    {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn watch_above(
        &mut self,
        _self_: wasmtime::component::Resource<AnalogInPin>,
        _value: f32,
    ) -> Result<wasmtime::component::Resource<Pollable>, bindings::wasi::gpio::general::GpioError>
    {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn watch_below_raw(
        &mut self,
        _self_: wasmtime::component::Resource<AnalogInPin>,
        _value: u32,
    ) -> Result<wasmtime::component::Resource<Pollable>, bindings::wasi::gpio::general::GpioError>
    {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn watch_below(
        &mut self,
        _self_: wasmtime::component::Resource<AnalogInPin>,
        _value: f32,
    ) -> Result<wasmtime::component::Resource<Pollable>, bindings::wasi::gpio::general::GpioError>
    {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<AnalogInPin>) -> wasmtime::Result<()> {
        self.table.delete(rep).expect("failed to delete resource");
        Ok(())
    }
}
