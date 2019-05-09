//! Animate and schedule code.
//!
//! This crate provides the functionality of running code at given times, in the same way as
//! animation artists create animated movies. The idea of this crate is to ease building code-driven
//! artistic animated programs.
//!
//! # Concepts

#![feature(try_trait)]

use std::ops::Sub;
use try_guard::guard;

/// A behavior that gives values of type `A` varying over time `T`.
///
/// A behavior is just whatever function that can provide a value at any time of `T`.
pub struct Behavior<'a, T, A> {
  behavior: Box<'a + Fn(T) -> Option<A>>
}

impl<'a, T, A> Behavior<'a, T, A> {
  pub fn from_fn<F>(f: F) -> Self where F: 'a + Fn(T) -> Option<A> {
    Behavior {
      behavior: Box::new(f)
    }
  }

  pub fn react(&self, t: T) -> Option<A> {
    (self.behavior)(t)
  }
}

/// A cut in a behavior at given time (`T`).
///
/// Cuts represent slice to behaviors, identified by the `C` type variable, with a given start and
/// stop times, identified by the the `T` type variable. The difference between the times gives the
/// duration of the cut.
///
/// A cut also embed transactions. Basically, it’s possible that several cuts are triggered at the
/// same time. In that case, each cut contains some additional information about how to deal with
/// such overlapping.
pub struct Cut<'a, T, A> {
  /// The behavior the cut refers to.
  pub behavior: &'a Behavior<'a, T, A>,
  /// Time (including) at which the cut starts in the behavior.
  pub start_t: T,
  /// Time (including) at which the cut stops in the behavior.
  pub stop_t: T,
}

impl<'a, T, A> Cut<'a, T, A> {
  fn new(behavior: &'a Behavior<'a, T, A>, start_t: T, stop_t: T) -> Option<Self> where T: PartialOrd {
    guard!(stop_t < start_t);

    Some(Cut { behavior, start_t, stop_t })
  }

  fn dur(&self) -> T where T: Copy + Sub<T, Output = T> {
    self.stop_t - self.start_t
  }
}

/// A collection of cuts.
pub struct Track<'c, T, A> {
  cuts: Vec<Cut<'c, T, A>>
}

/// A collection of tracks.
pub struct Timeline<'c, T, A> {
  tracks: Vec<Track<'c, T, A>>
}

/// A type that can generate time when asked.
pub trait TimeGenerator {
  type Time;

  /// Tick time forward.
  fn tick(&mut self) -> Self::Time;

  /// Tick time backwards.
  fn untick(&mut self) -> Self::Time;

  /// Reset the generator and time to their initial values.
  fn reset(&mut self);

  /// Change the internal delta.
  fn change_delta(&mut self, delta: Self::Time);
}

/// A simple generator that generates `f32` times by delta.
pub struct SimpleF32TimeGenerator {
  current: f32,
  reset_value: f32,
  delta: f32
}

impl SimpleF32TimeGenerator {
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
    self.current = self.reset_value
  }

  fn change_delta(&mut self, delta: Self::Time) {
    self.delta = delta;
  }
}

/// In the lack of a better name, I’ll call that shit Scheduler. And I’m drunk.
pub struct Scheduler<'a, T, A, G> {
  timeline: Timeline<'a, T, A>,
  time_generator: G,
}