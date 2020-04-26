//! This module defines the element that is used for entering attacks.
use crate::gui::style;
use critfail::{Roll, RollExpression, RollOutcome};
use iced::{
    button, text_input, Align, Button, Column, Element, Length, Row, Text, TextInput,
    VerticalAlignment,
};

#[derive(Debug, Clone)]
/// Messages that can be sent by an expression box.
pub(super) enum ExpressionMsg {
    /// The Roll button has been pressed.
    RollPressed,
    /// The delete button has been pressed.
    DeletePressed,
    /// The value of the name box has been changed.
    NameChanged(String),
    /// The value of the roll expression box has been changed.
    RollChanged(String),
}

#[derive(Default)]
pub(super) struct ExpressionBox {
    /// A text input that holds the name of the attack
    name_box: text_input::State,
    name: String,
    /// A text input that holds the attack string
    roll_box: text_input::State,
    roll: String,
    /// A button to delete this expression box
    delete_button: button::State,
    /// A button to roll the attack
    roll_button: button::State,
}

const TEXTENTRY_SIZE: u16 = 40;

impl ExpressionBox {
    pub(super) fn new() -> Self {
        ExpressionBox::default()
    }

    pub(super) fn update(&mut self, message: ExpressionMsg) {
        match message {
            ExpressionMsg::RollPressed => panic!("RollPressed should be handled upstream"),
            ExpressionMsg::DeletePressed => panic!("DeletePressed should be handled upstream"),
            ExpressionMsg::NameChanged(name) => self.name = name,
            ExpressionMsg::RollChanged(roll) => self.roll = roll,
        }
    }

    pub(super) fn view(&mut self) -> Element<ExpressionMsg> {
        let roll_box = TextInput::new(
            &mut self.roll_box,
            "Dice",
            &self.roll,
            ExpressionMsg::RollChanged,
        )
        .size(TEXTENTRY_SIZE)
        .on_submit(ExpressionMsg::RollPressed);

        let name_box = TextInput::new(
            &mut self.name_box,
            "Description",
            &self.name,
            ExpressionMsg::NameChanged,
        )
        .size(TEXTENTRY_SIZE);

        let roll_button = Button::new(
            &mut self.roll_button,
            Text::new("Roll")
                .size(TEXTENTRY_SIZE)
                .vertical_alignment(VerticalAlignment::Center),
        )
        .height(Length::Units(TEXTENTRY_SIZE * 2))
        .padding(20)
        .style(style::Button::Primary)
        .on_press(ExpressionMsg::RollPressed);

        let delete_button = Button::new(
            &mut self.delete_button,
            Text::new("x")
                .vertical_alignment(VerticalAlignment::Center)
                .size(25),
        )
        .style(style::Button::Secondary)
        .on_press(ExpressionMsg::DeletePressed);

        Row::new()
            .spacing(20)
            .align_items(Align::Start)
            .push(delete_button)
            .push(
                Column::new()
                    .width(Length::Fill)
                    .push(name_box)
                    .push(roll_box),
            )
            .push(roll_button)
            .height(Length::Shrink)
            .into()
    }

    pub(super) fn name(&self) -> &str {
        &self.name
    }

    pub(super) fn expression(&self) -> &str {
        &self.roll
    }

    pub(super) fn roll(&self) -> Result<RollOutcome, String> {
        let rollexp = Roll::new(&self.roll).map_err(|err| format!("{}", err))?;
        Ok(rollexp.roll())
    }
}
