//! Mod that includes all about templates

use crate::internal_prelude::*;
use crate::node::concrete::ElementNodeArc;
use crate::node::element::{HtmlBodyStore, HtmlButtonStore, HtmlElementStore, HtmlHtmlStore};

use super::element::HtmlUnknownStore;

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

impl TemplateWeak<ElementNodeArc> for HtmlHtmlTemplate {
    fn build(self, context: Weak<Sandbox>) -> ElementNodeArc {
        ElementNodeArc::new(
            context,
            Arc::new(ElementStore::HtmlElement(HtmlElementStore::HtmlHtml(
                HtmlHtmlStore,
            ))),
        )
    }
}

/// Template for body
pub struct HtmlBodyTemplate;

impl TemplateWeak<ElementNodeArc> for HtmlBodyTemplate {
    fn build(self, context: Weak<Sandbox>) -> ElementNodeArc {
        ElementNodeArc::new(
            context,
            Arc::new(ElementStore::HtmlElement(HtmlElementStore::HtmlBody(
                HtmlBodyStore,
            ))),
        )
    }
}

/// Template for button
pub struct HtmlButtonTemplate;

impl TemplateWeak<ElementNodeArc> for HtmlButtonTemplate {
    fn build(self, context: Weak<Sandbox>) -> ElementNodeArc {
        ElementNodeArc::new(
            context,
            Arc::new(ElementStore::HtmlElement(HtmlElementStore::HtmlButton(
                HtmlButtonStore,
            ))),
        )
    }
}

/// Template for unknown
pub struct HtmlUnknownTemplate(pub String);

impl TemplateWeak<ElementNodeArc> for HtmlUnknownTemplate {
    fn build(self, context: Weak<Sandbox>) -> ElementNodeArc {
        ElementNodeArc::new(
            context,
            Arc::new(ElementStore::HtmlElement(HtmlElementStore::HtmlUnknown(
                HtmlUnknownStore { tag_name: self.0 },
            ))),
        )
    }
}
