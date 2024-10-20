/**
 * > 구구단 
 * 
 * - 첫 번째 for 문으로 1~9까지 반복(행)
 * - 두 번째 for 문으로 1~9까지 반복(열)
 *  - 각 행에 1~9까지 열의 값을 곱한 값을 반환함
 * 
 */

fn main() {

    // 1차 답안 
    for i in 1..10 {
        for j in 1..10 {
            print!("{:3}, ", i * j); // "{:3}"과 같이 작성하면 문자열은 오른쪽 정렬함 
        }

        println!();
    }

    // 2차 답안 
    for i in 1..10 {
        let s = (1..10)
                .map(|j| format!("{:3}", i * j))
                .collect::<Vec<String>>()
                .join(",");

        println!("{}", s);
    }
}