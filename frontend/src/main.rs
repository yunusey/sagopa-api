mod components;

use components::MainApp;

fn main() {
    yew::Renderer::<MainApp>::new().render();
}
