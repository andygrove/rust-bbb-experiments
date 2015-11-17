extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main() {

    let trigger = Pin::new(62);
    let echo    = Pin::new(63);

    loop {

        println!("Trigger");

        trigger.with_exported(|| {
            trigger.set_direction(Direction::Out).unwrap();
            trigger.set_value(1).unwrap();
            // sleep for 10 microseconds (10,000 nanoseconds)
            sleep(Duration::new(0, 10000));
            trigger.set_value(0).unwrap();
            Ok(())
        }).unwrap();

        echo.with_exported(|| {

            echo.set_direction(Direction::In).unwrap();

            // wait for echo to go high
            println!("Waiting for ECHO high");
            loop {
                let value = echo.get_value().unwrap();
                if value == 1 {
                    break;
                }
                sleep(Duration::new(0, 10000));
            }

            println!("Waiting for ECHO low");
            let mut count = 0;
            loop {
                let value = echo.get_value().unwrap();
                if value == 0 {
                    break;
                }
                sleep(Duration::new(0, 10000));
                count = count + 1;
            }

            println!("Time: {} microseconds", count*10);

            Ok(())
        }).unwrap();

    }
}
