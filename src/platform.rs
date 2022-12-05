// SPDX-FileCopyrightText: 2022 Redox OS Developers <info@redox-os.org>
// SPDX-License-Identifier: MIT

use std::cell::*;
use std::rc::*;
use std::slice;

use orbclient::{Renderer, Window};

use crate::color::*;
use crate::event_reader::*;

static INITIAL_INSTANT: once_cell::sync::OnceCell<instant::Instant> =
    once_cell::sync::OnceCell::new();

/// Used to configure the platform window.
#[derive(Clone, Default, Debug)]
pub struct Config {
    x: Option<i32>,
    y: Option<i32>,
    width: u32,
    height: u32,
    title: String,
    resizable: bool,
    borderless: bool,
    uncloseable: bool,
}

impl Config {
    /// Sets the x position of the window.
    pub fn x(mut self, x: i32) -> Self {
        self.x = Some(x);
        self
    }

    /// Sets the y position of the window.
    pub fn y(mut self, y: i32) -> Self {
        self.y = Some(y);
        self
    }

    /// Sets the width of the window.
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Sets the width of the window.
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    /// Sets the title of the window.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// If set to `true` the window is resizable.
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// If set to `true` the window is borderless.
    pub fn borderless(mut self, borderless: bool) -> Self {
        self.borderless = borderless;
        self
    }

    /// If set to `true` the window is uncloseable.
    pub fn uncloseable(mut self, uncloseable: bool) -> Self {
        self.uncloseable = uncloseable;
        self
    }
}

impl From<Config> for OrbClientPlatform {
    fn from(config: Config) -> Self {
        let mut flags = vec![orbclient::WindowFlag::Async];

        if config.resizable {
            flags.push(orbclient::WindowFlag::Resizable);
        }

        if config.borderless {
            flags.push(orbclient::WindowFlag::Borderless);
        }

        if config.uncloseable {
            flags.push(orbclient::WindowFlag::Unclosable);
        }

        let x = if let Some(x) = config.x {
            x
        } else {
            // center window horizontal if no x is set.
            orbclient::get_display_size().map_or(0, |s| (s.0 - config.width) / 2) as i32
        };

        let y = if let Some(y) = config.y {
            y
        } else {
            // center window horizontal if no x is set.
            orbclient::get_display_size().map_or(0, |s| (s.1 - config.height) / 2) as i32
        };

        Self {
            slint_window: core::cell::RefCell::new(
                slint::platform::software_renderer::MinimalSoftwareWindow::new(),
            ),
            orb_window: RefCell::new(
                Window::new_flags(x, y, config.width, config.height, &config.title, &flags)
                    .unwrap(),
            ),
            event_reader: EventReader::default(),
        }
    }
}

/// Describes a `Slint` platform that is based on `OrbClient` and so can run on also on `Redox OS`.
pub struct OrbClientPlatform {
    slint_window: RefCell<Rc<slint::platform::software_renderer::MinimalSoftwareWindow<1>>>,
    orb_window: RefCell<Window>,
    event_reader: EventReader,
}

impl OrbClientPlatform {
    /// Returns a new platform object.
    pub fn new() -> Self {
        Config::default()
            .width(600)
            .height(400)
            .title("Slint orbclient window")
            .into()
    }
}

impl Default for OrbClientPlatform {
    fn default() -> Self {
        OrbClientPlatform::new()
    }
}

// todo: event converter with tests

impl slint::platform::Platform for OrbClientPlatform {
    fn create_window_adapter(&self) -> Rc<dyn slint::platform::WindowAdapter> {
        self.slint_window.borrow().clone()
    }

    fn set_clipboard_text(&self, text: &str) {
        self.orb_window.borrow_mut().set_clipboard(text);
    }

    fn clipboard_text(&self) -> Option<String> {
        Some(self.orb_window.borrow().clipboard())
    }

    fn duration_since_start(&self) -> core::time::Duration {
        let the_beginning = *INITIAL_INSTANT.get_or_init(instant::Instant::now);
        instant::Instant::now() - the_beginning
    }

    fn run_event_loop(&self) {
        let width = self.orb_window.borrow().width();
        let height = self.orb_window.borrow().height();

        self.slint_window
            .borrow()
            .as_ref()
            .set_size(slint::PhysicalSize::new(width, height));

        'events: loop {
            slint::platform::update_timers_and_animations();

            {
                let slint_window = self.slint_window.borrow();

                slint_window.draw_if_needed(|renderer| {
                    let width = slint_window.size().width as usize;
                    let height = slint_window.size().height as usize;
                    renderer.render(
                        unsafe {
                            slice::from_raw_parts_mut(
                                self.orb_window.borrow_mut().data_mut().as_mut_ptr() as *mut Color,
                                width * height
                            )
                        },
                        width
                    );

                    self.orb_window.borrow_mut().sync();
                });

                for event in self.event_reader.read(&mut self.orb_window.borrow_mut()) {
                    match event {
                        Event::WindowEvent(e) => slint_window.dispatch_event(e),
                        Event::Resize(s) => slint_window.set_size(s),
                        Event::Quit => break 'events,
                    }
                }

                if slint_window.has_active_animations() {
                    continue;
                }
            }

            #[cfg(target_os = "redox")]
            {
                use std::{io, os::unix::io::AsRawFd};

                let timeout = match slint::platform::duration_until_next_timer_update() {
                    Some(duration) => duration.as_millis().try_into().unwrap(),
                    None => -1,
                };

                let mut fds = [
                    libc::pollfd {
                        fd: self.orb_window.borrow().as_raw_fd(),
                        events: libc::POLLIN,
                        revents: 0,
                    }
                ];
                let _nevents = unsafe {
                    libc::poll(fds.as_mut_ptr(), fds.len().try_into().unwrap(), timeout)
                };
                //TODO: use nevents or check for error (nevents == -1)?
            }
        }
    }
}
