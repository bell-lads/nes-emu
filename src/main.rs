use nes_emu::Nes;
use yew::prelude::*;

#[function_component(NesEmulator)]
fn nes_emulator() -> Html {
    let nes = Nes::new();

    html! {
        <h1>{ "NES Emulator" }</h1>
    }
}

fn main() {
    yew::start_app::<NesEmulator>();
}
