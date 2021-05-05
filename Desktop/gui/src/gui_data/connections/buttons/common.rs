use gdk_pixbuf::Pixbuf;
use gio::{MemoryInputStream, NONE_CANCELLABLE};

pub fn get_recipe_ingredients(ingredients: &[String]) -> String {
    let mut list = String::new();
    for ingredient in ingredients {
        list += &(ingredient.clone() + "\n");
    }
    list
}

pub fn get_recipe_image(thumb: &str) -> Pixbuf {
    let result = reqwest::blocking::get(thumb).unwrap();
    let bytes = result.bytes().unwrap().to_vec();
    let bytes = glib::Bytes::from(&bytes.to_vec());
    let stream = MemoryInputStream::from_bytes(&bytes);
    Pixbuf::from_stream(&stream, NONE_CANCELLABLE).unwrap()
}
