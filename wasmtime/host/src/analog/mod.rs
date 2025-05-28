use super::bindings;
use super::host_component;
use super::{Pollable, AnalogInOutPin, AnalogInPin, AnalogOutPin};

impl bindings::wasi::gpio::analog::Host for host_component::HostComponent {}

impl bindings::wasi::gpio::analog::HostAnalogOutPin for host_component::HostComponent {
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
        self.table.delete(rep).expect("failed to delete resource");
        Ok(())
    }
}

impl bindings::wasi::gpio::analog::HostAnalogInOutPin for host_component::HostComponent {
    fn get(
        &mut self,
        pin_label: wasmtime::component::__internal::String,
        flags: wasmtime::component::__internal::Vec<bindings::wasi::gpio::analog::AnalogFlag>,
    ) -> Result<
        wasmtime::component::Resource<AnalogInOutPin>,
        bindings::wasi::gpio::general::GpioError,
    > {
        Err(bindings::wasi::gpio::general::GpioError::PinModeNotAvailable)
    }

    fn get_config(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInOutPin>,
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
        self_: wasmtime::component::Resource<AnalogInOutPin>,
        value: u32,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn set_value(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInOutPin>,
        value: f32,
    ) -> Result<(), bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn read_raw(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInOutPin>,
    ) -> Result<u32, bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn read(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInOutPin>,
    ) -> Result<f32, bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<AnalogInOutPin>) -> wasmtime::Result<()> {
        self.table.delete(rep).expect("failed to delete resource");
        Ok(())
    }
    
    fn set_pin_mode(&mut self,self_:wasmtime::component::Resource<AnalogInOutPin>,pin_mode:bindings::wasi::gpio::general::PinMode,) -> Result<(),bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }
}

impl bindings::wasi::gpio::analog::HostAnalogInPin for host_component::HostComponent {
    fn get(
        &mut self,
        pin_label: wasmtime::component::__internal::String,
        flags: wasmtime::component::__internal::Vec<bindings::wasi::gpio::analog::AnalogFlag>,
    ) -> Result<wasmtime::component::Resource<AnalogInPin>, bindings::wasi::gpio::general::GpioError>
    {
        Err(bindings::wasi::gpio::general::GpioError::PinModeNotAvailable)
    }

    fn get_config(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInPin>,
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
        self_: wasmtime::component::Resource<AnalogInPin>,
    ) -> Result<u32, bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn read(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInPin>,
    ) -> Result<f32, bindings::wasi::gpio::general::GpioError> {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn watch_above_raw(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInPin>,
        value: u32,
    ) -> Result<wasmtime::component::Resource<Pollable>, bindings::wasi::gpio::general::GpioError>
    {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn watch_above(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInPin>,
        value: f32,
    ) -> Result<wasmtime::component::Resource<Pollable>, bindings::wasi::gpio::general::GpioError>
    {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn watch_below_raw(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInPin>,
        value: u32,
    ) -> Result<wasmtime::component::Resource<Pollable>, bindings::wasi::gpio::general::GpioError>
    {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn watch_below(
        &mut self,
        self_: wasmtime::component::Resource<AnalogInPin>,
        value: f32,
    ) -> Result<wasmtime::component::Resource<Pollable>, bindings::wasi::gpio::general::GpioError>
    {
        Err(bindings::wasi::gpio::general::GpioError::ResourceInvalidated)
    }

    fn drop(&mut self, rep: wasmtime::component::Resource<AnalogInPin>) -> wasmtime::Result<()> {
        self.table.delete(rep).expect("failed to delete resource");
        Ok(())
    }
}