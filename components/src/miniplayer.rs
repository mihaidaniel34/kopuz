use crate::shared::{fmt_time, get_favorite, toggle_favorite};
use crate::titlebar::Titlebar;
use config::UiStyle;
use dioxus::prelude::*;
use hooks::use_player_controller::PlayerController;
use player::player::Player;
use reader::FavoritesStore;

#[component]
pub fn Miniplayer(
    favorites_store: Signal<FavoritesStore>,
    mut config: Signal<config::AppConfig>,
    mut player: Signal<Player>,
    mut is_playing: Signal<bool>,
    mut is_fullscreen: Signal<bool>,
    mut is_miniplayer: Signal<bool>,
    mut current_song_duration: Signal<u64>,
    mut current_song_progress: Signal<u64>,
    mut current_song_title: Signal<String>,
    mut current_song_artist: Signal<String>,
    mut current_song_cover_url: Signal<String>,
) -> Element {
    let mut ctrl = use_context::<PlayerController>();
    let mut is_dragging = use_signal(|| false);
    let mut drag_progress = use_signal(|| 0u64);
    let miniplayer_text = "Mini player";
    let open_fullscreen_text = "Open fullscreen player";
    let restore_window_text = "Restore main window";
    let previous_track_text = "Previous track";
    let next_track_text = "Next track";
    let play_text = "Play";
    let pause_text = "Pause";
    let seek_text = "Seek within current track";

    let display_progress = if *is_dragging.read() {
        *drag_progress.read()
    } else {
        *current_song_progress.read()
    };

    let progress_percent = if *current_song_duration.read() > 0 {
        (display_progress as f64 / *current_song_duration.read() as f64) * 100.0
    } else {
        0.0
    };

    let is_radio = *current_song_duration.read() == u64::MAX;
    let is_modern = matches!(config.read().ui_style, UiStyle::Modern);

    let current_track_snapshot = ctrl.current_track_snapshot.read().clone();
    let is_favorite = get_favorite(current_track_snapshot.as_ref(), &favorites_store);
    let favorite_title = if is_favorite {
        i18n::t("remove_from_favorites").to_string()
    } else {
        i18n::t("add_to_favorites").to_string()
    };
    let heart_class = if is_favorite {
        "text-red-400 hover:text-red-300"
    } else if is_modern {
        "text-slate-500 hover:text-red-400"
    } else {
        "text-slate-400 hover:text-red-400"
    };
    let heart_icon = if is_favorite {
        "fa-solid fa-heart"
    } else {
        "fa-regular fa-heart"
    };

    let shell_class = if is_modern {
        "flex flex-col h-screen bg-black/80 backdrop-blur-xl text-white"
    } else {
        "flex flex-col h-screen bg-black/72 text-white"
    };
    let content_class = if is_modern {
        "flex flex-1 items-center gap-3 px-3 py-2"
    } else {
        "flex flex-1 items-center gap-4 px-4 py-3"
    };
    let cover_class = if is_modern {
        "w-14 h-14 rounded-lg overflow-hidden bg-white/5 shrink-0 flex items-center justify-center"
    } else {
        "w-16 h-16 rounded-lg overflow-hidden bg-white/5 shrink-0 flex items-center justify-center"
    };
    let title_class = if is_modern {
        "text-sm font-semibold text-white/90 truncate"
    } else {
        "text-sm font-bold text-white/90 truncate"
    };
    let artist_class = if is_modern {
        "text-[11px] text-slate-400 truncate"
    } else {
        "text-xs text-slate-400 truncate"
    };
    let controls_class = if is_modern {
        "flex items-center gap-1.5 shrink-0"
    } else {
        "flex items-center gap-2 shrink-0"
    };
    let control_button_class = if is_modern {
        "w-7 h-7 flex items-center justify-center rounded-md text-slate-400 hover:text-white transition-colors active:scale-90 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/60 focus-visible:ring-offset-2 focus-visible:ring-offset-black"
    } else {
        "w-8 h-8 flex items-center justify-center rounded-md text-slate-300 hover:text-white transition-colors active:scale-90 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/60 focus-visible:ring-offset-2 focus-visible:ring-offset-black"
    };
    let play_button_class = if is_modern {
        "w-8 h-8 rounded-full bg-white/12 hover:bg-white/20 text-white flex items-center justify-center transition-all active:scale-95 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/60 focus-visible:ring-offset-2 focus-visible:ring-offset-black"
    } else {
        "w-10 h-10 rounded-full bg-white text-black hover:scale-105 flex items-center justify-center transition-all active:scale-95 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/60 focus-visible:ring-offset-2 focus-visible:ring-offset-black"
    };
    let action_button_class = if is_modern {
        "w-7 h-7 flex items-center justify-center rounded-md text-slate-500 hover:text-white hover:bg-white/8 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/60 focus-visible:ring-offset-2 focus-visible:ring-offset-black"
    } else {
        "w-8 h-8 flex items-center justify-center rounded-md text-slate-400 hover:text-white hover:bg-white/8 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/60 focus-visible:ring-offset-2 focus-visible:ring-offset-black"
    };
    let progress_fill_class = if is_modern {
        "absolute top-0 left-0 h-full bg-white/60 rounded-full transition-colors pointer-events-none"
    } else {
        "absolute top-0 left-0 h-full bg-white rounded-full transition-colors pointer-events-none"
    };
    let progress_time_class = if is_modern {
        "text-[9px] text-slate-600 font-mono shrink-0"
    } else {
        "text-[10px] text-slate-500 font-mono shrink-0"
    };
    let placeholder_icon_class = if is_modern {
        "fa-solid fa-music text-white/20 text-xs"
    } else {
        "fa-solid fa-music text-white/20"
    };
    let cover_alt = format!(
        "Cover art for {} by {}",
        current_song_title.read(),
        current_song_artist.read()
    );

    rsx! {
        div {
            class: "{shell_class}",

            if cfg!(any(target_os = "linux", target_os = "windows")) {
                div { dir: "ltr", Titlebar {} }
            }

            div {
                class: "{content_class}",
                role: "group",
                "aria-label": "{miniplayer_text}",

                div {
                    class: "{cover_class}",
                    if current_song_cover_url.read().is_empty() {
                        i { class: "{placeholder_icon_class}" }
                    } else {
                        img {
                            src: "{current_song_cover_url}",
                            class: "w-full h-full object-cover",
                            alt: "{cover_alt}"
                        }
                    }
                }

                div {
                    class: "flex-1 min-w-0 flex flex-col justify-center gap-1",

                    div {
                        class: "flex items-center gap-2 min-w-0",
                        span { class: "{title_class}", "{current_song_title}" }
                        button {
                            class: "{heart_class} transition-colors shrink-0 rounded-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/60 focus-visible:ring-offset-2 focus-visible:ring-offset-black",
                            title: "{favorite_title}",
                            "aria-label": "{favorite_title}",
                            onclick: move |_| {
                                toggle_favorite(
                                    ctrl.current_track_snapshot.read().clone(),
                                    favorites_store,
                                    config,
                                );
                            },
                            i { class: "{heart_icon} text-xs" }
                        }
                    }

                    div { class: "{artist_class}", "{current_song_artist}" }

                    div {
                        class: "flex items-center gap-2 w-full",
                        span { class: "{progress_time_class}", "{fmt_time(display_progress)}" }
                        div {
                            class: format!(
                                "flex-1 h-[3px] bg-white/10 rounded-full relative focus-within:ring-2 focus-within:ring-white/40 focus-within:ring-offset-2 focus-within:ring-offset-black {}",
                                if is_radio { "" } else { "group cursor-pointer" }
                            ),
                            div {
                                class: "{progress_fill_class}",
                                style: "width: {progress_percent}%",
                            }
                            if !is_modern {
                                div {
                                    class: "absolute top-1/2 -translate-y-1/2 w-2.5 h-2.5 bg-white rounded-full opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none -translate-x-1/2",
                                    style: "left: {progress_percent}%",
                                }
                            }
                            input {
                                r#type: "range",
                                min: "0",
                                max: "{*current_song_duration.read()}",
                                value: "{display_progress}",
                                class: format!(
                                    "absolute top-0 left-0 w-full h-full opacity-0 z-10 {}",
                                    if is_radio {
                                        "pointer-events-none"
                                    } else {
                                        "cursor-pointer"
                                    }
                                ),
                                disabled: is_radio,
                                title: "{seek_text}",
                                "aria-label": "{seek_text}",
                                onchange: move |evt| {
                                    if let Ok(val) = evt.value().parse::<f64>().map(|v| v as u64) {
                                        player.write().seek(std::time::Duration::from_secs(val));
                                        current_song_progress.set(val);
                                        drag_progress.set(val);
                                        is_dragging.set(false);
                                    }
                                },
                                oninput: move |evt| {
                                    if let Ok(val) = evt.value().parse::<f64>().map(|v| v as u64) {
                                        is_dragging.set(true);
                                        drag_progress.set(val);
                                    }
                                }
                            }
                        }
                        span { class: "{progress_time_class}", "{fmt_time(*current_song_duration.read())}" }
                    }
                }

                div {
                    class: "{controls_class}",
                    button {
                        class: "{control_button_class}",
                        title: "{previous_track_text}",
                        "aria-label": "{previous_track_text}",
                        onclick: move |_| ctrl.play_prev(),
                        i { class: if is_modern { "fa-solid fa-backward-step text-sm" } else { "fa-solid fa-backward-step" } }
                    }
                    button {
                        class: "{play_button_class}",
                        title: if *is_playing.read() { pause_text } else { play_text },
                        "aria-label": if *is_playing.read() { pause_text } else { play_text },
                        onclick: move |_| ctrl.toggle(),
                        i {
                            class: if *is_playing.read() {
                                if is_modern {
                                    "fa-solid fa-pause text-xs"
                                } else {
                                    "fa-solid fa-pause text-sm"
                                }
                            } else if is_modern {
                                "fa-solid fa-play text-xs ml-0.5"
                            } else {
                                "fa-solid fa-play text-sm ml-0.5"
                            }
                        }
                    }
                    button {
                        class: "{control_button_class}",
                        title: "{next_track_text}",
                        "aria-label": "{next_track_text}",
                        onclick: move |_| ctrl.play_next(),
                        i { class: if is_modern { "fa-solid fa-forward-step text-sm" } else { "fa-solid fa-forward-step" } }
                    }
                }

                div {
                    class: "flex flex-col items-center gap-1 shrink-0",
                    if cfg!(not(target_arch = "wasm32")) {
                        button {
                            class: "{action_button_class}",
                            title: "{open_fullscreen_text}",
                            "aria-label": "{open_fullscreen_text}",
                            onclick: move |_| {
                                is_miniplayer.set(false);
                                is_fullscreen.set(true);
                            },
                            i { class: "fa-solid fa-up-right-and-down-left-from-center text-[10px]" }
                        }
                        button {
                            class: "{action_button_class}",
                            title: "{restore_window_text}",
                            "aria-label": "{restore_window_text}",
                            onclick: move |_| is_miniplayer.set(false),
                            i { class: "fa-regular fa-window-maximize text-[10px]" }
                        }
                    }
                }
            }
        }
    }
}
