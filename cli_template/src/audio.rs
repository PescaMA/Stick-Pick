use bevy::{audio::Volume, prelude::*};

const SFX_COOLDOWN_SEC: f32 = 0.1;

#[derive(Resource)]
pub struct SfxCooldown {
    pub timer: Timer,
    pub volume: Volume,
}

impl Default for SfxCooldown {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0., TimerMode::Once),
            volume: Volume::Linear(1.0),
        }
    }
}
impl SfxCooldown {
    pub fn reset(&mut self) {
        self.timer = Timer::from_seconds(SFX_COOLDOWN_SEC, TimerMode::Once);
    }
}

pub fn plugin(app: &mut App) {
    app.insert_resource(SfxCooldown::default());
    app.add_systems(
        Update,
        (
            apply_global_volume.run_if(resource_changed::<GlobalVolume>),
            apply_sound_effect_cooldown,
        ),
    );
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "music" category (e.g. global background music, soundtrack).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Music;

/// A music audio instance.
pub fn music(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::LOOP, Music)
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "sound effect" category (e.g. footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SoundEffect;

/// A sound effect audio instance.
pub fn sound_effect(handle: Handle<AudioSource>) -> impl Bundle {
    (
        AudioPlayer(handle),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            volume: Volume::Linear(0.0), // setting it to 0 to be reset only after cooldown
            ..default()
        },
        SoundEffect,
    )
}

fn apply_sound_effect_cooldown(
    time: Res<Time>,
    global_volume: Res<GlobalVolume>,
    mut sfx_cooldown: ResMut<SfxCooldown>,
    mut new_sfx: Query<&mut AudioSink, Added<SoundEffect>>,
) {
    sfx_cooldown.timer.tick(time.delta());
    if new_sfx.is_empty() {
        return;
    }
    if sfx_cooldown.timer.is_finished() {
        if let Some(mut sink) = new_sfx.iter_mut().next() {
            sink.set_volume(global_volume.volume * sfx_cooldown.volume);
        }

        sfx_cooldown.reset();
        return;
    }
}
fn round_vol(vol: Volume) -> Volume {
    Volume::Linear((vol.to_linear() * 10.).round() / 10.)
}

/// [`GlobalVolume`] doesn't apply to already-running audio entities, so this system will update them.
fn apply_global_volume(
    global_volume: Res<GlobalVolume>,
    sfx_volume: Res<SfxCooldown>,
    mut audio_query: Query<(&PlaybackSettings, &mut AudioSink, Option<&SoundEffect>)>,
) {
    let volume = round_vol(global_volume.volume);

    for (playback, mut sink, is_sfx) in &mut audio_query {
        if is_sfx.is_some() {
            sink.set_volume(volume * playback.volume * sfx_volume.volume);
        } else {
            sink.set_volume(volume * playback.volume);
        }
    }
}
