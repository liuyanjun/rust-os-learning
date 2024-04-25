fn bubble_sort(nums: &mut Vec<i32>){

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

fn main(){

    let mut data = vec![15, 8, 10, 20, 30, 50, 12];

    bubble_sort2(&mut data);


    println!("{:?}", data);
}
