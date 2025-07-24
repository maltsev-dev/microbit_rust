#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::hal::pac::TIMER0;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut light_it_all = [[0; 5]; 5];
    let dur: u32 = 10;

    fn pulse_pixel(
        display: &mut Display,
        timer: &mut Timer<TIMER0>,
        matrix: &mut [[u8; 5]; 5],
        row: usize,
        col: usize,
        dur: u32,) 
        {
        matrix[row][col] = 1;
        display.show(timer, *matrix, dur);
        matrix[row][col] = 0;
    }

    loop {
// Right along the top line
        for col in 0..5 {
            pulse_pixel(&mut display, &mut timer, &mut light_it_all, 0, col, dur);
        }

// Down Right
        for row in 1..5 {
            pulse_pixel(&mut display, &mut timer, &mut light_it_all, row, 4, dur);
        }
// Left along the bottom line
        for col in (0..5).rev() {
            pulse_pixel(&mut display, &mut timer, &mut light_it_all, 4, col, dur);
        }

// Up left (to second line)
        for row in (1..4).rev() {
            pulse_pixel(&mut display, &mut timer, &mut light_it_all, row, 0, dur);
        }
    }
}
