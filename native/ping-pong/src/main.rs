fn main() {
    let gpio = rppal::gpio::Gpio::new().unwrap();

    let rx = gpio.get(2).unwrap().into_input();
    let mut tx = gpio.get(3).unwrap().into_output_low();

    loop {
        while rx.is_low() {}
        tx.set_high();

        while rx.is_high() {}
        tx.set_low();
    }
}