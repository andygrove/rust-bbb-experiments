extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep_ms;

fn main() {
    let my_led = Pin::new(30); // GPIO 30 on BBB
    let duration = 200;
    my_led.with_exported(|| {
        my_led.set_direction(Direction::Out).unwrap();
        loop {
            my_led.set_value(0).unwrap();
            sleep_ms(duration);
            my_led.set_value(1).unwrap();
            sleep_ms(duration);
        }
    }).unwrap();
}
