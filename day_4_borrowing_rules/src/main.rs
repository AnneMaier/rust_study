fn main() {
    let mut s = String::from("hello"); // s는 가변(mut) String 소유자입니다.

    // 1. 첫 번째 참조: s에 대한 불변 참조(read_only)를 생성하세요.
    let r1 = &s;

    // 2. 두 번째 참조: s에 대한 가변 참조(write_only)를 생성하세요. (이것이 오류의 원인)
    // let r2 = &mut s;

    // 3. 읽기 전용 참조를 사용하려고 시도합니다. (이때 컴파일러가 오류를 낼 것입니다.)
    println!("불변 참조로 읽기: {}", r1); 

    // 4. 가변 참조를 사용하려고 시도합니다.

    r2.push_str(" world");
}