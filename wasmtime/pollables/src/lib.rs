wit_bindgen::generate!({
    path: "../../wit",
    generate_all
});

pub struct Component;

impl Guest for Component {
    fn start(d:Delay,) -> () {
        let pin2 = wasi::gpio::digital::DigitalInPin::get("PIN2", &[wasi::gpio::digital::DigitalFlag::ACTIVE_HIGH, wasi::gpio::digital::DigitalFlag::PULL_DOWN]).unwrap();

        let pol = pin2.watch_active().unwrap();

        while(!pol.ready()) {}

        println!("Pollable triggered");
    }
}

export!(Component);