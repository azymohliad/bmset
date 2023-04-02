# bmset

BitmapSet - compact stack-allocated ordered set for small numbers.

It provides a similar API to standard [BTreeSet](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html), but limited to small numbers only, and more efficient for this use-case.

Defining features:
- Compact
- Performant
- No allocations
- No dependencies
- `no_std`
- Very simple

## Usage

```rust
use bmset::BitmapSet;

let set1: BitmapSet<10> = (0..45).collect();
let set2: BitmapSet<10> = (42..80).collect();
let mut set3 = set1.intersection(&set2);
set3.insert(73);
set3.remove(44);
assert_eq!(set3, [42, 43, 73].iter().collect());
let mut set4 = BitmapSet::<10>::new();
set4.insert(42);
// set4.insert(85); // PANIC: max value for 10-bytes set is 79
assert!(set4.is_subset(&set3));
```

## Details

It occupies from 1 to 32 bytes (defined explicitly by const generic), and can store a maximum value of up to a total number of bits (non-inclusive, i.e. 255 for 32 bytes).

It is ridiculously simple: just a sequence of bits, each representing a presence of its index in the set. This means that all set operations are very efficient and boil down to bitwise operations on the bitmaps, having time complexity of O(1) (independent of the number of values in the set, though proportional to the max set size):
- Insert/remove/check value - set/clear/check a bit by its index
- Union - bitwise OR
- Intersection - bitwise AND
- Complement - bitwise NOT
- Difference - bitwise AND with complement
- Subset/superset check - equality check with intersection/union

See [benchmarks](benches/bench.rs) and run them (`cargo bench`) for performance comparison with BTreeSet.
