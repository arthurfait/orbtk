use std::rc::Rc;

use crate::{
    prelude::*,
    proc_macros::{Event, IntoHandler},
    shell::MouseButton,
    utils::*,
};

/// Checks if the given point is inside of a widget.
pub fn check_mouse_condition(mouse_position: Point, widget: &WidgetContainer<'_>) -> bool {
    let enabled = widget.get::<bool>("enabled");

    if !enabled {
        return false;
    }

    let bounds = widget.get::<Rectangle>("bounds");
    let position = widget.get::<Point>("position");

    let mut rect = Rectangle::new(0.0, 0.0, bounds.width(), bounds.height());

    rect.set_x(position.x);
    rect.set_y(position.y);

    rect.contains((mouse_position.x, mouse_position.y))
}

/// `MouseMoveEvent` indicates if the mouse position is changed on the window.
#[derive(Event)]
pub struct MouseMoveEvent {
    /// Current x position of the mouse on the window.
    pub x: f64,

    /// Current y position of the mouse on the window.
    pub y: f64,
}

/// `ScrollEvent` occurs when the mouse wheel is moved.
#[derive(Event)]
pub struct ScrollEvent {
    /// Indicates the scroll offset x and y.
    pub delta: Point,
}

/// Represents the current mouse state of an mouse event.
#[derive(Debug, Copy, Clone)]
pub struct Mouse {
      /// Indicates the mouse button that is connected to the event.
      pub button: MouseButton,

      /// Indicates the x position of the event on the window.
      pub x: f64,
  
      /// Indicates the y position of the event on the window.
      pub y: f64,
}

/// `MouseUpEvent` occurs when a mouse button is released.
#[derive(Event)]
pub struct MouseUpEvent {
    /// Indicates the mouse button that is released.
    pub button: MouseButton,

    /// Indicates the x position of the event on the window.
    pub x: f64,

    /// Indicates the y position of the event on the window.
    pub y: f64,
}

/// `ClickEvent` occurs when a user clicked on an element.
#[derive(Event)]
pub struct ClickEvent {
    /// Indicates the x and y position of the click event.
    pub position: Point,
}

/// `MouseDownEvent` occurs when a mouse button is pressed.
#[derive(Event)]
pub struct MouseDownEvent {
     /// Indicates the mouse button that is pressed.
     pub button: MouseButton,

     /// Indicates the x position of the event on the window.
     pub x: f64,
 
     /// Indicates the y position of the event on the window.
     pub y: f64,
}

/// `GlobalMouseUpEvent` occurs when a mouse button is released. 
///
/// Global events could not be handled and could be read on each state.
#[derive(Event)]
pub struct GlobalMouseUpEvent {
    /// Indicates the mouse button that is released.
    pub button: MouseButton,

    /// Indicates the x position of the event on the window.
    pub x: f64,

    /// Indicates the y position of the event on the window.
    pub y: f64,
}

/// Defines the mouse handler function.
pub type MouseHandlerFunction = dyn Fn(&mut StatesContext, Mouse) -> bool + 'static;

//// Defines a position based event handler.
pub type PositionHandlerFunction = dyn Fn(&mut StatesContext, Point) -> bool + 'static;

/// Defines the global bouse handler function.
pub type GlobalMouseHandlerFunction = dyn Fn(&mut StatesContext, Mouse) + 'static;

/// Used to handle click events. Could be attached to a widget.
pub struct ClickEventHandler {
    handler: Rc<PositionHandlerFunction>,
}

impl Into<Rc<dyn EventHandler>> for ClickEventHandler {
    fn into(self) -> Rc<dyn EventHandler> {
        Rc::new(self)
    }
}

impl EventHandler for ClickEventHandler {
    fn handle_event(&self, state_context: &mut StatesContext, event: &EventBox) -> bool {
        event
            .downcast_ref::<ClickEvent>()
            .ok()
            .map_or(false, |event| (self.handler)(state_context, event.position))
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<ClickEvent>()
    }
}

/// Used to handle mouse down events. Could be attached to a widget.
#[derive(IntoHandler)]
pub struct MouseDownEventHandler {
    handler: Rc<MouseHandlerFunction>,
}

impl EventHandler for MouseDownEventHandler {
    fn handle_event(&self, state_context: &mut StatesContext, event: &EventBox) -> bool {
        event
            .downcast_ref::<MouseDownEvent>()
            .ok()
            .map_or(false, |event| {
                (self.handler)(state_context, Mouse { button: event.button, x: event.x, y: event.y })
            })
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<MouseDownEvent>()
    }
}

