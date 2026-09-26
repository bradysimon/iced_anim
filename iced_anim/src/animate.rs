//! Core definitions for the `Animate` trait and its implementations.
//!
//! The `Animate` trait is the main building block for animations. It is primarily designed for
//! Spring/velocity-based animations to make animations interruptible without any jarring effects.
//!
//! You can implement this trait for custom types using the "derive" feature.
use iced_core::{theme::palette, Theme};
use std::sync::Arc;

/// A trait for types that can be animated on a per-property basis.
///
/// You can derive this trait with `#[derive(Animate)]` with the `derive` feature enabled.
/// Otherwise, you can manually implement it while ensuring that `Animate::components` returns
/// the same length vector as `Animate::distance_to` returns.
///
/// Also, ensure that `Animate::update` and `Animate::distance_to` are consistent with each other
/// in both the number of components consumed and the order of the components. Keeping these in
/// sync is important to ensure that updates affect the correct properties.
pub trait Animate: Clone + PartialEq {
    /// The number if animatable components in the type.
    ///
    /// Simple types like `f32` have 1 component, while more complex types like `Color` have 4.
    /// This is used so the animation knows how many properties may be animated.
    fn components() -> usize;

    /// Update the type with the next set of components.
    ///
    /// The `components` is an iterator of new fractional values that should be added to the
    /// current value.
    fn update(&mut self, components: &mut impl Iterator<Item = f32>);

    /// The distance between the current value and the end value.
    ///
    /// The `end` value is the target value that the current value should be animated towards.
    /// This distance can be positive or negative and returns a vector that should be consistent
    /// with `Animate::components` and the update order in `Animate::update`.
    fn distance_to(&self, end: &Self) -> Vec<f32>;

    /// Linearly interpolate between two values based on a progress ratio.
    ///
    /// The `start` value is the initial value, the `end` value is the target value, and `progress`
    /// is a value between 0.0 and 1.0 representing the interpolation progress.
    fn lerp(&mut self, start: &Self, end: &Self, progress: f32);
}

impl Animate for f32 {
    fn components() -> usize {
        1
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        *self += components.next().unwrap();
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        vec![self - end]
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        *self = start + (end - start) * progress
    }
}

impl Animate for iced_core::Point<f32> {
    fn components() -> usize {
        2
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.x += components.next().unwrap();
        self.y += components.next().unwrap();
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [self.x.distance_to(&end.x), self.y.distance_to(&end.y)].concat()
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.x.lerp(&start.x, &end.x, progress);
        self.y.lerp(&start.y, &end.y, progress);
    }
}

impl Animate for iced_core::Color {
    fn components() -> usize {
        4
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.r = (self.r + components.next().unwrap()).clamp(0.0, 1.0);
        self.g = (self.g + components.next().unwrap()).clamp(0.0, 1.0);
        self.b = (self.b + components.next().unwrap()).clamp(0.0, 1.0);
        self.a = (self.a + components.next().unwrap()).clamp(0.0, 1.0);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.r.distance_to(&end.r),
            self.g.distance_to(&end.g),
            self.b.distance_to(&end.b),
            self.a.distance_to(&end.a),
        ]
        .concat()
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.r.lerp(&start.r, &end.r, progress);
        self.g.lerp(&start.g, &end.g, progress);
        self.b.lerp(&start.b, &end.b, progress);
        self.a.lerp(&start.a, &end.a, progress);
    }
}

impl Animate for iced_core::theme::palette::Seed {
    fn components() -> usize {
        6 * iced_core::Color::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.background.update(components);
        self.text.update(components);
        self.primary.update(components);
        self.success.update(components);
        self.warning.update(components);
        self.danger.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.background.distance_to(&end.background),
            self.text.distance_to(&end.text),
            self.primary.distance_to(&end.primary),
            self.success.distance_to(&end.success),
            self.warning.distance_to(&end.warning),
            self.danger.distance_to(&end.danger),
        ]
        .concat()
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.background
            .lerp(&start.background, &end.background, progress);
        self.text.lerp(&start.text, &end.text, progress);
        self.primary.lerp(&start.primary, &end.primary, progress);
        self.success.lerp(&start.success, &end.success, progress);
        self.warning.lerp(&start.warning, &end.warning, progress);
        self.danger.lerp(&start.danger, &end.danger, progress);
    }
}

