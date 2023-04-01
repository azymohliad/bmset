use bmset::BitmapSet;

fn main() {
    let set1: BitmapSet<10> = (0..45).collect();
    let set2: BitmapSet<10> = (42..80).collect();
    let mut set3 = set1.intersection(&set2);
    set3.insert(73);
    set3.remove(44);
    assert_eq!(set3, [42, 43, 73].iter().collect());
}
