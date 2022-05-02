use arduino_hal::port::Pin;
use arduino_hal::port::mode::Output;
use embedded_hal::digital::v2::OutputPin;

pub(crate) struct MotorChassis {
    pin_enable_a: Pin<Output>,
    pin_enable_b: Pin<Output>,
    pin_a1: Pin<Output>,
    pin_a2: Pin<Output>,
    pin_b1: Pin<Output>,
    pin_b2: Pin<Output>,
}

pub enum ChassisDirection {
    Forward,
    Backward,
    Left,
    Right,
}

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

    pub fn set_enabled(&mut self, pair_a_en: bool, pair_b_en: bool){
        // This should not panic because setting state on Arduinos is infallible.
        self.pin_enable_a.set_state(pair_a_en.into()).unwrap();
        self.pin_enable_b.set_state(pair_b_en.into()).unwrap();
    }

}