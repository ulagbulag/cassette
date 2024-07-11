use std::fmt;

use yew::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum OpenTarget {
    Blank,
}

impl fmt::Display for OpenTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl OpenTarget {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Blank => "_blank",
        }
    }
}

#[hook]
pub fn use_open<IN>(url: impl Into<String>, target: OpenTarget) -> Callback<IN, ()>
where
    IN: 'static,
{
    use_callback((url.into(), target.as_str()), |_, (url, target)| {
        let _ = gloo_utils::window().open_with_url_and_target(url, target);
    })
}
