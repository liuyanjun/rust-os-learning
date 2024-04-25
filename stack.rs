
struct Stack <T, const N: usize> {
    data: [T; N],
    top: usize,
    

}

impl<T: Default, const N: usize> Stack<T, N>{
    fn new() -> Self{
        Stack { 
            top: 0, 
            data: std::array::from_fn(|_| T::default()),
        }

    }

    fn push(&mut self, item: T) -> Result<(), &'static str>{
        if self.top==N {
            return Err("stack is full");
        }
        self.data[self.top] = item;
        self.top += 1;
        Ok(())
    }

    fn pop(&mut self) -> Result<T, &'static str>{
        if self.top==0 {
            return Err("stack is empty");
        }else {
            
            self.top -= 1;
            let mut k = T::default();
            // let ret = self.data[self.top -1];
            std::mem::swap(&mut k, &mut self.data[self.top]);
            Ok(k)
        }
    }
}

fn main(){
    println!("hello world");

    // let mut a: [i32; 5] = [0; 5];

    // let mut i: usize = 0;

    // fn push(a: &mut [i32], i: &mut usize, x:i32){
    //     a[*i] = x;
    //     *i += 1;
    // }

    // push(&mut a, &mut i, 3);
    // push(&mut a, &mut i, 4);
    // push(&mut a, &mut i, 5);
    let mut stack = Stack::<i32, 10>::new();   

     for i in 0..10 {
        stack.push(i).unwrap();
     };

     for _ in 0..10 {
        println!("{}", stack.pop().unwrap());
     }   
    // println!("{:?}", a);
}
