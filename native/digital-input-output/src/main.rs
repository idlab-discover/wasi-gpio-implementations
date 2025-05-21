fn main() {
    let gpio = rppal::gpio::Gpio::new().unwrap();
    let mut gpio2 = gpio.get(2).unwrap().into_output_low();
    let mut gpio3 = gpio.get(3).unwrap().into_io(rppal::gpio::Mode::Input);
    let gpio4 = gpio.get(4).unwrap().into_input();

    loop {
        println!("setting gpio3 mode to input");
        gpio3.set_mode(rppal::gpio::Mode::Input);
        println!("gpio3 reading gpio2 state: {}", gpio3.read());
        println!("gpio 2 will be turned to the high state");
        gpio2.set_high();
        println!("gpio3 reading gpio2 state: {}", gpio3.read());
        println!("gpio 2 will be turned to the low state");
        gpio2.set_low();
        println!("gpio3 reading gpio2 state: {}", gpio3.read());

        std::thread::sleep(std::time::Duration::from_secs(1));
        
        println!("setting gpio3 mode to output");
        gpio3.set_mode(rppal::gpio::Mode::Output);
        gpio3.set_low();
        println!("gpio4 reading gpio3 state: {}", gpio4.read());
        println!("gpio 3 will be turned to the high state");
        gpio3.set_high();
        println!("gpio4 reading gpio3 state: {}", gpio4.read());
        println!("gpio 3 will be turned to the low state");
        gpio3.set_low();
        println!("gpio4 reading gpio3 state: {}", gpio4.read());
        
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
