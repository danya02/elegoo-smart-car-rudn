//! The HC-SR04 is an ultrasonic distance sensor.
//! 
//! It has two pins, Trig and Echo.
//! When you send a pulse on the Trig pin, the sensor will emit an ultrasonic pulse,
//! and the Echo pin will be high for a certain amount of time.
//! When the echo comes back, the Echo pin will go low, and the time that it was high for
//! is twice the distance between the sensor and the object.
//! 
//! For measuring the distance accurately, we use the TC1 timer, which has a resolution of 4µs,
//! which corresponds to a distance of 6805.5µm per tick.
//! The sensor measures distances between 2cm and about 4m. 


use arduino_hal::port::Pin;
use arduino_hal::port::mode::{Input, Output};

use ufmt::derive::uDebug;
use ufmt::uDisplay;

/// This struct represents a HC-SR04 sensor, holding references to Trig and Echo pins and the TC1 timer.
#[allow(non_camel_case_types)]
pub struct HC_SR04 {
    trigger_pin: Pin<Output>,
    echo_pin: Pin<Input>,
    tc1: arduino_hal::pac::TC1,
}

/// A measurement can come back with three states, which are as follows:
/// 
/// - `Measured(Distance)`: The measurement was successful, and its [`Distance`] is included.
/// - `Infinity`: The sensor's Echo pin was high for too long (more than 100ms, which corresponds to a distance of 17 meters),
///    and we say the distance is too large.
/// - `Unknown`: When we pulsed the Trig pin, the Echo pin did not go high for a while (750µs),
///   which means that the sensor didn't react to the pulse.
#[derive(uDebug)]
pub enum DistanceMeasurement {
    Infinity,
    Unknown,
    Measured(Distance),
}

/// A value of a distance measurement. Holds the number of timer ticks spent by the echo pin being high.
#[derive(uDebug)]
pub struct Distance {
    ticks: u16,  // bidirectional ticks, to get distance divide by 2
}

impl Distance {
    fn new(ticks: u16) -> Self {
        Self { ticks }
    }

    /// Returns the distance in micrometers.
    pub fn to_um(&self) -> u64 {
        // 1 tick = 4µs = 1.3611mm
        // https://www.wolframalpha.com/input?i=4%C2%B5s+speed+of+sound
        // divide by 2 -> 0.68055mm = 6805.5µm ~~ 6805µm

        // NOTE: we would prefer float values, but any program using them will halt at startup.

        let ums: u64 = self.ticks as u64 * 6805;
        ums
    }
    
    /// Returns the distance in millimeter.
    pub fn to_mm(&self) -> u64 {
        self.to_um() / 1000
    }
}

impl uDisplay for Distance {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        self.to_mm().fmt(f)?;
        f.write_str(&"mm")?;

        return Ok(());
    }
}

impl uDisplay for DistanceMeasurement {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        match self {
            DistanceMeasurement::Infinity => {
                f.write_str("∞")?;
            },
            DistanceMeasurement::Unknown => {
                f.write_str("Ø")?;
            },
            DistanceMeasurement::Measured(distance) => {
                distance.fmt(f)?;
            },
        }

        return Ok(());
    }
}

impl HC_SR04 {
    /// Creates a new HC-SR04 driver from the timer and the pins.
    /// 
    /// The timer is configured to have a prescaler of 64 to get a resolution of 4µs.
    pub fn new(tc1: arduino_hal::pac::TC1, trigger_pin: Pin<Output>, echo_pin: Pin<Input>) -> Self {
        // Configure the timer for the smallest available interval (prescaling 64)
        // which will count once per 4µs.
        // The timer will overflow after 65535 * 4µs = 262.14ms, which is plenty enough for this task.
        tc1.tccr1b.write(|w| w.cs1().prescale_64());


        Self {
            trigger_pin,
            echo_pin,
            tc1,
        }
    }

    /// Send an acoustic pulse and measure the distance between the sensor and the object.
    pub fn get_distance(&mut self) -> DistanceMeasurement {
        // Pulse the trigger pin for 10 µs as per the HC-SR04 datasheet
        self.trigger_pin.set_high();
        arduino_hal::delay_us(10);
        self.trigger_pin.set_low();
     
        // After the trigger pin is pulsed, audio pulses will begin.
        // After the pulses are sent, the echo pin will be set high (usually about 500µs, see hc-sr04-ping-delay.png)
        // The time that the echo pin is high is the in-flight time of the pulses.
        
        // If the pulses never return, the echo pin will stay high for about 130ms (see hc-sr04-infinity-time.png).
        // We will set the timeout to 100ms, which corresponds to a distance of about 17m -- after that we will return Infinity.

        // First wait for the echo pin to go high. This usually happens in about 500µs;
        // we wait in a loop, checking the echo pin, until it is high,
        // and if it isn't high in 750µs, we will return Unknown.
        // 750µs / (4µs per tick) = 187.5 = 188 ticks.

        self.tc1.tcnt1.write(|w| unsafe { w.bits(0) }); // Reset the timer

        while self.echo_pin.is_low() {
            if self.tc1.tcnt1.read().bits() > 188 {
                return DistanceMeasurement::Unknown;
            }
        }

        // Now the echo pin is high, so we reset the timer and wait for it to go low again.
        
        self.tc1.tcnt1.write(|w| unsafe { w.bits(0) }); // Reset the timer

        // Timeout is 100ms; 100ms / (4µs per tick) = 25000 ticks.
        self.tc1.tcnt1.write(|w| unsafe { w.bits(0) }); // Reset the timer

        while self.echo_pin.is_high() {
            if self.tc1.tcnt1.read().bits() > 25000 {
                return DistanceMeasurement::Infinity;
            }
        }

        // The echo pin is now low, so we know the pulse has returned.
        // Now return the distance.

        return DistanceMeasurement::Measured(Distance::new(self.tc1.tcnt1.read().bits()));
    }
}