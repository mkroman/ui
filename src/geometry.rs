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

/// Geometric size structure.
#[derive(Debug, Default)]
pub struct Size<T = f64> {
    pub width: T,
    pub height: T,
}

/// Geometric point structure.
#[derive(Debug, Default)]
pub struct Point<T = f64> {
    /// The points X offset.
    pub x: T,
    /// The points Y offset.
    pub y: T,
}

/// Rectangle structure.
#[derive(Debug, Default)]
pub struct Rect<T = f64> {
    pub point: Point<T>,
    pub size: Size<T>,
}

impl<T> Point<T> {
    /// Creates a new geometric point.
    fn new(x: T, y: T) -> Point<T> {
        Point {
            x: x,
            y: y
        }
    }
}

impl<T> Size<T> {
    /// Creates a new geometric size.
    fn new(width: T, height: T) -> Size<T> {
        Size {
            width: width,
            height: height
        }
    }
}

impl<T> Rect<T> {
    /// Creates a new rectangle.
    pub fn new(width: T, height: T, x: T, y: T) -> Rect<T> {
        Rect {
            point: Point::new(x, y),
            size: Size::new(width, height),
        }
    }
}
