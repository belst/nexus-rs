use nexus::{
    event::event_subscribe,
    gui::{register_render, render, RenderType},
    imgui::Window,
    keybind::{keybind_handler, register_keybind_with_string},
    paths::get_addon_dir,
    quick_access::add_shortcut,
    texture::{load_texture_from_file, texture_receive, Texture},
    AddonFlags, UpdateProvider,
};
use std::cell::Cell;

nexus::export! {
    name: "Example Addon",
    signature: -0x12345678, // raidcore addon id or NEGATIVE random unique signature
    load,
    unload,
    flags: AddonFlags::None,
    provider: UpdateProvider::GitHub,
    update_link: "https://github.com/zerthox/nexus-rs",
}

fn load() {
    log::info!("Loading addon");

    let addon_dir = get_addon_dir("example").expect("invalid addon dir");

    register_render(
        RenderType::Render,
        render!(|ui| {
            Window::new("Test window").build(ui, || {
                // this is fine since imgui is single threaded
                thread_local! { static SHOW: Cell<bool> = Cell::new(false); }

                let mut show = SHOW.get();

                if show {
                    show = !ui.button("hide");
                    ui.text("Hello world");
                } else {
                    show = ui.button("show");
                }

                SHOW.set(show);
            });
        }),
    )
    .revert_on_unload();

    add_shortcut(
        "MY_SHORTCUT",
        "MY_ICON",
        "MY_ICON_HOVER",
        "MY_KEYBIND",
        "This is my tooltip text",
    )
    .revert_on_unload();

    let receive_texture =
        texture_receive!(|id: &str, _texture: Option<&Texture>| log::info!("texture {id} loaded"));
    load_texture_from_file("MY_ICON", addon_dir.join("icon.png"), Some(receive_texture));
    load_texture_from_file(
        "MY_ICON_HOVER",
        addon_dir.join("icon_hover.png"),
        Some(receive_texture),
    );

    let keybind_handler = keybind_handler!(|id, is_release| log::info!(
        "keybind {id} {}",
        if is_release { "released" } else { "pressed" }
    ));
    register_keybind_with_string("MY_KEYBIND", keybind_handler, "ALT+SHIFT+T").revert_on_unload();

    unsafe { event_subscribe!("MY_EVENT" => i32, |data| log::info!("received event {data:?}")) }
        .revert_on_unload();
}

fn unload() {
    log::info!("Unloading addon");
    // all actions passed to on_load() or revert_on_unload() are performed automatically
}
