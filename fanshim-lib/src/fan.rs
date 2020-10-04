use rppal::gpio::Gpio;
use rppal::gpio::Level::High;
use rppal::gpio::Mode::Output;

const FAN_ENABLED_PIN: u8 = 18;

pub fn fan_is_enabled() -> bool {
    let pin = Gpio::new()
        .expect("unable to get gpio")
        .get(FAN_ENABLED_PIN)
        .expect("unable to get fan pin");

    pin.mode() == Output && pin.read() == High
}

pub fn toggle_fan() {
    let mut pin = Gpio::new()
        .expect("unable to get gpio")
        .get(FAN_ENABLED_PIN)
        .expect("unable to get fan pin")
        .into_output();

    pin.set_reset_on_drop(false);
    pin.toggle();
}

pub fn turn_fan_off() {
    let mut pin = Gpio::new()
        .expect("unable to get gpio")
        .get(FAN_ENABLED_PIN)
        .expect("unable to get fan pin")
        .into_output();

    pin.set_reset_on_drop(false);
    pin.set_low();
}

pub fn turn_fan_full_on() {
    let mut pin = Gpio::new()
        .expect("unable to get gpio")
        .get(FAN_ENABLED_PIN)
        .expect("unable to get fan pin")
        .into_output();

    pin.set_reset_on_drop(false);
    pin.set_high();
}
