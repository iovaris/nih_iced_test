use atomic_float::AtomicF32;
use nih_plug::prelude::{util, Editor, GuiContext};
use nih_plug::nih_log;
use nih_plug_iced::button::State;
use nih_plug_iced::widget::image::Handle;
use nih_plug_iced::widgets as nih_widgets;
use nih_plug_iced::widget::{Button, Image};
use nih_plug_iced::*;
use std::sync::Arc;
use std::time::Duration;

use crate::GainParams;

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(600, 50)
}

pub(crate) fn create(
    params: Arc<GainParams>,
    peak_meter: Arc<AtomicF32>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<GainEditor>(editor_state, (params, peak_meter))
}

struct GainEditor {
    params: Arc<GainParams>,
    context: Arc<dyn GuiContext>,

    peak_meter: Arc<AtomicF32>,

    gain_slider_state: nih_widgets::param_slider::State,
    peak_meter_state: nih_widgets::peak_meter::State,
    edit_button_state: button::State,
    preset_state: text_input::State,
    preset_value: String
}

#[derive(Debug, Clone)]
enum Message {
    /// Update a parameter's value.
    ParamUpdate(nih_widgets::ParamMessage),
    OpenEditor,
    PresetChanged(String),
}

fn update_text_input(val: String) -> Message {
    nih_log!("asdf {}", val);
    Message::PresetChanged("blah".to_string())
}

impl IcedEditor for GainEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = (Arc<GainParams>, Arc<AtomicF32>);

    fn new(
        (params, peak_meter): Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = GainEditor {
            params,
            context,

            peak_meter,

            gain_slider_state: Default::default(),
            peak_meter_state: Default::default(),
            edit_button_state: Default::default(),
            preset_state: Default::default(),
            preset_value: "default string".to_string()
        };


        (editor, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        _window: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        match message {
            Message::ParamUpdate(message) => self.handle_param_message(message),

            Message::OpenEditor => {
                nih_log!("Button pressed!");
            }

            Message::PresetChanged(new_value) => {
                nih_log!("Text Input: {:#?}", &new_value);
                self.preset_value = new_value
            }
        }

        Command::none()
    }


    fn view(&mut self) -> Element<'_, Self::Message> {

        let main_bar = Row::new()
            .align_items(Alignment::Center)
            .push(Space::with_width(10.into()))
            .push(
                Button::new(&mut self.edit_button_state, Text::new("Edit"))
                    .on_press(Message::OpenEditor)
                    .height(30.into())
            )
            .push(
                TextInput::new(&mut self.preset_state, "placeholder", &self.preset_value, update_text_input)
                    .padding(5)
            )
            .push(
                Text::new("TEXT")
                    .font(assets::NOTO_SANS_BOLD)
                    .size(40)
                    .color(nih_plug_iced::Color::WHITE)
                    .height(50.into())
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center),
            );

        Column::new()
            .align_items(Alignment::Center)
            .push(main_bar)
            // .push(
            //     Text::new("Gain")
            //         .height(20.into())
            //         .width(Length::Fill)
            //         .horizontal_alignment(alignment::Horizontal::Center)
            //         .vertical_alignment(alignment::Vertical::Center),
            // )
            // .push(
            //     nih_widgets::ParamSlider::new(&mut self.gain_slider_state, &self.params.gain)
            //         .map(Message::ParamUpdate),
            // )
            // .push(Space::with_height(10.into()))
            // .push(
            //     nih_widgets::PeakMeter::new(
            //         &mut self.peak_meter_state,
            //         util::gain_to_db(self.peak_meter.load(std::sync::atomic::Ordering::Relaxed)),
            //     )
            //     .hold_time(Duration::from_millis(600)),
            // )
            .into()
    }

    fn background_color(&self) -> nih_plug_iced::Color {
        nih_plug_iced::Color {
            r: 0.2,
            g: 0.2,
            b: 0.2,
            a: 1.0,
        }
    }
}