impl Animate for Theme {
    fn components() -> usize {
        iced_core::theme::Palette::components() + iced_core::theme::palette::Seed::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        let mut seed = self.seed();
        seed.update(components);

        let mut palette = *self.palette();
        palette.update(components);

        *self = Theme::Custom(Arc::new(iced_core::theme::Custom::with_fn(
            "Animating Theme".to_owned(),
            seed,
            move |_| palette,
        )))
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.seed().distance_to(&end.seed()),
            self.palette().distance_to(end.palette()),
        ]
        .concat()
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        let mut seed = start.seed();
        seed.lerp(&start.seed(), &end.seed(), progress);

        let mut palette = *start.palette();
        palette.lerp(start.palette(), end.palette(), progress);

        *self = Theme::Custom(Arc::new(iced_core::theme::Custom::with_fn(
            "Animating Theme".to_owned(),
            seed,
            move |_| palette,
        )));
    }
}

impl Animate for palette::Pair {
    fn components() -> usize {
        2 * iced_core::Color::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.color.update(components);
        self.text.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.color.distance_to(&end.color),
            self.text.distance_to(&end.text),
        ]
        .concat()
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.color.lerp(&start.color, &end.color, progress);
        self.text.lerp(&start.text, &end.text, progress);
    }
}

impl Animate for palette::Swatch {
    fn components() -> usize {
        3 * palette::Pair::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.base.update(components);
        self.weak.update(components);
        self.strong.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.base.distance_to(&end.base),
            self.weak.distance_to(&end.weak),
            self.strong.distance_to(&end.strong),
        ]
        .concat()
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.base.lerp(&start.base, &end.base, progress);
        self.weak.lerp(&start.weak, &end.weak, progress);
        self.strong.lerp(&start.strong, &end.strong, progress);
    }
}

impl Animate for palette::Background {
    fn components() -> usize {
        8 * palette::Pair::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.base.update(components);
        self.weakest.update(components);
        self.weaker.update(components);
        self.weak.update(components);
        self.neutral.update(components);
        self.strong.update(components);
        self.stronger.update(components);
        self.strongest.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.base.distance_to(&end.base),
            self.weakest.distance_to(&end.weakest),
            self.weaker.distance_to(&end.weaker),
            self.weak.distance_to(&end.weak),
            self.neutral.distance_to(&end.neutral),
            self.strong.distance_to(&end.strong),
            self.stronger.distance_to(&end.stronger),
            self.strongest.distance_to(&end.strongest),
        ]
        .concat()
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.base.lerp(&start.base, &end.base, progress);
        self.weakest.lerp(&start.weakest, &end.weakest, progress);
        self.weaker.lerp(&start.weaker, &end.weaker, progress);
        self.weak.lerp(&start.weak, &end.weak, progress);
        self.neutral.lerp(&start.neutral, &end.neutral, progress);
        self.strong.lerp(&start.strong, &end.strong, progress);
        self.stronger.lerp(&start.stronger, &end.stronger, progress);
        self.strongest
            .lerp(&start.strongest, &end.strongest, progress);
    }
}

impl Animate for iced_core::theme::Palette {
    fn components() -> usize {
        palette::Background::components() + palette::Swatch::components() * 5
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.primary.update(components);
        self.secondary.update(components);
        self.success.update(components);
        self.warning.update(components);
        self.danger.update(components);
        self.background.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.primary.distance_to(&end.primary),
            self.secondary.distance_to(&end.secondary),
            self.success.distance_to(&end.success),
            self.warning.distance_to(&end.warning),
            self.danger.distance_to(&end.danger),
            self.background.distance_to(&end.background),
        ]
        .concat()
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.is_dark = if progress >= 0.5 {
            end.is_dark
        } else {
            start.is_dark
        };
        self.primary.lerp(&start.primary, &end.primary, progress);
        self.secondary
            .lerp(&start.secondary, &end.secondary, progress);
        self.success.lerp(&start.success, &end.success, progress);
        self.warning.lerp(&start.warning, &end.warning, progress);
        self.danger.lerp(&start.danger, &end.danger, progress);
        self.background
            .lerp(&start.background, &end.background, progress);
    }
}

impl<T> Animate for Option<T>
where
    T: Animate,
{
    fn components() -> usize {
        T::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        if let Some(inner) = self {
            inner.update(components);
        } else {
            components.nth(T::components() - 1);
        }
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        match (self, end) {
            (Some(current), Some(end)) => current.distance_to(end),
            _ => vec![0.0; T::components()],
        }
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        // TODO: Maybe figure out something better here. Potentially only use Option with Default impls if missing.
        if let (Some(start), Some(end)) = (start, end) {
            if let Some(value) = self.as_mut() {
                value.lerp(start, end, progress);
            }
        }
    }
}

