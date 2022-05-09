//! Driver for the servo motor attached to the distance sensor stick.
//!
//! Information:
//! The servo is connected to Arduino port 3 (PD3).
//! Servos use PWM to control the angle.
//!
//! To control a servo, you must send a rising edge once every 20ms.
//! The time between the rising edge and the falling edge is the pulse width, and it determines the angle.
//! The smallest angle is achieved when the pulse width is 1ms, and the largest angle is when the pulse width is 2ms.

use arduino_hal::port::Pin;
use arduino_hal::port::mode::Output;
use arduino_hal::hal::port::PD3;

/// The representation of a servo position.
/// 
/// You can create one of these using [`ServoPhase::from_angle`].
/// This also contains a value which is implementation detail.
#[derive(Debug, Clone, Copy)]
pub struct ServoPhase {
    value: u32, // From 0 to 1000, equivalent to µs.
}

impl ServoPhase {
    pub fn from_angle(angle: u8) -> Self {
        Self {
            value: ((angle as u32) * 1000) / 180,
        }
    }
}

/// The driver for the servo motor attached to the pin 3 (PD3).
pub struct Servo {
    pin: Pin<Output, PD3>,
    current_phase: ServoPhase,
}

impl Servo {
    pub fn new(pin: Pin<Output, PD3>) -> Self {
        let mut new_servo = Self {
            pin,
            current_phase: ServoPhase::from_angle(90),
        };

        new_servo.set_angle(90);
        new_servo
    }

    /// Set the angle of the servo, in degrees.
    pub fn set_angle(&mut self, angle: u8) {
        let phase = ServoPhase::from_angle(angle);
        self.set_phase(phase);
    }

    /// Set the servo by a [ServoPhase], sending 5 pulses to the servo.
    pub fn set_phase(&mut self, phase: ServoPhase) {
        self.current_phase = phase;
        // To make sure the servo is in the right position, we send the pulse 5 times
        for _ in 0..5 {
            self.write_phase(phase);
        }
    }

    /// Send a single pulse to the servo with the given [ServoPhase].
    fn write_phase(&mut self, phase: ServoPhase) {
        // Start the pulse: set the pin high
        self.pin.set_high();
        // Wait for 1ms -- the minimum pulse width
        arduino_hal::delay_ms(1);
        // Wait for the microseconds specified by the phase
        arduino_hal::delay_us(phase.value);
        // The pulse is over, so set the pin low
        self.pin.set_low();
        // Wait for the next pulse -- 20ms - 1ms - ???µs = 18ms + (1000 - ???µs)
        arduino_hal::delay_ms(18);
        arduino_hal::delay_us(1000 - phase.value);
    }
}