#![deny(unsafe_code)]
#![no_main]
#![no_std]


use cortex_m_rt::entry;
use panic_halt as _;
use microbit::board::Board;
};


#[entry]
fn main() -> ! {
    // Init board
    let _board = microbit::Board::take().unwrap();

    // Add your code here
}
