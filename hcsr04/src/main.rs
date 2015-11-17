extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep_ms;

fn main() {

    let led = Pin::new(30);
    let trigger = Pin::new(31);
    let echo = Pin::new(48);

    // turn LED off
    led.with_exported(|| {
        my_led.set_direction(Direction::Out).unwrap();
	    my_led.set_value(0).unwrap();
    });

    loop {

        // trigger
        trigger.with_exported(|| {
            trigger.set_direction(Direction::Out).unwrap();
            trigger.set_value(1).unwrap();
            sleep_ms(10);
            trigger.set_value(0).unwrap();
        });

        let count = 0;
        loop {
            echo.with_exported(|| {
                echo.set_direction(Direction::In).unwrap();
                let value = echo.get_value().unwrap();
                println!("Echo: {}", echo);
                sleep_ms(100);
            })
        }

    }
}
