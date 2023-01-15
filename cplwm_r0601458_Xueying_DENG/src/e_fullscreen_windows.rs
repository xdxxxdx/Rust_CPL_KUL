//! Optional: Fullscreen Windows
//!
//! Extend your window manager with support for fullscreen windows, i.e. the
//! ability to temporarily make a window take up the whole screen, thereby
//! obscuring all other windows. See the documentation of the
//! [`FullscreenSupport`] trait for the precise requirements. Don't confuse
//! this with the first assignment, in which you built a window manager that
//! displayed all windows fullscreen.
//!
//! Like in the previous assignments, either make a copy of, or define a
//! wrapper around your previous window manager to implement the
//! [`FullscreenSupport`] trait as well. Note that this window manager must
//! still implement all the traits from previous assignments.
//!
//! [`FullscreenSupport`]: ../../cplwm_api/wm/trait.FullscreenSupport.html
//!
//! # Status
//!
//! **TODO**: Replace the question mark below with YES, NO, or PARTIAL to
//! indicate the status of this assignment. If you want to tell something
//! about this assignment to the grader, e.g., you have a bug you can't fix,
//! or you want to explain your approach, write it down after the comments
//! section.
//!
//! COMPLETED: YES
//!
//! COMMENTS:
//!
//!
//!

#![allow(unused_variables)]
// Add imports here
use std::error;
use std::fmt;
use cplwm_api::types::{FloatOrTile, Geometry, PrevOrNext, Screen, Window, WindowLayout,
                       WindowWithInfo};
use cplwm_api::wm::WindowManager;
use cplwm_api::wm::TilingSupport;
use cplwm_api::wm::FloatSupport;
use cplwm_api::wm::MinimiseSupport;
use cplwm_api::wm::FullscreenSupport;

/// FloatscreenWM
pub type WMName = FloatscreenWM;




/// Stuct of Tile screen windows manager
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct FloatscreenWM {
    /// The vectors of the windows, the first one is on the master tile
    pub windows: Vec<Window>,
    /// The full screen windows,contains all full_screen windows, but only the last one is visible. when you close all full screen windows.
    /// you will go back to the previous look, when you set the first full screen window.
    /// 1st is current full screen window
    /// 2nd is it proviously geometry
    /// 3rd is its type
    /// last one is the provious focused window , before that one is set to be full screen.
    pub full_screen_windows: Vec<(Window, Geometry, FloatOrTile, Option<Window>)>,
    /// The vectors of the focus window, when it is empty then there is no focused window, which only can contain 1 object at maximum.
    pub focused_window: Option<Window>,
    /// all float windows that are under controlled
    pub float_windows: Vec<(Window, Geometry)>,
    /// all tiling windows that are under controlled
    pub tile_windows: Vec<Window>,
    /// we need to know the current size of the screen.
    pub screen: Screen,
    /// original windows with the geometry
    pub original_windows: Vec<(Window, Geometry)>,
    /// minimised windows with the geometry
    pub minimised_windows: Vec<(Window, Geometry, FloatOrTile)>,
}



#[derive(Debug)]
/// The error for tile screen
pub enum FloatscreenWMError {
    /// This window is not known by the window manager.
    UnknownWindow(Window),
    /// This window is not a tile window.
    NotATileWindow(Window),
    /// This window is not a float window.
    NotAFloatWindow(Window),
    /// This window is minimized, which can go full screen directly
    AMinimizedWindow(Window),
    /// This window is managed by the windows manager right now which can't be added again.
    ManagedWindow(Window),
}

/// Display fuction for Tile Screen Error
impl fmt::Display for FloatscreenWMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FloatscreenWMError::UnknownWindow(ref window) => {
                write!(f, "Unknown window: {}", window)
            }
            FloatscreenWMError::NotATileWindow(ref window) => {
                write!(f, "Not a Tile window: {}", window)
            }
            FloatscreenWMError::NotAFloatWindow(ref window) => {
                write!(f, "Not a Float window: {}", window)
            }
            FloatscreenWMError::AMinimizedWindow(ref window) => {
                write!(f, "A minimized window: {}", window)
            }
            FloatscreenWMError::ManagedWindow(ref window) => {
                write!(f, "Managed window: {}", window)
            }
        }
    }
}

/// Impliment of error::Error for Tile Screen Eror
impl error::Error for FloatscreenWMError {
    fn description(&self) -> &'static str {
        match *self {
            FloatscreenWMError::UnknownWindow(_) => "Unknown window",
            FloatscreenWMError::NotATileWindow(_) => "Not a Tile window",
            FloatscreenWMError::NotAFloatWindow(_) => "Not a Float window",
            FloatscreenWMError::AMinimizedWindow(_) => "A minimized window which should be unminimized to go full screen",
            FloatscreenWMError::ManagedWindow(_) => "Managed window",
        }
    }
}


/// Comment
impl WindowManager for FloatscreenWM {
    /// use `FloatscreenWMError` as  `Error` type.
    type Error = FloatscreenWMError;

    /// initiate a new windows manager
    fn new(screen: Screen) -> FloatscreenWM {
        FloatscreenWM {
            windows: Vec::new(),
            focused_window: None,
            full_screen_windows: Vec::new(),
            float_windows: Vec::new(),
            tile_windows: Vec::new(),
            screen: screen,
            original_windows: Vec::new(),
            minimised_windows: Vec::new(),
        }
    }

    /// get all windows which are managed by windows manager
    fn get_windows(&self) -> Vec<Window> {
        self.windows.clone()
    }

