fn main() {
    let reference_to_nothing = no_dangle();

    println!("{}", reference_to_nothing);
}

/* dangling pointer: 아래 함수는 참조자를 반환하는데 지역 변수 s는 함수 종료와
동시에 drop되므로 반환된 참조자가 유효하지 않은 메모리 공간을 가리키게 된다.
변수 값 반환 시 함수 호출자로의 소유권 이동이 발생하는데 참조자가 반환하는 경우
지역 변수의 소유권 이동이 발생하지 않으므로 변수 값은 함수 스코프를 벗어나지 못한다.
따라서 함수가 종료되면 내부 변수가 drop되고 참조자 dangle이 발생하게 된다.
이러한 상황이 발생하지 않게 컴파일러가 검사하여 막고 에러를 출력한다.

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
*/

// 아래 함수는 변수 값을 반환하므로 함수 호출자로의 소유권 이동이 이뤄진다.
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
