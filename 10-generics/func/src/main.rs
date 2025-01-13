fn largest(list: &[i32]) -> i32 {
    println!("The size of &[i32] is {}", std::mem::size_of::<&[i32]>());
    println!("The size of &list is {}", std::mem::size_of_val(&list));

    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice = &arr[0..7];

    println!("size of array is {}", std::mem::size_of_val(&arr));
    println!("size of slice is {}", std::mem::size_of_val(slice));


    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
}