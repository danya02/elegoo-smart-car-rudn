//! The L287N motor driver drives the two motors on the robot.
//! 
//! It is controlled by 6 pins: two to set the direction for each motor, and two to enable the motor pairs.

use arduino_hal::port::Pin;
use arduino_hal::port::mode::Output;
use embedded_hal::digital::v2::OutputPin;

/// The driver for the motor driver. 
pub struct MotorChassis {
    pin_enable_a: Pin<Output>,
    pin_enable_b: Pin<Output>,
    pin_a1: Pin<Output>,
    pin_a2: Pin<Output>,
    pin_b1: Pin<Output>,
    pin_b2: Pin<Output>,
}

/// The direction for the robot to go.
/// 
/// Rotations are tank-style, with the pairs moving in opposite directions.
pub enum ChassisDirection {
    Forward,
    Backward,
    Left,
    Right,
}

/// The direction for a single motor to go.
pub enum PairDirection {
    Forward,
    Backward,
}

impl MotorChassis {
    pub fn new(
        pin_enable_a: Pin<Output>,
        pin_enable_b: Pin<Output>,
        pin_a1: Pin<Output>,
        pin_a2: Pin<Output>,
        pin_b1: Pin<Output>,
        pin_b2: Pin<Output>,
    ) -> Self {
        Self {
            pin_enable_a,
            pin_enable_b,
            pin_a1,
            pin_a2,
            pin_b1,
            pin_b2,
        }
    }

    /// Set the direction for the A motor (the left one).
    ///
    /// Only sets the direction pins, does not change the state of the motor:
    /// if the motor is currently running, it will continue to run in the new direction,
    /// and if it is not running it will stay not running. 
    fn set_pair_a_direction(&mut self, direction: PairDirection){
        match direction {
            PairDirection::Forward => {
                self.pin_a1.set_high();
                self.pin_a2.set_low();
            },
            PairDirection::Backward => {
                self.pin_a1.set_low();
                self.pin_a2.set_high();
            },
        }
    }

    /// Set the direction for the B motor (the right one).
    ///
    /// Only sets the direction pins, does not change the state of the motor:
    /// if the motor is currently running, it will continue to run in the new direction,
    /// and if it is not running it will stay not running. 
    fn set_pair_b_direction(&mut self, direction: PairDirection){
        match direction {
            PairDirection::Forward => {
                self.pin_b2.set_high();
                self.pin_b1.set_low();
            },
            PairDirection::Backward => {
                self.pin_b2.set_low();
                self.pin_b1.set_high();
            },
        }
    }
    /// Set the direction for both motors.
    ///
    /// Only sets the direction pins, does not change the state of the motor:
    /// if the motor is currently running, it will continue to run in the new direction,
    /// and if it is not running it will stay not running. 
    pub fn set_direction(&mut self, direction: ChassisDirection){
        match direction {
            ChassisDirection::Forward => {
                self.set_pair_a_direction(PairDirection::Forward);
                self.set_pair_b_direction(PairDirection::Forward);
            },
            ChassisDirection::Backward => {
                self.set_pair_a_direction(PairDirection::Backward);
                self.set_pair_b_direction(PairDirection::Backward);
            },
            ChassisDirection::Left => {
                self.set_pair_a_direction(PairDirection::Backward);
                self.set_pair_b_direction(PairDirection::Forward);

            },
            ChassisDirection::Right => {
                self.set_pair_a_direction(PairDirection::Forward);
                self.set_pair_b_direction(PairDirection::Backward);
            },
        }
    }

    /// Set the enablement state for both motors.
    ///
    /// This is separate from setting the direction for the motors.
    /// First you need to set the direction, then run the motors with the needed direction.
    pub fn set_enabled(&mut self, pair_a_en: bool, pair_b_en: bool){
        // This should not panic because setting state on Arduinos is infallible.
        self.pin_enable_a.set_state(pair_a_en.into()).unwrap();
        self.pin_enable_b.set_state(pair_b_en.into()).unwrap();
    }
}