use web_sys::{File, HtmlInputElement, Url};
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum MediaKind {
    Image,
    Video,
}

fn detect_media_kind(file: &File) -> Option<MediaKind> {
    let mime = file.type_();
    if mime.starts_with("image/") {
        Some(MediaKind::Image)
    } else if mime.starts_with("video/") {
        Some(MediaKind::Video)
    } else {
        None
    }
}

#[function_component(MediaPlayer)]
pub fn media_player() -> Html {
    let media_url = use_state(|| None::<String>);
    let media_kind = use_state(|| None::<MediaKind>);
    let error_message = use_state(|| None::<String>);
    let file_input_ref = use_node_ref();

    let on_file_change = {
        let media_url = media_url.clone();
        let media_kind = media_kind.clone();
        let error_message = error_message.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let Some(files) = input.files() else {
                return;
            };
            let Some(file) = files.get(0) else {
                return;
            };

            if let Some(old_url) = (*media_url).as_ref() {
                Url::revoke_object_url(old_url).unwrap_or_else(|_| log::warn!("Failed to revoke media URL"));
            }

            match detect_media_kind(&file) {
                Some(kind) => {
                    match Url::create_object_url_with_blob(&file) {
                        Ok(url) => {
                            media_url.set(Some(url));
                            media_kind.set(Some(kind));
                            error_message.set(None);
                        }
                        Err(err) => {
                            media_url.set(None);
                            media_kind.set(None);
                            error_message.set(Some(format!("Error creating media URL: {:?}", err)));
                        }
                    }
                }
                None => {
                    media_url.set(None);
                    media_kind.set(None);
                    error_message.set(Some("Unsupported file type. Please select an image or video.".to_string()));
                }
            }
        })
    };

    let trigger_file_input = {
        let file_input_ref = file_input_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
                input.click();
            }
        })
    };

    {
        let media_url_handle = media_url.clone();
        use_effect_with(media_url_handle, move |url_state_handle| {
            let url_to_revoke = (**url_state_handle).clone();
            move || {
                if let Some(url) = url_to_revoke {
                    Url::revoke_object_url(&url)
                        .unwrap_or_else(|_| log::warn!("Failed to revoke media URL on unmount/change"));
                }
            }
        });
    }

    let has_media = (*media_url).is_some() && (*media_kind).is_some();

    html! {
        <div class="max-w-3xl mx-auto flex flex-col items-center text-center gap-8 py-12">
            <div class="space-y-2">
                <h2 class="text-3xl font-black tracking-tight self-center">{ "Media Player" }</h2>
                <p class="text-sm opacity-40 uppercase font-black tracking-widest">{ "Images & Video" }</p>
            </div>

            <input
                type="file"
                accept="image/*,video/*"
                ref={file_input_ref}
                class="hidden"
                onchange={on_file_change}
            />

            <button
                onclick={trigger_file_input}
                class="btn btn-primary btn-wide btn-lg rounded-2xl transition-all duration-300"
            >
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
                </svg>
                { "Load Media" }
            </button>

            if let Some(message) = (*error_message).as_ref() {
                <div class="alert alert-error rounded-2xl w-full">
                    <span class="font-medium">{ message }</span>
                </div>
            }

            <div class="w-full relative group">
                if has_media {
                    if let (Some(url), Some(kind)) = ((*media_url).as_ref(), *media_kind) {
                        {
                            match kind {
                                MediaKind::Image => html! {
                                    <div class="relative rounded-2xl overflow-hidden bg-base-200 p-2 ring-1 ring-base-content/5">
                                        <img
                                            src={url.clone()}
                                            alt="Loaded media"
                                            class="w-full rounded-xl object-contain max-h-[70vh] transition-transform duration-500"
                                        />
                                    </div>
                                },
                                MediaKind::Video => html! {
                                    <div class="relative rounded-2xl overflow-hidden bg-black aspect-video ring-1 ring-white/10">
                                        <video
                                            controls=true
                                            src={url.clone()}
                                            class="w-full h-full"
                                        >
                                            { "Your browser does not support the video tag." }
                                        </video>
                                    </div>
                                },
                            }
                        }
                    } else {
                        <div class="rounded-2xl py-20 px-4 border-2 border-dashed border-base-300"></div>
                    }
                } else {
                    <div class="rounded-2xl py-20 px-4 border-2 border-dashed border-base-300 flex flex-col items-center gap-4 hover:border-primary/50 transition-colors duration-300">
                        <div class="p-4 rounded-full bg-base-300/50 text-base-content/20">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-12 w-12" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
                            </svg>
                        </div>
                        <p class="text-base-content/40 font-medium">{ "Drop your media file or click to browse" }</p>
                    </div>
                }
            </div>
        </div>
    }
}