impl Animate for iced_core::border::Radius {
    fn components() -> usize {
        4
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.top_left.distance_to(&end.top_left),
            self.top_right.distance_to(&end.top_right),
            self.bottom_left.distance_to(&end.bottom_left),
            self.bottom_right.distance_to(&end.bottom_right),
        ]
        .concat()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.top_left.update(components);
        self.top_right.update(components);
        self.bottom_left.update(components);
        self.bottom_right.update(components);
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.top_left.lerp(&start.top_left, &end.top_left, progress);
        self.top_right
            .lerp(&start.top_right, &end.top_right, progress);
        self.bottom_left
            .lerp(&start.bottom_left, &end.bottom_left, progress);
        self.bottom_right
            .lerp(&start.bottom_right, &end.bottom_right, progress);
    }
}

impl Animate for iced_core::Border {
    fn components() -> usize {
        f32::components() + iced_core::Color::components() + iced_core::border::Radius::components()
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.width.distance_to(&end.width),
            self.color.distance_to(&end.color),
            self.radius.distance_to(&end.radius),
        ]
        .concat()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.width.update(components);
        self.color.update(components);
        self.radius.update(components);
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.width.lerp(&start.width, &end.width, progress);
        self.color.lerp(&start.color, &end.color, progress);
        self.radius.lerp(&start.radius, &end.radius, progress);
    }
}

impl<T> Animate for iced_core::Vector<T>
where
    T: Animate,
{
    fn components() -> usize {
        2 * T::components()
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [self.x.distance_to(&end.x), self.y.distance_to(&end.y)].concat()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.x.update(components);
        self.y.update(components);
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.x.lerp(&start.x, &end.x, progress);
        self.y.lerp(&start.y, &end.y, progress);
    }
}

impl<T> Animate for iced_core::Size<T>
where
    T: Animate,
{
    fn components() -> usize {
        2 * T::components()
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.width.distance_to(&end.width),
            self.height.distance_to(&end.height),
        ]
        .concat()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.width.update(components);
        self.height.update(components);
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.width.lerp(&start.width, &end.width, progress);
        self.height.lerp(&start.height, &end.height, progress);
    }
}

impl<T> Animate for iced_core::Rectangle<T>
where
    T: Animate,
{
    fn components() -> usize {
        4 * T::components()
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.x.distance_to(&end.x),
            self.y.distance_to(&end.y),
            self.width.distance_to(&end.width),
            self.height.distance_to(&end.height),
        ]
        .concat()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.x.update(components);
        self.y.update(components);
        self.width.update(components);
        self.height.update(components);
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.x.lerp(&start.x, &end.x, progress);
        self.y.lerp(&start.y, &end.y, progress);
        self.width.lerp(&start.width, &end.width, progress);
        self.height.lerp(&start.height, &end.height, progress);
    }
}

impl Animate for iced_core::Shadow {
    fn components() -> usize {
        iced_core::Color::components() + iced_core::Vector::<f32>::components() + f32::components()
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.color.distance_to(&end.color),
            self.offset.distance_to(&end.offset),
            self.blur_radius.distance_to(&end.blur_radius),
        ]
        .concat()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.color.update(components);
        self.offset.update(components);
        self.blur_radius.update(components);
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.color.lerp(&start.color, &end.color, progress);
        self.offset.lerp(&start.offset, &end.offset, progress);
        self.blur_radius
            .lerp(&start.blur_radius, &end.blur_radius, progress);
    }
}

impl Animate for iced_core::Radians {
    fn components() -> usize {
        f32::components()
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        self.0.distance_to(&end.0)
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.0.update(components);
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.0.lerp(&start.0, &end.0, progress);
    }
}

impl Animate for iced_core::gradient::ColorStop {
    fn components() -> usize {
        f32::components() + iced_core::Color::components()
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.offset.distance_to(&end.offset),
            self.color.distance_to(&end.color),
        ]
        .concat()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.offset.update(components);
        self.color.update(components);
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.offset.lerp(&start.offset, &end.offset, progress);
        self.color.lerp(&start.color, &end.color, progress);
    }
}

impl<T, const N: usize> Animate for [T; N]
where
    T: Animate,
{
    fn components() -> usize {
        N * T::components()
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        self.iter()
            .zip(end.iter())
            .flat_map(|(start, end)| start.distance_to(end))
            .collect()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        for item in self.iter_mut() {
            item.update(components);
        }
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        for (item, (start, end)) in self.iter_mut().zip(start.iter().zip(end.iter())) {
            item.lerp(start, end, progress);
        }
    }
}

