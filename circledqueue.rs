#[derive(Debug)]
struct CircledQueue<T, const N: usize>{
    data : [T; N],
    bottom: usize,
    top: usize,
    size: usize,
}

impl<T: Default, const N: usize> CircledQueue<T, N>{
    fn new() -> Self{
        Self {
             data: std::array::from_fn(|_| T::default()), 
             bottom: 0, 
             top: 0,
             size: 0,
             }
    }

    fn push(&mut self, item: T) -> Result<(), &'static str>{
        if self.size == N {
            return Err("CircledQueue is full");
        }
        self.data[self.top] = item;
        self.top = (self.top + 1) % N;
        self.size += 1;
        Ok(())
    }

    fn pop(&mut self) -> Result<T, &'static str>{
        if self.size ==0 {
            Err("CircledQueue is empty")
        }else{
            let mut k = T::default();
            std::mem::swap(&mut k, &mut self.data[self.bottom]);
            self.bottom = (self.bottom + 1) % N;
            self.size -= 1;
            Ok(k)
        }
    }
}

fn main(){
    let mut cq = CircledQueue::<i32, 20>::new();
    for i in 1..10 {
        cq.push(i).unwrap();
    }

    for _i in 1..10 {
        println!("{:?}", cq.pop().unwrap());
    }
}

mod test {
    use super::*;
    #[test]
    fn check_queue(){
        let mut a = CircledQueue::<i32, 3>::new();

        assert_eq!(a.push(1), Ok(()));
        assert_eq!(a.push(2), Ok(()));
        assert_eq!(a.push(3), Ok(()));
        assert_eq!(a.push(4), Err("CircledQueue is full"));
        assert_eq!(a.pop(), Ok(1));
        assert_eq!(a.pop(), Ok(2));
        assert_eq!(a.pop(), Ok(3));
        assert_eq!(a.pop(), Err("CircledQueue is empty"));
    }
}
