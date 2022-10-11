//! The User Interface

use super::super::BI;

use bevy::{prelude::*, window::PresentMode};
use bevy_asset_loader::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
//use bevy_editor_pls::prelude::*; // Wait til this is in crates.io
// use bevy_inspector_egui::WorldInspectorPlugin;

pub struct Ui;

pub mod assets;
pub use assets::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const SIDE_PANEL_WIDTH: f32 = 150.0;

#[derive(Default)]
struct UiState {
    scale_factor: f64,
    total: BI,
    increment: BI,
    status: String,
    _egui_texture_handle: Option<egui::TextureHandle>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    Next,
}

impl Ui {
    pub fn run() {
        App::new()
            // Main window, with title
            .insert_resource(WindowDescriptor {
                title: format!(r#"AEC: Aeon Energy Conglomerate v{}"#, VERSION),
                present_mode: PresentMode::AutoVsync,
                // detect dragging in the menu bar (but not on a menu), and use Window::set_position(Window::position() + drag_delta)
                // or something like that, the function names are similar if i didn't get them exactly right, but they're on the Window object
                // -Kromey (https://github.com/kromey)
                decorations: true,
                resizable: true,
                ..default()
            })

            // Add the GameState states to the the loading state.
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .continue_to_state(GameState::Next)
                    .with_collection::<assets::SpriteCollection>()
            )

            //Start off with the default loading state (AssetLoading) and then
            // once the AssetLoading is finished, moved onto the Next state.
            .add_state(GameState::AssetLoading)

            // Color used to clear the buffer between frames. It's a "background" for unrendered content
            .insert_resource(ClearColor(Color::BLACK))

            // Set multi-sample anti-aliasing (WGPU only supports 1 or 4)
            .insert_resource(Msaa { samples: 4 })

            // The UI state
            .init_resource::<UiState>()

            // Default Bevy
            .add_plugins(DefaultPlugins)

            // Egui Plugins
            .add_plugin(EguiPlugin)

            // bevy_editor_pls Plugin
            //.add_plugin(EditorPlugin) // Wait til this is in crates.io

            // bevy-inspector-egui Plugin
            // .add_plugin(WorldInspectorPlugin::new())

            // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
            // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
            .add_startup_system(Self::configure_visuals)
            .add_startup_system(Self::configure_ui_state)
            .add_system(Self::update_ui_scale_factor)
            .add_system(Self::main_ui)
            .add_system_set(
                SystemSet::on_enter(GameState::Next)
                    .with_system(Self::use_my_assets)
            )
            .add_system(Self::counter)
            .run();
    }

