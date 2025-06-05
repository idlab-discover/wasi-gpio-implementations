use std::{fs::{DirBuilder, DirEntry}, thread::sleep, time::Duration};

use crate::wasi::gpio::{analog::{AnalogFlag, AnalogOutPin}, digital::{DigitalFlag, DigitalOutPin}};

wit_bindgen::generate!({
    path: "../../wit",
    generate_all
});

pub struct Component;

impl Guest for Component {
    fn start(d:Delay,) -> () {
        {
            println!("Analog");
            let pwm = AnalogOutPin::get("PWM", &[AnalogFlag::PWM]).unwrap();
            
            for i in 0..5 {
                for j in 0..101 {
                    pwm.set_value(j as f32 / 100.).unwrap();
                    sleep(Duration::from_millis(10));
                }
            }
            println!("Analog done");
        }
        
        {
            println!("Digital");
            let digital = DigitalOutPin::get("GPIO", &[DigitalFlag::ACTIVE_HIGH, DigitalFlag::INACTIVE]).unwrap();
            
            for i in 0..5 {
                digital.set_active().unwrap();
                sleep(Duration::from_secs(1));
                digital.set_inactive().unwrap();
                sleep(Duration::from_secs(1));
            }
            println!("Digital done");
        }
    }
}

export!(Component);