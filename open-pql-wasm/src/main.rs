use std::io;

use open_pql::StatementsRunner;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub struct PqlResult {
    pub output: String,
    pub error: Option<String>,
}

fn execute_pql(query: &str) -> PqlResult {
    let out_stream = Box::new(Vec::<u8>::new());
    let err_stream = Box::new(Vec::<u8>::new());

    let mut runner =
        StatementsRunner::new(query, 10000, 1, out_stream, err_stream);

    runner.run();

    let StatementsRunner {
        stream_out,
        stream_err,
        ..
    } = runner;

    let output = to_string(stream_out);
    let error_output = to_string(stream_err);

    if error_output.is_empty() {
        PqlResult {
            output,
            error: None,
        }
    } else {
        PqlResult {
            output,
            error: Some(error_output),
        }
    }
}

fn to_string(stream: Box<dyn io::Write>) -> String {
    let ptr: *mut Vec<u8> = Box::into_raw(stream).cast();
    unsafe {
        String::from_utf8(*Box::from_raw(ptr))
            .unwrap_or_else(|_| "Invalid UTF-8 output".to_string())
    }
}

fn render_example_query(
    title: &str,
    query: &str,
    pql_input: UseStateHandle<String>,
) -> Html {
    let query = query.to_string();
    let title = title.to_string();

    let on_click = {
        let query = query.clone();
        Callback::from(move |_| {
            pql_input.set(query.clone());
        })
    };

    html! {
        <div class="bg-slate-50 border border-slate-200 rounded-lg p-4 hover:bg-slate-100 transition-colors cursor-pointer" onclick={on_click}>
            <h3 class="font-semibold text-slate-800 mb-2">{title}</h3>
            <code class="text-xs text-slate-600 font-mono bg-white px-2 py-1 rounded border leading-relaxed block overflow-x-auto">
                {query}
            </code>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let pql_input = use_state(String::new);
    let result = use_state(|| PqlResult {
        output: String::new(),
        error: None,
    });

    let on_input_change = {
        let pql_input = pql_input.clone();
        Callback::from(move |e: InputEvent| {
            let target: HtmlTextAreaElement = e.target_unchecked_into();
            pql_input.set(target.value());
        })
    };

    let on_run_click = {
        let pql_input = pql_input.clone();
        let result = result.clone();
        Callback::from(move |_| {
            let query = (*pql_input).clone();

            if query.trim().is_empty() {
                result.set(PqlResult {
                    output: String::new(),
                    error: Some("No query provided".to_string()),
                });
                return;
            }

            let pql_result = execute_pql(&query);
            result.set(pql_result);
        })
    };

    html! {
        <div class="min-h-screen bg-gradient-to-br from-slate-50 to-blue-50">
            <div class="container mx-auto px-6 py-12 max-w-6xl">
                <div class="bg-white/80 backdrop-blur-sm rounded-2xl shadow-xl border border-gray-200/50 overflow-hidden">
                    <div class="bg-gradient-to-r from-blue-600 to-indigo-700 px-8 py-6">
                        <h1 class="text-4xl font-bold text-white text-center tracking-tight">
                            {"Open PQL"}
                        </h1>
                        <p class="text-blue-100 text-center mt-2 text-lg font-medium">
                            {"WebAssembly Query Interface"}
                        </p>
                        <div class="text-center mt-6">
                            <a href="https://github.com/solve-poker/open-pql" 
                               target="_blank" 
                               rel="noopener noreferrer"
                               class="inline-flex items-center px-6 py-3 bg-white/20 hover:bg-white/30 text-white font-semibold rounded-xl border-2 border-white/30 hover:border-white/50 transition-all duration-200 backdrop-blur-sm shadow-lg hover:shadow-xl transform hover:-translate-y-1">
                                <svg class="w-6 h-6 mr-3" fill="currentColor" viewBox="0 0 20 20">
                                    <path fill-rule="evenodd" d="M10 0C4.477 0 0 4.484 0 10.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0110 4.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.203 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.942.359.31.678.921.678 1.856 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0020 10.017C20 4.484 15.522 0 10 0z" clip-rule="evenodd"></path>
                                </svg>
                                {"View Source on GitHub"}
                                <svg class="w-4 h-4 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"></path>
                                </svg>
                            </a>
                        </div>
                    </div>

                    <div class="p-8">
                        <div class="mb-8 p-4 bg-amber-50 border-l-4 border-amber-400 rounded-r-lg">
                            <div class="flex items-center">
                                <div class="flex-shrink-0">
                                    <span class="text-2xl">{"⚠️"}</span>
                                </div>
                                <div class="ml-3">
                                    <p class="text-amber-800 font-medium">
                                        {"Development Preview"}
                                    </p>
                                    <p class="text-amber-700 text-sm mt-1">
                                        {"This interface is under active development and may not be suitable for production use."}
                                    </p>
                                </div>
                            </div>
                        </div>

                        <div class="mb-8 p-4 bg-blue-50 border-l-4 border-blue-400 rounded-r-lg">
                            <div class="flex items-center">
                                <div class="flex-shrink-0">
                                    <span class="text-2xl">{"🌐"}</span>
                                </div>
                                <div class="ml-3">
                                    <p class="text-blue-800 font-medium">
                                        {"Browser Performance Notice"}
                                    </p>
                                    <p class="text-blue-700 text-sm mt-1">
                                        {"This runs in your browser. For full speed performance, please use the CLI instead."}
                                    </p>
                                </div>
                            </div>
                        </div>

                        <div class="mb-8">
                            <h2 class="text-xl font-bold text-slate-800 mb-4">{"Example Queries"}</h2>
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                {render_example_query("Average equity calculation", "select avg(equity(hero, river)) from game='holdem', hero='TsAc', villain='JsQs', board='2s3s4s'", pql_input.clone())}
                                {render_example_query("Max flop hand category", "select max(flopHandCategory(hero)) from game='holdem', hero='7h Ah', board='7s 8h Tc'", pql_input.clone())}
                            </div>
                        </div>

                        <div class="space-y-8">
                            // PQL Input Section - Full Width
                            <div class="space-y-4">
                                <div class="flex items-center justify-between">
                                    <label class="text-lg font-semibold text-slate-700">
                                        {"Query Editor"}
                                    </label>
                                    <span class="text-sm text-slate-500 bg-slate-100 px-2 py-1 rounded-full">
                                        {"PQL"}
                                    </span>
                                </div>
                                <textarea
                                    class="w-full p-4 border-2 border-slate-200 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-all duration-200 resize-none font-mono text-sm bg-slate-50 hover:bg-white shadow-sm"
                                    rows="12"
                                    placeholder="// Enter your PQL query here...
// Example:
SELECT * FROM table_name
WHERE condition = 'value'"
                                    value={(*pql_input).clone()}
                                    oninput={on_input_change}
                                />
                                <div class="flex justify-center">
                                    <button
                                        class="group inline-flex items-center px-6 py-3 bg-gradient-to-r from-blue-600 to-indigo-600 text-white font-semibold rounded-xl hover:from-blue-700 hover:to-indigo-700 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-all duration-200 shadow-lg hover:shadow-xl transform hover:-translate-y-0.5"
                                        onclick={on_run_click}
                                    >
                                        <span class="mr-2">{"▶"}</span>
                                        {"Execute Query"}
                                    </button>
                                </div>
                            </div>

                            // Output Section - Full Width Below Editor
                            <div class="space-y-4">
                                <div class="flex items-center justify-between">
                                    <label class="text-lg font-semibold text-slate-700">
                                        {"Results"}
                                    </label>
                                    <div class="flex items-center space-x-2">
                                        {
                                            if result.error.is_some() {
                                                html! {
                                                    <span class="text-sm text-red-600 bg-red-50 px-2 py-1 rounded-full flex items-center">
                                                        <span class="w-2 h-2 bg-red-500 rounded-full mr-1"></span>
                                                        {"Error"}
                                                    </span>
                                                }
                                            } else if !result.output.is_empty() {
                                                html! {
                                                    <span class="text-sm text-green-600 bg-green-50 px-2 py-1 rounded-full flex items-center">
                                                        <span class="w-2 h-2 bg-green-500 rounded-full mr-1"></span>
                                                        {"Success"}
                                                    </span>
                                                }
                                            } else {
                                                html! {
                                                    <span class="text-sm text-slate-500 bg-slate-100 px-2 py-1 rounded-full flex items-center">
                                                        <span class="w-2 h-2 bg-slate-400 rounded-full mr-1"></span>
                                                        {"Ready"}
                                                    </span>
                                                }
                                            }
                                        }
                                    </div>
                                </div>
                                <div class="bg-slate-900 border-2 border-slate-200 rounded-xl p-4 min-h-80 shadow-inner">
                                    {
                                        if let Some(error) = &result.error {
                                            html! {
                                                <div class="text-red-400 font-mono text-sm whitespace-pre-wrap leading-relaxed">
                                                    {error}
                                                </div>
                                            }
                                        } else if !result.output.is_empty() {
                                            html! {
                                                <div class="text-green-400 font-mono text-sm whitespace-pre-wrap leading-relaxed">
                                                    {&result.output}
                                                </div>
                                            }
                                        } else {
                                            html! {
                                                <div class="text-slate-500 italic flex items-center justify-center h-full">
                                                    <div class="text-center">
                                                        <div class="text-3xl mb-2">{"⚡"}</div>
                                                        <div>{"Execute a query to see results"}</div>
                                                    </div>
                                                </div>
                                            }
                                        }
                                    }
                                </div>
                            </div>
                        </div>

                        <div class="mt-8 pt-6 border-t border-slate-200">
                            <div class="flex flex-col sm:flex-row items-center justify-between text-sm text-slate-600">
                                <div class="flex items-center space-x-4">
                                    <span>{"Powered by WebAssembly"}</span>
                                    <span class="w-1 h-1 bg-slate-400 rounded-full"></span>
                                    <span>{"Built with Rust & Yew"}</span>
                                </div>
                                <div class="mt-2 sm:mt-0 flex items-center space-x-4">
                                    <span>{"Open Source Query Language"}</span>
                                    <a href="https://github.com/solve-poker/open-pql" 
                                       target="_blank" 
                                       rel="noopener noreferrer"
                                       class="inline-flex items-center text-slate-600 hover:text-blue-600 transition-colors">
                                        <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                            <path fill-rule="evenodd" d="M10 0C4.477 0 0 4.484 0 10.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0110 4.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.203 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.942.359.31.678.921.678 1.856 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0020 10.017C20 4.484 15.522 0 10 0z" clip-rule="evenodd"></path>
                                        </svg>
                                        {"GitHub"}
                                    </a>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

pub fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
