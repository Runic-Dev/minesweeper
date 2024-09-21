pub mod bomb_content;
pub mod number_content;
pub mod undug_content;

pub fn get_generic_styles() -> Vec<&'static str> {
    vec![
        "w-10",
        "h-10",
        "my-1",
        "rounded",
        "flex",
        "justify-center",
        "items-center",
    ]
}
