//! Tiling Window Manager
//!
//! Write a more complex window manager that will *tile* its windows. Tiling
//! is described in the first section of the assignment. Your window manager
//! must implement both the [`WindowManager`] trait and the [`TilingSupport`]
//! trait. See the documentation of the [`TilingSupport`] trait for the
//! precise requirements and an explanation of the tiling layout algorithm.
//!
//! [`WindowManager`]: ../../cplwm_api/wm/trait.WindowManager.html
//! [`TilingSupport`]: ../../cplwm_api/wm/trait.TilingSupport.html
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
//! I don't save the type of the window here,because all are tiles when call the fucntion get_window_info
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


/// TilescreenWM

pub type WMName = TilescreenWM;


/// Stuct of Tile screen windows manager
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct TilescreenWM {
    /// The vectors of the windows, the first one is on the master tile
    pub windows: Vec<Window>,
    /// The vectors of the focus window, when it is empty then there is no focused window, which only can contain 1 object at maximum.
    pub focused_window: Vec<Window>,
    /// we need to know the current size of the screen.
    pub screen: Screen,
}



#[derive(Debug)]
/// The error for tile screen
pub enum TilescreenWMError {
    /// This window is not known by the window manager.
    UnknownWindow(Window),
    /// This window is managed by the windows manager right now which can't be added again.
    ManagedWindow(Window),
}

/// Display fuction for Tile Screen Error
impl fmt::Display for TilescreenWMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TilescreenWMError::UnknownWindow(ref window) => write!(f, "Unknown window: {}", window),
            TilescreenWMError::ManagedWindow(ref window) => write!(f, "Managed window: {}", window),
        }
    }
}

/// Impliment of error::Error for Tile Screen Eror
impl error::Error for TilescreenWMError {
    fn description(&self) -> &'static str {
        match *self {
            TilescreenWMError::UnknownWindow(_) => "Unknown window",
            TilescreenWMError::ManagedWindow(_) => "Managed window",
        }
    }
}



impl WindowManager for TilescreenWM {
    /// use `TilescreenWMError` as  `Error` type.
    type Error = TilescreenWMError;
    /// initiate a new windows manager
    fn new(screen: Screen) -> TilescreenWM {
        TilescreenWM {
            windows: Vec::new(),
            focused_window: Vec::new(),
            screen: screen,
        }
    }
    /// get all windows which are managed by windows manager
    fn get_windows(&self) -> Vec<Window> {
        self.windows.clone()
    }

    /// get current focused window ,if no window is foucsed ,the function returns None
    fn get_focused_window(&self) -> Option<Window> {
        self.focused_window.last().map(|w| *w)
    }

    /// add a new window to the windows manager.
    fn add_window(&mut self, window_with_info: WindowWithInfo) -> Result<(), Self::Error> {
        if !self.is_managed(window_with_info.window) {
            self.windows.push(window_with_info.window);
            self.focused_window.clear();
            self.focused_window.push(window_with_info.window);
            Ok(())
        } else {
            Err(TilescreenWMError::ManagedWindow(window_with_info.window))
        }


    }

    /// remove the window from the window manager
    /// if the removed one is the fouced one ,then we set the fouceds one none
    fn remove_window(&mut self, window: Window) -> Result<(), Self::Error> {
        match self.windows.iter().position(|w| *w == window) {
            None => Err(TilescreenWMError::UnknownWindow(window)),
            Some(i) => {
                self.windows.remove(i);
                if self.focused_window.last().map(|w| *w) == Some(window) {
                    self.focused_window.clear();
                    Ok(())
                } else {
                    Ok(())
                }
            }
        }
    }

