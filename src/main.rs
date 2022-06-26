/* Exercise 1 */

/**** Cho 2 mảng kiểm tra mảng này có phải là mảng con của mảng kia không?

    Ví dụ: let org_arr = [1,2,3,5,6,8,10,11];
            let sub_arr = [6,8,10];

******/

/* Exercise 2 */

/**** Cho 1 chuỗi str Slice như link. Nhập 1 từ bất kỳ từ bàn phím,
      in ra số lượng từ này xuất hiện trong chuỗi đã cho

******/

use std::fs;
use std::io;
use std::result;

#[derive(Debug)]

struct Solution {}

impl Solution {
    // Exercise 1:
    fn check_sub_arr(a: &[i32], b: &[i32]) -> bool {
        let mut result = false;
        // let mut i = 0;
        // let mut j = 0;
        let a_len = a.len();
        let b_len = b.len();
        let mut count = 0;

        if a_len == 0 || b_len == 0 || a_len < b_len {
            result = false;
        } else {
            for i in a.iter() {
                for j in b.iter() {
                    if i == j {
                        count += 1;
                    } else {
                        /* Do nothing */
                    }
                }
            }
        }

        if count == b_len {
            result = true;
        } else {
            result = false;
        }

        result
    }

}
fn main() {
    /* Exercise 1 */
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [1, 8, 10];

    let result_exercise_1 = Solution::check_sub_arr(&org_arr, &sub_arr);

    if result_exercise_1 {
        println!("A is sub array of B");
    } else {
        println!("A is NOT sub array of B");
    }


    /* Exercise 2 */
    //Input String outside
    let contents = fs::read_to_string("Exercise2.txt").expect("Something went wrong");

    println!("Please input your string: ");
    // Input string
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    input_string.pop();

    println!("Your string is {}", input_string);
    //return result
    let counts = contents.matches(&input_string).count();

    println!("The word {:?} is {} in this file", input_string, counts);
}
