#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hal::rcc::RccExt;
use hal::stm32;
use stm32f4xx_hal as hal;

use stm32_flash::Flash;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(_cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        //setup CPU clock to 168MHz
        let rcc = p.RCC.constrain();
        let _clocks = rcc
            .cfgr
            .sysclk(stm32f4xx_hal::time::MegaHertz(168))
            .freeze();

        let flash = Flash::new(p.FLASH, 11);

        let mut value: u32;
        let offset = 0;

        value = flash.read(offset);

        hprintln!("{}", value).unwrap();

        value += 1;

        flash.erase().unwrap();
        flash.write(offset, &value).unwrap();
    }

    loop {}
}
