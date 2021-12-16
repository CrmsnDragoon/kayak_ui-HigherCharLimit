use bevy::{
    math::Vec2,
    prelude::{App as BevyApp, AssetServer, Commands, Handle, Res, ResMut},
    window::{WindowDescriptor, Windows},
    DefaultPlugins,
};
use kayak_ui::bevy::{BevyContext, BevyKayakUIPlugin, ImageManager, UICameraBundle};
use kayak_ui::core::{render, Index};
use kayak_widgets::{App, Image};

fn startup(
    mut commands: Commands,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
    mut image_manager: ResMut<ImageManager>,
) {
    commands.spawn_bundle(UICameraBundle::new());

    let window_size = if let Some(window) = windows.get_primary() {
        Vec2::new(window.width(), window.height())
    } else {
        panic!("Couldn't find primary window!");
    };

    let handle: Handle<bevy::render::texture::Image> = asset_server.load("panel.png");
    let ui_image_handle = image_manager.get(&handle);

    let context = BevyContext::new(window_size.x, window_size.y, |styles, context| {
        render! {
            <App styles={Some(styles.clone())}>
                <Image handle={ui_image_handle} />
            </App>
        }
    });

    commands.insert_resource(context);
}

fn main() {
    BevyApp::new()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("UI Example"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyKayakUIPlugin)
        .add_startup_system(startup)
        .run();
}
