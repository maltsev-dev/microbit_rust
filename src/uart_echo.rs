#![no_main]
#![no_std]

use core::fmt::Write;

use core::str;
use cortex_m_rt::entry;
use heapless::Vec;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::hal::uarte::{self, Baudrate, Parity};

use microbit_rust::UartePort;

/*
    connect putty terminal to the micro:bit
    set baud rate to 115200
    open session
    send message to server with ENTER
*/

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        let byte = serial.read().unwrap();

        if byte != b'\n' && byte != b'\r' && byte != 13 {
            buffer.push(byte).unwrap();
        } else {
            buffer.push(b'\n').unwrap();
            buffer.push(b'\r').unwrap();
            if let Ok(s) = str::from_utf8(&buffer) {
                serial.write_str(s).unwrap();
                serial.flush().unwrap();
            } else {
                serial.write_str("<invalid utf8>").unwrap();
                serial.flush().unwrap();
            }
            buffer.clear();
        }
    }
}
