use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value)
        }
    };
    html! {
        <div class="container mx-auto text-xl">
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full" {onclick}>{"+1"}</button>
            <p>{*counter}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
