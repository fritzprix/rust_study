# Error Handling in Rust
## Unrecoverable error와 panic
- Panic의 발생
  - 명백한 오류 상황 (divide by zero, accessing array with wrong index, ...)
  - ```panic!()``` 호출

  
- Unwinding Stack vs. Abort
  - Unwinding
    - Default 동작
    - panic 발생 지점에서 call stack을 역추적하며 할당된 자원을 해제
    - 단, 이러한 동작은 불필요하게 많은 동작을 수반하게됨
    - 이러한 문제점의 대안으로써 abort가 대안 동작으로 제공됨
  
  - Abort
    - 즉시 Program을 종료함
  
  - Overriding
    - 기본값은 ```unwind```이며 아래와 같이 ```Cargo.toml```에 빌드 프로파일에 추가함으로써 ```abort```로 변경할 수 있음

```TOML
[profile.release]
panic = 'abort' # abort as panic strategy
```

### Handling panic gracefully
- using ```std::panic::catch_unwind```
  - closure를 실행하고 해당 closure 내부의 panic을 Result로 caller에게 반환함으로써 panic을 recoverable error와 같이 다룰 수 있도록 해줌
  - Limit
    > panic에 대한 상세 정보를 전달하지 않기 때문에 caller에서 panic에 대한 적절한 조치를 구현하는데 한계가 있음

```RUST
fn main() {
    let result = panic_example();
    println!("Result: {}", result);
}

fn panic_example() -> i32 {
    let x = vec![1, 2, 3];
    let result = std::panic::catch_unwind(|| {
        x[99];  // this will cause a panic
        return 4;
    });
    match result {
        Ok(val) => val,
        Err(_) => {
            println!("Caught panic!");
            return 5;
        }
    }
}
```
---
## Recoverable Errors with Result
- ```Result<R,E>```를 통해 Error 정보를 caller에 전달
- 정상 결과 ```Ok(R)``` / Error의 반환 ```Err(E)```
- unwrap(), expect() 등 error 발생 시 panic을 유발하는 함수를 제공 -> Unrecoverable case 대응
- ```match```, ```map()```, ```and_then()``` 등 method를 통해 gracefully error를 처리하는 것을 지원
- ```?``` or ```try!```
  - Result를 return하면서 같은 error type을 갖는 함수의 내부에서 사용
  - 결과가 error일 경우 caller로 해당 error를 전달
  ```rust
  fn multiply(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let a = a.parse::<i32>()?; // 함수의 return과 호환되는 Result를 ? operator를 사용하여 간결하게 표현
    let b = try!(b.parse::<i32>()); // 위와 마찬가지 하지만 deprecated
    Ok(a * b)
  }
  ```