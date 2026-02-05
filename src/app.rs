use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{ChatPage, HomePage, MediaPlayer, Sidebar};
use crate::routes::Route;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Media => html! { <MediaPlayer /> },
        Route::Video => html! { <MediaPlayer /> },
        Route::Image => html! { <MediaPlayer /> },
        Route::Chat => html! { <ChatPage /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="drawer lg:drawer-open">
                <input id="my-drawer" type="checkbox" class="drawer-toggle" />
                <div class="drawer-content flex flex-col h-screen overflow-hidden">
                    <div class="navbar bg-base-100 lg:hidden border-b border-base-content/5 flex-none">
                        <div class="flex-none">
                            <label for="my-drawer" class="btn btn-square btn-ghost">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-5 h-5 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
                            </label>
                        </div>
                        <div class="flex-1 px-2 mx-2 font-bold text-lg">{ "Yew Demo" }</div>
                    </div>

                    <main class="flex-1 overflow-y-auto bg-base-200">
                        <Switch<Route> render={switch} />
                    </main>
                </div>

                <div class="drawer-side z-20">
                    <label for="my-drawer" aria-label="close sidebar" class="drawer-overlay"></label>
                    <Sidebar />
                </div>
            </div>
        </BrowserRouter>
    }
}
