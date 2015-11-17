extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep_ms;

fn main() {

    //let led     = Pin::new(30);
    let trigger = Pin::new(31);
    let echo    = Pin::new(48);


    loop {

        // trigger
        println!("Trigger");
        trigger.with_exported(|| {
            trigger.set_direction(Direction::Out).unwrap();
            trigger.set_value(1).unwrap();
            sleep_ms(10);
            trigger.set_value(0).unwrap();
        }).unwrap();

        let count = 0;
        loop {
            echo.with_exported(|| {
                echo.set_direction(Direction::In).unwrap();
                let value = echo.get_value().unwrap();
                println!("Echo: {}", value);
                sleep_ms(100);
            }).unwrap();
        }

    }
}
