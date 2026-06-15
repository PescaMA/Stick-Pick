//! The credits menu.
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{menus::Menu, theme::prelude::*};

const HORIZONTAL_SIZE: f32 = 800.;
const VERTICAL_SIZE: f32 = 80.;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Credits), spawn_credits_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Credits).and(input_just_pressed(KeyCode::Escape))),
    );
}

pub fn spawn_credits_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Credits Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Credits),
        children![
            widget::header("Creation"),
            created_by(),
            widget::header("Assets"),
            assets(),
            widget::button("Back", go_back_on_click),
        ],
    ));
}

pub fn created_by() -> impl Bundle {
    grid(vec![
        ["Created by ", " Pescariu Matei-Alexandru"],
        ["Inspired by ", "Hack And Climb by Kodachi Games"],
    ])
}

pub fn assets() -> impl Bundle {
    grid(vec![
        ["Bevy logo", "All rights reserved by the Bevy Foundation"],
        ["pixel-tilemap-platformer", "by bdragon1727"],
        ["Pickaxe (100 Item Pack)", "by Furkan Duran"],
        ["Button SFX", "CC0 by Jaszunio15"],
        [
            "Gameplay Music",
            "Determined Video Game Music by Seth_Makes_Sounds",
        ],
        ["Main Menu Music", "Enigmatic Polyphony by IgorChagas"],
    ])
}

pub fn grid<const N: usize>(content: Vec<[&'static str; N]>) -> impl Bundle {
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            height: px(VERTICAL_SIZE * content.len() as f32),
            overflow: Overflow::scroll(),
            row_gap: px(10),
            column_gap: px(10),
            grid_template_columns: RepeatedGridTrack::px(N, HORIZONTAL_SIZE / N as f32),
            ..default()
        },
        Children::spawn(SpawnIter(content.into_iter().flatten().enumerate().map(
            |(i, text)| {
                (
                    widget::label(text),
                    Node {
                        // overflow: Overflow::scroll_y(),
                        justify_self: if i.is_multiple_of(2) {
                            JustifySelf::End
                        } else {
                            JustifySelf::Start
                        },
                        ..default()
                    },
                )
            },
        ))),
    )
}

fn go_back_on_click(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}
