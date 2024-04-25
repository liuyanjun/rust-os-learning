fn bubble_sort<T: PartialOrd>(nums: &mut Vec<T>){

    //println!("{:?}", nums);
    let n = nums.len();
    for i in 0..n {
        for j in 1..(n-i){
            if nums[j-1] > nums[j]{
        //        std::mem::swap(&mut nums[j-1], &mut nums[j]);
                nums.swap(j-1, j);
            }
        }
    }

}

fn bubble_sort2(nums: &mut Vec<i32>){

    //println!("{:?}", nums);
    let n = nums.len();
    for i in 0..n {
        for j in (i+1..n).rev(){
            if nums[j-1] > nums[j]{
        //        std::mem::swap(&mut nums[j-1], &mut nums[j]);
                nums.swap(j-1, j);
            }
        }
    }

}

pub fn quick_sort(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }
    let (mut i, mut j) = (0, nums.len() - 1);
    while i < j {
        while i < j && nums[j] >= nums[0] {
            j -= 1;
        }
        while i < j && nums[i] <= nums[0] {
            i += 1;
        }
        nums.swap(i, j);
    }
    println!("1--{:?}", nums);
    nums.swap(i, 0);
    println!("2--{:?}", nums);
    quick_sort(&mut nums[..i]);
    quick_sort(&mut nums[i + 1..]);
}

fn main(){

    let mut data = vec![15, 8, 10, 20, 30, 50, 12];

    //bubble_sort(&mut data);
    quick_sort(& mut data);

    println!("{:?}", data);
}
