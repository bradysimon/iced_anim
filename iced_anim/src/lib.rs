//! A library for easily creating animations in Iced. Animations are designed around spring physics
//! to make animations feel natural and interactive.
//!
//! # Overview
//!
//! The primary widgets are `Animation` and `AnimationBuilder`, which are widgets that help you
//! animates a value when it changes. The `Animation` widget allows you to store the animated value
//! in your app state, while the `AnimationBuilder` widget maintains the animated value so your app
//! state is always up-to-date with the latest value. Refer to those widget modules for documentation
//! and the `examples` directory for examples on how to use them.
//!
//! ## Animated widgets
//!
//! A subset of the standard `iced` widgets are exported under a `widgets` feature
//! flag. You can use these as drop-in replacements for the existing widgets:
//!
//! ```rust
//! # use iced_widget::Widget;
//! # #[derive(Clone)] enum Message { DoSomething }
//! use iced_anim::widget::button;
//!
//! fn fancy_button<'a>() -> impl Widget<Message> + 'a {
//!     button("Animated button")
//!         .on_press(Message::DoSomething)
//! }
//! ```
//!
//! ## Animating types
//!
//! You can animate anything that implements [`Animate`]. A handful of common types already
//! implement [`Animate`] like [`f32`], `Color` and `Theme`, with more to come
//! in the future. You can also derive [`Animate`] on your own types to animate them as well
//! by enabling the `derive` feature flag and adding `#[derive(Animate)]` to your type:
//!
//! ```rust
//! # use iced_core::Color;
//! use iced_anim::Animate;
//!
//! // Note that animate also requires `Clone` and `PartialEq` impls.
//! #[derive(Animate, Clone, PartialEq)]
//! struct MyType {
//!     size: f32,
//!     color: Color,
//! }
//! ```
//!
//! ## Controlling the spring motion
//!
//! The spring motion of an [`AnimationBuilder`] can be customized. There are some presets like
//! [`spring::Motion::smooth`] and [`spring::Motion::bouncy`], but you can also create your own.
//!
//! ## Supported Iced versions
//!
//! This crate supports Iced 0.14 and newer.
pub mod animate;
pub mod animated;
mod animated_state;
pub mod animation;
pub mod animation_builder;
pub mod event;
pub mod spring;
pub mod transition;
#[cfg(feature = "widgets")]
pub mod widget;

pub use animate::Animate;
pub use animated::{Animated, AnimationType};
pub use animated_state::AnimatedState;
pub use animation::Animation;
pub use animation_builder::*;
pub use event::Event;
pub use spring::{Motion, Spring};
pub use transition::{Easing, Transition};

#[cfg(feature = "derive")]
pub use iced_anim_derive::Animate;
