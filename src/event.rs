use ggez::{Context, GameError};
use ggez::event::{ControlFlow, ErrorOrigin, EventLoop, MouseButton, process_event};
use ggez::input::keyboard::{KeyCode, KeyInput, KeyMods};
use ggez::winit::dpi;
use ggez::winit::event::{ElementState, Event, Ime, KeyboardInput, MouseScrollDelta, WindowEvent};
use tracing::{debug, error};
use crate::net::command::Command;
use crate::state::State;

/// Runs the game's main loop, calling event callbacks on the given state
/// object as events occur.
///
/// It does not try to do any type of framerate limiting.  See the
/// documentation for the [`timer`](../timer/index.html) module for more info.
#[allow(clippy::needless_return)] // necessary as the returns used here are actually necessary to break early from the event loop
pub fn run<S: 'static, E>(mut ctx: Context, event_loop: EventLoop<()>, mut app: S, mut state: State) -> !
    where
        S: AppEventHandler<E>,
        E: std::fmt::Debug,
{
    event_loop.run(move |mut event, _, control_flow| {
        let ctx = &mut ctx;
        let app = &mut app;
        let state = &mut state;

        if ctx.quit_requested {
            let res = app.quit_event(ctx, state);
            ctx.quit_requested = false;
            if let Ok(false) = res {
                ctx.continuing = false;
            } else if catch_error(ctx, res, app, control_flow, ErrorOrigin::QuitEvent, state) {
                return;
            }
        }
        if !ctx.continuing {
            *control_flow = ControlFlow::Exit;
            return;
        }

        *control_flow = ControlFlow::Poll;

        process_event(ctx, &mut event);
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
                    // let actual_size = logical_size;
                    let res = app.resize_event(
                        ctx,
                        logical_size.width as f32,
                        logical_size.height as f32, state
                    );
                    if catch_error(ctx, res, app, control_flow, ErrorOrigin::ResizeEvent, state) {
                        return;
                    };
                }
                WindowEvent::CloseRequested => {
                    let res = app.quit_event(ctx, state);
                    if let Ok(false) = res {
                        ctx.continuing = false;
                    } else if catch_error(ctx, res, app, control_flow, ErrorOrigin::QuitEvent, state) {
                        return;
                    }
                }
                WindowEvent::Focused(gained) => {
                    let res = app.focus_event(ctx, gained, state);
                    if catch_error(ctx, res, app, control_flow, ErrorOrigin::FocusEvent, state) {
                        return;
                    };
                }
                WindowEvent::ReceivedCharacter(ch) => {
                    let res = app.text_input_event(ctx, ch, state);
                    if catch_error(ctx, res, app, control_flow, ErrorOrigin::TextInputEvent, state) {
                        return;
                    };
                },
                WindowEvent::Ime(ime) => {
                    let res = app.ime_input_event(ctx, ime, state);
                    if catch_error(ctx, res, app, control_flow, ErrorOrigin::TextInputEvent, state) {
                        return;
                    };
                },
                WindowEvent::ModifiersChanged(mods) => {
                    ctx.keyboard.set_modifiers(KeyMods::from(mods))
                }
                WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: keycode,
                        scancode,
                        ..
                    },
                    ..
                } => {
                    let repeat = ctx.keyboard.is_key_repeated();
                    let res = app.key_down_event(
                        ctx,
                        KeyInput {
                            scancode,
                            keycode,
                            mods: ctx.keyboard.active_mods(),
                        },
                        repeat, state
                    );
                    if catch_error(ctx, res, app, control_flow, ErrorOrigin::KeyDownEvent, state) {
                        return;
                    };
                }
                WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: keycode,
                        scancode,
                        ..
                    },
                    ..
                } => {
                    let res = app.key_up_event(
                        ctx,
                        KeyInput {
                            scancode,
                            keycode,
                            mods: ctx.keyboard.active_mods(),
                        }, state
                    );
                    if catch_error(ctx, res, app, control_flow, ErrorOrigin::KeyUpEvent, state) {
                        return;
                    };
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    let (x, y) = match delta {
                        MouseScrollDelta::LineDelta(x, y) => (x, y),
                        MouseScrollDelta::PixelDelta(pos) => {
                            let scale_factor = ctx.gfx.window().scale_factor();
                            let dpi::LogicalPosition { x, y } = pos.to_logical::<f32>(scale_factor);
                            (x, y)
                        }
                    };
                    let res = app.mouse_wheel_event(ctx, x, y, state);
                    if catch_error(ctx, res, app, control_flow, ErrorOrigin::MouseWheelEvent, state) {
                        return;
                    };
                }
                WindowEvent::MouseInput {
                    state: element_state,
                    button,
                    ..
                } => {
                    let position = ctx.mouse.position();
                    match element_state {
                        ElementState::Pressed => {
                            let res =
                                app.mouse_button_down_event(ctx, button, position.x, position.y, state);
                            if catch_error(
                                ctx,
                                res,
                                app,
                                control_flow,
                                ErrorOrigin::MouseButtonDownEvent, state
                            ) {
                                return;
                            };
                        }
                        ElementState::Released => {
                            let res =
                                app.mouse_button_up_event(ctx, button, position.x, position.y, state);
                            if catch_error(
                                ctx,
                                res,
                                app,
                                control_flow,
                                ErrorOrigin::MouseButtonUpEvent, state
                            ) {
                                return;
                            };
                        }
                    }
                }
                WindowEvent::CursorMoved { .. } => {
                    let position = ctx.mouse.position();
                    let delta = ctx.mouse.last_delta();
                    let res =
                        app.mouse_motion_event(ctx, position.x, position.y, delta.x, delta.y, state);
                    if catch_error(ctx, res, app, control_flow, ErrorOrigin::MouseMotionEvent, state) {
                        return;
                    };
                }
                // WindowEvent::Touch(touch) => {
                //     let res =
                //         state.touch_event(ctx, touch.phase, touch.location.x, touch.location.y);
                //     if catch_error(ctx, res, state, control_flow, ErrorOrigin::TouchEvent) {
                //         return;
                //     };
                // }
                WindowEvent::CursorEntered { device_id: _ } => {
                    let res = app.mouse_enter_or_leave(ctx, true, state);
                    if catch_error(
                        ctx,
                        res,
                        app,
                        control_flow,
                        ErrorOrigin::MouseEnterOrLeave, state
                    ) {
                        return;
                    }
                }
                WindowEvent::CursorLeft { device_id: _ } => {
                    let res = app.mouse_enter_or_leave(ctx, false, state);
                    if catch_error(
                        ctx,
                        res,
                        app,
                        control_flow,
                        ErrorOrigin::MouseEnterOrLeave, state
                    ) {
                        return;
                    }
                }
                _x => {
                    // trace!("ignoring window event {:?}", x);
                }
            },
            Event::DeviceEvent { .. } => (),
            Event::Resumed => (),
            Event::Suspended => (),
            Event::NewEvents(_) => (),
            Event::UserEvent(_) => (),
            Event::MainEventsCleared => {
                // If you are writing your own event loop, make sure
                // you include `timer_context.tick()` and
                // `ctx.process_event()` calls.  These update ggez's
                // internal state however necessary.
                ctx.time.tick();

                // Handle gamepad events if necessary.
                // #[cfg(feature = "gamepad")]
                // while let Some(gilrs::Event { id, event, .. }) = ctx.gamepad.next_event() {
                //     match event {
                //         gilrs::EventType::ButtonPressed(button, _) => {
                //             let res = state.gamepad_button_down_event(ctx, button, GamepadId(id));
                //             if catch_error(
                //                 ctx,
                //                 res,
                //                 state,
                //                 control_flow,
                //                 ErrorOrigin::GamepadButtonDownEvent,
                //             ) {
                //                 return;
                //             };
                //         }
                //         gilrs::EventType::ButtonReleased(button, _) => {
                //             let res = state.gamepad_button_up_event(ctx, button, GamepadId(id));
                //             if catch_error(
                //                 ctx,
                //                 res,
                //                 state,
                //                 control_flow,
                //                 ErrorOrigin::GamepadButtonUpEvent,
                //             ) {
                //                 return;
                //             };
                //         }
                //         gilrs::EventType::AxisChanged(axis, value, _) => {
                //             let res = state.gamepad_axis_event(ctx, axis, value, GamepadId(id));
                //             if catch_error(
                //                 ctx,
                //                 res,
                //                 state,
                //                 control_flow,
                //                 ErrorOrigin::GamepadAxisEvent,
                //             ) {
                //                 return;
                //             };
                //         }
                //         _ => {}
                //     }
                // }

                let res = app.update(ctx, state);
                if catch_error(ctx, res, app, control_flow, ErrorOrigin::Update, state) {
                    return;
                };

                if let Err(e) = ctx.gfx.begin_frame() {
                    error!("Error on GraphicsContext::begin_frame(): {e:?}");
                    eprintln!("Error on GraphicsContext::begin_frame(): {e:?}");
                    *control_flow = ControlFlow::Exit;
                }

                if let Err(e) = app.draw(ctx, state) {
                    error!("Error on EventHandler::draw(): {e:?}");
                    eprintln!("Error on EventHandler::draw(): {e:?}");
                    if app.on_error(ctx, ErrorOrigin::Draw, e, state) {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }

                if let Err(e) = ctx.gfx.end_frame() {
                    error!("Error on GraphicsContext::end_frame(): {e:?}");
                    eprintln!("Error on GraphicsContext::end_frame(): {e:?}");
                    *control_flow = ControlFlow::Exit;
                }

                // reset the mouse delta for the next frame
                // necessary because it's calculated cumulatively each cycle
                ctx.mouse.reset_delta();

                // Copy the state of the keyboard into the KeyboardContext
                // and the mouse into the MouseContext
                ctx.keyboard.save_keyboard_state();
                ctx.mouse.save_mouse_state();
            }
            Event::RedrawRequested(_) => (),
            Event::RedrawEventsCleared => (),
            Event::LoopDestroyed => (),
        }
    })
}

