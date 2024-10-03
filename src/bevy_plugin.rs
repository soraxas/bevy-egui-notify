use bevy_egui::egui;

use std::time::Duration;

use crate::Toasts;
use bevy::prelude::*;
use bevy_egui::EguiContext;
use egui::FontId;

const DEFAULT_TOAST_FONT_SIZE: f32 = 25.0;
const TOAST_VERTICAL_MARGIN: f32 = 30.0;

/// resources that stores the toasts context
#[derive(Resource)]
pub struct EguiToasts(pub Toasts);

/// Plugin for add egui-toasts to bevy system
pub struct EguiToastsPlugin {
    builder: Option<fn() -> Toasts>,
}

// opinionated defaults
impl Default for EguiToastsPlugin {
    fn default() -> Self {
        Self {
            builder: Some(|| {
                Toasts::default()
                    .with_default_font(FontId::proportional(DEFAULT_TOAST_FONT_SIZE))
                    .with_margin([0., TOAST_VERTICAL_MARGIN].into())
            }),
        }
    }
}

impl Plugin for EguiToastsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EguiToasts(
            self.builder
                .map(|f| f())
                .unwrap_or_default(),
        ))
        .add_systems(Update, update_toasts);
    }
}

fn update_toasts(mut toasts: ResMut<EguiToasts>, mut ctx: Query<&mut EguiContext>) {
    toasts.0.show(ctx.single_mut().get_mut());
}

/// Show a toast with the given message, in an error state.
pub fn error_to_toast<Error>(In(result): In<Result<(), Error>>, toasts: ResMut<EguiToasts>)
where
    Error: std::fmt::Display,
{
    let toasts = &mut toasts.into_inner().0;
    if let Err(err) = result {
        toasts
            .error(format!("{}", err))
            .duration(Some(Duration::from_secs(10)));
    }
}
