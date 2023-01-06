#[inline(never)]
pub fn stdlib_max<T: Ord + Copy>(a: &[T]) -> Option<T> {
    a.iter().max().copied()
}

#[inline(never)]
pub fn custom_max<T: Ord + Copy>(a: &[T]) -> Option<T> {
    let first = *a.first()?;
    Some(a.iter().fold(first, |x, y| std::cmp::max(x, *y)))
}
