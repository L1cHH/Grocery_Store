use crate::pages::catalog_page_state::CatalogPageState;
use crate::pages::category_page_state::CategoryPageState;
use crate::pages::entry_page_state::EntryPageState;
use crate::pages::purchase_page_state::PurchasePageState;

pub mod entry_page_state;
pub mod category_page_state;
pub mod catalog_page_state;
pub mod purchase_page_state;

pub enum Page {
    EntryPage(EntryPageState),
    CategoryPage(CategoryPageState),
    CatalogPage(CatalogPageState),
    PurchasePage(PurchasePageState),
}