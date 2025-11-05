## Test 打印日志
```sh
cargo test -- --nocapture
```



### unit test
```sh
cargo t std_mod -- --show-output
```



### 打印sub thread 中的内容
```sh
cargo t std_mod --  --nocapture --show-output
cargo t test_thread --  --nocapture --show-output
```







## debugger

```sh
cargo build
nnd ./target/debug/debuger
```