    /// get current focused window ,if no window is foucsed ,the function returns None
    fn get_focused_window(&self) -> Option<Window> {
        self.focused_window
    }


    /// add a new window to the windows manager.
    /// At here , add the implementation of
    /// A window manager that implements
    /// [`FullscreenSupport`](trait.FullscreenSupport.html) should make the
    /// window fullscreen when the `fullscreen` field of `WindowWithInfo` is
    /// set to `true`.
    fn add_window(&mut self, window_with_info: WindowWithInfo) -> Result<(), Self::Error> {
        if !self.is_managed(window_with_info.window) {
            self.windows.push(window_with_info.window);
            self.original_windows.push((window_with_info.window, window_with_info.geometry));

            // check whether the new screen should be displayed full screen

            if window_with_info.fullscreen == true {
                // get current focused window
                let t_f = self.focused_window;
                self.full_screen_windows.push((window_with_info.window,
                                               window_with_info.geometry,
                                               window_with_info.float_or_tile,
                                               t_f));
                self.focused_window = Some(window_with_info.window);
                Ok(())
            } else {
                if window_with_info.float_or_tile == FloatOrTile::Float {
                    self.float_windows.push((window_with_info.window, window_with_info.geometry));
                    // When the window added ,it should be the focused one
                    self.focused_window = Some(window_with_info.window);
                    Ok(())
                } else {

                    self.tile_windows.push(window_with_info.window);
                    // When the window added ,it should be the focused one
                    self.focused_window = Some(window_with_info.window);
                    Ok(())
                }
            }
        } else {
            Err(FloatscreenWMError::ManagedWindow(window_with_info.window))
        }

    }

    /// remove the window from the window manager
    /// if the window is the focused one ,then set the focused window None

    fn remove_window(&mut self, window: Window) -> Result<(), Self::Error> {
        match self.windows.iter().position(|w| *w == window) {
            None => Err(FloatscreenWMError::UnknownWindow(window)),
            Some(i) => {
                self.windows.remove(i);
                self.original_windows.remove(i);
                if self.focused_window == Some(window) {
                    self.focused_window = None;
                }
                // if the window is minimised now
                if self.minimised_windows
                    .clone()
                    .iter()
                    .map(|w| w.0)
                    .collect::<Vec<_>>()
                    .contains(&window) {
                    let index = self.minimised_windows
                        .clone()
                        .iter()
                        .map(|w| w.0)
                        .collect::<Vec<_>>()
                        .iter()
                        .position(|w| *w == window)
                        .unwrap();
                    self.minimised_windows.remove(index);
                    Ok(())

                } else {
                    // if the window is tile
                    if self.tile_windows.contains(&window) {
                        let index = self.tile_windows.iter().position(|w| *w == window).unwrap();
                        self.tile_windows.remove(index);
                        Ok(())
                    } else {
                        // if the window is full screen
                        if self.full_screen_windows
                            .clone()
                            .iter()
                            .map(|w| w.0)
                            .collect::<Vec<_>>()
                            .contains(&window) {
                            let index = self.full_screen_windows
                                .clone()
                                .iter()
                                .map(|w| w.0)
                                .collect::<Vec<_>>()
                                .iter()
                                .position(|w| *w == window)
                                .unwrap();
                            self.full_screen_windows.remove(index);
                            Ok(())
                        } else {
                            // if the window is float
                            let index = self.float_windows
                                .clone()
                                .iter()
                                .map(|w| w.0)
                                .collect::<Vec<_>>()
                                .iter()
                                .position(|w| *w == window)
                                .unwrap();
                            self.float_windows.remove(index);
                            Ok(())
                        }

                    }
                }
            }
        }
    }


    /// get the layout of windows which are managed
    fn get_window_layout(&self) -> WindowLayout {
        let fullscreen_geometry = self.screen.to_geometry();




        // If there is no full screen window, then we do it as usual

        if self.full_screen_windows.is_empty() == true {
            // First deal with tile windows
            match self.tile_windows.len() {
                // If there is no windows, return an empty WindowLayout
                0 => {
                    // if float window is also empty
                    if self.float_windows.is_empty() == true {
                        WindowLayout::new()
                    } else {
                        let mut tempwindows = Vec::new();
                        for i in &self.float_windows {
                            tempwindows.push(*i);
                        }
                        WindowLayout {
                            // the focus window is fucosed
                            focused_window: self.focused_window,

                            windows: tempwindows,
                        }
                    }

                }

                // If there is only one window, the one should be the focused on and take the whole screen:)
                1 => {

                    let mut tempwindows = vec![(self.tile_windows.first().map(|w| *w).unwrap(),
                                                fullscreen_geometry)];
                    for i in &self.float_windows {
                        tempwindows.push(*i);
                    }
                    WindowLayout {
                        // the focus window is fucosed
                        focused_window: self.focused_window,

                        windows: tempwindows,
                    }

                }
                2 => {
                    let mut tempwindows = Vec::new();
                    let mut count = 1;
                    for i in &self.tile_windows {
                        let cal = (count - 1) * (self.screen.width / 2);
                        tempwindows.push((*i,
                                          Geometry {
                            x: cal as i32,
                            y: 0,
                            width: self.screen.width / 2,
                            height: self.screen.height,
                        }));
                        count = count + 1;
                    }
                    for i in &self.float_windows {
                        tempwindows.push(*i);
                    }

                    WindowLayout {
                        focused_window: self.focused_window,
                        windows: tempwindows,
                    }
                }
                _ => {
                    let num = self.tile_windows.len() as u32;
                    let mut tempwindows = Vec::new();
                    let mut count = 1;
                    for i in &self.tile_windows {
                        let cal_w = self.screen.width / 2;
                        let cal_h = (num - count) * (self.screen.height / (num - 1));
                        if count > 1 {
                            tempwindows.push((*i,
                                              Geometry {
                                x: cal_w as i32,
                                y: cal_h as i32,
                                width: self.screen.width / 2,
                                height: self.screen.height / (num - 1),
                            }));
                        } else {
                            tempwindows.push((*i,
                                              Geometry {
                                x: 0,
                                y: 0,
                                width: self.screen.width / 2,
                                height: self.screen.height,
                            }));
                        }
                        count = count + 1;
                    }
                    for i in &self.float_windows {
                        tempwindows.push(*i);
                    }
                    WindowLayout {
                        focused_window: self.focused_window,
                        windows: tempwindows,
                    }
                }
            }

        } else {
            WindowLayout {
                // the focus window is fucosed
                focused_window: self.focused_window,

                windows: vec![(self.full_screen_windows.last().unwrap().0,
                               self.get_screen().to_geometry())],
            }
        }




    }





