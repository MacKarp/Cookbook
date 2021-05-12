use crate::gui_data::GuiData;

pub mod drink;
pub mod meal;
pub mod stack;

pub fn connect(gui_data: &GuiData) {
    meal::category_tree_selection(&gui_data);
    drink::category_tree_selection(&gui_data);
    stack::stack_tree_selection(&gui_data);
}
