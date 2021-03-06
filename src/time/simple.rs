//! The simple floating, 32-bit time generator.

use crate::time::TimeGenerator;

/// A simple [`TimeGenerator`] that generates `f32` times.
///
/// You can create one by giving it its initial state (typically `0.`) and a delta. For instance,
/// for a video game that runs at 100 Hz, you want a delta set to 0.01, because you need to generate
/// a frame every 10ms. If the framerate drops, you can change the value of the delta parameter to
/// adapt.
pub struct SimpleF32TimeGenerator {
  current: f32,
  reset_value: f32,
  delta: f32
}

impl SimpleF32TimeGenerator {
  /// Create a new [`SimpleF32TimeGenerator`].
  pub fn new(reset_value: f32, delta: f32) -> Self {
    SimpleF32TimeGenerator {
      current: reset_value,
      reset_value,
      delta
    }
  }
}

impl TimeGenerator for SimpleF32TimeGenerator {
  type Time = f32;

  fn current(&self) -> Self::Time {
    self.current
  }

  fn tick(&mut self) -> Self::Time {
    let t = self.current;
    self.current += self.delta;
    t
  }

  fn untick(&mut self) -> Self::Time {
    let t = self.current;
    self.current -= self.delta;
    t
  }

  fn reset(&mut self) {
    self.set(self.reset_value);
  }

  fn set(&mut self, value: Self::Time) {
    self.current = value;
  }

  fn change_delta(&mut self, delta: Self::Time) {
    self.delta = delta;
  }
}

