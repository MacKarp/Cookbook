use gtk::prelude::*;
#[derive(Clone, PartialEq, Debug)]
pub struct MainWindowText {
    pub recipe_name_text_buffer: gtk::TextBuffer,
    pub recipe_ingredients_text_buffer: gtk::TextBuffer,
    pub recipe_text_buffer: gtk::TextBuffer,
}

impl MainWindowText {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let recipe_name_text_buffer: gtk::TextBuffer = builder
            .get_object("recipe_name_text_buffer")
            .expect("\"recipe_name_text_buffer\" ID in \"Main_Window.glade\" should exist.");
        let recipe_ingredients_text_buffer: gtk::TextBuffer = builder
            .get_object("recipe_ingredients_text_buffer")
            .expect("\"recipe_ingredients_text_buffer\" ID in \"Main_Window.glade\" should exist.");
        let recipe_text_buffer: gtk::TextBuffer = builder
            .get_object("recipe_text_buffer")
            .expect("\"recipe_text_buffer\" ID in \"Main_Window.glade\" should exist.");

        recipe_name_text_buffer.set_text(get_recipe_name());
        recipe_ingredients_text_buffer.set_text(get_recipe_ingredients());
        recipe_text_buffer.set_text(get_recipe_text());

        Self {
            recipe_name_text_buffer,
            recipe_ingredients_text_buffer,
            recipe_text_buffer,
        }
    }
}

fn get_recipe_name() -> &'static str {
    "Home-made Mandazi"
}
fn get_recipe_ingredients() -> &'static str {
    "750g Self-raising Flour
        6 tablespoons Sugar
        2 Eggs
        1 cup Milk"
}

fn get_recipe_text() -> &'static str {
    "This is one recipe a lot of people have requested and I have tried to make it as simple as possible and I hope it will work for you. Make sure you use the right flour which is basically one with raising agents. Adjust the amount of sugar to your taste and try using different flavours to have variety whenever you have them. You can use Coconut milk instead of regular milk, you can also add desiccated coconut to the dry flour or other spices like powdered cloves or cinnamon. For “healthy looking” mandazis do not roll the dough too thin before frying and use the procedure I have indicated above.\n1. Mix the flour,cinnamon and sugar in a suitable bowl.\n2. In a separate bowl whisk the egg into the milk\n3. Make a well at the centre of the flour and add the milk and egg mixture and slowly mix to form a dough.\n4. Knead the dough for 3-4 minutes or until it stops sticking to the sides of the bowl and you have a smooth surface.\n5. Cover the dough with a damp cloth  and allow to rest for 15 minutes.\n6. Roll the dough on a lightly floured surface into a 1cm thick piece.\n7. Using a sharp small knife, cut the dough into the desired size setting aside ready for deep frying.\n8. Heat your oil in a suitable pot and gently dip the mandazi pieces to cook until light brown on the first side then turn to cook on the second side.\n9. Serve them warm or cold"
}
#[test]
fn get_recipe_name_test() {
    assert_eq!("Home-made Mandazi", get_recipe_name());
}

#[test]
fn get_recipe_ingredients_test() {
    assert_eq!(
        "750g Self-raising Flour
        6 tablespoons Sugar
        2 Eggs
        1 cup Milk",
        get_recipe_ingredients()
    );
}

#[test]
fn get_recipe_text_test() {
    assert_eq!("This is one recipe a lot of people have requested and I have tried to make it as simple as possible and I hope it will work for you. Make sure you use the right flour which is basically one with raising agents. Adjust the amount of sugar to your taste and try using different flavours to have variety whenever you have them. You can use Coconut milk instead of regular milk, you can also add desiccated coconut to the dry flour or other spices like powdered cloves or cinnamon. For “healthy looking” mandazis do not roll the dough too thin before frying and use the procedure I have indicated above.\n1. Mix the flour,cinnamon and sugar in a suitable bowl.\n2. In a separate bowl whisk the egg into the milk\n3. Make a well at the centre of the flour and add the milk and egg mixture and slowly mix to form a dough.\n4. Knead the dough for 3-4 minutes or until it stops sticking to the sides of the bowl and you have a smooth surface.\n5. Cover the dough with a damp cloth  and allow to rest for 15 minutes.\n6. Roll the dough on a lightly floured surface into a 1cm thick piece.\n7. Using a sharp small knife, cut the dough into the desired size setting aside ready for deep frying.\n8. Heat your oil in a suitable pot and gently dip the mandazi pieces to cook until light brown on the first side then turn to cook on the second side.\n9. Serve them warm or cold"
        , get_recipe_text());
}

#[test]
fn create_from_builder_test() {
    gtk::init().expect("Failed to initialize GTK...");
    let glade_src = include_str!("../../../ui/Cookbook.glade").to_string();
    let builder = gtk::Builder::from_string(glade_src.as_str());

    let recipe_name_text_buffer: gtk::TextBuffer = builder
        .get_object("recipe_name_text_buffer")
        .expect("\"recipe_name_text_buffer\" ID in \"Main_Window.glade\" should exist.");
    let recipe_ingredients_text_buffer: gtk::TextBuffer = builder
        .get_object("recipe_ingredients_text_buffer")
        .expect("\"recipe_ingredients_text_buffer\" ID in \"Main_Window.glade\" should exist.");
    let recipe_text_buffer: gtk::TextBuffer = builder
        .get_object("recipe_text_buffer")
        .expect("\"recipe_text_buffer\" ID in \"Main_Window.glade\" should exist.");

    let should_be = MainWindowText {
        recipe_name_text_buffer,
        recipe_ingredients_text_buffer,
        recipe_text_buffer,
    };

    let tested = MainWindowText::create_from_builder(&builder);

    assert_eq!(tested, should_be);
}
