#[derive(Debug,PartialEq)]
struct Stop(usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_stop_equals_itself() {
        let stop = Stop(42);
        assert_eq!(stop, stop)
    }

    #[test]
    fn two_distinct_stop_are_different() {
        let stop_a = Stop(17);
        let stop_b = Stop(23);
        assert_ne!(stop_a, stop_b);
    }
    #[test]
    fn two_with_same_value_are_equals() {
        let stop_a = Stop(42);
        let stop_b = Stop(42);
        assert_eq!(stop_a, stop_b);
    }
}
