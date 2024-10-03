pub mod bomb_content;
pub mod number_content;
pub mod undug_content;

pub fn get_generic_classes() -> Vec<&'static str> {
    vec![
        "w-7 sm:w-8 md:w-9 lg:w-10 xl:w-15",
        "h-7 sm:h-8 md:h-9 lg:h-10 xl:h-15",
        "my-1",
        "rounded",
        "flex",
        "justify-center",
        "items-center",
    ]
}
