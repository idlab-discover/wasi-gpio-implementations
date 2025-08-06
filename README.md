# Implementations and demos of WASI GPIO

## Host implementation

The host implementation can be found in the `wasmtime-gpio-host` folder, using it is explained by running it withy the `-h` flag.

## Client demos

- `digital-input-output`: Checks the functionality of a digital-input-output-pin by switching between these states. Setting up this demo requires looking at the provided policies.toml file. Pin OUT should be connected to pin INOUT via a 10kΩ resistor and pin IN to pin INOUT
with a 10kΩ as well.
- `misc/alternate-analog-digital`: An example to show that pins can have multiple allowed modes and thus can switch between them
- `pollables`: Checks the functionality of `digital-input-pin.watch-inactive()`
- `pwm`: Examples to show the PWM functionality of the API, software based PWM is used because hardware PWM requires configuration on the Raspberry Pi.

## IMPORTANT

DO NOT EDIT THE `wit` FOLDER IN ANY WAY, it is fine in it's current state to demonstrate the examples

## Acknowledgements

This work has been partially supported by the ELASTIC project, which received funding from the Smart Networks and Services Joint Undertaking (SNS JU) under the European Union’s Horizon Europe research and innovation programme under Grant Agreement No 101139067. Views and opinions expressed are however those of the author(s) only and do not necessarily reflect those of the European Union. Neither the European Union nor the granting authority can be held responsible for them. This funding supported individual contributor organisations, not the W3C or this community group as a whole.
