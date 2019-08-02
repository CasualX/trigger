Trigger
=======

Seamlessly detect state changes.

Examples
--------

### Detect when a variable changes when assigned

```rust
// State variable to detect when changed
let mut state = (1, 2.0);

// Assign a different value and on_changed returns true
assert!(trigger::on_changed(&mut state, (2, 2.0)));

// Indeed the state has changed
assert_eq!(state, (2, 2.0));

// Assign the same value and on_changed returns false
assert!(!trigger::on_changed(&mut state, (2, 2.0)));
```

### Detect the rising and falling edges of boolean state

```rust
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

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
