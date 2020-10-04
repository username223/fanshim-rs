use crate::config::FanshimConfig;
use log::debug;
#[cfg(test)]
use mockall::automock;
use std::thread;
use std::time::Duration;

pub mod button;
pub mod config;
pub mod cpu;
pub mod error;
pub mod fan;
pub mod led;

pub type Result<T> = std::result::Result<T, error::FanshimError>;

const MILLI_CELSIUS_IN_CELSIUS: i32 = 1000;

/// Temperature in millidegrees celsius. Multiply by 1000 to get "regular" celsius.
#[derive(Debug, Clone, Copy)]
pub struct MilliCelsius(pub i32);

pub fn initial_setup<T>(config: &FanshimConfig, s: &mut T)
where
    T: ShimLayer,
{
    let temp = s.get_cpu_temperature();
    if temp.0 > config.enable_fan_at_temperature.0 {
        s.turn_fan_on();
        s.set_led_rgb_brightness(
            config.led_on_color.0,
            config.led_on_color.1,
            config.led_on_color.2,
            config.led_brightness,
        );
    } else {
        s.turn_fan_off();
        s.set_led_rgb_brightness(
            config.led_off_color.0,
            config.led_off_color.1,
            config.led_off_color.2,
            config.led_brightness,
        );
    }
}

pub fn main_loop<T>(config: &FanshimConfig, s: &mut T)
where
    T: ShimLayer,
{
    let mut number_of_sequential_above_reads = 0;
    let mut number_of_sequential_below_reads = 0;
    loop {
        let cpu_temperature = s.get_cpu_temperature();
        debug!("Temp: {:?}", cpu_temperature.0);

        let fan_enabled = s.fan_is_enabled();

        let cpu_temp_is_above_threshold = cpu_temperature.0 > config.enable_fan_at_temperature.0;
        if cpu_temp_is_above_threshold && !fan_enabled {
            number_of_sequential_above_reads += 1;
        } else {
            number_of_sequential_above_reads = 0;
        }

        let cpu_temp_is_below_disable_threshold =
            cpu_temperature.0 < config.disable_fan_at_temperature.0;
        if cpu_temp_is_below_disable_threshold && fan_enabled {
            number_of_sequential_below_reads += 1;
        } else {
            number_of_sequential_below_reads = 0;
        }

        let fan_should_enable =
            number_of_sequential_above_reads >= config.number_of_readings_before_action;
        let fan_should_disable =
            number_of_sequential_below_reads >= config.number_of_readings_before_action;

        if fan_should_enable {
            s.turn_fan_on();
            s.set_led_rgb_brightness(
                config.led_on_color.0,
                config.led_on_color.1,
                config.led_on_color.2,
                config.led_brightness,
            );
            number_of_sequential_below_reads = 0;
            number_of_sequential_above_reads = 0;
        } else if fan_should_disable {
            s.turn_fan_off();
            s.set_led_rgb_brightness(
                config.led_off_color.0,
                config.led_off_color.1,
                config.led_off_color.2,
                config.led_brightness,
            );
            number_of_sequential_below_reads = 0;
            number_of_sequential_above_reads = 0;
        }

        s.sleep(config.delay_between_readings);

        #[cfg(test)]
        if s.should_exit() {
            return;
        }
    }
}

#[cfg_attr(test, automock)]
pub trait ShimLayer {
    fn get_cpu_temperature(&mut self) -> MilliCelsius;

    fn fan_is_enabled(&mut self) -> bool;
    fn turn_fan_on(&mut self);
    fn turn_fan_off(&mut self);
    fn set_led_rgb_brightness(&mut self, r: u8, g: u8, b: u8, brightness: f32);

    fn sleep(&mut self, dur: Duration);

    fn should_exit(&mut self) -> bool;
}

pub struct RealShim {}
impl RealShim {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for RealShim {
    fn default() -> Self {
        Self::new()
    }
}

impl ShimLayer for RealShim {
    fn get_cpu_temperature(&mut self) -> MilliCelsius {
        cpu::get_cpu_temperature()
    }

    fn fan_is_enabled(&mut self) -> bool {
        fan::fan_is_enabled()
    }

    fn turn_fan_on(&mut self) {
        fan::turn_fan_full_on();
    }

    fn turn_fan_off(&mut self) {
        fan::turn_fan_off();
    }

    fn set_led_rgb_brightness(&mut self, r: u8, g: u8, b: u8, brightness: f32) {
        led::set_led_rgb_brightness(r, g, b, brightness);
    }

    fn sleep(&mut self, dur: Duration) {
        thread::sleep(dur);
    }

    fn should_exit(&mut self) -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use crate::config::FanshimConfig;
    use crate::{initial_setup, main_loop, MilliCelsius};
    use mockall::predicate::eq;
    use mockall::*;

    fn temperature_above_enable() -> MilliCelsius {
        MilliCelsius(FanshimConfig::default().enable_fan_at_temperature.0 + 1_000)
    }

