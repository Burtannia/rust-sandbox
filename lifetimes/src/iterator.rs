struct Single<T> {
    next: Option<T>
}

fn single<T>(t: T) -> Single<T> {
    Single {
        next: Some(t)
    }
}

impl<T> Iterator for Single<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // let mut res = None;
        // std::mem::swap(&mut res, &mut self.next);
        // res

        // std::mem::replace(&mut self.next, None)

        self.next.take()
    }
}

pub fn main() {
    let actual: Vec<u32> = single(42).collect();
    assert_eq!(vec![42], actual);
}