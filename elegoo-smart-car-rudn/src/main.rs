#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![allow(dead_code)]

#[allow(unused_imports)]
use arduino_hal::prelude::*;
#[allow(unused_imports)]
use embedded_hal::serial::Read;


mod l287n_motor_driver;
#[allow(unused_imports)]
use l287n_motor_driver::{MotorChassis, ChassisDirection};
use servo::Servo;

mod clock;

mod hc_sr04_distance_sensor;
mod servo;
mod panic;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let enable_a = pins.d5.into_output().downgrade();
    let enable_b = pins.d6.into_output().downgrade();
    let in1 = pins.d7.into_output().downgrade();
    let in2 = pins.d8.into_output().downgrade();
    let in3 = pins.d9.into_output().downgrade();
    let in4 = pins.d11.into_output().downgrade();

    let mut chassis = MotorChassis::new(
        enable_a,
        enable_b,
        in1,
        in2,
        in3,
        in4,
    );

    let mut led = pins.d13.into_output();


    clock::millis_init(dp.TC0);

    // Enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    #[allow(unused_variables)]
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    chassis.set_enabled(true, true);

    /*
    loop {
        
        ufmt::uwriteln!(&mut serial, "Moving\n").void_unwrap();
        led.toggle();
        chassis.set_direction(ChassisDirection::Forward);
        arduino_hal::delay_ms(1000);
        chassis.set_direction(ChassisDirection::Backward);
        arduino_hal::delay_ms(1000);

        ufmt::uwriteln!(&mut serial, "Turning\n").void_unwrap();
        led.toggle();
        chassis.set_direction(ChassisDirection::Left);
        arduino_hal::delay_ms(1000);
        chassis.set_direction(ChassisDirection::Right);
        arduino_hal::delay_ms(1000);
    }
    */

    let dist_trigger_pin = pins.a5.into_output().downgrade();
    let dist_echo_pin = pins.a4.into_pull_up_input().downgrade().forget_imode();

    let mut dist_sensor = hc_sr04_distance_sensor::HC_SR04::new(
        dp.TC1,
        dist_trigger_pin,
        dist_echo_pin,
    );

    ufmt::uwriteln!(&mut serial, "Running!").void_unwrap();

    let mut servo = Servo::new(pins.d3.into_output());

    loop {
//        let dist = dist_sensor.get_distance();
//        ufmt::uwriteln!(&mut serial, "Distance: {}", dist).void_unwrap();
//        led.toggle();
//        arduino_hal::delay_ms(1000);
        for i in [0, 90, 180, 90].iter() {
            servo.set_angle(*i);
            ufmt::uwriteln!(&mut serial, "Angle: {}", i).void_unwrap();
            arduino_hal::delay_ms(1000);
        }

    }
}