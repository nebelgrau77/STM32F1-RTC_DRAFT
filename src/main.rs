//! simple weather station using a Bosch BME280 sensor
//! and an SSD1306 OLED display
//! 
//! in order to share the I2C bus between the two devices
//! the shared-bus crate is used
//! 
//! this project uses the STM32F103C8T6 "blue pill" board
//! 
//! the BME280 driver requires BlockingI2c trait
//! available in the the STM32F1xx-HAL crate but not in STM32F0xx nor STM32F4xx
//! 
//! as the BME280 initialization consumes the delay instance
//! the delay in the loop is done blocking the program for n instructions


#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32f1xx_hal as hal;

extern crate cortex_m;
extern crate ds1307;

use cortex_m_semihosting::hprintln;

use ds1307::{DateTime, Hours, DS1307};

use cortex_m_rt::entry;

use hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::*,
    stm32,
    delay::Delay,
    
};


#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);
    
    
    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    // delay provider
    let mut delay = Delay::new(cp.SYST, clocks);
    
    
    let mut rtc = DS1307::new(i2c);
    
    
    let now = DateTime {
        year: 2020,
        month: 02,
        day: 28,
        weekday: 5,
        hour: Hours::H24(13),
        minute: 55,
        second: 00,
    };

    // this does not seem to work correctly

    rtc.set_datetime(&now).unwrap();

    

    loop {
        
        let mut current = rtc.get_datetime().unwrap();
            
        match current.hour {Hours::H24(h) => hprintln!("time is {:02}:{:02}:{:02}",
                h,
                current.minute,
                current.second),

                Hours::AM(h) => hprintln!("time is {:02}:{:02}:{:02} AM",
                h,
                current.minute,
                current.second),
            
                Hours::PM(h) => hprintln!("time is {:02}:{:02}:{:02} PM",
                h,
                current.minute,
                current.second),
            };

       
    
        delay.delay_ms(1000_u16);

    }

}
