use crate::{GIT_VERSION, VERSION};
use critfail::AdvState;
use iced::{
    button, scrollable, Align, Button, Column, Container, Element, HorizontalAlignment, Length,
    Row, Sandbox, Scrollable, Settings, Space, Text, VerticalAlignment,
};

mod style;

mod examples;
mod expression_box;
mod result_box;

use examples::*;
use expression_box::*;
use result_box::*;

pub fn run() {
    Window::run(Settings::default());
}

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

    examples_main: ExampleGroup,
    examples_check: ExampleGroup,
    examples_damage: ExampleGroup,
    examples_attack: ExampleGroup,

    shameless_plug_button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    ExpressionMsg(usize, ExpressionMsg),
    AddPressed,
    ToggleView,
    ExampleRolled(SectionId, usize, Option<AdvState>),
    OpenGitHub,
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
        Self {
            result_box: ResultBox::with_title(
                "Enter something to roll in the boxes below and click 'roll'",
            ),

            examples_main: ExampleGroup::new(SectionId::Main)
                .push(Example::new("Check: Roll a d20, add 6", "r+6"))
                .push(Example::new("Damage: Roll 2d8 and adds 4", "2d8+4"))
                .push(Example::new(
                    "Attack: d20+3 to hit, 1d12+3 damage",
                    "r+3?1d12+3",
                )),
            examples_check: ExampleGroup::new(SectionId::Check)
                .push(Example::new("Roll a d20", "r"))
                .push(Example::new("Roll a d20 with advantage then add 5", "a+5"))
                .push(Example::new(
                    "Roll a d20 with disadvantage then add 4 and 1d4",
                    "d+4+1d4",
                )),
            examples_damage: ExampleGroup::new(SectionId::Damage)
                .push(Example::new("A simple damage roll", "2d8+5"))
                .push(Example::new(
                    "A more complicated damage roll",
                    "3d12-1d4+6-2",
                )),
            examples_attack: ExampleGroup::new(SectionId::Attack)
                .push(Example::new("+3 to hit, 1d8 of damage", "r+3?1d8"))
                .push(Example::new(
                    "attack with advantage and +5 to hit, 1d4+4+5d6 of damage",
                    "a+5?1d4+4+5d6",
                )),

            view: Default::default(),
            expressions: Default::default(),
            expressions_scroll: Default::default(),
            help_scroll: Default::default(),
            add_button: Default::default(),
            help_button: Default::default(),
            shameless_plug_button: Default::default(),
        }
    }

    fn title(&self) -> String {
        String::from("Critfail")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ExpressionMsg(i, ExpressionMsg::RollPressed(adv)) => {
                let expression = &self.expressions[i];
                let result = expression.roll(adv);

                self.result_box
                    .update(ResultMessage::from_roll(&expression, result))
            }
            Message::ExpressionMsg(i, ExpressionMsg::DeletePressed) => {
                self.expressions.remove(i);
            }
            Message::ExpressionMsg(i, msg) => self.expressions[i].update(msg),
            Message::AddPressed => self.expressions.push(ExpressionBox::new()),
            Message::ToggleView => {
                self.view = match self.view {
                    View::Help => View::Main,
                    View::Main => View::Help,
                }
            }
            Message::ExampleRolled(section, i, adv) => match section {
                SectionId::Main => &mut self.examples_main,
                SectionId::Check => &mut self.examples_check,
                SectionId::Damage => &mut self.examples_damage,
                SectionId::Attack => &mut self.examples_attack,
            }
            .update(ExampleGroupMessage::Roll(i, adv)),
            Message::OpenGitHub => open_url(env!("CARGO_PKG_REPOSITORY")),
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
                                .map(move |msg| Message::ExpressionMsg(i, msg)),
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
                .push(self.examples_main.view())

                .push(style::text::header("Checks"))
                .push(style::text::paragraph("Checks are used for anything where you roll a d20, optionally with modifiers or disadvantage."))
                .push(self.examples_check.view())

                .push(style::text::header("Damage"))
                .push(style::text::paragraph("Damage rolls are specified in the usual format. Any number of dice and modifiers can be added"))
                .push(self.examples_damage.view())

                .push(style::text::header("Attacks"))
                .push(style::text::paragraph("An attack consts of both a check and a damage roll, separated by a '?'. If the check part of an attack rolls a 20, all of the positive dice in the damage part of the roll will be rolled twice. (Modifiers will only be counted once)."))
                .push(self.examples_attack.view())

                .push(style::text::header("About"))
                .push(Text::new(format!("Critfail v{}-{}", VERSION, GIT_VERSION)))
                .push(Button::new(&mut self.shameless_plug_button, Text::new("View on GitHub")).on_press(Message::OpenGitHub).style(style::Button::Primary))
                .into(),
        };

        Container::new(
            Column::new()
                .max_width(800)
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

fn open_url(url: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window().map(|w| w.open_with_url(url));
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        match webbrowser::open(url) {
            _ => (),
        };
    }
}
