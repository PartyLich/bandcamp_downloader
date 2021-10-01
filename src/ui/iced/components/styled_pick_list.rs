//! Pick list widget with app styling
use std::borrow::Cow;

use iced::PickList;

use crate::ui::{iced::components::TEXT_SIZE, iced::Message};

/// Returns a styled Picklist
pub fn styled_pick_list<'a, F, T>(
    state: &'a mut iced::pick_list::State<T>,
    options: impl Into<Cow<'a, [T]>>,
    selected: Option<T>,
    message: F,
) -> PickList<'a, T, Message>
where
    F: 'static + Fn(T) -> Message,
    T: ToString + Eq,
    [T]: ToOwned<Owned = Vec<T>>,
{
    PickList::new(state, options, selected, message).text_size(TEXT_SIZE)
}
