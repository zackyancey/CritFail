use iced::{
    button, scrollable, Align, Button, Color, Column, Container, Element, HorizontalAlignment,
    Length, Row, Sandbox, Scrollable, Settings, Space, Text, VerticalAlignment,
};

mod expression_box;
mod style;
use expression_box::*;
mod result_box;
use result_box::*;

pub fn run() {
    Window::run(Settings::default());
}

#[derive(Default)]
struct Window {
    view: View,

    /// Entries for roll expressions
    expressions: Vec<ExpressionBox>,
    expressions_scroll: scrollable::State,
    help_scroll: scrollable::State,
    /// Button to add an expression box
    add_button: button::State,
    /// The result of the last roll
    result_box: ResultBox,
    help_button: button::State,

    example_button_1: button::State,
    example_button_2: button::State,
    example_button_3: button::State,
    example_button_4: button::State,
    example_button_5: button::State,
    example_button_6: button::State,
    example_button_7: button::State,
    example_button_8: button::State,
    example_button_9: button::State,
    example_button_10: button::State,
    example_result_main: ResultBox,
    example_result_check: ResultBox,
    example_result_damage: ResultBox,
    example_result_attack: ResultBox,
}

#[derive(Debug, Clone)]
enum Message {
    ExpressionMessage(usize, ExpressionMessage),
    AddPressed,
    ToggleView,
    ExampleRolled(ExampleSection, String),
}

#[derive(Debug, Clone)]
enum ExampleSection {
    Main,
    Check,
    Damage,
    Attack,
}

#[derive(Debug, Clone)]
enum View {
    Main,
    Help,
}
impl Default for View {
    fn default() -> Self {
        View::Main
    }
}

impl Sandbox for Window {
    type Message = Message;

    fn new() -> Self {
        let example_title = "Click roll on the examples above to see results here";
        Window {
            result_box: ResultBox::with_title(
                "Enter something to roll in the boxes below and click 'roll'".into(),
            ),
            example_result_main: ResultBox::with_title(example_title),
            example_result_check: ResultBox::with_title(example_title),
            example_result_damage: ResultBox::with_title(example_title),
            example_result_attack: ResultBox::with_title(example_title),

            ..Default::default()
        }
    }

