#[derive(Clone)]
struct InfiniteUnit;

pub fn main() {
    let mut count = 0;

    for _ in InfiniteUnit {
        count += 1;
        println!("count == {}", count);
        if count >= 5 {
            break;
        }
    }
}

impl IntoIterator for InfiniteUnit {
    type Item = InfiniteUnit;
    type IntoIter = std::iter::Repeat<()>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::repeat(())
    }
}