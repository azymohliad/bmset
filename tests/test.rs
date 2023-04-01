use bmset::BitmapSet;

#[test]
fn test_contains() {
    let set = BitmapSet::from([1, 8, 42, 220].as_slice());
    assert!(set.contains(1));
    assert!(set.contains(8));
    assert!(set.contains(42));
    assert!(set.contains(220));
    assert!(!set.contains(0));
    assert!(!set.contains(u8::MAX));
}

#[test]
fn test_insert() {
    let mut set = BitmapSet::new();
    set.insert(1);
    assert_eq!(set, BitmapSet::from([1].as_slice()));
}

#[test]
fn test_remove() {
    let mut set = BitmapSet::from([1, 8, 42, 220].as_slice());
    set.remove(42);
    assert_eq!(set, BitmapSet::from([1, 8, 220].as_slice()));
}

#[test]
fn test_clear() {
    let mut set = BitmapSet::from([1, 8, 42, 220].as_slice());
    set.clear();
    assert!(set.is_empty());
}

#[test]
fn test_intersection() {
    let set1 = BitmapSet::from([1, 8, 32, 220].as_slice());
    let set2 = BitmapSet::from([4, 8, 16, 32].as_slice());
    let expected = BitmapSet::from([8, 32].as_slice());
    assert_eq!(set1.intersection(&set2), expected);
}

#[test]
fn test_union() {
    let set1 = BitmapSet::from([1, 8, 32, 220].as_slice());
    let set2 = BitmapSet::from([4, 8, 16, 32].as_slice());
    let expected = BitmapSet::from([1, 4, 8, 16, 32, 220].as_slice());
    assert_eq!(set1.union(&set2), expected);
}

#[test]
fn test_difference() {
    let set1 = BitmapSet::from([1, 8, 32, 220].as_slice());
    let set2 = BitmapSet::from([4, 8, 16, 32].as_slice());
    let expected = BitmapSet::from([1, 220].as_slice());
    assert_eq!(set1.difference(&set2), expected);
}

#[test]
fn test_complement() {
    let set: BitmapSet = (5..u8::MAX).collect();
    let expected = BitmapSet::from([0, 1, 2, 3, 4, u8::MAX].as_slice());
    assert_eq!(set.complement(), expected);
}

#[test]
fn test_is_empty() {
    assert!(BitmapSet::new().is_empty());
    assert!(!BitmapSet::from([1,2,3].as_slice()).is_empty());
}

#[test]
fn test_is_disjoint() {
    let set1 = BitmapSet::from([1, 2, 3, 4].as_slice());
    let set2 = BitmapSet::from([3, 4, 5, 6].as_slice());
    let set3 = BitmapSet::from([5, 6, 7, 8].as_slice());
    assert!(set1.is_disjoint(&set3));
    assert!(!set1.is_disjoint(&set2));
    assert!(!set2.is_disjoint(&set3));
}

#[test]
fn test_is_subset() {
    let set1 = BitmapSet::new();
    let set2 = BitmapSet::from([1, 2].as_slice());
    let set3 = BitmapSet::from([1, 2, 8].as_slice());
    assert!(set1.is_subset(&set3));
    assert!(set2.is_subset(&set3));
    assert!(set3.is_subset(&set3));
    assert!(!set3.is_subset(&set2));

}

#[test]
fn test_is_superset() {
    let set1 = BitmapSet::new();
    let set2 = BitmapSet::from([1, 2].as_slice());
    let set3 = BitmapSet::from([1, 2, 8].as_slice());
    assert!(set3.is_superset(&set1));
    assert!(set3.is_superset(&set2));
    assert!(set3.is_superset(&set3));
    assert!(!set2.is_superset(&set3));
}

#[test]
fn test_iter() {
    let set = BitmapSet::from([0, 255, 1, 2, 8].as_slice());
    let mut iter = set.iter();
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(8));
    assert_eq!(iter.next(), Some(255));
    assert_eq!(iter.next(), None);
}