    fn temperature_below_enable() -> MilliCelsius {
        MilliCelsius(FanshimConfig::default().enable_fan_at_temperature.0 - 1_000)
    }

    fn temperature_above_disable() -> MilliCelsius {
        MilliCelsius(FanshimConfig::default().disable_fan_at_temperature.0 + 1_000)
    }

    fn temperature_below_disable() -> MilliCelsius {
        MilliCelsius(FanshimConfig::default().disable_fan_at_temperature.0 - 1_000)
    }

    fn test_setup() -> (FanshimConfig, Sequence, super::MockShimLayer) {
        let c = FanshimConfig::default();
        let seq = Sequence::new();
        let mock = super::MockShimLayer::new();
        (c, seq, mock)
    }

    fn should_not_exit(mock: &mut super::MockShimLayer, seq: &mut Sequence) {
        mock.expect_should_exit()
            .times(1)
            .in_sequence(seq)
            .return_const(false);
    }

    fn should_exit(mock: &mut super::MockShimLayer, seq: &mut Sequence) {
        mock.expect_should_exit()
            .times(1)
            .in_sequence(seq)
            .return_const(true);
    }

    #[test]
    fn initial_setup_enables_when_hot() {
        let (c, mut seq, mut mock) = test_setup();

        // GIVEN:
        mock.expect_get_cpu_temperature()
            .return_const(temperature_above_enable());

        // THEN:
        mock.expect_turn_fan_on()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(());
        mock.expect_set_led_rgb_brightness()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(());

        // WHEN:
        initial_setup(&c, &mut mock);
    }

    #[test]
    fn initial_setup_disables_when_cold() {
        let (c, mut seq, mut mock) = test_setup();

        // GIVEN:
        mock.expect_get_cpu_temperature()
            .return_const(temperature_below_disable());

        // THEN:
        mock.expect_turn_fan_off()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(());
        mock.expect_set_led_rgb_brightness()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(());

        // WHEN:
        initial_setup(&c, &mut mock);
    }

    #[test]
    fn fan_enables_above_temperature_and_after_count() {
        let (c, mut seq, mut mock) = test_setup();

        // GIVEN:
        mock.expect_sleep().return_const(());
        mock.expect_get_cpu_temperature()
            .return_const(temperature_above_enable());
        mock.expect_fan_is_enabled().return_const(false);
        let (r, g, b) = c.led_on_color;
        mock.expect_set_led_rgb_brightness()
            .with(eq(r), eq(g), eq(b), eq(c.led_brightness))
            .return_const(());

        // THEN:
        for _ in 1..c.number_of_readings_before_action {
            should_not_exit(&mut mock, &mut seq);
        }

        mock.expect_turn_fan_on()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(());

        should_exit(&mut mock, &mut seq);

        // WHEN:
        main_loop(&c, &mut mock);
    }

    #[test]
    fn fan_disables_below_temperature_and_after_count() {
        let (c, mut seq, mut mock) = test_setup();

        // GIVEN:
        mock.expect_sleep().return_const(());
        mock.expect_get_cpu_temperature()
            .return_const(temperature_below_disable());
        mock.expect_fan_is_enabled().return_const(true);
        let (r, g, b) = c.led_off_color;
        mock.expect_set_led_rgb_brightness()
            .with(eq(r), eq(g), eq(b), eq(c.led_brightness))
            .return_const(());

        // THEN:
        for _ in 1..c.number_of_readings_before_action {
            should_not_exit(&mut mock, &mut seq);
        }

        mock.expect_turn_fan_off()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(());

        should_exit(&mut mock, &mut seq);

        // WHEN:
        main_loop(&c, &mut mock);
    }

    #[test]
    fn fan_does_not_enable_when_below_temperature() {
        let (c, mut seq, mut mock) = test_setup();

        // GIVEN:
        mock.expect_sleep().return_const(());
        mock.expect_get_cpu_temperature()
            .return_const(temperature_below_enable());
        mock.expect_fan_is_enabled().return_const(false);

        // THEN:
        for _ in 0..=1000 {
            should_not_exit(&mut mock, &mut seq);
        }

        should_exit(&mut mock, &mut seq);

        // WHEN:
        main_loop(&c, &mut mock);
    }

    #[test]
    fn fan_does_not_disable_when_above_temperature() {
        let (c, mut seq, mut mock) = test_setup();

        // GIVEN:
        mock.expect_sleep().return_const(());
        mock.expect_get_cpu_temperature()
            .return_const(temperature_above_disable());
        mock.expect_fan_is_enabled().return_const(true);

        // THEN:
        should_not_exit(&mut mock, &mut seq);

        should_exit(&mut mock, &mut seq);

        // WHEN:
        main_loop(&c, &mut mock);
    }

