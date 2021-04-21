//! Mod that includes all about templates

use super::{concrete::ElementNodeArc, element::HtmlHtmlStore};
use crate::internal_prelude::*;

/// Template for building nodes from context
pub trait Template<T> {
    /// Performs the build
    fn build(self, context: Arc<Sandbox>) -> T;
}

impl<T, F: Fn(Arc<Sandbox>) -> T> Template<T> for F {
    fn build(self, context: Arc<Sandbox>) -> T {
        (self)(context)
    }
}

/// Template for html
pub struct HtmlHtmlTemplate;

impl Template<ElementNodeArc> for HtmlHtmlTemplate {
    fn build(self, context: Arc<Sandbox>) -> ElementNodeArc {
        ElementNodeArc::new(
            Arc::downgrade(&context),
            Arc::new(ElementStore::HtmlHtml(HtmlHtmlStore {
                default_view: Arc::downgrade(&context.window()),
            })),
        )
    }
}
