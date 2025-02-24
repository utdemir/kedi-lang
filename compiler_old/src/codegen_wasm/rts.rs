pub static OBJECT_TYPE_ID: u32 = 0;

pub static OBJECT_TAG_I32: i32 = 1;

pub fn object_val_type() -> wasm_encoder::ValType {
    wasm_encoder::ValType::Ref(wasm_encoder::RefType {
        nullable: false,
        heap_type: wasm_encoder::HeapType::Concrete(OBJECT_TYPE_ID),
    })
}

pub fn object_field_type() -> wasm_encoder::FieldType {
    wasm_encoder::FieldType {
        element_type: wasm_encoder::StorageType::Val(object_val_type()),
        mutable: false,
    }
}