/// Defines an event handler for a global mouse up event. Global mouse up events could not be handled.
#[derive(IntoHandler)]
pub struct GlobalMouseUpEventHandler {
    handler: Rc<GlobalMouseHandlerFunction>,
}

impl EventHandler for GlobalMouseUpEventHandler {
    fn handle_event(&self, state_context: &mut StatesContext, event: &EventBox) -> bool {
        event
            .downcast_ref::<GlobalMouseUpEvent>()
            .ok()
            .map_or(false, |event| {
                (self.handler)(state_context,  Mouse { button: event.button, x: event.x, y: event.y });
                false
            })
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<GlobalMouseUpEvent>()
    }
}

/// Used to handle mouse down events. Could be attached to a widget.
#[derive(IntoHandler)]
pub struct MouseUpEventHandler {
    handler: Rc<MouseHandlerFunction>,
}

impl EventHandler for MouseUpEventHandler {
    fn handle_event(&self, state_context: &mut StatesContext, event: &EventBox) -> bool {
        event
            .downcast_ref::<MouseUpEvent>()
            .ok()
            .map_or(false, |event| {
                (self.handler)(state_context,  Mouse { button: event.button, x: event.x, y: event.y })
            })
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<MouseUpEvent>()
    }
}

/// Used to handle mouse down events. Could be attached to a widget.
#[derive(IntoHandler)]
pub struct MouseMoveEventHandler {
    handler: Rc<PositionHandlerFunction>,
}

impl EventHandler for MouseMoveEventHandler {
    fn handle_event(&self, state_context: &mut StatesContext, event: &EventBox) -> bool {
        event
            .downcast_ref::<MouseMoveEvent>()
            .ok()
            .map_or(false, |event| {
                (self.handler)(state_context, Point::new(event.x, event.y))
            })
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<MouseMoveEvent>()
    }
}

/// Used to handle scroll events. Could be attached to a widget.
#[derive(IntoHandler)]
pub struct ScrollEventHandler {
    handler: Rc<PositionHandlerFunction>,
}

impl EventHandler for ScrollEventHandler {
    fn handle_event(&self, state_context: &mut StatesContext, event: &EventBox) -> bool {
        event
            .downcast_ref::<ScrollEvent>()
            .ok()
            .map_or(false, |event| {
                (self.handler)(state_context, Point::new(event.delta.x, event.delta.y))
            })
    }

    fn handles_event(&self, event: &EventBox) -> bool {
        event.is_type::<ScrollEvent>()
    }
}

pub trait MouseHandler: Sized + Widget {
    /// Inserts a click handler.
    fn on_click<H: Fn(&mut StatesContext, Point) -> bool + 'static>(self, handler: H) -> Self {
        self.insert_handler(ClickEventHandler {
            handler: Rc::new(handler),
        })
    }

    /// Insert a mouse down handler.
    fn on_mouse_down<H: Fn(&mut StatesContext, Mouse) -> bool + 'static>(self, handler: H) -> Self {
        self.insert_handler(MouseDownEventHandler {
            handler: Rc::new(handler),
        })
    }

    /// Insert a mouse up handler.
    fn on_mouse_up<H: Fn(&mut StatesContext, Mouse) -> bool + 'static>(self, handler: H) -> Self {
        self.insert_handler(MouseUpEventHandler {
            handler: Rc::new(handler),
        })
    }

    /// Insert a mouse handler for global up event.
    fn on_global_mouse_up<H: Fn(&mut StatesContext, Mouse) + 'static>(self, handler: H) -> Self {
        self.insert_handler(GlobalMouseUpEventHandler {
            handler: Rc::new(handler),
        })
    }

    /// Insert a mouse move handler.
    fn on_mouse_move<H: Fn(&mut StatesContext, Point) -> bool + 'static>(self, handler: H) -> Self {
        self.insert_handler(MouseMoveEventHandler {
            handler: Rc::new(handler),
        })
    }

    /// Insert a mouse up handler.
    fn on_scroll<H: Fn(&mut StatesContext, Point) -> bool + 'static>(self, handler: H) -> Self {
        self.insert_handler(ScrollEventHandler {
            handler: Rc::new(handler),
        })
    }
}