impl Animate for iced_core::gradient::Linear {
    fn components() -> usize {
        iced_core::Radians::components() + 8 * iced_core::gradient::ColorStop::components()
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.angle.distance_to(&end.angle),
            self.stops.distance_to(&end.stops),
        ]
        .concat()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.angle.update(components);

        for stop in &mut self.stops {
            stop.update(components);
        }
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.angle.lerp(&start.angle, &end.angle, progress);

        for (stop, (start, end)) in self
            .stops
            .iter_mut()
            .zip(start.stops.iter().zip(end.stops.iter()))
        {
            stop.lerp(start, end, progress);
        }
    }
}

impl Animate for iced_core::Gradient {
    fn components() -> usize {
        iced_core::gradient::Linear::components()
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        match (self, end) {
            (iced_core::Gradient::Linear(start), iced_core::Gradient::Linear(end)) => {
                start.distance_to(end)
            }
        }
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        match self {
            iced_core::Gradient::Linear(start) => start.update(components),
        }
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        match (self, start, end) {
            (
                iced_core::Gradient::Linear(value),
                iced_core::Gradient::Linear(start),
                iced_core::Gradient::Linear(end),
            ) => {
                value.lerp(start, end, progress);
            }
        }
    }
}

impl Animate for iced_core::Background {
    fn components() -> usize {
        iced_core::gradient::Gradient::components().max(iced_core::Color::components())
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        match (self, end) {
            (iced_core::Background::Color(start), iced_core::Background::Color(end)) => {
                let mut distance = start.distance_to(end);
                distance.resize(Self::components(), 0.0);
                distance
            }
            (iced_core::Background::Color(_), iced_core::Background::Gradient(_)) => {
                vec![0.0; Self::components()]
            }
            (iced_core::Background::Gradient(start), iced_core::Background::Gradient(end)) => {
                let mut distance = start.distance_to(end);
                distance.resize(Self::components(), 0.0);
                distance
            }
            (iced_core::Background::Gradient(_), iced_core::Background::Color(_)) => {
                vec![0.0; Self::components()]
            }
        }
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        match self {
            iced_core::Background::Color(color) => {
                color.update(components);
                let extra = Self::components() - iced_core::Color::components() - 1;
                components.nth(extra);
            }
            iced_core::Background::Gradient(gradient) => gradient.update(components),
        }
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        match (self, start, end) {
            (
                iced_core::Background::Color(value),
                iced_core::Background::Color(start),
                iced_core::Background::Color(end),
            ) => {
                value.lerp(start, end, progress);
            }
            (
                iced_core::Background::Gradient(value),
                iced_core::Background::Gradient(start),
                iced_core::Background::Gradient(end),
            ) => {
                value.lerp(start, end, progress);
            }
            _ => {}
        }
    }
}

#[cfg(feature = "widgets")]
impl Animate for iced_widget::button::Style {
    fn components() -> usize {
        Option::<iced_core::Background>::components()
            + iced_core::Color::components()
            + iced_core::Border::components()
            + iced_core::Shadow::components()
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.background.distance_to(&end.background),
            self.text_color.distance_to(&end.text_color),
            self.border.distance_to(&end.border),
            self.shadow.distance_to(&end.shadow),
        ]
        .concat()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.background.update(components);
        self.text_color.update(components);
        self.border.update(components);
        self.shadow.update(components);
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.background
            .lerp(&start.background, &end.background, progress);
        self.text_color
            .lerp(&start.text_color, &end.text_color, progress);
        self.border.lerp(&start.border, &end.border, progress);
        self.shadow.lerp(&start.shadow, &end.shadow, progress);
    }
}

#[cfg(feature = "widgets")]
impl Animate for iced_widget::svg::Style {
    fn components() -> usize {
        Option::<iced_core::Color>::components()
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        self.color.distance_to(&end.color)
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.color.update(components);
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.color.lerp(&start.color, &end.color, progress);
    }
}

impl<T1, T2> Animate for (T1, T2)
where
    T1: Animate,
    T2: Animate,
{
    fn components() -> usize {
        T1::components() + T2::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.0.update(components);
        self.1.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [self.0.distance_to(&end.0), self.1.distance_to(&end.1)].concat()
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.0.lerp(&start.0, &end.0, progress);
        self.1.lerp(&start.1, &end.1, progress);
    }
}

