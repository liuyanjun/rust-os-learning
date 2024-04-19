Rust的迭代器给用户提供了一种灵活，通用的迭代序列的做法，并且和其所有权系统密切相连，和同样是作为系统级语言的C/Cpp有着比较大的区别，要完全理解其迭代器的设计思想和设计细节不是一件容易的事。我个人在学习初期也感觉学起来非常难受，明明在C里用的好好的写法移植到Rust里要不就是啰嗦要不就是有额外开销，甚至有时候还要上unsafe。但是在熟悉了用法和底层原理之后，发现大部分场景其实使用safe的迭代器也能很好的解决，并且速度反而更快。
另外推荐一个迭代器库itertools    https://crates.io/crates/itertools，这个库提供了比标准库更丰富的操作迭代器的接口，和标准库的trait兼容，相当于是一个功能扩展，具体使用方式可以参照其文档。
 
热身：构造出<=100且能被3整除的正整数
方式一：for循环
let mut list1: Vec<i32> = vec![];
for i in 1..=100 {
if i%3 == 0 {
list1.push(i);
}
}
println!("{:?}", list1);

方式二：通过迭代器提供的filter()和collect()方法。
filter()方法，对迭代器中的元素进行“过滤”，只留下满足条件的元素；
collect()方法负责将迭代器中的元素按顺序收集到一个集合容器中。
let list2: Vec<_> = (1..=100).filter(|i| i%3 == 0).collect();
assert_eq!(list1, list2);

显然，方式二在代码书写的便捷性和代码的可读性上有显著优势。
认识迭代器（Iterator）
我们知道，切片是对内存中连续数据的表达抽象。
与此相仿，迭代器可认为是数据按顺序访问的抽象，是软件开发中一种常用的设计模式（迭代器模式）。
通过迭代器，可以很方便地对一个数据流进行各种处理，包括映射(Map)+过滤(Filter)+求值(Reduce)等。
Rust中的迭代器，通过next()方法拿到下一个元素，如果next()返回的是None，表明迭代器中元素已全部取出。
观察一个细节：next()方法传入了self的引用，而不是self本身（会转移所有权），因此next()方法可以被多次调用。
pub trait Iterator {
type Item;
fn next(&mut self) -> Option<Self::Item>;
}

迭代器的构造
迭代器的构造有多种方式。
use std::slice::Iter;
use std::iter::Iterator;
let it = 0..10;//[0,9]的整数
let it = 0..=10;//[0,10]的整数
let it = 0..;// 产生从0开始的无限整数流（迭代器）
let it = [1,2,3,4].iter();
let it = vec![5; 3].iter();
// chain()将两个迭代器顺序拼接合并，后续有例子
let it = (0..10).chain(20..30);
// zip()将2个迭代器，并列为一个元组迭代器
// 元素个数跟个数较少的迭代器相同
let a = [1,2,3].iter();
let b = ['a', 'b', 'c', 'd'].iter();
let x = a.zip(b);
//(1, 'a')(2, 'b')(3, 'c')
for c in x {
print!("{:?}", c);
}

迭代器的遍历（while-next模式）
let mut iter = [1,2,3,4,5].iter();
//输出：1 2 3 4 5
while let Some(x) = iter.next() {
print!("{:?} ", x);
}

迭代器的遍历（for-in模式）
let it = 0..10;//[0,9]
// for-in循环对迭代器进行遍历
for i in it {
print!("{} ", i);
}

迭代器的遍历（for_each()）
// for_each()对元素逐个处理，内部调用的是fold()，
// fold()在后面有解释其用法
(1..=3).for_each(|x| {
print!("{x} ");//1 2 3
});

可修改的迭代器IterMut
通过iter_mut()可以获取到可修改的迭代器，能够在遍历的过程修改迭代器中的值。
当然这要求原始数据结构是可修改的，例如下面例子中的arr被mut关键字进行了修饰。
let mut arr = [1,2,3,4];
arr.iter_mut().for_each(|x|*x*=2);//迭代器的类型为：IterMut<i32>
println!("{:?}", arr);//[2, 4, 6, 8]

