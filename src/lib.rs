/*!
Trigger
=======

Check when a variable has changed during assignment:

```
// State variable to detect when changed
let mut state = (1, 2.0);

// Assign a different value and on_changed returns true
assert!(trigger::on_changed(&mut state, (2, 2.0)));

// Indeed the state has changed
assert_eq!(state, (2, 2.0));

// Assign the same value and on_changed returns false
assert!(!trigger::on_changed(&mut state, (2, 2.0)));
```

Check when a boolean becomes true or false:

```
// Boolean state variable
let mut state = trigger::State::default();

// State remains unchanged because its is initialized to false by default
assert!(!state.update(false).changed());

// When updated to true the signal is 'raised'
assert!(state.update(true).raised());

// When updated to true again the signal is unchanged
assert!(!state.update(true).changed());

// When updated to false the signal 'fell'
assert!(state.update(false).fell());
```

!*/

#![no_std]

use core::mem;

/// Indicates the transition of the state.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(i8)]
pub enum Signal {
	/// The state transitioned from high to low.
	Fell = -1,
	/// The state remained unchanged, either high or low.
	Level = 0,
	/// The state transitioned from low to high.
	Raised = 1,
}
impl Signal {
	unsafe fn transmute(value: i8) -> Signal {
		mem::transmute(value)
	}
	/// Returns true if the signal is Raised.
	#[inline(always)]
	pub fn raised(self) -> bool {
		self as i8 > 0
	}
	/// Returns true if the signal is Fell.
	#[inline(always)]
	pub fn fell(self) -> bool {
		(self as i8) < 0
	}
	/// Returns true if the signal is either Raised or Fell.
	#[inline(always)]
	pub fn changed(self) -> bool {
		self as i8 != 0
	}
}

/// Detect rising and falling edges of boolean state.
#[derive(Clone, Debug, Default)]
pub struct State {
	state: bool,
}
impl From<bool> for State {
	#[inline(always)]
	fn from(state: bool) -> State {
		State { state }
	}
}
impl From<State> for bool {
	#[inline(always)]
	fn from(state: State) -> bool {
		state.state
	}
}
impl State {
	pub const fn default() -> State {
		State { state: false }
	}
	pub const fn from(state: bool) -> State {
		State { state }
	}
	/// Updates the state and returns how it has changed.
	#[inline(always)]
	pub fn update(&mut self, state: bool) -> Signal {
		// Cool trick, instantly calculate the signal by subtracting the states
		// true - true => 0 (Level)
		// false - false => 0 (Level)
		// true - false => 1 (Raised)
		// false - true => -1 (Fell)
		let signal = state as i8 - self.state as i8;
		self.state = state;
		unsafe { Signal::transmute(signal) }
	}
}

//----------------------------------------------------------------

/// Assigns value to state and returns if state has changed.
///
/// # Examples
///
/// ```
/// // Some variable we want to keep track of
/// let mut state = (1, 2.0);
///
/// // Assign a different value and on_changed returns true
/// assert!(trigger::on_changed(&mut state, (2, 2.0)));
///
/// // Indeed the state has changed
/// assert_eq!(state, (2, 2.0));
///
/// // Assign the same value and on_changed returns false
/// assert!(!trigger::on_changed(&mut state, (2, 2.0)));
/// ```
pub fn on_changed<T: PartialEq>(state: &mut T, value: T) -> bool {
	let changed = *state != value;
	*state = value;
	changed
}

//----------------------------------------------------------------

use core::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Default)]
pub struct AtomicState {
	state: AtomicBool,
}
impl AtomicState {
	pub const fn new(init: bool) -> AtomicState {
		let state = AtomicBool::new(init);
		AtomicState { state }
	}
	#[inline(always)]
	pub fn update(&self, state: bool) -> Signal {
		let signal = state as i8 - self.state.swap(state, Ordering::SeqCst) as i8;
		unsafe { Signal::transmute(signal) }
	}
}

//----------------------------------------------------------------

#[macro_export]
macro_rules! run_once {
	($stmt:stmt) => {
		use core::sync::atomic::{AtomicBool, Ordering};
		static STATE: AtomicBool = AtomicBool::new(false);
		if !STATE.swap(true, Ordering::SeqCst) {
			$stmt
		}
	};
}
