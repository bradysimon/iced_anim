//! An animated button that will automatically transition between different styles.
use crate::{animated::Mode, AnimatedState};
use iced_core::{
    layout,
    mouse::{self, Cursor},
    overlay, renderer, touch,
    widget::{tree, Operation, Tree},
    window, Background, Color, Element, Event, Layout, Length, Padding, Rectangle, Shell, Size,
    Vector, Widget,
};

// Re-export the widget types for convenience
pub use iced_widget::button::{
    danger, primary, secondary, success, text, Catalog, Status, Style, StyleFn,
};

/// An animated button that will automatically transition between different styles.
pub struct Button<'a, Message, Theme = iced_core::Theme, Renderer = iced_widget::Renderer>
where
    Renderer: iced_core::Renderer,
    Theme: Catalog,
{
    content: Element<'a, Message, Theme, Renderer>,
    on_press: Option<OnPress<'a, Message>>,
    width: Length,
    height: Length,
    padding: Padding,
    clip: bool,
    class: Theme::Class<'a>,
    mode: Mode,
}

enum OnPress<'a, Message> {
    Direct(Message),
    Closure(Box<dyn Fn() -> Message + 'a>),
}

impl<'a, Message: Clone> OnPress<'a, Message> {
    fn get(&self) -> Message {
        match self {
            OnPress::Direct(message) => message.clone(),
            OnPress::Closure(f) => f(),
        }
    }
}

impl<'a, Message, Theme, Renderer> Button<'a, Message, Theme, Renderer>
where
    Renderer: iced_core::Renderer,
    Theme: Catalog,
{
    /// Creates a new [`Button`] with the given content.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        let content = content.into();

        Button {
            content,
            on_press: None,
            width: Length::Fit,
            height: Length::Fit,
            padding: DEFAULT_PADDING,
            clip: false,
            class: Theme::default(),
            mode: Mode::default(),
        }
    }

    /// Sets the width of the [`Button`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Button`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the [`Padding`] of the [`Button`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the message that will be produced when the [`Button`] is pressed.
    ///
    /// Unless `on_press` is called, the [`Button`] will be disabled.
    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = Some(OnPress::Direct(on_press));
        self
    }

    /// Sets the message that will be produced when the [`Button`] is pressed.
    ///
    /// This is analogous to [`Button::on_press`], but using a closure to produce
    /// the message.
    ///
    /// This closure will only be called when the [`Button`] is actually pressed and,
    /// therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow.
    pub fn on_press_with(mut self, on_press: impl Fn() -> Message + 'a) -> Self {
        self.on_press = Some(OnPress::Closure(Box::new(on_press)));
        self
    }

    /// Sets the message that will be produced when the [`Button`] is pressed,
    /// if `Some`.
    ///
    /// If `None`, the [`Button`] will be disabled.
    pub fn on_press_maybe(mut self, on_press: Option<Message>) -> Self {
        self.on_press = on_press.map(OnPress::Direct);
        self
    }

    /// Sets whether the contents of the [`Button`] should be clipped on
    /// overflow.
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Sets the style of the [`Button`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`Button`].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// Sets the animation of the [`Button`].
    pub fn animation(mut self, mode: impl Into<Mode>) -> Self {
        self.mode = mode.into();
        self
    }

    /// The initial status that this widget will have based on its properties.
    ///
    /// This will be used as the initial state value.
    fn get_initial_status(&self) -> Status {
        if self.on_press.is_some() {
            Status::Active
        } else {
            Status::Disabled
        }
    }

    /// Gets the status of the [`Button`] based on the current [`State`].
    fn get_status(&self, state: &State, cursor: Cursor, layout: Layout) -> Status {
        let is_mouse_over = cursor.is_over(layout.bounds());
        if self.on_press.is_none() {
            Status::Disabled
        } else if is_mouse_over {
            if state.is_pressed {
                Status::Pressed
            } else {
                Status::Hovered
            }
        } else {
            Status::Active
        }
    }
}

#[derive(Debug)]
struct State {
    is_pressed: bool,
    animated_state: AnimatedState<Status, Style>,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Button<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_core::Renderer,
    Theme: Catalog,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        let status = self.get_initial_status();
        // Initialize the state with the current style.
        let state = State {
            is_pressed: false,
            animated_state: AnimatedState::new(status, self.mode),
        };