从迭代器取元素
next()
可多次调用，一次取出一个值，直至返回None。
// next()方法取得迭代器中下一个元素
let mut it = 1..3;
assert_eq!(Some(1), it.next());
assert_eq!(Some(2), it.next());
assert_eq!(None, it.next());

take(k)取前面k个元素，只可调用一次
迭代器调用take()后，迭代器的所有权会被转移到take方法内部，因此一个迭代器的take方法只能调用一次。
assert_eq!(vec![1,2,3], (1..10).take(3).collect::<Vec<_>>());

nth(k)
取得迭代器剩余元素中第k个位置的元素，位置从0开始；之后，迭代器跳转到下一个位置。
let mut it = [1, 2, 3].iter();
assert_eq!(Some(&1), it.nth(0));
assert_eq!(Some(&2), it.nth(0));
assert_eq!(Some(&3), it.nth(0));
assert_eq!(None, it.nth(0));
assert_eq!(Some(3), (0..4).nth(3));

last()
只取最后一个元素，只能调用一次。
assert_eq!((1..4).last(), Some(3));

对迭代器进行变换
rev()反转
//反向遍历
println!("{:?}", "-".repeat(10));
//输出：4,3,2,1,0,
vec![0, 1, 2, 3, 4].iter().rev().for_each(|x|print!("{x},"));
println!();
//输出：9,8,7,6,5,4,3,2,1,0,
for i in (0..10).rev() {
print!("{:?},", i);
}
println!("\n{:?}", "-".repeat(10));

skip(k)跳过k个元素
assert_eq!(vec![2,3], (0..4).skip(2).collect::<Vec<_>>());

step_by(k)，从第一个元素开始，每k个取一个出来
//0 2 4 6 8 10
(0..=10).step_by(2).for_each(|x| print!("{x} "));
assert_eq!(vec![0,2,4,6], (0..7).step_by(2).collect::<Vec<_>>());

chain()方法对迭代器进行顺序拼接合并
let it = (0..5).chain(15..20);
//[0, 1, 2, 3, 4, 15, 16, 17, 18, 19]
println!("{:?}", it.collect::<Vec<_>>());

zip()将2个迭代器合并为一对一元组迭代器
let it = [1,3,5].iter().zip([2,4,6].iter());
assert_eq!(vec![(&1,&2),(&3,&4),(&5,&6)], it.collect::<Vec<(_,_)>>());
assert_eq!(vec![(0,'f'),(1,'o'),(2,'o')], (0..).zip("foo".chars()).collect::<Vec<_>>());
// 计算2个等长字符串中相同位置上字符不同的个数
let s1 = "amily";
let s2 = "emily";
let diff = s1.chars().zip(s2.chars()).filter(|x|x.0 != x.1).count();
println!("{diff}");//1

map()方法，对迭代器中每一个元素进行映射，返回一个新的迭代器
assert_eq!(vec![0,1,4,9,16], (0..5).map(|x|x*x).collect::<Vec<_>>());
对迭代器进行求值结算
max()/min()/count()/sum()
//最大值
assert_eq!([1,2,3].iter().max(), Some(&3));
//最小值
assert_eq!([1,2,3].iter().min(), Some(&1));
// count()计算迭代器中元素的个数
assert_eq!([1,2,3].iter().count(), 3);
// 求和
assert_eq!([1,2,3].iter().sum::<i32>(), 6);
all(predicate)
判断迭代器中是否所有元素都符合闭包predicate指定的测试。
let b = (2..10).into_iter().all(|i|i>0);
println!("{}", b);//true
fold(初始值, 累加器)
fold()方法，通过传入一个初始值和一个闭包累加器，对迭代器中的每一个元素依次进行处理并“累加”，最后返回“累加”结果。这里用“累加”来指代函数操作，并不仅仅是能做加法。
assert_eq!(3, (1..3).fold(0, |acc, x|acc+x));//1+2
assert_eq!(6, (1..3).fold(0, |acc, x|acc+2*x));//2*1 + 2*2
