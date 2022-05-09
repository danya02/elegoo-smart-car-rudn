//! Functions for a real-time measurement of time.
//! 
//! Using the TC0 timer, we set up interrupts to measure milliseconds.
//! You can use the [millis] function to get the time since the program was started.
//!
//! Code taken from: https://github.com/Rahix/avr-hal/blob/main/examples/arduino-uno/src/bin/uno-millis.rs

use core::cell;

// Possible Values:
//
// ╔═══════════╦══════════════╦═══════════════════╗
// ║ PRESCALER ║ TIMER_COUNTS ║ Overflow Interval ║
// ╠═══════════╬══════════════╬═══════════════════╣
// ║        64 ║          250 ║              1 ms ║
// ║       256 ║          125 ║              2 ms ║
// ║       256 ║          250 ║              4 ms ║
// ║      1024 ║          125 ║              8 ms ║
// ║      1024 ║          250 ║             16 ms ║
// ╚═══════════╩══════════════╩═══════════════════╝
const PRESCALER: u64 = 64;
const TIMER_COUNTS: u64 = 250;

/// The number of milliseconds that pass between timer overflows.
const MILLIS_INCREMENT: u64 = PRESCALER * TIMER_COUNTS / 16000;

/// The counter used to keep track of the number of milliseconds.
static MILLIS_COUNTER: avr_device::interrupt::Mutex<cell::Cell<u64>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(0));

/// Function to initialize timer TC0's interrupt to increment the millisecond counter.
pub fn millis_init(tc0: arduino_hal::pac::TC0) {
    // Configure the timer for the above interval (in CTC mode)
    // and enable its interrupt.
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| unsafe { w.bits(TIMER_COUNTS as u8) });
    tc0.tccr0b.write(|w| match PRESCALER {
        8 => w.cs0().prescale_8(),
        64 => w.cs0().prescale_64(),
        256 => w.cs0().prescale_256(),
        1024 => w.cs0().prescale_1024(),
        _ => panic!(),
    });
    tc0.timsk0.write(|w| w.ocie0a().set_bit());

    // Reset the global millisecond counter
    avr_device::interrupt::free(|cs| {
        MILLIS_COUNTER.borrow(cs).set(0);
    });
}

/// Function to increment the global millisecond counter on each timer interrupt.
#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    avr_device::interrupt::free(|cs| {
        let counter_cell = MILLIS_COUNTER.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + MILLIS_INCREMENT);
    })
}

/// Get how many milliseconds have passed since the program started, or since the last timer reset.
pub fn millis() -> u64 {
    avr_device::interrupt::free(|cs| MILLIS_COUNTER.borrow(cs).get())
}

/// Set the millisecond counter to zero. 
pub fn reset_millis() {
    avr_device::interrupt::free(|cs| MILLIS_COUNTER.borrow(cs).set(0));
}