    fn use_my_assets(
        my_assets: Res<assets::SpriteCollection>,
        mut commands: Commands,
        texture_atlases: Res<Assets<TextureAtlas>>,
    ) {
        // do something using the asset handles from the resource
        println!("Use my assets!");
        //draw the original image (whole atlas)
        let atlas = texture_atlases
            .get(&my_assets.tiles)
            .expect("Failed to find our atlas");
        commands.spawn_bundle(SpriteBundle {
            texture: atlas.texture.clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        });
        // draw single texture from sprite sheet starting at index 0
        commands
            .spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(0., 150., 0.),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(0),
                texture_atlas: my_assets.tiles.clone(),
                ..Default::default()
            });
            //.insert(atlas);
        }

    fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
        // Default is Dark Mode
        egui_ctx.ctx_mut().set_visuals(egui::Visuals {
            window_rounding: (5.0).into(), // 5 points radius for window borders
            ..Default::default()
        });
    }

    fn configure_ui_state(mut ui_state: ResMut<UiState>) {
        ui_state.total = BI::from(0);
        ui_state.increment = BI::from(1);
        ui_state.status = String::from("The Status Quo");
    }

    // fn load_assets(mut egui_ctx: ResMut<EguiContext>) {

    // }

    fn update_ui_scale_factor(
        keyboard_input: Res<Input<KeyCode>>,
        mut egui_settings: ResMut<EguiSettings>,
        mut ui_state: ResMut<UiState>,
        //toggle_scale_factor: Local<Option<bool>>,
        //windows: Res<Windows>,
    ) {
        if keyboard_input.pressed(KeyCode::LControl) &&
           keyboard_input.just_pressed(KeyCode::Equals)
        {
            // println!("LControl + Equals");
            ui_state.scale_factor += 0.1;
        }
        if keyboard_input.pressed(KeyCode::LControl) &&
           keyboard_input.just_pressed(KeyCode::Minus)
        {
            // println!("LControl + Minus");
            ui_state.scale_factor -= 0.1;
        }
        if ui_state.scale_factor < 1.0 {
            // println!("scale_factor < 1.0, setting to 1.0");
            ui_state.scale_factor = 1.0;
        }
        if ui_state.scale_factor > 2.0 {
            // println!("scale_factor > 2.0, setting to 2.0");
            ui_state.scale_factor = 2.0;
        }
        // println!("scale_factor is currently {}", ui_state.scale_factor);
        egui_settings.scale_factor = ui_state.scale_factor;
        // if keyboard_input.just_pressed(KeyCode::Slash) || toggle_scale_factor.is_none() {
        //     *toggle_scale_factor = Some(!toggle_scale_factor.unwrap_or(true));

        //     if let Some(window) = windows.get_primary() {
        //         let scale_factor = if toggle_scale_factor.unwrap() {
        //             1.0
        //         } else {
        //             1.0 / window.scale_factor()
        //         };
        //         egui_settings.scale_factor = scale_factor;
        //     }
        // }
    }

    fn main_ui(
        mut egui_ctx: ResMut<EguiContext>,
        mut ui_state: ResMut<UiState>,
    ) {
        // top_menu(&mut egui_ctx);

        egui::TopBottomPanel::top("total")
            .show(egui_ctx.ctx_mut(), |ui| {
                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    ui.label(egui::RichText::new(ui_state.total.to_string()).heading());
                    if ui.button("+").clicked() {
                        ui_state.increment += BI::from(1000);
                    }
                    if ui.button("-").clicked() {
                        let increment = BI::from(1000);
                        match (ui_state.increment - increment).checked_neg() {
                            None => ui_state.increment = BI::from(1),
                            _ => ui_state.increment -= increment,
                        }
                    }
                    ui.label(format!("Increment: {}", ui_state.increment));
                });
            });

        egui::TopBottomPanel::bottom("status")
            .show(egui_ctx.ctx_mut(), |ui| {
                ui.label(ui_state.status.to_string());
            });

        egui::SidePanel::left("info")
            .min_width(SIDE_PANEL_WIDTH)
            .resizable(false)
            .show(egui_ctx.ctx_mut(), |ui| {
                ui.heading("Information");

                // ui.horizontal(|ui| {
                //     ui.label("Write something: ");
                //     ui.text_edit_singleline(&mut ui_state.label);
                // });

                // ui.add(egui::widgets::Image::new(
                //     egui_texture_handle.id(),
                //     egui_texture_handle.size_vec2(),
                // ));

                // ui.add(egui::Slider::new(&mut ui_state.value, 0.0..=10.0).text("value"));
                // if ui.button("Increment").clicked() {
                //     ui_state.value += 1.0;
                // }

                // ui.allocate_space(egui::Vec2::new(1.0, 100.0));
                // ui.horizontal(|ui| {
                //     load = ui.button("Load").clicked();
                //     invert = ui.button("Invert").clicked();
                //     remove = ui.button("Remove").clicked();
                // });

                // ui.allocate_space(egui::Vec2::new(1.0, 10.0));
                // ui.checkbox(&mut ui_state.is_window_open, "Window Is Open");

                // ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                //     ui.add(egui::Hyperlink::from_label_and_url(
                //         "powered by egui",
                //         "https://github.com/emilk/egui/",
                //     ));
                // });
            });

        egui::SidePanel::right("actions")
            .min_width(SIDE_PANEL_WIDTH)
            .resizable(false)
            .show(egui_ctx.ctx_mut(), |ui| {
                ui.heading("Actions");

                // ui.horizontal(|ui| {
                //     ui.label("Write something: ");
                //     ui.text_edit_singleline(&mut ui_state.label);
                // });

                // ui.add(egui::widgets::Image::new(
                //     egui_texture_handle.id(),
                //     egui_texture_handle.size_vec2(),
                // ));

                // ui.add(egui::Slider::new(&mut ui_state.value, 0.0..=10.0).text("value"));
                // if ui.button("Increment").clicked() {
                //     ui_state.value += 1.0;
                // }

                // ui.allocate_space(egui::Vec2::new(1.0, 100.0));
                // ui.horizontal(|ui| {
                //     load = ui.button("Load").clicked();
                //     invert = ui.button("Invert").clicked();
                //     remove = ui.button("Remove").clicked();
                // });

                // ui.allocate_space(egui::Vec2::new(1.0, 10.0));
                // ui.checkbox(&mut ui_state.is_window_open, "Window Is Open");

                // ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                //     ui.add(egui::Hyperlink::from_label_and_url(
                //         "powered by egui",
                //         "https://github.com/emilk/egui/",
                //     ));
                // });
            });

        egui::CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Aeon Energy Conglomerate");

            // ui.hyperlink("https://github.com/emilk/egui_template");
            // ui.add(egui::github_link_file_line!(
            //     "https://github.com/mvlabat/bevy_egui/blob/main/",
            //     "Direct link to source code."
            // ));
            // egui::warn_if_debug_build(ui);

            // ui.separator();

            // ui.heading("Central Panel");
            // ui.label("The central panel the region left after adding TopPanel's and SidePanel's");
            // ui.label("It is often a great place for big things, like drawings:");
            ui.label("");
            //commands.spawn_bundle(Camera2dBundle::default());
        });

        // egui::Window::new("Window")
        //     .vscroll(true)
        //     .open(&mut ui_state.is_window_open)
        //     .show(egui_ctx.ctx_mut(), |ui| {
        //         ui.label("Windows can be moved by dragging them.");
        //         ui.label("They are automatically sized based on contents.");
        //         ui.label("You can turn on resizing and scrolling if you like.");
        //         ui.label("You would normally chose either panels OR windows.");
        //     });
    }

    fn counter(mut ui_state: ResMut<UiState>) {
        let increment = ui_state.increment.clone();
        ui_state.total += increment;
    }
}
