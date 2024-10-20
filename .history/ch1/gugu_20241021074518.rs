/**
 * > 구구단 
 * 
 * - 첫 번째 for 문으로 1~9까지 반복(행)
 * - 두 번째 for 문으로 1~9까지 반복(열)
 *  - 각 행에 1~9까지 열의 값을 곱한 값을 반환함
 * 
 */

fn main() {

    for (i in 1 .. 10) {
        for (j in 1 .. 10) {
            print!("{}, ", (i*j));
        }
        println!();
    }
}