#[derive(Debug,PartialEq)]
pub struct Minute(usize);

impl Minute{
    pub fn next(&self) -> Option<Minute> {
        let t = self.0 + 1;
        if t < 480 {
            Some(Minute(t))
        } else {
            None
        }
    }
}

pub const FIRST_MINUTE:Minute = Minute(0);

#[cfg(test)]
mod tests {
    use super::*;

    fn the_nth_minute(n: usize) -> Minute {
        let mut minute = FIRST_MINUTE;
        let range = 0..n;
        for _i in range {
            let m = minute.next();
            match m {
                Some(min) => minute = min,
                _ => panic!("next() returns None before 480"),
            }
        }
        minute
    }
    #[test]
    fn a_minute_equals_itself() {
        let minute = FIRST_MINUTE;
        assert_eq!(minute, minute)
    }
    #[test]
    fn a_minute_has_a_next_minute_that_is_not_itself() {
        let minute = FIRST_MINUTE;
        assert_ne!(minute.next(), Some(minute));
    }
    #[test]
    fn a_minute_has_a_next_minute() {
        let minute = FIRST_MINUTE;
        assert_ne!(minute.next(), None)
    }
    #[test]
    fn a_given_minute_has_always_the_same_next_minute() {
        let minute = FIRST_MINUTE.next();
        assert_eq!(minute, FIRST_MINUTE.next());
    }
    #[test]
    fn there_is_no_minute_after_480_next_minutes() {
        let minute = the_nth_minute(479).next();
        assert_eq!(minute, None);
    }
}