    /// set a new fouced window
    /// if the new focused window is in the float windows list,put it at the end of the float window vector
    fn focus_window(&mut self, window: Option<Window>) -> Result<(), Self::Error> {

        match window {
            Some(i) => {
                match self.windows.iter().position(|w| *w == i) {
                    None => Err(FloatscreenWMError::UnknownWindow(i)),
                    Some(w) => {
                        // if new focused window is in the float windows list
                        // put it to the end of the stack
                        if self.tile_windows.contains(&window.unwrap()) {
                            self.focused_window = window;
                        } else {
                            let index = self.float_windows
                                .clone()
                                .iter()
                                .map(|w| w.0)
                                .collect::<Vec<_>>()
                                .iter()
                                .position(|w| *w == window.unwrap())
                                .unwrap();
                            let temp = self.float_windows[index];
                            self.float_windows.remove(index);
                            self.float_windows.push(temp);
                            self.focused_window = window;
                        }
                        Ok(())

                    }
                }

            }
            None => {
                self.focused_window = window;
                Ok(())
            }
        }
    }


    /// cycle focus the window ,when this is no window focused right now ,focus a random one
    fn cycle_focus(&mut self, dir: PrevOrNext) {
        if self.windows.is_empty() == true {
            return ();
        } else {
            if self.get_focused_window() == None {
                // focuse the last in the vector,not random one
                let temp_prev = self.windows.last().map(|i| *i);
                self.focus_window(temp_prev).unwrap();
            } else {
                match dir {
                    PrevOrNext::Prev => {
                        // get the length of the vec
                        let index = self.windows
                            .iter()
                            .position(|&w| w == self.focused_window.unwrap())
                            .unwrap();
                        if index == 0 {
                            let temp_prev = self.windows.last().map(|i| *i);
                            self.focus_window(temp_prev).unwrap();
                        } else {
                            let temp_prev = self.windows.get(index - 1).map(|i| *i);
                            self.focus_window(temp_prev).unwrap();
                        }
                    }
                    PrevOrNext::Next => {
                        let index = self.windows
                            .iter()
                            .position(|&w| w == self.focused_window.unwrap())
                            .unwrap();
                        let len = self.windows.len();

                        if index == len - 1 {
                            let temp_prev = self.windows.first().map(|i| *i);
                            self.focus_window(temp_prev).unwrap();
                        } else {
                            let temp_prev = self.windows.get(index + 1).map(|i| *i);
                            self.focus_window(temp_prev).unwrap();
                        }
                    }
                }
            }

        }

    }


    /// get the information of the window that is provided .
    fn get_window_info(&self, window: Window) -> Result<WindowWithInfo, Self::Error> {

        match self.windows.iter().position(|w| *w == window) {
            None => Err(FloatscreenWMError::UnknownWindow(window)),
            Some(i) => {

                let mut temp_geometry = Geometry {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                };
                // locate the geometry for the window we want.
                for w in self.get_window_layout().windows {
                    if w.0 == window {
                        temp_geometry = w.1;
                        break;
                    } else {
                        continue;
                    }
                }
                let temp_tile_or_float: FloatOrTile;
                if self.tile_windows.contains(&window) {
                    temp_tile_or_float = FloatOrTile::Tile;
                } else {
                    temp_tile_or_float = FloatOrTile::Float;
                }
                // check the window is full screen or not here.
                let temp_full_screen: bool;
                if temp_geometry == self.get_screen().to_geometry() {

                    temp_full_screen = true;
                } else {
                    temp_full_screen = false;
                }
                Ok(WindowWithInfo {
                    window: window,
                    geometry: temp_geometry,
                    float_or_tile: temp_tile_or_float,
                    fullscreen: temp_full_screen,
                })
            }
        }
    }


    /// get the sreen of current windows manager
    fn get_screen(&self) -> Screen {
        self.screen
    }

    /// resize the window with provide screen size
    fn resize_screen(&mut self, screen: Screen) {
        self.screen = screen
    }
}

/// Implementation of TilingSupport
impl TilingSupport for FloatscreenWM {
    /// get the master window of current windows manager
    /// The first one in the vector is the master window
    fn get_master_window(&self) -> Option<Window> {
        // The first one in the vector is the master window
        self.tile_windows.first().map(|w| *w)

    }


