#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_halt as _;

use xiao_rp2040 as bsp;

use bsp::hal;
use bsp::pac;

use embedded_hal::digital::v2::OutputPin;
use embedded_time::rate::*;
use hal::clocks::Clock;
use hal::pio::PIOExt;

use smart_leds::brightness;
use smart_leds::SmartLedsWrite;
use smart_leds::RGB;

use ws2812_pio::Ws2812;

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

    let sin = hal::rom_data::float_funcs::fsin::ptr();

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);

    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    let mut ws = Ws2812::new(
        pins.neopix.into_mode(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );
    let mut led: RGB<u8> = (0, 0, 0).into();
    let mut t = 0.0;

    let strip_brightness = 64u8;

    let animation_speed = 0.1;

    let mut neo_pow = pins.neo_pow.into_push_pull_output();
    neo_pow.set_high().unwrap();

    loop {
        let sin_11 = sin(t * 2.0 * core::f32::consts::PI);
        let sin_01 = (sin_11 + 1.0) * 0.5;

        let hue = 360.0 * sin_01;
        let sat = 1.0;
        let val = 1.0;

        let rgb = hsv2rgb_u8(hue, sat, val);
        led = rgb.into();

        ws.write(brightness([led].iter().copied(), strip_brightness))
            .unwrap();

        delay.delay_ms(16);

        t += (16.0 / 1000.0) * animation_speed;
        while t > 1.0 {
            t -= 1.0;
        }
    }
}

pub fn hsv2rgb(hue: f32, sat: f32, val: f32) -> (f32, f32, f32) {
    let c = val * sat;
    let v = (hue / 60.0) % 2.0 - 1.0;
    let v = if v < 0.0 { -v } else { v };
    let x = c * (1.0 - v);
    let m = val - c;
    let (r, g, b) = if hue < 60.0 {
        (c, x, 0.0)
    } else if hue < 120.0 {
        (x, c, 0.0)
    } else if hue < 180.0 {
        (0.0, c, x)
    } else if hue < 240.0 {
        (0.0, x, c)
    } else if hue < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (r + m, g + m, b + m)
}

pub fn hsv2rgb_u8(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let r = hsv2rgb(h, s, v);

    (
        (r.0 * 255.0) as u8,
        (r.1 * 255.0) as u8,
        (r.2 * 255.0) as u8,
    )
}
