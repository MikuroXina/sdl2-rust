use crate::color::Color;
use crate::video::geo::{Line, Point};
use crate::{bind, Sdl};

use super::Renderer;

pub struct Pen<'renderer> {
    renderer: &'renderer Renderer<'renderer>,
}

impl<'renderer> Pen<'renderer> {
    pub fn new(renderer: &'renderer Renderer) -> Self {
        Self { renderer }
    }

    pub fn set_color(&self, Color { r, g, b }: Color) {
        let ret = unsafe { bind::SDL_SetRenderDrawColor(self.renderer.as_ptr(), r, g, b, 255) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen color")
        }
    }

    pub fn clear(&self) {
        let ret = unsafe { bind::SDL_RenderClear(self.renderer.as_ptr()) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen clear")
        }
    }

    pub fn line(&self, line: Line) {
        let ret = unsafe {
            bind::SDL_RenderDrawLine(
                self.renderer.as_ptr(),
                line.start.x,
                line.start.y,
                line.end.x,
                line.end.y,
            )
        };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen line")
        }
    }

    pub fn lines(&self, points: impl IntoIterator<Item = Point>) {
        let points: Vec<_> = points.into_iter().map(|p| p.into()).collect();
        let ret = unsafe {
            bind::SDL_RenderDrawLines(self.renderer.as_ptr(), points.as_ptr(), points.len() as i32)
        };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen lines")
        }
    }

    pub fn point(&self, point: Point) {
        let ret = unsafe { bind::SDL_RenderDrawPoint(self.renderer.as_ptr(), point.x, point.y) };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen point")
        }
    }

    pub fn points(&self, points: impl IntoIterator<Item = Point>) {
        let points: Vec<_> = points.into_iter().map(|p| p.into()).collect();
        let ret = unsafe {
            bind::SDL_RenderDrawPoints(self.renderer.as_ptr(), points.as_ptr(), points.len() as i32)
        };
        if ret != 0 {
            Sdl::error_then_panic("Sdl pen points")
        }
    }

    // TODO(MikuroXina): draw rects
    // TODO(MikuroXina): fill rect
}

impl<'renderer> Drop for Pen<'renderer> {
    fn drop(&mut self) {
        unsafe { bind::SDL_RenderPresent(self.renderer.as_ptr()) }
    }
}
