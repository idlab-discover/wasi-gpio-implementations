use std::{thread::sleep, time::Duration};
use rppal::gpio::*;
use clap::Parser;

#[derive(Parser)]
struct Config {
    #[arg(short, long, default_value_t = 2)]
    PWM: u8
}

fn main() {
    let config = Config::parse();
    
    let gpio = Gpio::new().unwrap();
    
    let mut pin2 = gpio.get(config.PWM).unwrap().into_output_low();
    
    loop {
        for i in 0..101 {
            pin2.set_pwm_frequency(1000., i as f64 / 100.0).unwrap();
            sleep(Duration::from_millis(20));
        }
        
        for i in (0..99).rev() {
            pin2.set_pwm_frequency(1000., i as f64 / 100.0).unwrap();
            sleep(Duration::from_millis(20));
        }
    }
}
