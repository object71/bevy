mod axis;
pub mod gamepad;
mod input;
pub mod keyboard;
pub mod mouse;
pub mod touch;

pub use axis::*;
pub use input::*;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        gamepad::{
            Gamepad, GamepadAxis, GamepadAxisType, GamepadButton, GamepadButtonType, GamepadEvent,
            GamepadEventType, Gamepads,
        },
        keyboard::{KeyCode, ScanCode},
        mouse::MouseButton,
        touch::{TouchInput, Touches},
        Axis, Input,
    };
}

use bevy_app::prelude::*;
use bevy_ecs::schedule::{IntoSystemDescriptor, SystemLabel};
use bevy_reflect::{FromReflect, Reflect};
use keyboard::{keyboard_input_system, KeyCode, KeyboardInput, ScanCode};
use mouse::{
    mouse_button_input_system, MouseButton, MouseButtonInput, MouseMotion, MouseScrollUnit,
    MouseWheel,
};
use touch::{touch_screen_input_system, ForceTouch, TouchInput, TouchPhase, Touches};

use gamepad::{
    gamepad_connection_system, gamepad_event_system, AxisSettings, ButtonAxisSettings,
    ButtonSettings, Gamepad, GamepadAxis, GamepadAxisType, GamepadButton, GamepadButtonType,
    GamepadEvent, GamepadEventRaw, GamepadEventType, GamepadSettings, Gamepads,
};

#[cfg(feature = "serialize")]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

/// Adds keyboard and mouse input to an App
#[derive(Default)]
pub struct InputPlugin;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct InputSystem;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            // keyboard
            .add_event::<KeyboardInput>()
            .init_resource::<Input<KeyCode>>()
            .init_resource::<Input<ScanCode>>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                keyboard_input_system.label(InputSystem),
            )
            // mouse
            .add_event::<MouseButtonInput>()
            .add_event::<MouseMotion>()
            .add_event::<MouseWheel>()
            .init_resource::<Input<MouseButton>>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                mouse_button_input_system.label(InputSystem),
            )
            // gamepad
            .add_event::<GamepadEvent>()
            .add_event::<GamepadEventRaw>()
            .init_resource::<GamepadSettings>()
            .init_resource::<Gamepads>()
            .init_resource::<Input<GamepadButton>>()
            .init_resource::<Axis<GamepadAxis>>()
            .init_resource::<Axis<GamepadButton>>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                gamepad_event_system.label(InputSystem),
            )
            .add_system_to_stage(
                CoreStage::PreUpdate,
                gamepad_connection_system.after(InputSystem),
            )
            // touch
            .add_event::<TouchInput>()
            .init_resource::<Touches>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                touch_screen_input_system.label(InputSystem),
            );

        // Register common types
        app.register_type::<ButtonState>();

        // Register keyboard types
        app.register_type::<KeyboardInput>()
            .register_type::<KeyCode>()
            .register_type::<ScanCode>();

        // Register mouse types
        app.register_type::<MouseButtonInput>()
            .register_type::<MouseButton>()
            .register_type::<MouseMotion>()
            .register_type::<MouseScrollUnit>()
            .register_type::<MouseWheel>();

        // Register touch types
        app.register_type::<TouchInput>()
            .register_type::<ForceTouch>()
            .register_type::<TouchPhase>();

        // Register gamepad types
        app.register_type::<Gamepad>()
            .register_type::<GamepadEventType>()
            .register_type::<GamepadEvent>()
            .register_type::<GamepadEventRaw>()
            .register_type::<GamepadButtonType>()
            .register_type::<GamepadButton>()
            .register_type::<GamepadAxisType>()
            .register_type::<GamepadAxis>()
            .register_type::<GamepadSettings>()
            .register_type::<ButtonSettings>()
            .register_type::<AxisSettings>()
            .register_type::<ButtonAxisSettings>();
    }
}

/// The current "press" state of an element
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Reflect, FromReflect)]
#[reflect(Debug, Hash, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub enum ButtonState {
    Pressed,
    Released,
}

impl ButtonState {
    pub fn is_pressed(&self) -> bool {
        matches!(self, ButtonState::Pressed)
    }
}

impl From<winit::event::ElementState> for ButtonState {
    fn from(element_state: winit::event::ElementState) -> Self {
        match element_state {
            winit::event::ElementState::Pressed => ButtonState::Pressed,
            winit::event::ElementState::Released => ButtonState::Released,
        }
    }
}