    /// swap the provided window with current master tile window
    /// if the user asks a float window swap with the master of tile ,
    /// we make the master tile window float and make the float window becomes the master tile one
    fn swap_with_master(&mut self, window: Window) -> Result<(), Self::Error> {
        match self.tile_windows.iter().position(|w| *w == window) {
            None => {
                if self.float_windows
                    .clone()
                    .iter()
                    .map(|w| w.0)
                    .collect::<Vec<_>>()
                    .contains(&window) {
                    // get current window in the master tile
                    let temp_prev = self.tile_windows.first().map(|i| *i).unwrap();
                    // make it float
                    self.toggle_floating(temp_prev).unwrap();
                    // put the window to the master tile

                    self.tile_windows.insert(0, window);
                    // make the master one focused.
                    let temp_prev = self.tile_windows.first().map(|i| *i);
                    self.focus_window(temp_prev).unwrap();
                    Ok(())
                } else {
                    // otherwise return an error ,
                    Err(FloatscreenWMError::UnknownWindow(window))
                }

            }
            Some(i) => {
                // if the windows is in the master tile now
                // then it should be focused.
                if i == 0 {
                    let temp_prev = self.tile_windows.first().map(|i| *i);
                    self.focus_window(temp_prev).unwrap();
                    Ok(())
                } else {
                    // put the window to the front, I mean , put it to the master tile :)
                    // put the window in the master tile to the position .
                    let temp_first = self.tile_windows.first().map(|i| *i).unwrap();
                    self.tile_windows[i] = temp_first;
                    self.tile_windows[0] = window;

                    // and make it focused
                    let temp_prev = self.tile_windows.first().map(|i| *i);
                    self.focus_window(temp_prev).unwrap();
                    Ok(())
                }
            }

        }
    }


    /// Swap the focused window with the one in the next or previous tile.
    /// if current focused is a tile window
    /// The following will happen
    fn swap_windows(&mut self, dir: PrevOrNext) {

        // unimplemented!()
        if self.focused_window == None {
            return ();
        } else {
            // If current focused one is a tile window
            if self.tile_windows.contains(&self.focused_window.unwrap()) == true {

                match dir {
                    PrevOrNext::Prev => {
                        match self.tile_windows.len() {

                            0 | 1 => {
                                return ();
                            }
                            _ => {
                                // get index of current focused window
                                let index = self.tile_windows
                                    .iter()
                                    .position(|&w| w == self.focused_window.unwrap())
                                    .unwrap();
                                match index {
                                    // if current focused window is in the master tile
                                    0 => {
                                        // get the current focused window
                                        let temp_focused = self.focused_window.unwrap();
                                        // get the one need to be swaped with it
                                        let temp_last =
                                            self.tile_windows.last().map(|i| *i).unwrap();
                                        let len = self.tile_windows.len();
                                        // swap two windows
                                        self.tile_windows[0] = temp_last;
                                        self.tile_windows[len - 1] = temp_focused;

                                    }

                                    _ => {
                                        // get the current focused window
                                        let temp_focused = self.tile_windows[index];
                                        // get the one need to be swaped with it
                                        let temp_swap = self.tile_windows[index - 1];
                                        let len = self.tile_windows.len();
                                        // swap two windows
                                        self.tile_windows[index] = temp_swap;
                                        self.tile_windows[index - 1] = temp_focused;
                                    }

                                }
                            }

                        }
                    }
                    PrevOrNext::Next => {
                        match self.tile_windows.len() {

                            0 | 1 => {
                                return ();
                            }
                            _ => {
                                // get index of current focused window
                                let index = self.tile_windows
                                    .iter()
                                    .position(|&w| w == self.focused_window.unwrap())
                                    .unwrap();
                                let len = self.tile_windows.len();
                                // if current focused window at the last of windows vector
                                if index == len - 1 {
                                    // get the current focused window
                                    let temp_focused = self.focused_window.unwrap();
                                    // get the one need to be swaped with it
                                    let temp_first = self.tile_windows.first().map(|i| *i).unwrap();
                                    let len = self.tile_windows.len();
                                    // swap two windows
                                    self.tile_windows[0] = temp_focused;
                                    self.tile_windows[len - 1] = temp_first;

                                } else {
                                    // get the current focused window
                                    let temp_focused = self.tile_windows[index];
                                    // get the one need to be swaped with it
                                    let temp_swap = self.tile_windows[index + 1];
                                    let len = self.tile_windows.len();
                                    // swap two windows
                                    self.tile_windows[index] = temp_swap;
                                    self.tile_windows[index + 1] = temp_focused;
                                }


                            }

                        }

                    }
                }

            } else {
                return ();
            }
        }
    }
}


/// Implementation of TilingSupport
impl FloatSupport for FloatscreenWM {
    /// get the windows are managed which is tiling window
    fn get_floating_windows(&self) -> Vec<Window> {

        let mut tempwindows = vec![];
        for i in &self.float_windows {
            tempwindows.push(i.0);
        }
        return tempwindows;
    }

    /// check the window is float or not
    fn is_floating(&self, window: Window) -> bool {
        self.get_floating_windows().contains(&window)
    }

