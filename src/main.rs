#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtic::app;
use stm32f3xx_hal as _;

#[app(device = stm32f3xx_hal::stm32, peripherals = true, dispatchers = [TIM20_BRK, TIM20_UP, TIM20_TRG_COM])]
mod app {
    use dwt_systick_monotonic::DwtSystick;
    use rtic::time::duration::Seconds;
    use rtt_target::{rprintln, rtt_init_print};

    const MONO_HZ: u32 = 8_000_000; // 8 MHz
    #[monotonic(binds = SysTick, default = true)]
    type MyMono = DwtSystick<MONO_HZ>;

    #[init]
    fn init(cx: init::Context) -> (init::LateResources, init::Monotonics) {

        // Cortex-M peripherals
        let core : cortex_m::Peripherals = cx.core;
        let mut dcb = core.DCB;
        let dwt = core.DWT;
        let systick = core.SYST;
        let mono = DwtSystick::new(&mut dcb, dwt, systick, 8_000_000);

        rtt_init_print!(NoBlockSkip, 4096);
        rprintln!("init");

        // Not default
        let _h1: Result<foo::MyMono::SpawnHandle, ()> =
        foo::MyMono::spawn_at(monotonics::MyMono::now());  // <<<< Panicked here
        let handle: Result<foo::MyMono::SpawnHandle, ()> = foo::MyMono::spawn_after(Seconds(1_u32));
        let _h2: Result<foo::MyMono::SpawnHandle, ()> =
        handle.unwrap().reschedule_after(Seconds(1_u32));

        cortex_m::asm::nop();
 
        (init::LateResources {}, init::Monotonics(mono))
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {

        loop {
            cortex_m::asm::nop();
        }
    }

    #[task]
    fn foo(_: foo::Context) {
        rprintln!("foo");
        cortex_m::asm::nop();
    }

}