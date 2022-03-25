#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_halt as _;

use xiao_rp2040 as bsp;

use bsp::hal;
use bsp::pac;

use bsp::hal::Clock;
use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;

#[bsp::entry]
fn main() -> ! {
    info!("Program start");

    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::watchdog::Watchdog::new(pac.WATCHDOG);
    let sio = hal::sio::Sio::new(pac.SIO);

    let clocks = hal::clocks::init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_green = pins.led_green.into_push_pull_output();
    let mut led_red = pins.led_red.into_push_pull_output();
    let mut led_blue = pins.led_blue.into_push_pull_output();

    led_green.set_high().unwrap();
    led_red.set_high().unwrap();
    led_blue.set_high().unwrap();

    loop {
        led_green.set_low().unwrap();
        delay.delay_ms(200);
        led_green.set_high().unwrap();
        led_red.set_low().unwrap();
        delay.delay_ms(200);
        led_red.set_high().unwrap();
        led_blue.set_low().unwrap();
        delay.delay_ms(200);
        led_blue.set_high().unwrap();
    }
}
