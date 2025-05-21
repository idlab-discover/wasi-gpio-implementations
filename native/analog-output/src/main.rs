fn main() {
    let pwm = rppal::pwm::Pwm::new(rppal::pwm::Channel::Pwm0).unwrap();
    pwm.enable().unwrap();
    
    loop {
        for i in 0..100 {
            pwm.set_duty_cycle((i as f64) / 100.0).unwrap();
        }
        
        for i in (0..100).rev() {
            pwm.set_duty_cycle((i as f64) / 100.0).unwrap();
        }
    }
}
