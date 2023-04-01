use std::fmt::Debug;

#[derive(Clone, PartialEq, Eq)]
pub struct BitmapSet<const SIZE: usize = 32> {
    data: [u8; SIZE],
}

impl<const SIZE: usize> BitmapSet<SIZE> {
    const MAXVAL: u8 = (SIZE * 8 - 1) as u8;

    pub fn new() -> Self {
        assert!(SIZE <= 32);
        Self { data: [0; SIZE] }
    }

    pub fn contains(&self, value: u8) -> bool {
        assert!(value <= Self::MAXVAL);
        *self.byte(value) & Self::mask(value) != 0
    }

    pub fn insert(&mut self, value: u8) {
        assert!(value <= Self::MAXVAL);
        *self.byte_mut(value) |= Self::mask(value);
    }

    pub fn remove(&mut self, value: u8) {
        assert!(value <= Self::MAXVAL);
        *self.byte_mut(value) &= !Self::mask(value);
    }

    pub fn clear(&mut self) {
        self.data = [0; SIZE];
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let mut result = self.clone();
        result.intersect(other);
        result
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut result = self.clone();
        result.unite(other);
        result
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut result = self.clone();
        result.subtract(other);
        result
    }

    pub fn complement(&self) -> Self {
        let mut result = self.clone();
        result.invert();
        result
    }

    pub fn intersect(&mut self, other: &Self) {
        for (dst, src) in self.data.iter_mut().zip(other.data.iter()) {
            *dst &= *src;
        }
    }

    pub fn unite(&mut self, other: &Self) {
        for (dst, src) in self.data.iter_mut().zip(other.data.iter()) {
            *dst |= *src;
        }
    }

    pub fn subtract(&mut self, other: &Self) {
        for (dst, src) in self.data.iter_mut().zip(other.data.iter()) {
            *dst &= !*src;
        }
    }

    pub fn invert(&mut self) {
        for byte in self.data.iter_mut() {
            *byte = !*byte
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data == [0; SIZE]
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).is_empty()
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        &self.intersection(other) == self
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(self)
    }

    pub fn iter(&self) -> Iter<SIZE> {
        Iter { set: self, value: Some(0) }
    }

    fn byte(&self, value: u8) -> &u8 {
        &self.data[Self::idx(value)]
    }

    fn byte_mut(&mut self, value: u8) -> &mut u8 {
        &mut self.data[Self::idx(value)]
    }

    fn idx(value: u8) -> usize {
        (value / 8) as usize
    }

    fn mask(value: u8) -> u8 {
        1 << (value % 8)
    }
}

impl<const SIZE: usize> Debug for BitmapSet<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let seq = self.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "BitmapSet {{{}}}", seq)
    }
}

impl<const SIZE: usize> From<&[u8]> for BitmapSet<SIZE> {
    fn from(slice: &[u8]) -> Self {
        slice.iter().collect()
    }
}

impl<const SIZE: usize> FromIterator<u8> for BitmapSet<SIZE> {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut set = Self::new();
        for item in iter {
            set.insert(item)
        }
        set
    }
}

impl<'a, const SIZE: usize> FromIterator<&'a u8> for BitmapSet<SIZE> {
    fn from_iter<T: IntoIterator<Item = &'a u8>>(iter: T) -> Self {
        iter.into_iter().copied().collect()
    }
}



pub struct Iter<'a, const SIZE: usize> {
    set: &'a BitmapSet<SIZE>,
    value: Option<u8>,
}

impl<'a, const SIZE: usize> Iter<'a, SIZE> {
    const MAXVAL: u8 = (SIZE * 8 - 1) as u8;

    fn advance(&mut self) {
        self.value = self.value.and_then(|v| {
            if v < Self::MAXVAL { Some(v + 1) } else { None }
        })
    }
}

impl<'a, const SIZE: usize> Iterator for Iter<'a, SIZE> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(value) = self.value {
            self.advance();
            if self.set.contains(value) {
                return Some(value);
            }
        }
        None
    }
}
