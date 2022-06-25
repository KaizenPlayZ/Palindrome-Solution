struct Solution {}
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x<0 || x%10==0 && x!=0{
            return false;
        };
        let mut y:i32 = 0 as i32;
        let mut given_number:i32 = x;
        while given_number>0 {
            y *= 10;
            y+= given_number%10;
            given_number /= 10;
        };
        if x == y {
            return true;
        };
        return false;
    }
}
fn main() {
    println!("{:?}",Solution::is_palindrome(00));
}
