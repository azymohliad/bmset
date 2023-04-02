use bmset::BitmapSet;

fn main() {
    let set1: BitmapSet<10> = (0..45).collect();
    dbg!(&set1);
    let set2: BitmapSet<10> = (42..80).collect();
    dbg!(&set2);
    let mut set3 = set1.intersection(&set2);
    set3.insert(73);
    set3.remove(44);
    dbg!(&set3);
    assert_eq!(set3, [42, 43, 73].iter().collect());
    let mut set4 = BitmapSet::<10>::new();
    set4.insert(42);
    // set4.insert(85); // PANIC: max value for 10-bytes set is 79
    dbg!(&set4);
    assert!(set4.is_subset(&set3));
}