    fn title(&self) -> String {
        String::from("Critfail")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ExpressionMessage(i, ExpressionMessage::RollPressed) => {
                let expression = &self.expressions[i];
                let result = expression.roll();

                self.result_box
                    .update(ResultMessage::from_roll(&expression, result))
            }
            Message::ExpressionMessage(i, ExpressionMessage::DeletePressed) => {
                self.expressions.remove(i);
            }
            Message::ExpressionMessage(i, msg) => self.expressions[i].update(msg),
            Message::AddPressed => self.expressions.push(ExpressionBox::new()),
            Message::ToggleView => {
                self.view = match self.view {
                    View::Help => View::Main,
                    View::Main => View::Help,
                }
            }
            Message::ExampleRolled(section, expression) => match section {
                ExampleSection::Main => &mut self.example_result_main,
                ExampleSection::Check => &mut self.example_result_check,
                ExampleSection::Damage => &mut self.example_result_damage,
                ExampleSection::Attack => &mut self.example_result_attack,
            }
            .update(ResultMessage::from_example(expression)),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let title_bar = Row::new()
            .push(
                Container::new(
                    Button::new(
                        &mut self.help_button,
                        Text::new(match self.view {
                            View::Main => "?",
                            View::Help => "Back",
                        })
                        .size(25)
                        .width(match self.view {
                            View::Main => Length::Units(25),
                            View::Help => Length::Shrink,
                        })
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center),
                    )
                    .on_press(Message::ToggleView)
                    .style(style::Button::Primary),
                )
                .width(Length::Fill),
            )
            .push(
                Text::new("Critfail")
                    .size(50)
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .push(Space::with_width(Length::Fill));

        let view: Element<_> = match self.view {
            View::Main => {
                if self.expressions.is_empty() {
                    self.expressions.push(ExpressionBox::new())
                }

                let expressions = self.expressions.iter_mut().enumerate().fold(
                    Column::new().spacing(20),
                    |col, (i, exp)| {
                        col.push(
                            exp.view()
                                .map(move |msg| Message::ExpressionMessage(i, msg)),
                        )
                    },
                );

                let add_button = Button::new(
                    &mut self.add_button,
                    Text::new("+")
                        .size(35)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .width(Length::Fill),
                )
                .on_press(Message::AddPressed)
                .style(style::Button::Secondary);

                Column::new()
                    .spacing(20)
                    .align_items(Align::Center)
                    .push(self.result_box.view())
                    .push(
                        Scrollable::new(&mut self.expressions_scroll)
                            .spacing(20)
                            .push(expressions)
                            .push(
                                Row::new()
                                    .push(Space::with_width(Length::Fill))
                                    .push(add_button.width(Length::FillPortion(3)))
                                    .push(Space::with_width(Length::Fill)),
                            ),
                    )
                    .into()
            }
            View::Help => Scrollable::new(&mut self.help_scroll)
                .padding(20)
                .spacing(20)
                .push(style::text::paragraph("Critfail is a dice simulator, designed in particular for D&D 5e. There are 3 kinds of rolls that can be made: Checks, damage, and attacks."))
                .push(example("Check: Roll a d20, add 6", "r+6", &mut self.example_button_1, ExampleSection::Main))
                .push(example("Damage: Roll 2d8 and adds 4", "2d8+4", &mut self.example_button_2, ExampleSection::Main))
                .push(example("Attack: d20+3 to hit, 1d12+3 damage", "r+3?1d12+3", &mut self.example_button_3, ExampleSection::Main))
                .push(self.example_result_main.view())

                .push(style::text::header("Checks"))
                .push(style::text::paragraph("Checks are used for anything where you roll a d20, optionally with modifiers or disadvantage."))
                .push(example("Roll a d20", "r", &mut self.example_button_4, ExampleSection::Check))
                .push(example("Roll a d20 with advantage then add 5", "a+5", &mut self.example_button_5, ExampleSection::Check))
                .push(example("Roll a d20 with disadvantage then add 4 and 1d4", "d+4+1d4", &mut self.example_button_6, ExampleSection::Check))
                .push(self.example_result_check.view())

                .push(style::text::header("Damage"))
                .push(style::text::paragraph("Damage rolls are specified in the usual format. Any number of dice and modifiers can be added"))
                .push(example("A simple damage roll", "2d8+5", &mut self.example_button_7, ExampleSection::Damage))
                .push(example("A more complicated damage roll", "3d12-1d4+6-2", &mut self.example_button_8, ExampleSection::Damage))
                .push(self.example_result_damage.view())

                .push(style::text::header("Attacks"))
                .push(style::text::paragraph("An attack consts of both a check and a damage roll, separated by a '?'. If the check part of an attack rolls a 20, all of the positive dice in the damage part of the roll will be rolled twice. (Modifiers will only be counted once)."))
                .push(example("+3 to hit, 1d8 of damage", "r+3?1d8", &mut self.example_button_9, ExampleSection::Attack))
                .push(example("attack with advantage and +5 to hit, 1d4+4+5d6 of damage", "a+5?1d4+4+5d6", &mut self.example_button_10, ExampleSection::Attack))
                .push(self.example_result_attack.view())
                .into(),
        };

        Container::new(
            Column::new()
                .max_width(1000)
                .align_items(Align::Center)
                .push(title_bar)
                .push(view),
        )
        .padding(20)
        .width(Length::Fill)
        .center_x()
        .into()
    }
}

fn example<'a>(
    description: &str,
    expression: &str,
    button: &'a mut button::State,
    section: ExampleSection,
) -> Element<'a, Message> {
    Column::new()
        .spacing(10)
        .push(Text::new(description).color(Color::from_rgb(0.6, 0.6, 0.6)))
        .push(
            Row::new()
                .push(Text::new(expression).width(Length::Units(300)))
                .push(
                    Button::new(button, Text::new("Roll"))
                        .style(style::Button::Primary)
                        .on_press(Message::ExampleRolled(section, expression.into())),
                ),
        )
        .into()
}
