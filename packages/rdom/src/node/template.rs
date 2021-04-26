//! Mod that includes all about templates

use crate::internal_prelude::*;
use crate::node::concrete::ElementNodeArc;
use crate::node::element::{HtmlBodyStore, HtmlButtonStore, HtmlHtmlStore};

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

/// Template for body
pub struct HtmlBodyTemplate;

impl Template<ElementNodeArc> for HtmlBodyTemplate {
    fn build(self, context: Arc<Sandbox>) -> ElementNodeArc {
        ElementNodeArc::new(
            Arc::downgrade(&context),
            Arc::new(ElementStore::HtmlBody(HtmlBodyStore)),
        )
    }
}

/// Template for button
pub struct HtmlButtonTemplate;

impl Template<ElementNodeArc> for HtmlButtonTemplate {
    fn build(self, context: Arc<Sandbox>) -> ElementNodeArc {
        ElementNodeArc::new(
            Arc::downgrade(&context),
            Arc::new(ElementStore::HtmlButton(HtmlButtonStore)),
        )
    }
}
