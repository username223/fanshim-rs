use blinkt::Blinkt;

const LED_DATA_PIN: u8 = 15;
const LED_CLOCK_PIN: u8 = 14;
const LED_NUMBER_OF_PIXELS: usize = 8;

pub fn set_led_rgb(red: u8, green: u8, blue: u8) {
    let mut led = Blinkt::with_settings(LED_DATA_PIN, LED_CLOCK_PIN, LED_NUMBER_OF_PIXELS).unwrap();
    led.set_clear_on_drop(false);

    led.set_all_pixels(red, green, blue);
    led.show().unwrap();
}

pub fn set_led_rgb_brightness(red: u8, green: u8, blue: u8, brightness: f32) {
    let mut led = Blinkt::with_settings(LED_DATA_PIN, LED_CLOCK_PIN, LED_NUMBER_OF_PIXELS).unwrap();
    led.set_clear_on_drop(false);

    led.set_all_pixels_rgbb(red, green, blue, brightness);
    led.show().unwrap();
}
