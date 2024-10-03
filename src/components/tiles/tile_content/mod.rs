pub mod bomb_content;
pub mod number_content;
pub mod undug_content;

pub fn get_generic_classes() -> Vec<&'static str> {
    vec![
        "w-5",
        "h-5",
        "my-1",
        "rounded",
        "flex",
        "justify-center",
        "items-center",
    ]
}
