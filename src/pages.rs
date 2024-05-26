use crate::grocery_shop::catalog::Category;
use crate::pages::catalog_page_state::CatalogPageState;
use crate::pages::category_page_state::CategoryPageState;
use crate::pages::entry_page_state::EntryPageState;


pub mod entry_page_state;
pub mod category_page_state;
pub mod catalog_page_state;


pub enum Page {
    EntryPage(EntryPageState),
    CategoryPage(CategoryPageState),
    CatalogPage(CatalogPageState, Category),
}