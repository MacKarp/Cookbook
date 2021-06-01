use gdk_pixbuf::{InterpType, Pixbuf};
use gio::MemoryInputStream;
use gio::NONE_CANCELLABLE;

pub fn get_user_image(img: Option<String>) -> Pixbuf {
    match img {
        Some(s) => {
            let result = reqwest::blocking::get(s).unwrap();
            let bytes = result.bytes().unwrap().to_vec();
            let bytes = glib::Bytes::from(&bytes.to_vec());
            let stream = MemoryInputStream::from_bytes(&bytes);
            Pixbuf::from_stream(&stream, NONE_CANCELLABLE).unwrap()
        }
        None => {
            let pix = Pixbuf::from_file("gui/ui/default_user.png").unwrap();
            pix.scale_simple(70, 70, InterpType::Bilinear).unwrap()
        }
    }
}

#[test]
fn get_user_image_none_test() {
    gtk::init().expect("Failed to initialize GTK...");

    let should_be = Pixbuf::from_file("gui/ui/default_user.png").unwrap();
    let should_be = should_be
        .scale_simple(70, 70, InterpType::Bilinear)
        .unwrap();
    let should_be = unsafe { should_be.get_pixels() };
    let tested = get_user_image(None);
    let tested = unsafe { tested.get_pixels() };

    assert_eq!(should_be, tested);
}
