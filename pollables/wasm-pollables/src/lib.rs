wit_bindgen::generate!({
    path: "../../wit",
    generate_all
});

pub struct Component;

impl Guest for Component {
    fn start(d:Delay,) -> () {
        let gpio2 = wasi::gpio::digital::DigitalInPin::get("POLL_PIN", &[wasi::gpio::digital::DigitalFlag::ACTIVE_HIGH, wasi::gpio::digital::DigitalFlag::PULL_UP]).unwrap();

        loop {
            println!("Getting pollable");
            let pol = gpio2.watch_inactive().unwrap();
            
            while !pol.ready() {}
            println!("Pollable triggered");
            while !pol.ready() {}
            println!("Pollable is still triggered, resetting in 1s");
            
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}

export!(Component);