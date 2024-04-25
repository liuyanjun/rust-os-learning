
struct Queue<T, const N: usize>{
    data : [T; N],
    bottom: usize,
    top: usize,
}

impl<T: Default, const N: usize> Queue<T, N>{
    fn new() -> Self{
        Self {
             data: std::array::from_fn(|_| T::default()), 
             bottom: 0, 
             top: 0 }
    }

    fn push(&mut self, item: T) -> Result<(), &'static str>{
        if self.top == N {
            return Err("Queue is full");
        }
        self.data[self.top] = item;
        self.top += 1;
        Ok(())
    }

    fn pop(&mut self) -> Result<T, &'static str>{
        if self.top == self.bottom {
            Err("Queue is empty")
        }else{
            let mut k = T::default();
            std::mem::swap(&mut k, &mut self.data[self.bottom]);
            self.bottom += 1;
            Ok(k)
        }
    }
}

fn main(){
    let mut queue = Queue::<i32, 20>::new();
    for i in 1..10 {
        queue.push(i).unwrap();
    }

    for _i in 1..10 {
        println!("{:?}", queue.pop().unwrap());
    }
}

mod test {
    use super::*;
    #[test]
    fn check_queue(){
        let mut a = Queue::<i32, 3>::new();

        assert_eq!(a.push(1), Ok(()));
        assert_eq!(a.push(2), Ok(()));
        assert_eq!(a.push(3), Ok(()));
        assert_eq!(a.push(4), Err("Queue is full"));
        assert_eq!(a.pop(), Ok(1));
        assert_eq!(a.pop(), Ok(2));
        assert_eq!(a.pop(), Ok(3));
        assert_eq!(a.pop(), Err("Queue is empty"));
    }
}