    /// toggle the window, the given window is floating, let it *sink*, if it is not floating,
    fn toggle_floating(&mut self, window: Window) -> Result<(), Self::Error> {
        match self.windows.iter().position(|w| *w == window) {
            None => Err(FloatscreenWMError::UnknownWindow(window)),
            Some(i) => {
                if self.tile_windows.contains(&window) {
                    let index = self.tile_windows.iter().position(|w| *w == window).unwrap();
                    self.tile_windows.remove(index);
                    let index2 = self.windows.iter().position(|w| *w == window).unwrap();
                    // When a non-floating window starts to float, its original geometry
                    // (passed to `add_window`) should be restored.
                    let temp_geometry = self.original_windows[index2].1;
                    self.float_windows.push((window, temp_geometry));
                    Ok(())
                } else {
                    let index = self.float_windows
                        .clone()
                        .iter()
                        .map(|w| w.0)
                        .collect::<Vec<_>>()
                        .iter()
                        .position(|w| *w == window)
                        .unwrap();
                    self.float_windows.remove(index);
                    self.tile_windows.push(window);
                    Ok(())
                }

            }
        }
    }

    /// Resize/move the given floating window according to the given geometry.
    fn set_window_geometry(&mut self,
                           window: Window,
                           new_geometry: Geometry)
                           -> Result<(), Self::Error> {
        match self.windows.iter().position(|w| *w == window) {
            None => Err(FloatscreenWMError::UnknownWindow(window)),
            Some(i) => {
                if self.float_windows
                    .clone()
                    .iter()
                    .map(|w| w.0)
                    .collect::<Vec<_>>()
                    .contains(&window) {
                    let index = self.float_windows
                        .clone()
                        .iter()
                        .map(|w| w.0)
                        .collect::<Vec<_>>()
                        .iter()
                        .position(|w| *w == window)
                        .unwrap();
                    self.float_windows[index].1 = new_geometry;
                    Ok(())
                } else {
                    Err(FloatscreenWMError::NotAFloatWindow(window))
                }
            }
        }
    }
}

/// Implementation of MinimiseSupport
impl MinimiseSupport for FloatscreenWM {
    /// Return a vector of all the minimised windows.
    fn get_minimised_windows(&self) -> Vec<Window> {
        let mut tempwindows = vec![];
        for i in &self.minimised_windows {
            tempwindows.push(i.0);
        }
        return tempwindows;
    }

    /// Return `true` if the given window is minimised.
    fn is_minimised(&self, window: Window) -> bool {
        self.get_minimised_windows().contains(&window)
    }

    /// Minimise the given window, or when it is already minimised, unminise
    /// it.
    /// Here modify the implementation of this fuction in minimised file
    /// The fuction support minimise the full screen window directly to minimise window
    fn toggle_minimised(&mut self, window: Window) -> Result<(), Self::Error> {

        if self.is_minimised(window) == false {
            match self.windows.iter().position(|w| *w == window) {
                None => Err(FloatscreenWMError::UnknownWindow(window)),
                Some(i) => {
                    if self.full_screen_windows
                        .clone()
                        .iter()
                        .map(|w| w.0)
                        .collect::<Vec<_>>()
                        .contains(&window) {
                        let index = self.full_screen_windows
                            .clone()
                            .iter()
                            .map(|w| w.0)
                            .collect::<Vec<_>>()
                            .iter()
                            .position(|w| *w == window)
                            .unwrap();
                        let temp_geo = self.get_window_info(window).unwrap().geometry;
                        self.minimised_windows.push((window, temp_geo, FloatOrTile::Tile));
                        self.full_screen_windows.remove(index);
                        Ok(())
                    } else {
                        if self.tile_windows.contains(&window) {
                            let index =
                                self.tile_windows.iter().position(|w| *w == window).unwrap();
                            let temp_geo = self.get_window_info(window).unwrap().geometry;
                            self.minimised_windows.push((window, temp_geo, FloatOrTile::Tile));
                            self.tile_windows.remove(index);
                            Ok(())

                        } else {
                            let index = self.float_windows
                                .clone()
                                .iter()
                                .map(|w| w.0)
                                .collect::<Vec<_>>()
                                .iter()
                                .position(|w| *w == window)
                                .unwrap();
                            let temp_geo = self.get_window_info(window).unwrap().geometry;
                            self.minimised_windows.push((window, temp_geo, FloatOrTile::Float));
                            self.float_windows.remove(index);
                            Ok(())
                        }
                    }
                }
            }
        } else {
            match self.windows.iter().position(|w| *w == window) {
                None => Err(FloatscreenWMError::UnknownWindow(window)),
                Some(i) => {
                    let index = self.minimised_windows
                        .clone()
                        .iter()
                        .map(|w| w.0)
                        .collect::<Vec<_>>()
                        .iter()
                        .position(|w| *w == window)
                        .unwrap();
                    if self.minimised_windows[index].2 == FloatOrTile::Tile {
                        self.minimised_windows.remove(index);
                        self.tile_windows.push(window);
                        Ok(())

                    } else {
                        self.float_windows.push((window, self.minimised_windows[index].1));
                        self.minimised_windows.remove(index);
                        Ok(())
                    }
                }
            }
        }


    }
}

/// Implementation of FullscreenSupport
impl FullscreenSupport for FloatscreenWM {
    /// Return the current fullscreen, if any.
    fn get_fullscreen_window(&self) -> Option<Window> {
        if self.full_screen_windows.is_empty() == true {
            return None;
        } else {
            return Some(self.full_screen_windows.last().unwrap().0);
        }
    }

