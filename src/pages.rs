use crate::grocery_shop::catalog::Category;
use crate::pages::entry_page_state::EntryPageState;


pub mod entry_page_state;
pub mod category_page_state;
pub mod catalog_page_state;


pub enum Page {
    EntryPage(EntryPageState),
    CategoryPage,
    CatalogPage(Category),
}