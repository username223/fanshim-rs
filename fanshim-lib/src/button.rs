use rppal::gpio::Gpio;
use rppal::gpio::Level::Low;

const BUTTON_ENABLED_PIN: u8 = 17;

pub fn button_is_depressed() -> bool {
    let mut pin = Gpio::new()
        .expect("unable to get gpio")
        .get(BUTTON_ENABLED_PIN)
        .expect("unable to get fan pin")
        .into_input_pullup();

    pin.set_reset_on_drop(false);

    pin.read() == Low
}
