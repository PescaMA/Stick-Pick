//! The credits menu.
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{menus::Menu, screens::Screen, theme::prelude::*};

const WIN_BACK_COLOR: Color = Color::srgb_u8(212, 175, 55);
const WIN_TEXT_COLOR: Color = Color::srgb_u8(8, 60, 60);

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Win), spawn_win_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Win).and(input_just_pressed(KeyCode::Escape))),
    );
}

pub fn spawn_win_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Win menu"),
        GlobalZIndex(2),
        BackgroundColor(WIN_BACK_COLOR),
        DespawnOnExit(Menu::Win),
        children![
            widget::header_colored("You Win!", WIN_TEXT_COLOR),
            widget::label_colored("Then you sold the gold block for 10$.", WIN_TEXT_COLOR),
            widget::button("Return", go_back_on_click),
        ],
    ));
}

fn go_back_on_click(
    _: On<Pointer<Click>>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_menu.set(Menu::Main);
    next_screen.set(Screen::Title);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_menu.set(Menu::Main);
    next_screen.set(Screen::Title);
}
