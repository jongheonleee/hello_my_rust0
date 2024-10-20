// FizzBuzz
// 3의 배수일 때 Fizz, 5의 배수일 때 Buzz, 3과 5의 공배수일 때 FizzBuzz를 출력하게끔 구현

/**
 * - 1~100까지 반복
 *   - (1) 3과 5의 공배수 -> FizzBuzz
 *   - (2) 3의 배수 -> Fizz
 *   - (3) 5의 배수 -> Buzz 
 */

fn main() { // 파이썬은 순차적으로 실행되지만, 러스트는 위치에 상관없이 main()이 가장 만저 실행됨 
    for i in 1 .. 100 {
        if i % 3 == 0 && i% 5 == 0 {
            println!("FizzBuzz"); // 문장 끝에 ; 사용. 하지만 값을 반환해야 할 때는 문장 끝에 세미콜론을 붙이지 않아야함 
        } else if i % 3 == 0 {
            println!("Fizz"); // println! 매크로의 첫 번재 인수로는 문자열로 서식을 지정하고 두 번째 인수부터 값을 지정함 
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}
 