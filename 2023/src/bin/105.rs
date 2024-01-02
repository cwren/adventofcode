fn main() {
}

#[cfg(test)]
mod tests {
    use interval::Interval;
    use interval::IntervalSet;
    use interval::ops::*;
    use gcollections::ops::*;
    use interval::interval_set::ToIntervalSet;

    #[test]
    fn test_basics() {
        let a = Interval::new(0, 5);
        let b = Interval::singleton(10);
        let c = Interval::new(3, 9);
        assert_eq!(c.upper(), 9);
        assert_eq!(c.lower(), 3);
        assert_eq!(a.is_disjoint(&b), true);
        assert_eq!(a.is_disjoint(&c), false);
        assert_eq!(a.overlap(&c), true);
        assert_eq!(c.is_disjoint(&b), true);
        assert_eq!(c.overlap(&b), false);
    }
    
    #[test]
    fn test_interval_ops() {
        let a = Interval::new(0, 5);
        let b = Interval::new(3, 7);
        let c = Interval::new(9, 11);
        assert_eq!(a.intersection(&b), Interval::new(3, 5));
        assert_eq!(a.intersection(&c), Interval::empty());
        assert_eq!(a.join(b), Interval::new(3, 5)); // synonym for intersect?
        assert_eq!(a.meet(b), Interval::new(0, 7)); // synonym for union?
        assert_eq!(a.meet(c), Interval::new(0, 11)); // hull of the union?
        assert_eq!(a + 10, Interval::new(10, 15)); // elementwise add
        assert_eq!(a - 10, Interval::new(-10, -5)); // elementwise sub
        assert_eq!(b * 6, Interval::new(18, 42)); // elementwise mul
    }

    #[test]
    fn test_intervalset_ops() {
        let a = IntervalSet::new(0, 5);
        let b = IntervalSet::new(6, 7);

        // elementwise add
        let c = a.clone() + &b;
        assert_eq!(c, IntervalSet::new(6, 12)); 

        let d = a.union(&b);
        assert_eq!(d.interval_count(), 1);
        assert_eq!(d, IntervalSet::new(0, 7));
        assert_eq!(a.intersection(&b).is_empty(), true);

        let e = a.union(&vec![(10, 12)].to_interval_set());
        assert_eq!(e.interval_count(), 2);
        assert_eq!(e.iter().next(), Some(&Interval::new(0, 5)));
        assert_eq!(e.iter().nth(1), Some(&Interval::new(10, 12)));
        
        let f = IntervalSet::new(2, 3);
        assert_eq!(a.difference(&f), vec![(0, 1), (4, 5)].to_interval_set());
        assert_eq!(f.difference(&a).is_empty(), true);
        assert_eq!(a.intersection(&f), vec![(2, 3)].to_interval_set());
        assert_eq!(f.overlap(&vec![(2, 3)].to_interval_set()), true);
        assert_eq!(f.overlap(&vec![(0, 1)].to_interval_set()), false);
        assert_eq!(f.overlap(&vec![(4, 5)].to_interval_set()), false);
    }
}