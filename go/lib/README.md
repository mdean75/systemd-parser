## Commands used for Rust ffi -> Go CGO

### Rust

cbindgen -l c -o ipaddr.h .
cargo build --lib --release

cp ipaddr.h go/lib/
cp target/release/libsystemd_parser.a go/lib/

### Go

cd go
go build -ldflags="-r $(pwd)lib" main.go 
./main