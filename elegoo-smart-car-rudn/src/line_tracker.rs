use arduino_hal::port::Pin;
use arduino_hal::port::mode::{Input, AnyInput};
use ufmt::derive::uDebug;

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

#[derive(uDebug, Clone, Copy)]
pub enum LineTrackerDirection {
    Left,
    Center,
    Right,
}

#[derive(uDebug)]
pub struct LinePosition {
    left: LineState,
    mid: LineState,
    right: LineState,
}

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

    pub fn measure_direction(&mut self, direction: LineTrackerDirection) -> LineState {
        let pin = match direction {
            LineTrackerDirection::Left => &self.pin_left,
            LineTrackerDirection::Center => &self.pin_center,
            LineTrackerDirection::Right => &self.pin_right,
        };

        let state = pin.is_high();
        LineState::from(state)
    }

    pub fn measure_full(&mut self) -> LinePosition {
        LinePosition {
            left: LineState::from(self.pin_left.is_low()),
            mid: LineState::from(self.pin_center.is_low()),
            right: LineState::from(self.pin_right.is_low()),
        }
    }

}