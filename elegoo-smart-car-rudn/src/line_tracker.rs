//! The line tracker allows the robot to follow a line.
//! 
//! There are three separate sensors, to the left, right, and center.
//! Together, these can inform the robot on the direction to go to follow a line.

use arduino_hal::port::Pin;
use arduino_hal::port::mode::{Input, AnyInput};
use ufmt::derive::uDebug;

/// The state of a single line tracker.
/// 
/// Dark means that it is on the line, light means that it is not.
#[derive(uDebug, Clone, Copy)]
pub enum LineState {
    Light,
    Dark,
}

impl From<bool> for LineState {
    fn from(value: bool) -> Self {
        match value {
            true => LineState::Dark,
            false => LineState::Light,
        }
    }
}

/// Represents a choice of the three possible line trackers.
#[derive(uDebug, Clone, Copy)]
pub enum LineTrackerDirection {
    Left,
    Center,
    Right,
}

/// The result of the measurement of the three line trackers taken together.
#[derive(uDebug)]
pub struct LinePosition {
    left: LineState,
    mid: LineState,
    right: LineState,
}

/// The driver for the line tracker module board, which has three pins corresponding to each one of the three line trackers.
pub struct LineTracker {
    pin_left: Pin<Input<AnyInput>>,
    pin_center: Pin<Input<AnyInput>>,
    pin_right: Pin<Input<AnyInput>>,
}

impl LineTracker {
    pub fn new(pin_left: Pin<Input<AnyInput>>, pin_center: Pin<Input<AnyInput>>, pin_right: Pin<Input<AnyInput>>) -> Self {
        Self {
            pin_left,
            pin_center,
            pin_right,
        }
    }

    /// Measure a single line tracker in the specified direction.
    pub fn measure_direction(&mut self, direction: LineTrackerDirection) -> LineState {
        let pin = match direction {
            LineTrackerDirection::Left => &self.pin_left,
            LineTrackerDirection::Center => &self.pin_center,
            LineTrackerDirection::Right => &self.pin_right,
        };

        // The line tracker drives the pin low when it is on the line, and it is tied high otherwise.
        let state = pin.is_low();
        LineState::from(state)
    }

    /// Measure the three line trackers together, packed into a [LinePosition].
    pub fn measure_full(&mut self) -> LinePosition {
        LinePosition {
            left: LineState::from(self.pin_left.is_low()),
            mid: LineState::from(self.pin_center.is_low()),
            right: LineState::from(self.pin_right.is_low()),
        }
    }
}