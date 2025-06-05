use clap::Parser;

#[derive(Parser)]
struct Config {
    #[arg(short, long, default_value_t = 2)]
    OUT: u8,
    #[arg(short, long, default_value_t = 3)]
    INOUT: u8,
    #[arg(short, long, default_value_t = 4)]
    IN: u8
}

fn main() {
    let config = Config::parse();
    
    let gpio = rppal::gpio::Gpio::new().unwrap();
    let mut OUT = gpio.get(config.OUT).unwrap().into_output_low();
    let mut INOUT = gpio.get(config.INOUT).unwrap().into_io(rppal::gpio::Mode::Input);
    let IN = gpio.get(config.IN).unwrap().into_input();

    loop {
        println!("setting INOUT mode to input");
        INOUT.set_mode(rppal::gpio::Mode::Input);
        println!("INOUT reading OUT state: {}", INOUT.read());
        println!("gpio 2 will be turned to the high state");
        OUT.set_high();
        println!("INOUT reading OUT state: {}", INOUT.read());
        println!("gpio 2 will be turned to the low state");
        OUT.set_low();
        println!("INOUT reading OUT state: {}", INOUT.read());

        std::thread::sleep(std::time::Duration::from_secs(1));
        
        println!("setting INOUT mode to output");
        INOUT.set_mode(rppal::gpio::Mode::Output);
        INOUT.set_low();
        println!("IN reading INOUT state: {}", IN.read());
        println!("gpio 3 will be turned to the high state");
        INOUT.set_high();
        println!("IN reading INOUT state: {}", IN.read());
        println!("gpio 3 will be turned to the low state");
        INOUT.set_low();
        println!("IN reading INOUT state: {}", IN.read());
        
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
