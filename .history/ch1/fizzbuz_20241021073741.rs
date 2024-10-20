// FizzBuzz
// 3의 배수일 때 Fizz, 5의 배수일 때 Buzz, 3과 5의 공배수일 때 FizzBuzz를 출력하게끔 구현

/**
 * - 1~100까지 반복
 *   - (1) 3과 5의 공배수 -> FizzBuzz
 *   - (2) 3의 배수 -> Fizz
 *   - (3) 5의 배수 -> Buzz 
 */

fn main() {
    for i in 1 .. 100 {
        if i % 3 == 0 && i% 5 == 0 {
            println!("FizzBuzz");
        } 
        
        else if i % 3 == 0 {
            println!("Fizz");
        } 

        else if i % 5 == 0 {
            println!("Buzz");
        }

        else {
            println!("{}", i);
        }
    }
}
 