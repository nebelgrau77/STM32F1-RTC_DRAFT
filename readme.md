An attempt to use DS1307 RTC via I2C.

Currently not working correctly: the set_datetime doesn't seem to work.
Tried the same with different boards (STM32F0 and STM32F4) which HAL crates use I2c trait and not the BlockingI2c: 
same problem. 