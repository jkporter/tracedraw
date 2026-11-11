// TraceDraw: the size a window's wgpu surface is given (see
// vendor/README.md). Kept apart so that TraceDraw's own tests compile and
// run it (`gpu::eframe_surface_size::tests`): this copy is outside the
// workspace and its own tests run in no gate.

use std::num::NonZeroU32;

/// The surface size for a window of `width` x `height` pixels on a device
/// whose textures are at most `max` pixels a side (`None` before the
/// device exists: the window's size).
///
/// wgpu stops the program when a surface is configured larger than the
/// device allows. On Windows a window moved to a monitor with a larger
/// scale can grow past it (8912 pixels wide on a device of 8192): each
/// side is then cut to the maximum, and the window system stretches the
/// smaller picture over the window until the window is small enough
/// again.
pub fn fit(width: NonZeroU32, height: NonZeroU32, max: Option<usize>) -> (NonZeroU32, NonZeroU32) {
    let Some(max) = max
        .and_then(|m| u32::try_from(m).ok())
        .and_then(NonZeroU32::new)
    else {
        return (width, height);
    };
    (width.min(max), height.min(max))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn n(v: u32) -> NonZeroU32 {
        NonZeroU32::new(v).expect("not zero")
    }

    #[test]
    fn a_window_larger_than_the_device_gets_the_largest_surface() {
        assert_eq!(fit(n(8912), n(2565), Some(8192)), (n(8192), n(2565)));
        assert_eq!(fit(n(2565), n(8912), Some(8192)), (n(2565), n(8192)));
        assert_eq!(fit(n(20000), n(9000), Some(16384)), (n(16384), n(9000)));
    }

    #[test]
    fn a_window_the_device_can_draw_keeps_its_size() {
        assert_eq!(fit(n(8912), n(2565), Some(16384)), (n(8912), n(2565)));
        assert_eq!(fit(n(8192), n(8192), Some(8192)), (n(8192), n(8192)));
        assert_eq!(fit(n(1600), n(950), None), (n(1600), n(950)));
        // A device that reports no size limits nothing.
        assert_eq!(fit(n(1600), n(950), Some(0)), (n(1600), n(950)));
    }
}
