use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use debug::DebugPlugin;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use tilemap::TileMapPlugin;

use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

mod debug;
mod game;
mod main_menu;
mod tilemap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    AssetLoading,
    MainMenu,
    Game,
    Pause,
}

#[derive(AssetCollection)]
struct ImageAssets {
    #[asset(path = "ui/logo.png")]
    logo: Handle<Image>,
}

#[derive(AssetCollection)]
struct ModelAssets {
    #[asset(path = "model/tavern.glb#Scene0")]
    tavern: Handle<Scene>,
}

#[derive(AssetCollection)]
struct ShaderAssets {
    #[asset(path = "shader/instancing.wgsl")]
    instancing: Handle<Shader>,
}

fn main() {
    let mut app = App::new();

    AssetLoader::new(GameState::AssetLoading)
        .continue_to_state(GameState::MainMenu)
        .with_collection::<ImageAssets>()
        .with_collection::<ModelAssets>()
        .with_collection::<ShaderAssets>()
        .build(&mut app);

    app.add_state(GameState::AssetLoading)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: 1920.0,
            height: 1080.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(LookTransformPlugin)
        .add_plugin(FpsCameraPlugin::default())
        .add_plugin(MainMenuPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(init)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(FpsCameraBundle::new(
        FpsCameraController::default(),
        PerspectiveCameraBundle::default(),
        Vec3::new(-2.0, 5.0, 5.0),
        Vec3::new(0., 0., 0.),
    ));
}