impl<T1, T2, T3> Animate for (T1, T2, T3)
where
    T1: Animate,
    T2: Animate,
    T3: Animate,
{
    fn components() -> usize {
        T1::components() + T2::components() + T3::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.0.update(components);
        self.1.update(components);
        self.2.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.0.distance_to(&end.0),
            self.1.distance_to(&end.1),
            self.2.distance_to(&end.2),
        ]
        .concat()
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.0.lerp(&start.0, &end.0, progress);
        self.1.lerp(&start.1, &end.1, progress);
        self.2.lerp(&start.2, &end.2, progress);
    }
}

impl<T1, T2, T3, T4> Animate for (T1, T2, T3, T4)
where
    T1: Animate,
    T2: Animate,
    T3: Animate,
    T4: Animate,
{
    fn components() -> usize {
        T1::components() + T2::components() + T3::components() + T4::components()
    }

    fn update(&mut self, components: &mut impl Iterator<Item = f32>) {
        self.0.update(components);
        self.1.update(components);
        self.2.update(components);
        self.3.update(components);
    }

    fn distance_to(&self, end: &Self) -> Vec<f32> {
        [
            self.0.distance_to(&end.0),
            self.1.distance_to(&end.1),
            self.2.distance_to(&end.2),
            self.3.distance_to(&end.3),
        ]
        .concat()
    }

    fn lerp(&mut self, start: &Self, end: &Self, progress: f32) {
        self.0.lerp(&start.0, &end.0, progress);
        self.1.lerp(&start.1, &end.1, progress);
        self.2.lerp(&start.2, &end.2, progress);
        self.3.lerp(&start.3, &end.3, progress);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_components() {
        assert_eq!(f32::components(), 1);
    }

    #[test]
    fn f32_point_components() {
        assert_eq!(iced_core::Point::<f32>::components(), 2);
    }

    #[test]
    fn f32_color_components() {
        assert_eq!(iced_core::Color::components(), 4);
    }

    #[test]
    fn color_pair_components() {
        assert_eq!(
            iced_core::theme::palette::Pair::components(),
            2 * iced_core::Color::components()
        );
    }

    #[test]
    fn swatch_components() {
        assert_eq!(
            iced_core::theme::palette::Swatch::components(),
            3 * iced_core::theme::palette::Pair::components()
        );
    }

    #[test]
    fn background_components() {
        assert_eq!(
            iced_core::theme::palette::Background::components(),
            8 * iced_core::theme::palette::Pair::components()
        );
    }

    #[test]
    fn palette_components() {
        assert_eq!(
            iced_core::theme::Palette::components(),
            iced_core::theme::palette::Background::components()
                + iced_core::theme::palette::Swatch::components() * 5
        );
    }

    #[test]
    fn theme_components() {
        assert_eq!(
            Theme::components(),
            iced_core::theme::Palette::components() + iced_core::theme::palette::Seed::components()
        );
    }

    #[test]
    fn option_components() {
        assert_eq!(Option::<f32>::components(), 1);
    }

    /// `Some` value should update the value with the next component.
    #[test]
    fn option_update_some() {
        let mut option = Some(1.0);
        option.update(&mut [2.0].iter().copied());
        assert_eq!(option, Some(3.0));
    }

    /// Trying to update `None` should update the iterator, but not change the value.
    #[test]
    fn option_update_none() {
        let mut option: Option<f32> = None;
        let mut iter = [2.0, 4.0].iter().copied();
        option.update(&mut iter);
        assert_eq!(option, None);

        // The last component should be left alone by the option update.
        assert_eq!(iter.next(), Some(4.0));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn update_background() {
        let mut background = iced_core::Background::Color(iced_core::Color::BLACK);
        let components = vec![0.1_f32; iced_core::Background::components()];
        let mut components = components.iter().copied();
        background.update(&mut components);
        assert_ne!(
            background,
            iced_core::Background::Color(iced_core::Color::BLACK)
        );
        assert_eq!(components.len(), 0);
    }

    #[test]
    #[cfg(feature = "widgets")]
    fn update_button_style() {
        let style = iced_widget::button::Style {
            background: Some(iced_core::Background::Color(iced_core::Color::BLACK)),
            text_color: iced_core::Color::BLACK,
            border: iced_core::Border::default(),
            shadow: iced_core::Shadow::default(),
            snap: true,
        };
        let target = iced_widget::button::Style {
            background: Some(iced_core::Background::Color(iced_core::Color::WHITE)),
            text_color: iced_core::Color::WHITE,
            border: iced_core::Border::default().width(1.0),
            shadow: iced_core::Shadow::default(),
            snap: true,
        };

        let mut spring = crate::Spring::new(style);
        spring.set_target(target);
        spring.tick(iced_core::time::Instant::now());
        assert_ne!(*spring.value(), style);
    }
}