    #[test]
    fn fan_enables_after_long_below_temperature() {
        let (c, mut seq, mut mock) = test_setup();

        // GIVEN:
        mock.expect_sleep().return_const(());
        let (r, g, b) = c.led_on_color;
        mock.expect_set_led_rgb_brightness()
            .with(eq(r), eq(g), eq(b), eq(c.led_brightness))
            .return_const(());

        // THEN:
        // Below temperature for a long while
        for _ in 0..=1000 {
            mock.expect_get_cpu_temperature()
                .times(1)
                .in_sequence(&mut seq)
                .return_const(temperature_below_enable());
            mock.expect_fan_is_enabled()
                .times(1)
                .in_sequence(&mut seq)
                .return_const(false);
            should_not_exit(&mut mock, &mut seq);
        }

        // Then we go above
        for _ in 1..c.number_of_readings_before_action {
            mock.expect_get_cpu_temperature()
                .times(1)
                .in_sequence(&mut seq)
                .return_const(temperature_above_enable());
            mock.expect_fan_is_enabled()
                .times(1)
                .in_sequence(&mut seq)
                .return_const(false);
            should_not_exit(&mut mock, &mut seq);
        }
        mock.expect_get_cpu_temperature()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(temperature_above_enable());
        mock.expect_fan_is_enabled()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(false);

        mock.expect_turn_fan_on()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(());

        should_exit(&mut mock, &mut seq);

        // WHEN:
        main_loop(&c, &mut mock);
    }

    #[test]
    fn fan_disables_after_long_above_temperature() {
        let (c, mut seq, mut mock) = test_setup();

        // GIVEN:
        mock.expect_sleep().return_const(());
        let (r, g, b) = c.led_off_color;
        mock.expect_set_led_rgb_brightness()
            .with(eq(r), eq(g), eq(b), eq(c.led_brightness))
            .return_const(());

        // THEN:
        // Below temperature for a long while
        for _ in 0..=1000 {
            mock.expect_get_cpu_temperature()
                .times(1)
                .in_sequence(&mut seq)
                .return_const(temperature_above_enable());
            mock.expect_fan_is_enabled()
                .times(1)
                .in_sequence(&mut seq)
                .return_const(true);
            should_not_exit(&mut mock, &mut seq);
        }

        // Then we go above
        for _ in 1..c.number_of_readings_before_action {
            mock.expect_get_cpu_temperature()
                .times(1)
                .in_sequence(&mut seq)
                .return_const(temperature_below_disable());
            mock.expect_fan_is_enabled()
                .times(1)
                .in_sequence(&mut seq)
                .return_const(true);
            should_not_exit(&mut mock, &mut seq);
        }
        mock.expect_get_cpu_temperature()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(temperature_below_disable());
        mock.expect_fan_is_enabled()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(true);

        mock.expect_turn_fan_off()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(());

        should_exit(&mut mock, &mut seq);

        // WHEN:
        main_loop(&c, &mut mock);
    }

    #[test]
    fn fan_does_not_disable_when_temperature_fluctuating() {
        let (c, mut seq, mut mock) = test_setup();

        // GIVEN:
        mock.expect_sleep().return_const(());

        // THEN:
        for _ in 0..=1000 {
            mock.expect_get_cpu_temperature()
                .times(1)
                .in_sequence(&mut seq)
                .return_const(temperature_below_disable());
            mock.expect_fan_is_enabled()
                .times(1)
                .in_sequence(&mut seq)
                .return_const(true);
            should_not_exit(&mut mock, &mut seq);

            mock.expect_get_cpu_temperature()
                .times(1)
                .in_sequence(&mut seq)
                .return_const(temperature_above_disable());
            mock.expect_fan_is_enabled()
                .times(1)
                .in_sequence(&mut seq)
                .return_const(true);
            should_not_exit(&mut mock, &mut seq);
        }

        mock.expect_get_cpu_temperature()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(temperature_below_disable());
        mock.expect_fan_is_enabled()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(true);
        should_exit(&mut mock, &mut seq);

        // WHEN:
        main_loop(&c, &mut mock);
    }

    #[test]
    fn fan_does_not_enable_when_temperature_fluctuating() {
        let (c, mut seq, mut mock) = test_setup();

        // GIVEN:
        mock.expect_sleep().return_const(());

        // THEN:
        for _ in 0..=1000 {
            mock.expect_get_cpu_temperature()
                .times(1)
                .in_sequence(&mut seq)
                .return_const(temperature_below_enable());
            mock.expect_fan_is_enabled()
                .times(1)
                .in_sequence(&mut seq)
                .return_const(false);
            should_not_exit(&mut mock, &mut seq);

            mock.expect_get_cpu_temperature()
                .times(1)
                .in_sequence(&mut seq)
                .return_const(temperature_above_enable());
            mock.expect_fan_is_enabled()
                .times(1)
                .in_sequence(&mut seq)
                .return_const(false);
            should_not_exit(&mut mock, &mut seq);
        }

        mock.expect_get_cpu_temperature()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(temperature_below_enable());
        mock.expect_fan_is_enabled()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(false);
        should_exit(&mut mock, &mut seq);

        // WHEN:
        main_loop(&c, &mut mock);
    }
}
