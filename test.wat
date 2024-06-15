(module
  (type (;0;) (func (param i32) (result i32)))
  (func (;0;) (type 0) (param $local#0 i32) (result i32)
    (local $single_use#1 i32) (local $local#1 i32)
    i64.const 99
    local.set $single_use#1
    local.get $single_use#1
    local.set $local#1
    local.get $local#0
    return
  )
  (export "id" (func 0))
)