fn catch_error<T, E, S: 'static>(
    ctx: &mut Context,
    event_result: Result<T, E>,
    state: &mut S,
    control_flow: &mut ControlFlow,
    origin: ErrorOrigin,
    app_state: &mut State
) -> bool
    where
        S: AppEventHandler<E>,
        E: std::fmt::Debug,
{
    if let Err(e) = event_result {
        error!("Error on EventHandler {origin:?}: {e:?}");
        eprintln!("Error on EventHandler {origin:?}: {e:?}");
        if state.on_error(ctx, origin, e, app_state) {
            *control_flow = ControlFlow::Exit;
            return true;
        }
    }
    false
}

pub trait AppEventHandler<E = GameError>
    where
        E: std::fmt::Debug
{

    fn select(&mut self, _ctx: &mut Context, _state: &mut State) -> Option<Box<dyn AppEventHandler>> {
        None
    }

    fn net(&mut self, _ctx: &mut Context, _state: &mut State, cmd: Command) -> Result<bool, E> {
        Ok(false)
    }

    /// Called upon each logic update to the game.
    /// This should be where the game's logic takes place.
    fn update(&mut self, _ctx: &mut Context, _state: &mut State) -> Result<(), E>;

    /// Called to do the drawing of your game.
    /// You probably want to start this with
    /// [`Canvas::from_frame`](../graphics/struct.Canvas.html#method.from_frame) and end it
    /// with [`Canvas::finish`](../graphics/struct.Canvas.html#method.finish).
    fn draw(&mut self, _ctx: &mut Context, _state: &mut State) -> Result<(), E>;

    /// A mouse button was pressed
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32, _state: &mut State
    ) -> Result<(), E> {
        Ok(())
    }

    /// A mouse button was released
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32, _state: &mut State
    ) -> Result<(), E> {
        Ok(())
    }

    /// The mouse was moved; it provides both absolute x and y coordinates in the window,
    /// and relative x and y coordinates compared to its last position.
    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _x: f32,
        _y: f32,
        _dx: f32,
        _dy: f32, _state: &mut State
    ) -> Result<(), E> {
        Ok(())
    }

    /// mouse entered or left window area
    fn mouse_enter_or_leave(&mut self, _ctx: &mut Context, _entered: bool, _state: &mut State) -> Result<(), E> {
        Ok(())
    }

    /// The mousewheel was scrolled, vertically (y, positive away from and negative toward the user)
    /// or horizontally (x, positive to the right and negative to the left).
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _state: &mut State) -> Result<(), E> {
        Ok(())
    }

    /// A keyboard button was pressed.
    ///
    /// The default implementation of this will call [`ctx.request_quit()`](crate::Context::request_quit)
    /// when the escape key is pressed. If you override this with your own
    /// event handler you have to re-implement that functionality yourself.
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool, _state: &mut State
    ) -> Result<(), E> {
        if input.keycode == Some(KeyCode::Escape) {
            ctx.request_quit();
        }
        Ok(())
    }

    /// A keyboard button was released.
    fn key_up_event(&mut self, _ctx: &mut Context, _input: KeyInput, _state: &mut State) -> Result<(), E> {
        Ok(())
    }

    /// A unicode character was received, usually from keyboard input.
    /// This is the intended way of facilitating text input.
    fn text_input_event(&mut self, _ctx: &mut Context, _character: char, _state: &mut State) -> Result<(), E> {
        Ok(())
    }

    fn ime_input_event(&mut self, _ctx: &mut Context, _ime: Ime, _state: &mut State) -> Result<(), E> {
        Ok(())
    }

    /// Called when the window is shown or hidden.
    fn focus_event(&mut self, _ctx: &mut Context, _gained: bool, _state: &mut State) -> Result<(), E> {
        Ok(())
    }

    /// Called upon a quit event.  If it returns true,
    /// the game does not exit (the quit event is cancelled).
    fn quit_event(&mut self, _ctx: &mut Context, _state: &mut State) -> Result<bool, E> {
        debug!("quit_event() callback called, quitting...");
        Ok(false)
    }

    /// Called when the user resizes the window, or when it is resized
    /// via [`GraphicsContext::set_mode()`](../graphics/struct.GraphicsContext.html#method.set_mode).
    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32, _state: &mut State) -> Result<(), E> {
        Ok(())
    }

    /// Something went wrong, causing a `GameError` (or some other kind of error, depending on what you specified).
    /// If this returns true, the error was fatal, so the event loop ends, aborting the game.
    fn on_error(&mut self, _ctx: &mut Context, _origin: ErrorOrigin, _e: E, _state: &mut State) -> bool {
        true
    }
}