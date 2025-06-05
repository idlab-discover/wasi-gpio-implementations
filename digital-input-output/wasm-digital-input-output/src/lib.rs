use wasi::gpio::{digital::DigitalFlag, general::PinMode};

wit_bindgen::generate!({
    path: "../../wit",
    generate_all
});

pub struct Component;

impl Guest for Component {
    fn start(d:Delay,) -> () {
        let OUT = wasi::gpio::digital::DigitalOutPin::get("OUT", &[DigitalFlag::ACTIVE_HIGH, DigitalFlag::INACTIVE]).expect("Failed to get OUT");
        let INOUT = wasi::gpio::digital::DigitalInOutPin::get("INOUT", &[DigitalFlag::ACTIVE_HIGH, DigitalFlag::INPUT]).expect("Failed to get INOUT");
        let IN = wasi::gpio::digital::DigitalInPin::get("IN", &[DigitalFlag::ACTIVE_HIGH]).expect("Failed to get IN");
        
        loop {
            println!("setting INOUT mode to input");
            INOUT.set_pin_mode(PinMode::In).unwrap();
            let state = match INOUT.read().unwrap() {
                wasi::gpio::digital::PinState::Active => "High",
                wasi::gpio::digital::PinState::Inactive => "Low",
            };
            println!("INOUT reading OUT state: {}", state);
            println!("gpio 2 will be turned to the high state");
            OUT.set_active().unwrap();
            let state = match INOUT.read().unwrap() {
                wasi::gpio::digital::PinState::Active => "High",
                wasi::gpio::digital::PinState::Inactive => "Low",
            };
            println!("INOUT reading OUT state: {}", state);
            println!("gpio 2 will be turned to the low state");
            OUT.set_inactive().unwrap();
            let state = match INOUT.read().unwrap() {
                wasi::gpio::digital::PinState::Active => "High",
                wasi::gpio::digital::PinState::Inactive => "Low",
            };
            println!("INOUT reading OUT state: {}", state);
    
            std::thread::sleep(std::time::Duration::from_secs(1));
            
            println!("setting INOUT mode to output");
            INOUT.set_pin_mode(PinMode::Out).unwrap();
            
            INOUT.set_inactive().unwrap();
            let state = match IN.read().unwrap() {
                wasi::gpio::digital::PinState::Active => "High",
                wasi::gpio::digital::PinState::Inactive => "Low",
            };
            println!("IN reading INOUT state: {}", state);
            println!("gpio 3 will be turned to the high state");
            INOUT.set_active().unwrap();
            let state = match IN.read().unwrap() {
                wasi::gpio::digital::PinState::Active => "High",
                wasi::gpio::digital::PinState::Inactive => "Low",
            };
            println!("IN reading INOUT state: {}", state);
            println!("gpio 3 will be turned to the low state");
            INOUT.set_inactive().unwrap();
            let state = match IN.read().unwrap() {
                wasi::gpio::digital::PinState::Active => "High",
                wasi::gpio::digital::PinState::Inactive => "Low",
            };
            println!("IN reading INOUT state: {}", state);
            
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}

export!(Component);