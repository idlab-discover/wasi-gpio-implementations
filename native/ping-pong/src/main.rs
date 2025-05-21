use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    
    #[arg(short('i'), long("input"), required(true))]
    input: u8,
    
    #[arg(short('o'), long("output"), required(true))]
    output: u8
}

fn main() {
    let args = Args::parse();
    
    let gpio = rppal::gpio::Gpio::new().unwrap();

    let rx = gpio.get(args.input).unwrap().into_input();
    let mut tx = gpio.get(args.output).unwrap().into_output_low();

    loop {
        while rx.is_low() {}
        tx.set_high();

        while rx.is_high() {}
        tx.set_low();
    }
}