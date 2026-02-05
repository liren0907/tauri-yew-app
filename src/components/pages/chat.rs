use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
}

#[derive(Deserialize)]
pub struct ChatResponse {
    pub message: Message,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ModelInfo {
    pub name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct TagsResponse {
    pub models: Vec<ModelInfo>,
}

#[function_component(ChatPage)]
pub fn chat_page() -> Html {
    let messages = use_state(|| Vec::<Message>::new());
    let input_value = use_state(|| "".to_string());
    let is_loading = use_state(|| false);

    let api_url = use_state(|| "http://localhost:11434".to_string());
    let available_models = use_state(|| Vec::<String>::new());
    let selected_model = use_state(|| "".to_string());
    let is_connecting = use_state(|| false);
    let connection_error = use_state(|| "".to_string());

    {
        let api_url = api_url.clone();
        let available_models = available_models.clone();
        let selected_model = selected_model.clone();
        let is_connecting = is_connecting.clone();
        let connection_error = connection_error.clone();

        use_effect_with(api_url.clone(), move |url| {
            let url = url.clone();

            wasm_bindgen_futures::spawn_local(async move {
                is_connecting.set(true);
                connection_error.set("".to_string());

                let fetch_url = format!("{}/api/tags", *url);
                let response = Request::get(&fetch_url).send().await;

                match response {
                    Ok(resp) => {
                        if resp.ok() {
                            let tags_resp: Result<TagsResponse, _> = resp.json().await;
                            match tags_resp {
                                Ok(data) => {
                                    let names: Vec<String> =
                                        data.models.into_iter().map(|m| m.name).collect();
                                    if !names.is_empty() {
                                        if selected_model.is_empty()
                                            || !names.contains(&*selected_model)
                                        {
                                            selected_model.set(names[0].clone());
                                        }
                                    }
                                    available_models.set(names);
                                }
                                Err(e) => {
                                    connection_error.set(format!("Failed to parse models: {}", e));
                                    available_models.set(Vec::new());
                                }
                            }
                        } else {
                            connection_error.set(format!("API Error: {}", resp.status()));
                            available_models.set(Vec::new());
                        }
                    }
                    Err(e) => {
                        connection_error.set(format!("Connection Failed: {}", e));
                        available_models.set(Vec::new());
                    }
                }
                is_connecting.set(false);
            });
        });
    }

    let on_submit = {
        let messages = messages.clone();
        let input_value = input_value.clone();
        let is_loading = is_loading.clone();
        let api_url = api_url.clone();
        let selected_model = selected_model.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if input_value.is_empty() || selected_model.is_empty() {
                return;
            }

            let user_msg = Message {
                role: "user".to_string(),
                content: (*input_value).clone(),
            };

            let mut new_messages = (*messages).clone();
            new_messages.push(user_msg.clone());
            messages.set(new_messages.clone());

            input_value.set("".to_string());
            is_loading.set(true);

            let messages_clone = messages.clone();
            let is_loading_clone = is_loading.clone();
            let url = (*api_url).clone();
            let model = (*selected_model).clone();

            wasm_bindgen_futures::spawn_local(async move {
                let request_body = ChatRequest {
                    model,
                    messages: new_messages,
                    stream: false,
                };

                let fetch_url = format!("{}/api/chat", url);
                let response = Request::post(&fetch_url)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&request_body).unwrap())
                    .send()
                    .await;

                match response {
                    Ok(resp) => {
                        if resp.ok() {
                            let chat_resp: Result<ChatResponse, _> = resp.json().await;
                            match chat_resp {
                                Ok(data) => {
                                    let mut current_msgs = (*messages_clone).clone();
                                    current_msgs.push(data.message);
                                    messages_clone.set(current_msgs);
                                }
                                Err(e) => {
                                    let mut current_msgs = (*messages_clone).clone();
                                    current_msgs.push(Message {
                                        role: "system".to_string(),
                                        content: format!("Error parsing response: {}", e),
                                    });
                                    messages_clone.set(current_msgs);
                                }
                            }
                        } else {
                            let mut current_msgs = (*messages_clone).clone();
                            current_msgs.push(Message {
                                role: "system".to_string(),
                                content: format!("API Error: {}", resp.status()),
                            });
                            messages_clone.set(current_msgs);
                        }
                    }
                    Err(e) => {
                        let mut current_msgs = (*messages_clone).clone();
                        current_msgs.push(Message {
                            role: "system".to_string(),
                            content: format!("Network Error: {}", e),
                        });
                        messages_clone.set(current_msgs);
                    }
                }
                is_loading_clone.set(false);
            });
        })
    };

    let onplaininput = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input_value.set(input.value());
        })
    };

    let on_url_change = {
        let api_url = api_url.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            api_url.set(input.value());
        })
    };

    let on_model_change = {
        let selected_model = selected_model.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            selected_model.set(select.value());
        })
    };

    html! {
        <div class="flex flex-col h-full w-full max-w-5xl mx-auto p-4 md:p-6 lg:p-8">
            <div class="flex-none flex flex-col md:flex-row items-start md:items-center justify-between mb-6 gap-4 bg-base-100/50 p-4 rounded-3xl border border-base-content/5">
                 <div>
                    <h1 class="text-3xl font-black tracking-tighter uppercase mb-1">{ "Ollama Chat" }</h1>
                    <div class="flex items-center gap-2 text-xs font-bold uppercase tracking-widest opacity-60">
                        if !(*connection_error).is_empty() {
                            <span class="w-2 h-2 rounded-full bg-error"></span>
                            <span class="text-error">{ &*connection_error }</span>
                        } else if *is_connecting {
                             <span class="loading loading-spinner loading-xs text-warning"></span>
                             <span class="text-warning">{ "Connecting..." }</span>
                        } else {
                            <span class="w-2 h-2 rounded-full bg-success"></span>
                            { "Connected" }
                        }
                    </div>
                </div>

                <div class="flex flex-col sm:flex-row gap-2 w-full md:w-auto">
                    <div class="flex items-center gap-2 bg-base-100 border border-base-content/10 px-3 py-2 rounded-xl w-full sm:w-64">
                         <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 opacity-40" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a2 2 0 00-5.656-5.656l-1.1 1.1" />
                        </svg>
                        <input
                            type="text"
                            class="bg-transparent border-none focus:outline-none text-sm font-medium w-full"
                            value={(*api_url).clone()}
                            onchange={on_url_change}
                            placeholder="API URL (e.g. http://localhost:11434)"
                        />
                    </div>

                    <div class="relative">
                        <select
                            class="appearance-none bg-base-100 border border-base-content/10 px-4 py-2 pr-10 rounded-xl text-sm font-bold min-w-[160px] w-full focus:outline-none focus:border-primary/50"
                            disabled={available_models.is_empty()}
                            onchange={on_model_change}
                        >
                            if available_models.is_empty() {
                                <option disabled=true selected=true>{ "No models found" }</option>
                            } else {
                                { for available_models.iter().map(|m| {
                                    html! {
                                        <option value={m.clone()} selected={m == &*selected_model}>{ m }</option>
                                    }
                                }) }
                            }
                        </select>
                        <div class="absolute right-3 top-1/2 -translate-y-1/2 pointer-events-none opacity-50">
                             <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                            </svg>
                        </div>
                    </div>
                </div>
            </div>

            <div class="flex-1 overflow-y-auto min-h-0 mb-6 space-y-4 pr-2 scrollbar-thin scrollbar-thumb-base-content/10">
                if messages.is_empty() {
                    <div class="h-full flex flex-col items-center justify-center opacity-20 select-none">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-24 w-24 mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                           <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
                        </svg>
                        <p class="text-xl font-bold uppercase tracking-widest">{ "Start a conversation" }</p>
                         if !(*available_models).is_empty() {
                            <p class="mt-2 text-sm opacity-60">{ format!("Connected to {}", *selected_model) }</p>
                        }
                    </div>
                }

                { for messages.iter().map(|msg| {
                    let is_user = msg.role == "user";
                    let bubble_class = if is_user {
                        "bg-primary text-primary-content rounded-tr-none ml-auto"
                    } else if msg.role == "system" {
                         "bg-error/10 text-error border border-error/20 rounded-xl mx-auto text-center w-full"
                    } else {
                        "bg-base-100 border border-base-content/10 rounded-tl-none mr-auto"
                    };

                    let align_class = if is_user { "justify-end" } else { "justify-start" };

                    html! {
                        <div class={classes!("flex", align_class, "gap-4")}>
                            if !is_user {
                                <div class="w-8 h-8 rounded-full bg-base-content/5 flex items-center justify-center flex-shrink-0">
                                     <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 opacity-50" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                                    </svg>
                                </div>
                            }

                            <div class={classes!("max-w-[80%]", "p-4", "rounded-2xl", "whitespace-pre-wrap", "leading-relaxed", bubble_class)}>
                                { &msg.content }
                            </div>

                             if is_user {
                                <div class="w-8 h-8 rounded-full bg-primary flex items-center justify-center flex-shrink-0">
                                     <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-primary-content" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                                    </svg>
                                </div>
                            }
                        </div>
                    }
                }) }

                 if *is_loading {
                    <div class="flex justify-start gap-4">
                        <div class="w-8 h-8 rounded-full bg-base-content/5 flex items-center justify-center flex-shrink-0">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 opacity-50" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                            </svg>
                        </div>
                         <div class="bg-base-100 border border-base-content/10 p-4 rounded-2xl rounded-tl-none flex items-center gap-2">
                             <span class="loading loading-dots loading-sm opacity-50"></span>
                         </div>
                    </div>
                }
            </div>

            <form onsubmit={on_submit} class="relative">
                <input
                    type="text"
                    value={(*input_value).clone()}
                    oninput={onplaininput}
                    placeholder={ if available_models.is_empty() { "Please select a model..." } else { "Message Ollama..." } }
                    disabled={*is_loading || available_models.is_empty()}
                    class="input input-lg w-full rounded-2xl pr-16 bg-base-100 border-base-content/10 focus:outline-none focus:border-primary/50 transition-all shadow-sm disabled:opacity-50 disabled:cursor-not-allowed"
                />
                <button
                    disabled={*is_loading || input_value.is_empty() || available_models.is_empty()}
                    class="absolute right-2 top-2 btn btn-circle btn-primary btn-sm h-10 w-10 min-h-0"
                >
                     if *is_loading {
                        <span class="loading loading-spinner loading-xs"></span>
                    } else {
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M12 5l7 7-7 7" />
                        </svg>
                    }
                </button>
            </form>
        </div>
    }
}
