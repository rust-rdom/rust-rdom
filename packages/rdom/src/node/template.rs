//! Mod that includes all about templates

use crate::internal_prelude::*;
use crate::node::concrete::ElementNodeArc;
use crate::node::element::{HtmlBodyStore, HtmlButtonStore, HtmlHtmlStore};

/// Template for building nodes from context
pub trait TemplateArc<T> {
    /// Performs the build
    fn build(self, context: Arc<Sandbox>) -> T;
}

impl<T, F> TemplateArc<T> for F
where
    F: Fn(Arc<Sandbox>) -> T,
{
    fn build(self, context: Arc<Sandbox>) -> T {
        (self)(context)
    }
}

/// Template for building nodes from
/// weak ptr to context
pub trait TemplateWeak<T> {
    /// Performs the build
    fn build(self, context: Weak<Sandbox>) -> T;
}

impl<T, F> TemplateWeak<T> for F
where
    F: Fn(Weak<Sandbox>) -> T,
{
    fn build(self, context: Weak<Sandbox>) -> T {
        (self)(context)
    }
}

/// Template for html
pub struct HtmlHtmlTemplate;

impl TemplateArc<ElementNodeArc> for HtmlHtmlTemplate {
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

impl TemplateWeak<ElementNodeArc> for HtmlBodyTemplate {
    fn build(self, context: Weak<Sandbox>) -> ElementNodeArc {
        ElementNodeArc::new(context, Arc::new(ElementStore::HtmlBody(HtmlBodyStore)))
    }
}

/// Template for button
pub struct HtmlButtonTemplate;

impl TemplateWeak<ElementNodeArc> for HtmlButtonTemplate {
    fn build(self, context: Weak<Sandbox>) -> ElementNodeArc {
        ElementNodeArc::new(context, Arc::new(ElementStore::HtmlButton(HtmlButtonStore)))
    }
}
