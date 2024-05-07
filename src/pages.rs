use crate::pages::catalog_page_state::CatalogPageState;
use crate::pages::category_page_state::CategoryPageState;
use crate::pages::entry_page_state::EntryPageState;
use crate::pages::purchase_page_state::PurchasePageState;

mod entry_page_state;
mod category_page_state;
mod catalog_page_state;
mod purchase_page_state;

enum Page {
    EntryPage(EntryPageState),
    CategoryPage(CategoryPageState),
    CatalogPage(CatalogPageState),
    PurchasePage(PurchasePageState),
}