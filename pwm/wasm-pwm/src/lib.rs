use std::{thread::sleep, time::Duration};

use crate::wasi::gpio::analog::{AnalogFlag, AnalogOutPin};

wit_bindgen::generate!({
    path: "../../wit",
    generate_all
});

pub struct Component;

impl Guest for Component {
    fn start(d:Delay,) -> () {
        let pwm = AnalogOutPin::get("PWM", &[AnalogFlag::PWM]).unwrap();
        
        loop {
            for i in 0..101 {
                pwm.set_value(i as f32 / 100.0).unwrap();
                sleep(Duration::from_millis(20));
            }
            
            for i in (0..99).rev() {
                pwm.set_value(i as f32 / 100.0).unwrap();
                sleep(Duration::from_millis(20));
            }
        }
    }
}

export!(Component);