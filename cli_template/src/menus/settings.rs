//! The settings menu.
//!
//! Additional settings and accessibility options should go here.

use bevy::{audio::Volume, input::common_conditions::input_just_pressed, prelude::*};

use crate::{audio::SfxCooldown, menus::Menu, screens::Screen, theme::prelude::*};

const MIN_VOLUME: f32 = 0.0;
const MAX_VOLUME: f32 = 3.0;
const VOLUME_CHANGE_PER_SEC: f32 = 2.0;

enum VolumeType {
    SFX,
    Global,
}

#[derive(Resource)]
struct VolumeTimer {
    timer: Timer,
    active: bool,
    volume_change: f32,
    vol_type: VolumeType,
}

impl Default for VolumeTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1 / VOLUME_CHANGE_PER_SEC, TimerMode::Repeating),
            active: false,
            volume_change: 0.,
            vol_type: VolumeType::Global,
        }
    }
}

impl VolumeTimer {
    fn reset(&mut self, change: f32, vol_type: VolumeType) {
        self.timer.reset();
        self.active = true;
        self.volume_change = change;
        self.vol_type = vol_type;
    }
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(VolumeTimer::default());
    app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Settings).and(input_just_pressed(KeyCode::Escape))),
    );

    app.add_systems(
        Update,
        (
            update_global_volume_label,
            update_sfx_volume_label,
            hold_volume,
        )
            .run_if(in_state(Menu::Settings)),
    );
}

fn spawn_settings_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Settings Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Settings),
        children![
            widget::header("Settings"),
            settings_grid(),
            widget::button("Back", go_back_on_click),
        ],
    ));
}

fn settings_grid() -> impl Bundle {
    (
        Name::new("Settings Grid"),
        Node {
            display: Display::Grid,
            row_gap: px(10),
            column_gap: px(30),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        children![
            (
                widget::label("Master Volume"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                }
            ),
            global_volume_widget(),
            (
                widget::label("SFX Volume"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                }
            ),
            sfx_volume_widget(),
        ],
    )
}

fn global_volume_widget() -> impl Bundle {
    (
        Name::new("Global Volume Widget"),
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small("-", lower_global_volume),
            (
                Name::new("Current Volume"),
                Node {
                    padding: UiRect::horizontal(px(10)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(widget::label(""), GlobalVolumeLabel)],
            ),
            widget::button_small("+", raise_global_volume),
        ],
    )
}

fn sfx_volume_widget() -> impl Bundle {
    (
        Name::new("SFX Volume Widget"),
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small("-", lower_sfx_volume),
            (
                Name::new("Current SFX Volume"),
                Node {
                    padding: UiRect::horizontal(px(10)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(widget::label(""), SFXVolumeLabel)],
            ),
            widget::button_small("+", raise_sfx_volume),
        ],
    )
}

fn lower_global_volume(
    _: On<Pointer<Press>>,
    mut global_volume: ResMut<GlobalVolume>,
    mut volume: ResMut<VolumeTimer>,
) {
    volume.reset(-0.1, VolumeType::Global);
    let linear = (global_volume.volume.to_linear() - 0.1).max(MIN_VOLUME);
    global_volume.volume = Volume::Linear(linear);
}

fn raise_global_volume(
    _: On<Pointer<Press>>,
    mut global_volume: ResMut<GlobalVolume>,
    mut volume: ResMut<VolumeTimer>,
) {
    volume.reset(0.1, VolumeType::Global);
    let linear = (global_volume.volume.to_linear() + 0.1).max(MIN_VOLUME);
    global_volume.volume = Volume::Linear(linear);
}

fn lower_sfx_volume(
    _: On<Pointer<Press>>,
    mut global_volume: ResMut<SfxCooldown>,
    mut volume: ResMut<VolumeTimer>,
) {
    volume.reset(-0.1, VolumeType::SFX);
    let linear = (global_volume.volume.to_linear() - 0.1).max(MIN_VOLUME);
    global_volume.volume = Volume::Linear(linear);
}

fn raise_sfx_volume(
    _: On<Pointer<Press>>,
    mut global_volume: ResMut<SfxCooldown>,
    mut volume: ResMut<VolumeTimer>,
) {
    volume.reset(0.1, VolumeType::SFX);
    let linear = (global_volume.volume.to_linear() + 0.1).min(MAX_VOLUME);
    global_volume.volume = Volume::Linear(linear);
}

fn hold_volume(
    interactions: Query<&Interaction>,
    mut volume: ResMut<VolumeTimer>,
    mut global_volume: ResMut<GlobalVolume>,
    mut sfx_volume: ResMut<SfxCooldown>,
    time: Res<Time>,
) {
    for interaction in interactions {
        match *interaction {
            Interaction::Pressed => {
                if volume.timer.tick(time.delta()).just_finished() {
                    match volume.vol_type {
                        VolumeType::SFX => {
                            let mut new_volume =
                                sfx_volume.volume.to_linear() + volume.volume_change;
                            new_volume = new_volume.min(MAX_VOLUME).max(MIN_VOLUME);
                            sfx_volume.volume = Volume::Linear(new_volume);
                        }
                        VolumeType::Global => {
                            let mut new_volume =
                                global_volume.volume.to_linear() + volume.volume_change;
                            new_volume = new_volume.min(MAX_VOLUME).max(MIN_VOLUME);
                            global_volume.volume = Volume::Linear(new_volume);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GlobalVolumeLabel;

fn update_global_volume_label(
    global_volume: Res<GlobalVolume>,
    mut label: Single<&mut Text, With<GlobalVolumeLabel>>,
) {
    let percent = 100.0 * global_volume.volume.to_linear();
    label.0 = format!("{percent:3.0}%"); // 0 digits after decimal, 3 total
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct SFXVolumeLabel;

fn update_sfx_volume_label(
    volume: Res<SfxCooldown>,
    mut label: Single<&mut Text, With<SFXVolumeLabel>>,
) {
    let percent = 100.0 * volume.volume.to_linear();
    label.0 = format!("{percent:3.0}%"); // 0 digits after decimal, 3 total
}

fn go_back_on_click(
    _: On<Pointer<Click>>,
    screen: Res<State<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}

fn go_back(screen: Res<State<Screen>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}