        tree::State::new(state)
    }

    fn diff(&mut self, tree: &mut Tree) {
        // If the style changes from outside, then immediately update the style.
        let state = tree.state.downcast_mut::<State>();
        state.animated_state.diff(self.mode);
        tree.diff_children(std::slice::from_mut(&mut self.content));
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        layout::padded(
            tree,
            limits,
            self.width,
            self.height,
            self.padding,
            |tree, limits| {
                self.content.as_widget_mut().layout(tree, renderer, limits);

                tree.size
            },
        );
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds(), viewport);
        operation.traverse(&mut |operation| {
            let (layout, tree) = layout.iter_mut(&mut tree.children).next().unwrap();

            self.content
                .as_widget_mut()
                .operate(tree, layout, viewport, renderer, operation);
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let (content_layout, content_tree) = layout.iter_mut(&mut tree.children).next().unwrap();

        self.content.as_widget_mut().update(
            content_tree,
            event,
            content_layout,
            cursor,
            renderer,
            shell,
            viewport,
        );

        if shell.is_event_captured() {
            return;
        }

        // Redraw anytime the status changes and would trigger a style change.
        let state = tree.state.downcast_mut::<State>();
        let status = self.get_status(state, cursor, layout);
        let needs_redraw = state.animated_state.needs_redraw(status);

        if needs_redraw {
            shell.request_redraw();
        }

        match event {
            Event::Window(window::Event::RedrawRequested(now)) => {
                state.animated_state.tick(*now);
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. })
                if self.on_press.is_some() =>
            {
                let bounds = layout.bounds();

                if cursor.is_over(bounds) {
                    let state = tree.state.downcast_mut::<State>();

                    state.is_pressed = true;
                    shell.request_redraw();

                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if let Some(on_press) = self.on_press.as_ref().map(OnPress::get) {
                    let state = tree.state.downcast_mut::<State>();

                    if state.is_pressed {
                        state.is_pressed = false;
                        shell.request_redraw();

                        let bounds = layout.bounds();

                        if cursor.is_over(bounds) {
                            shell.publish(on_press);
                        }

                        shell.capture_event();
                    }
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                let state = tree.state.downcast_mut::<State>();
                shell.request_redraw();

                state.is_pressed = false;
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let state = tree.state.downcast_ref::<State>();
        let (content_layout, content_tree) = layout.iter(&tree.children).next().unwrap();

        let style = state
            .animated_state
            .current_value(|status| theme.style(&self.class, *status));

        if style.background.is_some() || style.border.width > 0.0 || style.shadow.color.a > 0.0 {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: style.border,
                    shadow: style.shadow,
                    snap: style.snap,
                },
                style
                    .background
                    .unwrap_or(Background::Color(Color::TRANSPARENT)),
            );
        }

        let viewport = if self.clip {
            bounds.intersection(viewport).unwrap_or(*viewport)
        } else {
            *viewport
        };

        self.content.as_widget().draw(
            content_tree,
            renderer,
            theme,
            &renderer::Style {
                text_color: style.text_color,
            },
            content_layout,
            cursor,
            &viewport,
        );
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let is_mouse_over = cursor.is_over(layout.bounds());

        if is_mouse_over && self.on_press.is_some() {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Message, Theme, Renderer>> {
        let (layout, tree) = layout.iter_mut(&mut tree.children).next().unwrap();

        self.content
            .as_widget_mut()
            .overlay(tree, layout, renderer, viewport, translation, window)
    }
}

impl<'a, Message, Theme, Renderer> From<Button<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: iced_core::Renderer + 'a,
{
    fn from(button: Button<'a, Message, Theme, Renderer>) -> Self {
        Self::new(button)
    }
}

/// The default [`Padding`] of a [`Button`].
pub(crate) const DEFAULT_PADDING: Padding = Padding {
    top: 5.0,
    bottom: 5.0,
    right: 10.0,
    left: 10.0,
};

pub fn button<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Button<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: iced_core::Renderer,
{
    Button::new(content)
}
