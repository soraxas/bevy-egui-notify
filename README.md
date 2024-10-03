
| <ins>__FORK NOTICE__</ins>: This is a simple fork that uses bevy_egui's re-exported egui crate, such that one can use the same egui version within the same applications (otherwise, there would be issue if you try to pass in an older/newer version of egui context to draw things). |
| ------------------------------------------------------------------------------ |

# bevy-egui-notify

Simple notifications library for [`egui`](https://github.com/emilk/egui)

![example_image](media/toasts_type.png)

![example_video](media/toasts_example_video.gif)

## Usage

```rust
use bevy_egui_notify::EguiToastsPlugin;

app.add_plugins(...)
    .add_plugins(EguiToastsPlugin::default());
```

```rust

fn my_system(
    toasts: ResMut<EguiToasts>,
    ...
) {
    ...
    toasts.0.info("hello");
}
```

## Installation

```sh
cargo add bevy-egui-notify
```