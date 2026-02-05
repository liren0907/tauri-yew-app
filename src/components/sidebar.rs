use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let current_route = use_route::<Route>();
    let is_media_active = matches!(
        current_route,
        Some(Route::Media) | Some(Route::Video) | Some(Route::Image)
    );
    let theme = use_state(|| {
        window()
            .and_then(|w| w.local_storage().ok().flatten())
            .and_then(|s| s.get_item("theme").ok().flatten())
            .unwrap_or_else(|| "light".to_string())
    });

    {
        let theme = theme.clone();
        use_effect_with(theme, |theme| {
            let theme_str = (**theme).as_str();

            if let Some(document) = window().and_then(|w| w.document()) {
                if let Some(root) = document.document_element() {
                    let _ = root.set_attribute("data-theme", theme_str);
                }
            }

            if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
                let _ = storage.set_item("theme", theme_str);
            }
        });
    }

    let toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let next_theme = if *theme == "light" { "dark" } else { "light" };
            theme.set(next_theme.to_string());
        })
    };

    html! {
        <div class="flex flex-col h-full w-80 bg-base-100 text-base-content border-r border-base-content/10">
            <div class="px-8 py-10">
                <div class="flex items-center gap-4 group cursor-default">
                    <div class="bg-primary text-primary-content p-3 rounded-2xl transition-transform duration-300">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                        </svg>
                    </div>
                    <div>
                        <h1 class="text-2xl font-black tracking-tighter uppercase">{ "Yew App" }</h1>
                        <p class="text-[10px] font-bold opacity-40 uppercase tracking-widest leading-none">{ "Premium Dashboard" }</p>
                    </div>
                </div>
            </div>

            <div class="flex-1 px-4 space-y-6">
                <div>
                    <h2 class="px-4 mb-2 text-xs font-semibold text-base-content/40 uppercase tracking-widest">{ "Main Navigation" }</h2>
                    <ul class="menu menu-md vertical gap-2">
                        <li>
                            <Link<Route>
                                to={Route::Home}
                                classes={classes!(
                                    "rounded-xl", "px-4", "py-3", "flex", "items-center", "gap-4", "transition-all", "duration-200",
                                    if current_route == Some(Route::Home) { "active bg-primary/10 text-primary font-bold" } else { "hover:bg-base-content/5" }
                                )}
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
                                </svg>
                                { "Home" }
                            </Link<Route>>
                        </li>
                        <li>
                            <Link<Route>
                                to={Route::Media}
                                classes={classes!(
                                    "rounded-xl", "px-4", "py-3", "flex", "items-center", "gap-4", "transition-all", "duration-200",
                                    if is_media_active { "active bg-primary/10 text-primary font-bold" } else { "hover:bg-base-content/5" }
                                )}
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h14a2 2 0 012 2v10a2 2 0 01-2 2H9l-6 4v-4H5a2 2 0 01-2-2V5z" />
                                </svg>
                                { "Media Player" }
                            </Link<Route>>
                        </li>
                        <li>
                            <Link<Route>
                                to={Route::Chat}
                                classes={classes!(
                                    "rounded-xl", "px-4", "py-3", "flex", "items-center", "gap-4", "transition-all", "duration-200",
                                    if current_route == Some(Route::Chat) { "active bg-primary/10 text-primary font-bold" } else { "hover:bg-base-content/5" }
                                )}
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
                                </svg>
                                { "AI Chat" }
                            </Link<Route>>
                        </li>
                    </ul>
                </div>
            </div>

            <div class="p-6 mt-auto border-t border-base-content/5">
                <button
                    onclick={toggle_theme}
                    class="btn btn-ghost w-full justify-start items-center gap-4 rounded-2xl hover:bg-base-content/5 group transition-all duration-300"
                >
                    <div class="relative w-10 h-10 flex items-center justify-center rounded-xl bg-base-200 group-hover:bg-primary/10 group-hover:text-primary transition-colors">
                        {
                            if *theme == "light" {
                                html! {
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                                    </svg>
                                }
                            } else {
                                html! {
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 9h-1m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
                                    </svg>
                                }
                            }
                        }
                    </div>
                    <div class="flex flex-col items-start translate-y-[1px]">
                        <span class="text-sm font-bold tracking-tight">
                            { if *theme == "light" { "Dark Mode" } else { "Light Mode" } }
                        </span>
                        <span class="text-[10px] opacity-40 uppercase font-black tracking-widest leading-none">
                            { "Appearance" }
                        </span>
                    </div>
                </button>
            </div>
        </div>
    }
}
