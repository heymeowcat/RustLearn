struct SkipEveryOther<I> {
    iter: I,
    skip_next: bool,
}
impl<I> SkipEveryOther<I> {
    fn new(iter: I) -> Self {
        SkipEveryOther {
            iter,
            skip_next: false,
        }
    }
}
impl<I> Iterator for SkipEveryOther<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next_item = self.iter.next();
            if self.skip_next {
                self.skip_next = false;
                continue;
            }

            self.skip_next = true;
            return next_item;
        }
    }
}
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let skipped: Vec<_> = SkipEveryOther::new(numbers.into_iter()).collect();

    println!("{:?}", skipped);
}
