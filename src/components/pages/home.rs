use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div class="min-h-[70vh] flex flex-col items-center justify-center text-center">
            <div class="max-w-2xl px-6">
                <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-primary/10 text-primary text-xs font-bold uppercase tracking-widest mb-6">
                    <span class="relative flex h-2 w-2">
                        <span class="relative inline-flex rounded-full h-2 w-2 bg-primary"></span>
                    </span>
                    { "New Premium Design" }
                </div>
                <h1 class="text-6xl md:text-7xl font-black tracking-tighter mb-8 leading-tight">
                    { "Build Faster with " }
                    <span class="text-primary">
                        { "Yew & DaisyUI" }
                    </span>
                </h1>
                <p class="mb-10 text-xl opacity-60 leading-relaxed font-medium">
                    { "Experience a seamless developer experience with Rust, WASM, and a modern utility-first UI kit. Everything you need for the next-gen web." }
                </p>
                <div class="flex flex-col sm:flex-row gap-6 justify-center">
                    <Link<Route>
                        to={Route::Media}
                        classes="btn btn-primary btn-lg rounded-2xl px-10 transition-all duration-300"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
                        </svg>
                        { "Media Player" }
                    </Link<Route>>
                </div>
            </div>
        </div>
    }
}
