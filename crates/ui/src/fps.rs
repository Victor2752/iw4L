//! IW4L's frames-per-second counter (`cg_drawFPS` / Settings → Show FPS), in
//! the screen's top-right corner.

use bevy::prelude::*;
use frame::{ClientSet, GameSettings};

use crate::layers::{GameUiFont, game_text_font};

/// How long the counter averages before it shows a new number.
const REFRESH_SECONDS: f32 = 0.25;

#[derive(Component)]
struct FpsText;

#[derive(Default)]
struct FpsWindow {
    frames: u32,
    elapsed: f32,
}

pub(crate) fn register(app: &mut App) {
    app.add_systems(Update, update_fps_counter.in_set(ClientSet::Ui));
}

fn update_fps_counter(
    mut commands: Commands,
    settings: Res<GameSettings>,
    time: Res<Time<Real>>,
    font: Option<Res<GameUiFont>>,
    mut window: Local<FpsWindow>,
    mut counter: Query<(&mut Text, &mut Visibility), With<FpsText>>,
) {
    let Ok((mut text, mut visibility)) = counter.single_mut() else {
        if let Some(font) = font {
            commands.spawn((
                FpsText,
                Text::new(""),
                game_text_font(&font.0, 18.0),
                TextColor(Color::srgb(1.0, 1.0, 0.4)),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(8.0),
                    right: Val::Px(12.0),
                    ..default()
                },
                GlobalZIndex(20_000),
                Visibility::Hidden,
            ));
        }
        return;
    };
    let wanted = if settings.show_fps {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };
    if *visibility != wanted {
        *visibility = wanted;
    }
    if !settings.show_fps {
        *window = FpsWindow::default();
        return;
    }
    window.frames += 1;
    window.elapsed += time.delta_secs();
    if window.elapsed >= REFRESH_SECONDS {
        let fps = window.frames as f32 / window.elapsed;
        text.0 = format!("{fps:.0} FPS");
        *window = FpsWindow::default();
    }
}
