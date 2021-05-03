pub mod drink;
pub mod meal;

use crate::gui_data::GuiData;

pub fn connect(gui_data: &GuiData) {
    meal::category_tree_selection(&gui_data);
}
