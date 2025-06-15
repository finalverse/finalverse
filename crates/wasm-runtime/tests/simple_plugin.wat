(module
  (import "env" "log" (func $log (param i32 i32)))
  (import "env" "read_u8" (func $read (param i32) (result i32)))
  (import "env" "write_u8" (func $write (param i32 i32)))
  (memory (export "memory") 1)
  (data (i32.const 0) "hello from wasm")
  (func (export "on_event") (param i64)
    i32.const 0
    i32.const 15
    call $log))