    /// Make the given window fullscreen, or when it is already fullscreen,
    /// undo it.
    /// when call with a minimised window as argument should first unminimise the window.
    fn toggle_fullscreen(&mut self, window: Window) -> Result<(), Self::Error> {

        // **Note**: methods of other traits like `focus_window`, `focus_window`,
        // `toggle_fullscreen`, ... called with a minimised window as argument should
        // first unminimise the window.
        if self.minimised_windows
            .clone()
            .iter()
            .map(|w| w.0)
            .collect::<Vec<_>>()
            .contains(&window) {
            self.toggle_minimised(window).unwrap();
            Ok(())

            // Err(FloatscreenWMError::AMinimizedWindow(window))
        } else {
            // if the window is not full screen now .
            if self.full_screen_windows
                .clone()
                .iter()
                .map(|w| w.0)
                .collect::<Vec<_>>()
                .contains(&window) == false {
                match self.windows.iter().position(|w| *w == window) {
                    None => Err(FloatscreenWMError::UnknownWindow(window)),
                    Some(i) => {
                        if self.tile_windows.contains(&window) {
                            let index =
                                self.tile_windows.iter().position(|w| *w == window).unwrap();
                            // get the poisiton of window in the tile vector
                            let temp_geo = self.get_window_info(window).unwrap().geometry;
                            // get the geometry of the window
                            self.full_screen_windows
                                .push((window, temp_geo, FloatOrTile::Tile, self.focused_window));
                            // set the foucsed window to new full screen one
                            self.focused_window = Some(window);
                            self.tile_windows.remove(index);
                            Ok(())

                        } else {
                            let index = self.float_windows
                                .clone()
                                .iter()
                                .map(|w| w.0)
                                .collect::<Vec<_>>()
                                .iter()
                                .position(|w| *w == window)
                                .unwrap();
                            // Even though , it is resized , and we close it, we still can get the resized window back.:)
                            let temp_geo = self.float_windows[index].1;
                            self.full_screen_windows
                                .push((window, temp_geo, FloatOrTile::Float, self.focused_window));
                            // set the foucsed window to new full screen one
                            self.focused_window = Some(window);
                            self.float_windows.remove(index);
                            Ok(())
                        }

                    }
                }
            } else {
                match self.windows.iter().position(|w| *w == window) {
                    None => Err(FloatscreenWMError::UnknownWindow(window)),
                    Some(i) => {
                        let index = self.full_screen_windows
                            .clone()
                            .iter()
                            .map(|w| w.0)
                            .collect::<Vec<_>>()
                            .iter()
                            .position(|w| *w == window)
                            .unwrap();
                        if self.full_screen_windows[index].2 == FloatOrTile::Tile {
                            let temp_full_screen = self.full_screen_windows[index].3;
                            self.full_screen_windows.remove(index);
                            self.tile_windows.push(window);
                            self.focused_window = temp_full_screen;
                            Ok(())

                        } else {
                            let temp_full_screen = self.full_screen_windows[index].3;
                            self.float_windows.push((window, self.full_screen_windows[index].1));
                            self.focused_window = temp_full_screen;
                            self.full_screen_windows.remove(index);
                            Ok(())
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::FloatscreenWM;

    // Repeat the imports we did in the super module.
    use cplwm_api::wm::WindowManager;
    use cplwm_api::wm::TilingSupport;
    use cplwm_api::wm::FloatSupport;
    use cplwm_api::wm::MinimiseSupport;
    use cplwm_api::wm::FullscreenSupport;
    use cplwm_api::types::*;

    // Static value

    static SCREEN: Screen = Screen {
        width: 800,
        height: 600,
    };


    static SOME_GEOM: Geometry = Geometry {
        x: 10,
        y: 10,
        width: 100,
        height: 100,
    };

    #[test]
    fn test_add_remove_windows() {

        let mut wm = FloatscreenWM::new(SCREEN);
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        assert!(wm.is_managed(1));
        assert_eq!(vec![1], wm.get_windows());
        assert_eq!(vec![1], wm.tile_windows);

        wm.add_window(WindowWithInfo::new_float(2, SOME_GEOM)).unwrap();
        assert_eq!(vec![1, 2], wm.get_windows());
        assert_eq!(vec![1], wm.tile_windows);
        assert_eq!(vec![(2, SOME_GEOM)], wm.float_windows);

        wm.add_window(WindowWithInfo::new_fullscreen(3, SOME_GEOM)).unwrap();
        assert_eq!(vec![1, 2, 3], wm.get_windows());
        assert_eq!(Some(3), wm.get_fullscreen_window());

        wm.remove_window(3).unwrap();

        // test remove the one is minimised now

        wm.add_window(WindowWithInfo::new_fullscreen(4, SOME_GEOM)).unwrap();
        assert_eq!(vec![1, 2, 4], wm.get_windows());
        wm.toggle_minimised(4).unwrap();

        wm.remove_window(4).unwrap();
        wm.remove_window(2).unwrap();
        assert_eq!(vec![1], wm.get_windows());
        assert_eq!(vec![1], wm.tile_windows);
        assert_eq!(true, wm.float_windows.is_empty());

        wm.remove_window(1).unwrap();
        assert_eq!(true, wm.get_windows().is_empty());
        assert_eq!(true, wm.tile_windows.is_empty());
        assert_eq!(true, wm.float_windows.is_empty());



    }
    #[test]
    fn test_windows_layout() {

        let mut wm = FloatscreenWM::new(SCREEN);
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();

        let wl1 = wm.get_window_layout();
        assert_eq!(vec![(1,
                         Geometry {
                            x: 0,
                            y: 0,
                            width: 400,
                            height: 600,
                        }),
                        (2,
                         Geometry {
                            x: 400,
                            y: 300,
                            width: 400,
                            height: 300,
                        }),
                        (3,
                         Geometry {
                            x: 400,
                            y: 0,
                            width: 400,
                            height: 300,
                        })],
                   wl1.windows);

        wm.add_window(WindowWithInfo::new_float(4, SOME_GEOM)).unwrap();

        let wl2 = wm.get_window_layout();
        assert_eq!(vec![(1,
                         Geometry {
                            x: 0,
                            y: 0,
                            width: 400,
                            height: 600,
                        }),
                        (2,
                         Geometry {
                            x: 400,
                            y: 300,
                            width: 400,
                            height: 300,
                        }),
                        (3,
                         Geometry {
                            x: 400,
                            y: 0,
                            width: 400,
                            height: 300,
                        }),
                        (4, SOME_GEOM)],
                   wl2.windows);

    }
    #[test]
    fn test_focuse_window() {

        let mut wm = FloatscreenWM::new(SCREEN);

        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        wm.focus_window(Some(2)).unwrap();
        assert_eq!(Some(2), wm.focused_window);

        // test cycle_focus
        // now the windows vec is vec![2,3,4,1]
        // set focuse first
        wm.focus_window(Some(2)).unwrap();
        wm.cycle_focus(PrevOrNext::Prev);
        assert_eq!(Some(1), wm.focused_window);
        wm.cycle_focus(PrevOrNext::Prev);
        assert_eq!(Some(4), wm.focused_window);
        wm.cycle_focus(PrevOrNext::Next);
        assert_eq!(Some(1), wm.focused_window);
        wm.cycle_focus(PrevOrNext::Next);
        assert_eq!(Some(2), wm.focused_window);


    }

    #[test]
    fn test_get_window_info() {

        let mut wm = FloatscreenWM::new(SCREEN);

        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_float(1, SOME_GEOM)).unwrap();

        static SOME_GEOM1: Geometry = Geometry {
            x: 0,
            y: 0,
            width: 800,
            height: 600,
        };

        // test get_windows_info
        assert_eq!(Some(WindowWithInfo {
                       window: 2,
                       geometry: SOME_GEOM1,
                       float_or_tile: FloatOrTile::Tile,
                       fullscreen: true,
                   }),
                   wm.get_window_info(2).ok());
        assert_eq!(Some(WindowWithInfo {
                       window: 1,
                       geometry: SOME_GEOM,
                       float_or_tile: FloatOrTile::Float,
                       fullscreen: false,
                   }),
                   wm.get_window_info(1).ok());


    }

    #[test]
    fn test_tiling_support_functions() {

        let mut wm = FloatscreenWM::new(SCREEN);

        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_float(5, SOME_GEOM)).unwrap();
        wm.focus_window(Some(2)).unwrap();

        // test  swap_with_master
        // initial tile_windows vec![2,3,4,1]
        // current master tile is 2
        // focused on is 2
        wm.swap_with_master(1).unwrap();
        assert_eq!(vec![1, 3, 4, 2], wm.tile_windows);
        assert_eq!(Some(1), wm.get_master_window());
        assert_eq!(Some(1), wm.focused_window);
        wm.swap_with_master(4).unwrap();
        assert_eq!(vec![4, 3, 1, 2], wm.tile_windows);

        // when there is no window is focused , do nothing
        wm.focus_window(None).unwrap();
        wm.swap_with_master(4).unwrap();
        assert_eq!(vec![4, 3, 1, 2], wm.tile_windows);



        // test swap_windows
        // initial windows vec![4,3,1,2]
        // when there is no window is focused , do nothing
        wm.focus_window(None).unwrap();
        assert_eq!(None, wm.get_focused_window());
        wm.swap_windows(PrevOrNext::Next);
        assert_eq!(vec![4, 3, 1, 2], wm.tile_windows);

        wm.focus_window(Some(4)).unwrap();
        // focused one is 4
        wm.swap_windows(PrevOrNext::Next);
        assert_eq!(vec![3, 4, 1, 2], wm.tile_windows);
        // check the focused on doesn't change
        assert_eq!(Some(4), wm.get_focused_window());


        // now is [3,4,1,2]
        wm.focus_window(Some(3)).unwrap();
        wm.swap_windows(PrevOrNext::Prev);
        assert_eq!(vec![2, 4, 1, 3], wm.tile_windows);

        wm.focus_window(Some(4)).unwrap();
        wm.swap_windows(PrevOrNext::Prev);
        assert_eq!(vec![4, 2, 1, 3], wm.tile_windows);

        wm.tile_windows = vec![1];
        wm.swap_windows(PrevOrNext::Prev);
        assert_eq!(vec![1], wm.tile_windows);

        wm.tile_windows = vec![];
        assert_eq!(true, wm.tile_windows.is_empty());
        wm.swap_windows(PrevOrNext::Prev);
        assert_eq!(true, wm.tile_windows.is_empty());


    }



    #[test]
    fn test_float_support_functions() {

        static SOME_GEOM1: Geometry = Geometry {
            x: 20,
            y: 10,
            width: 200,
            height: 100,
        };

        static SOME_GEOM3: Geometry = Geometry {
            x: 20,
            y: 10,
            width: 200,
            height: 100,
        };

        let mut wm = FloatscreenWM::new(SCREEN);
        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM3)).unwrap();
        wm.add_window(WindowWithInfo::new_float(3, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_float(5, SOME_GEOM)).unwrap();


        assert_eq!(vec![3, 5], wm.get_floating_windows());
        assert_eq!(true, wm.is_floating(3));
        assert_eq!(false, wm.is_floating(2));

        wm.toggle_floating(3).unwrap();
        assert_eq!(false, wm.is_floating(3));

        // make a tile window float

        wm.toggle_floating(2).unwrap();
        assert_eq!(true, wm.is_floating(2));
        assert_eq!(vec![5, 2], wm.get_floating_windows());
        assert_eq!(Some(WindowWithInfo {
                       window: 2,
                       geometry: SOME_GEOM3,
                       float_or_tile: FloatOrTile::Float,
                       fullscreen: false,
                   }),
                   wm.get_window_info(2).ok());
        // 	wm.toggle_floating(3).unwrap();
        assert_eq!(Some(WindowWithInfo {
                       window: 5,
                       geometry: SOME_GEOM,
                       float_or_tile: FloatOrTile::Float,
                       fullscreen: false,
                   }),
                   wm.get_window_info(5).ok());
        wm.set_window_geometry(5, SOME_GEOM1).unwrap();
        assert_eq!(Some(WindowWithInfo {
                       window: 5,
                       geometry: SOME_GEOM1,
                       float_or_tile: FloatOrTile::Float,
                       fullscreen: false,
                   }),
                   wm.get_window_info(5).ok());

    }


    #[test]
    fn test_minimise_support_functions() {
        let mut wm = FloatscreenWM::new(SCREEN);

        static SOME_GEOM3: Geometry = Geometry {
            x: 20,
            y: 10,
            width: 200,
            height: 100,
        };

        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM3)).unwrap();
        wm.add_window(WindowWithInfo::new_float(3, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_float(5, SOME_GEOM)).unwrap();

        // test    fn toggle_minimised(&mut self, window: Window) -> Result<(), Self::Error>
        wm.toggle_minimised(2).unwrap();
        wm.toggle_minimised(3).unwrap();
        // test   fn is_minimised(&self, window: Window) -> bool
        assert_eq!(true, wm.is_minimised(3));
        assert_eq!(true, wm.is_minimised(2));
        assert_eq!(vec![2, 3], wm.get_minimised_windows());

        wm.toggle_minimised(2).unwrap();
        wm.toggle_minimised(3).unwrap();
        assert_eq!(false, wm.is_minimised(3));
        assert_eq!(false, wm.is_minimised(2));

        assert_eq!(WindowWithInfo::new_float(3, SOME_GEOM),
                   wm.get_window_info(3).unwrap());
    }
    #[test]
    fn test_fullscreen_support_functions() {
        let mut wm = FloatscreenWM::new(SCREEN);

        static SOME_GEOM1: Geometry = Geometry {
            x: 20,
            y: 10,
            width: 200,
            height: 100,
        };

        static SOME_GEOM3: Geometry = Geometry {
            x: 20,
            y: 10,
            width: 200,
            height: 100,
        };

        static SCREEN_GEOM: Geometry = Geometry {
            x: 0,
            y: 0,
            width: 800,
            height: 600,
        };

        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM3)).unwrap();
        wm.add_window(WindowWithInfo::new_float(3, SOME_GEOM1)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM1)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM1)).unwrap();
        wm.add_window(WindowWithInfo::new_float(5, SOME_GEOM1)).unwrap();

        let wl1 = wm.get_window_layout();
        assert_eq!(vec![(2,
                         Geometry {
                            x: 0,
                            y: 0,
                            width: 400,
                            height: 600,
                        }),
                        (4,
                         Geometry {
                            x: 400,
                            y: 300,
                            width: 400,
                            height: 300,
                        }),
                        (1,
                         Geometry {
                            x: 400,
                            y: 0,
                            width: 400,
                            height: 300,
                        }),
                        (3, SOME_GEOM1),
                        (5, SOME_GEOM1)],
                   wl1.windows);

        assert_eq!(None, wm.get_fullscreen_window());
        wm.toggle_fullscreen(2).unwrap();
        assert_eq!(Some(2), wm.get_fullscreen_window());


        let wl1 = wm.get_window_layout();
        // it should be focused
        assert_eq!(Some(2), wl1.focused_window);
        // and fullscreen.
        assert_eq!(vec![(2, SCREEN_GEOM)], wl1.windows);

        // --------------bugs-------------------

        wm.toggle_fullscreen(5).unwrap();
        assert_eq!(Some(5), wm.get_fullscreen_window());
        wm.toggle_fullscreen(5).unwrap();

        wm.toggle_fullscreen(3).unwrap();
        assert_eq!(Some(3), wm.get_fullscreen_window());

        let wl1 = wm.get_window_layout();
        // it should be focused
        assert_eq!(Some(3), wl1.focused_window);
        // and fullscreen.
        assert_eq!(vec![(3, SCREEN_GEOM)], wl1.windows);

        wm.toggle_fullscreen(3).unwrap();

        assert_eq!(Some(2), wm.get_fullscreen_window());

        let wl1 = wm.get_window_layout();
        // it should be focused
        assert_eq!(Some(2), wl1.focused_window);
        // and fullscreen.
        assert_eq!(vec![(2, SCREEN_GEOM)], wl1.windows);

        wm.toggle_fullscreen(2).unwrap();
        let wl1 = wm.get_window_layout();
        assert_eq!(vec![(4,
                         Geometry {
                            x: 0,
                            y: 0,
                            width: 400,
                            height: 600,
                        }),
                        (1,
                         Geometry {
                            x: 400,
                            y: 300,
                            width: 400,
                            height: 300,
                        }),
                        (2,
                         Geometry {
                            x: 400,
                            y: 0,
                            width: 400,
                            height: 300,
                        }),
                        (5, SOME_GEOM1),
                        (3, SOME_GEOM1)],
                   wl1.windows);
    }


}