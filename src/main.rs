#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use stm32f4xx_hal::{
    prelude::*,
    pac,
    serial::{
        config::Config, 
        Serial
    },
    adc::{
        config::AdcConfig, 
        config::SampleTime, 
        Adc
    },
};
use core::fmt::Write;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Init peripherals
    let dp = pac::Peripherals::take().unwrap();
    let _core = cortex_m::Peripherals::take().unwrap();

    // Configure clock
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();
    
    // Init GPIOA
    let gpioa = dp.GPIOA.split();

    // Configure PA0 to analog input
    let adc_pin = gpioa.pa0.into_analog();
    let mut led_pin = gpioa.pa5.into_push_pull_output();

    // Configure UART (USART2 on PA2 (TX) and PA3 (RX))
    let tx_pin = gpioa.pa2.into_alternate();
    let rx_pin = gpioa.pa3.into_alternate();
    let serial = Serial::usart2(
        dp.USART2,
        (tx_pin, rx_pin),
        Config::default().baudrate(115200.bps()),
        clocks,
    ).unwrap();

    // Split the serial into transmitter and receiver
    let (mut tx, _rx) = serial.split();

    // Initialize the ADC
    let mut adc = Adc::adc1(dp.ADC1, true, AdcConfig::default());

    loop {
        // Read ADC value from PA0
        let adc_read_value = adc.convert(&adc_pin, SampleTime::Cycles_84);

        // Print ADC value to RTT
        rprintln!("ADC Value: {}", adc_read_value);

        // Send ADC value over UART
        let _ = write!(tx, "ADC Value: {}\r\n", adc_read_value);

        // Toggle LED
        led_pin.toggle();
        
        // Add delay
        cortex_m::asm::delay(8_000_000); // ~1 second delay at 84 MHz
    }
}