    /// get the layout of windows which are managed
    /// According to the algorithm provided by the wm.rs file
    /// If the windows just contain 1 windows, it should be focused and full screen display
    /// If there are 2 windows , Split the screen to left part and right part
    /// If there are more then 3 windows, we just split the right side of windows, for each windows share equal space
    fn get_window_layout(&self) -> WindowLayout {
        let fullscreen_geometry = self.screen.to_geometry();
        match self.windows.len() {
            // If there is no windows, return an empty WindowLayout
            0 => WindowLayout::new(),
            // If there is only one window,
            1 => {
                match self.windows.first() {
                    Some(w) => {
                        WindowLayout {
                            // the focus window is fucosed
                            focused_window: Some(*w),
                            windows: vec![(*w, fullscreen_geometry)],
                        }
                    }	
                    // This line cann't happen.
                    None => WindowLayout::new(),
                }
            }
            2 => {
                let mut tempwindows = Vec::new();
                let mut count = 1;
                for i in &self.windows {
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
                WindowLayout {
                    focused_window: self.focused_window.first().map(|w| *w),
                    windows: tempwindows,
                }
            }
            _ => {
                let num = self.windows.len() as u32;
                let mut tempwindows = Vec::new();
                let mut count = 1;
                for i in &self.windows {
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
                WindowLayout {
                    focused_window: self.focused_window.first().map(|w| *w),
                    windows: tempwindows,
                }
            }
        }
    }

    /// set a new focus window
    fn focus_window(&mut self, window: Option<Window>) -> Result<(), Self::Error> {
        match window {
            // the last window in the vector is the focused one.
            // when the windows in the list should be focused , we put it to the last of the vec.
            Some(i) => {
                match self.windows.iter().position(|w| *w == i) {
                    None => Err(TilescreenWMError::UnknownWindow(i)),
                    Some(w) => {
                        // remove the window from the vec
                        self.focused_window.clear();
                        self.focused_window.push(i);
                        Ok(())
                    }
                }

            }
            // when there is no window should be focusd, the vec turns to be empty.
            None => {
                self.focused_window.clear();
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
                // unwrap , we have checked it is in the list.
                self.focus_window(temp_prev).unwrap();
            } else {
                match dir {
                    PrevOrNext::Prev => {
                        // get the length of the vec
                        let index = self.windows
                            .iter()
                            .position(|&w| w == self.focused_window.last().map(|i| *i).unwrap())
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
                            .position(|&w| w == self.focused_window.last().map(|i| *i).unwrap())
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
            None => Err(TilescreenWMError::UnknownWindow(window)),
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

                let temp_full_screen: bool;
                if temp_geometry == self.get_screen().to_geometry() {

                    temp_full_screen = true;
                } else {
                    temp_full_screen = false;
                }

                Ok(WindowWithInfo {
                    window: window,
                    geometry: temp_geometry,
                    float_or_tile: FloatOrTile::Tile,
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
impl TilingSupport for TilescreenWM {
    /// get the master window of current windows manager
    /// the first one in the vector is the master window
    fn get_master_window(&self) -> Option<Window> {
        // The first one in the vector is the master window
        self.windows.first().map(|w| *w)

    }

    /// swap the provided window with current master tile window
    fn swap_with_master(&mut self, window: Window) -> Result<(), Self::Error> {
        match self.windows.iter().position(|w| *w == window) {
            None => Err(TilescreenWMError::UnknownWindow(window)),
            Some(i) => {
                // if the windows is in the master tile now
                // then it should be focused.
                if i == 0 {
                    let temp_prev = self.windows.first().map(|i| *i);
                    self.focus_window(temp_prev).unwrap();
                    Ok(())
                } else {
                    // put the window to the front, I mean , put it to the master tile :)
                    // put the window in the master tile to the position .
                    let temp_first = self.windows.first().map(|i| *i).unwrap();
                    self.windows.insert(i, temp_first);
                    self.windows.remove(0);
                    self.remove_window(window).unwrap();
                    self.windows.insert(0, window);

                    // and make it focused
                    let temp_prev = self.windows.first().map(|i| *i);
                    self.focus_window(temp_prev).unwrap();
                    Ok(())
                }
            }

        }
    }


    /// swap the focused window with the one in the next or previous tile.
    fn swap_windows(&mut self, dir: PrevOrNext) {
        // when no window is focused. do nothing
        if self.focused_window.is_empty() == true {
            return ();
        } else {
            match dir {
                PrevOrNext::Prev => {
                    match self.windows.len() {

                        0 | 1 => {
                            return ();
                        }
                        _ => {
                            // get index of current focused window
                            let index = self.windows
                                .iter()
                                .position(|&w| w == self.focused_window.last().map(|i| *i).unwrap())
                                .unwrap();
                            match index {
                                // if current focused window is in the master tile
                                0 => {
                                    // get the current focused window
                                    let temp_focused =
                                        self.focused_window.last().map(|i| *i).unwrap();
                                    // get the one need to be swaped with it
                                    let temp_last = self.windows.last().map(|i| *i).unwrap();
                                    let len = self.windows.len();
                                    // swap two windows
                                    self.windows[0] = temp_last;
                                    self.windows[len - 1] = temp_focused;

                                }

                                _ => {
                                    // get the current focused window
                                    let temp_focused = self.windows[index];
                                    // get the one need to be swaped with it
                                    let temp_swap = self.windows[index - 1];
                                    let len = self.windows.len();
                                    // swap two windows
                                    self.windows[index] = temp_swap;
                                    self.windows[index - 1] = temp_focused;
                                }

                            }
                        }

                    }
                }
                PrevOrNext::Next => {
                    match self.windows.len() {

                        0 | 1 => {
                            return ();
                        }
                        _ => {
                            // get index of current focused window
                            let index = self.windows
                                .iter()
                                .position(|&w| w == self.focused_window.last().map(|i| *i).unwrap())
                                .unwrap();
                            let len = self.windows.len();
                            // if current focused window at the last of windows vector
                            if index == len - 1 {
                                // get the current focused window
                                let temp_focused = self.focused_window.last().map(|i| *i).unwrap();
                                // get the one need to be swaped with it
                                let temp_first = self.windows.first().map(|i| *i).unwrap();
                                let len = self.windows.len();
                                // swap two windows
                                self.windows[0] = temp_focused;
                                self.windows[len - 1] = temp_first;

                            } else {
                                // get the current focused window
                                let temp_focused = self.windows[index];
                                // get the one need to be swaped with it
                                let temp_swap = self.windows[index + 1];
                                let len = self.windows.len();
                                // swap two windows
                                self.windows[index] = temp_swap;
                                self.windows[index + 1] = temp_focused;
                            }


                        }

                    }

                }

            }
        }
    }
}
#[cfg(test)]
mod tests {


    use super::TilescreenWM;

    // Repeat the imports we did in the super module.
    use cplwm_api::wm::WindowManager;
    use cplwm_api::wm::TilingSupport;
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
    fn test_tiling_windows() {
        let mut wm = TilescreenWM::new(SCREEN);
        assert_eq!(WindowLayout::new(), wm.get_window_layout());
        // add first window 1
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        // check whether the window is managed by WM
        assert!(wm.is_managed(1));
        // check the struct
        assert_eq!(vec![1], wm.get_windows());
        // check focused window
        let wl1 = wm.get_window_layout();
        assert_eq!(Some(1), wm.get_focused_window());
        assert_eq!(Some(1), wl1.focused_window);
        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
        let wl2 = wm.get_window_layout();
        assert_eq!(Some(2), wm.get_focused_window());
        assert_eq!(Some(2), wl2.focused_window);

        // check 2 windows, get the layout
        assert_eq!(vec![(1,
                         Geometry {
                            x: 0,
                            y: 0,
                            width: 800,
                            height: 600,
                        })],
                   wl1.windows);
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
                            y: 0,
                            width: 400,
                            height: 600,
                        })],
                   wl2.windows);

        // check 3 windows,get the layout
        wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();
        let wl3 = wm.get_window_layout();
        assert_eq!(Some(3), wm.get_focused_window());
        assert_eq!(Some(3), wl3.focused_window);
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
                   wl3.windows);
        // check 4 windows,get the layout
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
        let wl4 = wm.get_window_layout();
        assert_eq!(Some(4), wm.get_focused_window());
        assert_eq!(Some(4), wl4.focused_window);
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
                            y: 400,
                            width: 400,
                            height: 200,
                        }),
                        (3,
                         Geometry {
                            x: 400,
                            y: 200,
                            width: 400,
                            height: 200,
                        }),
                        (4,
                         Geometry {
                            x: 400,
                            y: 0,
                            width: 400,
                            height: 200,
                        })],
                   wl4.windows);




        // check remove_windows
        wm.remove_window(1).unwrap();
        assert_eq!(vec![2, 3, 4], wm.get_windows());
        let wl5 = wm.get_window_layout();
        assert_eq!(vec![(2,
                         Geometry {
                            x: 0,
                            y: 0,
                            width: 400,
                            height: 600,
                        }),
                        (3,
                         Geometry {
                            x: 400,
                            y: 300,
                            width: 400,
                            height: 300,
                        }),
                        (4,
                         Geometry {
                            x: 400,
                            y: 0,
                            width: 400,
                            height: 300,
                        })],
                   wl5.windows);


        // check the focused window
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        wm.focus_window(Some(1)).unwrap();
        assert_eq!(Some(1), wm.get_focused_window());
        let wl6 = wm.get_window_layout();
        assert_eq!(Some(1), wl6.focused_window);

        // When no window should  be focused, `get_focused_window()` must return `None` afterwards.
        wm.focus_window(None).unwrap();
        assert_eq!(None, wm.get_focused_window());
        let wl7 = wm.get_window_layout();
        // The `focused_window` field of the `WindowLayout` must also be `None`.
        assert_eq!(None, wl7.focused_window);


        static SOME_GEOM1: Geometry = Geometry {
            x: 0,
            y: 0,
            width: 400,
            height: 600,
        };

        static SOME_GEOM2: Geometry = Geometry {
            x: 400,
            y: 0,
            width: 400,
            height: 200,
        };
        // test get_windows_info
        assert_eq!(Some(WindowWithInfo {
                       window: 2,
                       geometry: SOME_GEOM1,
                       float_or_tile: FloatOrTile::Tile,
                       fullscreen: false,
                   }),
                   wm.get_window_info(2).ok());
        assert_eq!(Some(WindowWithInfo {
                       window: 1,
                       geometry: SOME_GEOM2,
                       float_or_tile: FloatOrTile::Tile,
                       fullscreen: false,
                   }),
                   wm.get_window_info(1).ok());
        wm.remove_window(1).unwrap();
        wm.remove_window(2).unwrap();
        wm.remove_window(3).unwrap();
        assert_eq!(Some(WindowWithInfo {
                       window: 4,
                       geometry: SCREEN.to_geometry(),
                       float_or_tile: FloatOrTile::Tile,
                       fullscreen: true,
                   }),
                   wm.get_window_info(4).ok());

        wm.remove_window(4).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        // test cycle_focus
        // now the windows vec is vec![2,3,4,1]
        // set focuse first
        wm.focus_window(Some(2)).unwrap();
        wm.cycle_focus(PrevOrNext::Prev);
        assert_eq!(vec![1], wm.focused_window);
        wm.cycle_focus(PrevOrNext::Prev);
        assert_eq!(vec![4], wm.focused_window);
        wm.cycle_focus(PrevOrNext::Next);
        assert_eq!(vec![1], wm.focused_window);
        wm.cycle_focus(PrevOrNext::Next);
        assert_eq!(vec![2], wm.focused_window);


        // test  swap_with_master
        // initial windows vec![2,3,4,1]
        // current master tile is 2
        // focused on is 2
        wm.swap_with_master(1).unwrap();
        assert_eq!(vec![1, 3, 4, 2], wm.windows);
        assert_eq!(Some(1), wm.get_master_window());
        assert_eq!(vec![1], wm.focused_window);
        wm.swap_with_master(4).unwrap();
        assert_eq!(vec![4, 3, 1, 2], wm.windows);



        // test swap_windows
        // initial windows vec![4,3,1,2]
        // when there is no window is focused , do nothing
        wm.focus_window(None).unwrap();
        assert_eq!(None, wm.get_focused_window());
        wm.swap_windows(PrevOrNext::Next);
        assert_eq!(vec![4, 3, 1, 2], wm.windows);

        wm.focus_window(Some(4)).unwrap();
        // focused one is 4
        wm.swap_windows(PrevOrNext::Next);
        assert_eq!(vec![3, 4, 1, 2], wm.windows);
        // check the focused on doesn't change
        assert_eq!(Some(4), wm.get_focused_window());


        // now is [3,4,1,2]
        wm.focus_window(Some(3)).unwrap();
        wm.swap_windows(PrevOrNext::Prev);
        assert_eq!(vec![2, 4, 1, 3], wm.windows);

        wm.focus_window(Some(4)).unwrap();
        wm.swap_windows(PrevOrNext::Prev);
        assert_eq!(vec![4, 2, 1, 3], wm.windows);

        wm.windows = vec![1];
        wm.swap_windows(PrevOrNext::Prev);
        assert_eq!(vec![1], wm.windows);

        wm.windows = vec![];
        assert_eq!(true, wm.windows.is_empty());
        wm.swap_windows(PrevOrNext::Prev);
        assert_eq!(true, wm.windows.is_empty());


    }
}
