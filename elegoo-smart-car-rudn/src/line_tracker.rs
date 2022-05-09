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

/// The direction that the robot is offset from the line.
pub enum LineBiasDirection {
    /// The robot only sees the line on the left.
    VeryLeft,
    /// The robot sees the line in the center and on the left.
    SlightlyLeft,
    /// The robot sees the line only in the center.
    Center,
    /// The robot sees the line in the center and on the right.
    SlightlyRight,
    /// The robot only sees the line on the right.
    VeryRight,
    /// The robot does not see a line.
    NotOnLine,
    /// The robot sees a line on all three sensors at the same time.
    OnPerpendicularLine,
}

impl LineBiasDirection {
    /// Converts the line bias direction to a line tracker direction, losing some information.
    /// 
    /// Converts `VeryLeft` and `SlightlyLeft` to `Left`, `VeryRight` and `SlightlyRight` to `Right`,
    /// and everything else to `Center`.
    pub fn to_line_tracker_direction(self) -> LineTrackerDirection {
        match self {
            LineBiasDirection::VeryLeft | LineBiasDirection::SlightlyLeft => LineTrackerDirection::Left,
            LineBiasDirection::VeryRight | LineBiasDirection::SlightlyRight => LineTrackerDirection::Right,
            _ => LineTrackerDirection::Center,
        }
    }
}


impl LinePosition {
    /// Returns the direction that the sensor state is pointing to,
    /// when the robot is following a dark line on a light background.
    /// 
    /// When you want to follow a line, you want the line to be visible on the center sensor only.
    /// If you can see the line on the left or right sensor, you want to compensate
    /// by turning in the opposite direction.
    pub fn get_bias_direction_dark(&self) -> LineBiasDirection {
        match (self.left, self.mid, self.right) {
            (LineState::Light, LineState::Light, LineState::Dark) => LineBiasDirection::VeryRight,
            (LineState::Light, LineState::Dark, LineState::Dark) => LineBiasDirection::SlightlyRight,
            (LineState::Light, LineState::Dark, LineState::Light) => LineBiasDirection::Center,
            (LineState::Dark, LineState::Dark, LineState::Light) => LineBiasDirection::SlightlyLeft,
            (LineState::Dark, LineState::Light, LineState::Light) => LineBiasDirection::VeryLeft,
            
            (LineState::Light, LineState::Light, LineState::Light) => LineBiasDirection::NotOnLine,
            (LineState::Dark, LineState::Dark, LineState::Dark) => LineBiasDirection::OnPerpendicularLine,

            // This corresponds to the situation where the robot is on two lines to the left and right.
            // Usually this will not happen for a robot on a single line, but it is possible,
            // so this state is interpreted as not being on a line.
            (LineState::Dark, LineState::Light, LineState::Dark) => LineBiasDirection::NotOnLine,
        }
    }

    /// Returns the direction that the sensor state is pointing to,
    /// when the robot is following a light line on a dark background.
    /// 
    /// When you want to follow a line, you want the line to be visible on the center sensor only.
    /// If you can see the line on the left or right sensor, you want to compensate
    /// by turning in the opposite direction.
    pub fn get_bias_direction_light(&self) -> LineBiasDirection {
        match (self.left, self.mid, self.right) {
            (LineState::Dark, LineState::Dark, LineState::Light) => LineBiasDirection::VeryRight,
            (LineState::Dark, LineState::Light, LineState::Light) => LineBiasDirection::SlightlyRight,
            (LineState::Dark, LineState::Light, LineState::Dark) => LineBiasDirection::Center,
            (LineState::Light, LineState::Light, LineState::Dark) => LineBiasDirection::SlightlyLeft,
            (LineState::Light, LineState::Dark, LineState::Dark) => LineBiasDirection::VeryLeft,
            
            (LineState::Dark, LineState::Dark, LineState::Dark) => LineBiasDirection::NotOnLine,
            (LineState::Light, LineState::Light, LineState::Light) => LineBiasDirection::OnPerpendicularLine,

            // This corresponds to the situation where the robot is on two lines to the left and right.
            // Usually this will not happen for a robot on a single line, but it is possible,
            // so this state is interpreted as not being on a line.
            (LineState::Light, LineState::Dark, LineState::Light) => LineBiasDirection::NotOnLine,
        }
    }
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