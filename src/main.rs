use nes_emu::{bus::Bus, cpu::Cpu, joypad::Joypad};
use yew::prelude::*;

#[function_component(NesEmulator)]
fn nes_emulator() -> Html {
    let mut memory = [0; 0xFFFF];
    let mut joypad_1 = Joypad::new(&mut memory[0x4016]);
    let mut joypad_2 = Joypad::new(&mut memory[0x4017]);
    let mut bus = Bus::new(&mut memory, &mut joypad_1, &mut joypad_2);
    let mut _cpu = Cpu::new(&mut bus);

    html! {
        <h1>{ "NES Emulator" }</h1>
    }
}

fn main() {
    yew::start_app::<NesEmulator>();
}
