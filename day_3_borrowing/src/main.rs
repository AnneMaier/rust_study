// 이 함수의 시그니처(함수 선언 부분)를 수정하여 
// 소유권을 가져오지 않고, 원본 데이터를 수정할 수 있게 만들기.
fn change_string(some_string: &mut String) {
    some_string.push_str(", world");
}

fn main() {
    let mut s = String::from("hello"); // s는 가변(mut) String 소유자이다.
    
    // change_string 함수를 호출하는 부분도 수정하여
    // s의 소유권을 이동시키지 않고, s를 빌려주도록 한다..
    change_string(&mut s);

    // 함수 호출 후에도 s를 사용할 수 있어야 한다.
    println!("{}", s); 
}