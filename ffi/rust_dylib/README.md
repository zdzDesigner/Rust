## rust compile cdylib


- `crate-type`: 生成库类型
```toml
[lib]
crate-type = ["cdylib","staticlib"] # 生成两种类型(动态库、静态库)
```




```sh
# 配置link_path, 遵循`lib`规则, 优先链接`动态库`(编译完成，无法运行, 动态库非标准路径, 必须在运行时指定库路径)
gcc -o dytest ./test/dy1.c -L ./target/debug -lrust_dylib

# -Wl,-rpath 指定运行时的库查询路径(可以运行)
gcc -o dytest ./test/dy1.c -L ./target/debug -lrust_dylib -Wl,-rpath,./target/debug/
ldd dytest
        linux-vdso.so.1 (0x00007ffdc8d35000)
        librust_dylib.so => ./target/debug/librust_dylib.so (0x00007bada31ce000)
        libc.so.6 => /usr/lib/libc.so.6 (0x00007bada2faf000)
        libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x00007bada2f81000)
        /lib64/ld-linux-x86-64.so.2 => /usr/lib64/ld-linux-x86-64.so.2 (0x00007bada3238000)


# 直接link librust_dylib.a(可以运行)
gcc -o dytest ./test/dy1.c ./target/debug/librust_dylib.a 
ldd dytest
        linux-vdso.so.1 (0x00007ffe20138000)
        libc.so.6 => /usr/lib/libc.so.6 (0x000078d9efad1000)
        libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x000078d9efcc2000)
        /lib64/ld-linux-x86-64.so.2 => /usr/lib64/ld-linux-x86-64.so.2 (0x000078d9efe23000)


```


