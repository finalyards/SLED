

/// An animation building on top of others.
struct StateAnim<T> {
    current: T, // inner mutability so we can change it with a const reference
}

impl StateAnim {
    fn show(&self, state: T) {
        self.current = state;
    }
}

impl Animation for StateAnim {
    fn sample(&self, dur: Duration) -> Rgb {
        self.current.sample(dur)
    }
}
