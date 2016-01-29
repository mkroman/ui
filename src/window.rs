// This is free and unencumbered software released into the public domain.
//
// Anyone is free to copy, modify, publish, use, compile, sell, or
// distribute this software, either in source code form or as a compiled
// binary, for any purpose, commercial or non-commercial, and by any
// means.
//
// In jurisdictions that recognize copyright laws, the author or authors
// of this software dedicate any and all copyright interest in the
// software to the public domain. We make this dedication for the benefit
// of the public at large and to the detriment of our heirs and
// successors. We intend this dedication to be an overt act of
// relinquishment in perpetuity of all present and future rights to this
// software under copyright law.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// For more information, please refer to <http://unlicense.org>

use std::default;

use geometry::Rect;

#[derive(Debug)]
/// Represents visibility.
pub enum Visibility {
    Windowed,
    Minimized,
    Maximized,
    FullScreen,
    Hidden,
}

impl default::Default for Visibility {
    fn default() -> Visibility {
        Visibility::Windowed
    }
}

#[derive(Debug, Default)]
/// Window structure.
pub struct Window {
    pub visibility: Visibility,
    pub frame: Rect,
    pub visible: bool
}

impl Window {
    /// Creates a new window.
    pub fn new() -> Window {
        Window {
            visibility: Visibility::Windowed,
            // TODO: Take the frame from the developer, or figure out a way to come up with an
            // optimal one in the library.
            frame: Rect::new(400.0, 300.0, 400.0, 300.0),
            visible: false,

        }
    }

    /// Shows the window.
    pub fn show(&mut self) {
        self.visible = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window() {
        let _ = Window::new();
    }
}
