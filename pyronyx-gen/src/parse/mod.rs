pub mod commands;
pub mod enums;
pub mod extensions;
pub mod registry;
pub mod types;

pub fn c_to_rust(ty: &str) -> &str {
    match ty {
        "void*" => "*const c_void",
        "void" => "c_void",
        "char" => "c_char",
        "float" => "f32",
        "double" => "f64",
        "int" => "c_int",
        "unsigned int" => "c_uint",
        "size_t" => "usize",
        "uint8_t" => "u8",
        "uint16_t" => "u16",
        "uint32_t" => "u32",
        "uint64_t" => "u64",
        "int8_t" => "i8",
        "int16_t" => "i16",
        "int32_t" => "i32",
        "int64_t" => "i64",
        "VkDevice" => "vkDevice",
        "VkInstance" => "vkInstance",
        "VkPhysicalDevice" => "vkPhysicalDevice",
        "VkCommandBuffer" => "vkCommandBuffer",
        "VkResult" => "vkResult",
        "VkQueue" => "vkQueue",
        other => other,
    }
}
