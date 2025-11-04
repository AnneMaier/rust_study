fn main() {
    // 1. 변수 '_name'을 불변 문자열로 선언
    let _name = "Anne";

    //2. 변수 'base_score'을 가변 정수형(i32)로 선언
    let mut base_score = 100;
    
    println!("현재 스코어 : {}",base_score);

    // 3. 'base_score' 값 변경
    base_score = 150;

    // 4. 불변 변수, 'score_multiplier' 선언
    let score_multiplier = 2;

    // 5. 최종 점수 'score'계산
    let score = base_score * score_multiplier;

    // 6. 최종 점수와 이름을 출력.
    println!("플레이어: {}, 최종 점수: {}", _name, score);

}
