#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32f0xx_hal as hal;
extern crate ds1307;

use cortex_m_rt::entry;

use cortex_m_semihosting::hprintln;

use crate::hal::{
    prelude::*,
    i2c::I2c,
    delay::Delay,
    stm32,
    };

use ds1307::{DateTime, Hours, DS1307};

#[entry]
fn main() -> ! {

    if let (Some(mut p), Some(cp)) = (stm32::Peripherals::take(),cortex_m::peripheral::Peripherals::take()) {
        
        cortex_m::interrupt::free(move |cs| {

        let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);
        
        let gpiob = p.GPIOB.split(&mut rcc);
        let gpioc = p.GPIOC.split(&mut rcc);
        let scl = gpiob.pb8.into_alternate_af1(cs);
        let sda = gpiob.pb9.into_alternate_af1(cs);
        let mut led = gpioc.pc13.into_push_pull_output(cs);
        let i2c = I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), &mut rcc);
        
        // Get delay provider
        let mut delay = Delay::new(cp.SYST, &rcc);
        
        // set up RTC
        let mut rtc = DS1307::new(i2c);
               
        
        let now = DateTime {
            year: 2020,
            month: 02,
            day: 28,
            weekday: 5,
            hour: Hours::H24(12),
            minute: 40,
            second: 00,
        };

        rtc.set_datetime(&now).unwrap();
        

        


        loop {
        
            let mut current = rtc.get_datetime().unwrap();
            
            match current.hour {Hours::H24(h) => hprintln!("date is: {:04}/{:02}/{:02}, time is {:02}:{:02}:{:02}",
                current.year, 
                current.month,
                current.day,
                h,
                current.minute,
                current.second),

                Hours::AM(h) => hprintln!("date is: {:04}/{:02}/{:02}, time is {:02}:{:02}:{:02}",
                current.year, 
                current.month,
                current.day,
                h,
                current.minute,
                current.second),
            
                Hours::PM(h) => hprintln!("date is: {:04}/{:02}/{:02}, time is {:02}:{:02}:{:02}",
                current.year, 
                current.month,
                current.day,
                h,
                current.minute,
                current.second),
            };

            led.toggle().unwrap();
        
            delay.delay_ms(1000_u16);
        

        }

        

    });
    
}

    loop {continue;}
    
}





