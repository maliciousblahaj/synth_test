use iced::{advanced::{graphics::core::{event, touch}, layout, mouse, renderer::Quad, Shell, Widget}, border::Radius, Background, Border, Color, Element, Event, Length, Rectangle, Shadow, Size};

use super::core::{knob_angle_range::KnobAngleRange, math, normal::Normal, normal_param::NormalParam, slider_status::SliderStatus};

const RADIUS: f32 = 64.0;
const BACKGROUND_COLOR: Color = Color::from_rgb(0.97, 0.97, 0.97);
const DEFAULT_SCALAR: f32 = 0.00385;
const BORDER_COLOR: Color = Color::from_rgb(0.315, 0.315, 0.315);
const BORDER_WIDTH: f32 = 0.5;
const NOTCH_COLOR: Color = BORDER_COLOR;

const NOTCH_RELATIVE_DIAMETER: f32 = 0.17;
const NOTCH_RELATIVE_OFFSET: f32 = 0.15;

pub struct Knob<Message> {
    normal_param: NormalParam,
    on_change: Box<dyn Fn(Normal) -> Message>,
    scalar: f32,

    radius: f32,
    background_color: Color,
    notch_color: Color,
}

impl<Message> Knob<Message> {
    pub fn new<F: 'static + Fn(Normal) -> Message>(normal_param: NormalParam, on_change: F) -> Self {
        Self {
            normal_param,
            on_change: Box::new(on_change),
            scalar: DEFAULT_SCALAR,

            radius: RADIUS,
            background_color: BACKGROUND_COLOR,
            notch_color: NOTCH_COLOR,
        }
    }

    fn move_virtual_slider(&mut self, state: &mut KnobState, normal_delta: f32) -> SliderStatus {
        if normal_delta.abs() < f32::EPSILON {
            return SliderStatus::Unchanged;
        }

        //why isn't it current_normal + normaldelta??
        self.normal_param.value.set_clipped(state.current_normal.as_f32() - normal_delta);
        state.current_normal = self.normal_param.value;

        SliderStatus::Moved
    }

    ///will run the on_change function
    fn fire_on_change(&self, shell: &mut Shell<'_, Message>) {
        shell.publish((self.on_change)(self.normal_param.value));
    }
}

struct KnobState {
    dragging_status: Option<SliderStatus>,
    previous_click: Option<mouse::Click>,

    previous_drag_y: f32,
    previous_normal: Normal,

    current_normal: Normal,
}

impl KnobState {
    fn new(normal: Normal) -> Self {
        Self {
            dragging_status: None,
            previous_click: None,
            previous_drag_y: 0.0,
            previous_normal: normal,
            current_normal: normal
        }
    }
}


impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Knob<Message>
where 
    Renderer: iced::advanced::Renderer,
{
    //it should stay as a fixed size and not fill the area
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    //basically how big it's going to be
    fn layout(
        &self,
        _tree: &mut iced::advanced::widget::Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(self.radius*2.0, self.radius*2.0))
    }

    fn draw(
        &self,
        _tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        let angle_range = KnobAngleRange::default();

        let layout_bounds = layout.bounds();
        let bounds = {
            let bounds = Rectangle {
                x: layout_bounds.x.round(),
                y: layout_bounds.y.round(),
                width: layout_bounds.width.round(),
                height: layout_bounds.height.round(),
            };

            if bounds.width == bounds.height {
                bounds
            } else if bounds.width > bounds.height {
                Rectangle {
                    x: (bounds.x + (bounds.width - bounds.height) / 2.0)
                        .round(),
                    y: bounds.y,
                    width: bounds.height,
                    height: bounds.height,
                }
            } else {
                Rectangle {
                    x: bounds.x,
                    y: (bounds.y + (bounds.height - bounds.width) / 2.0)
                        .round(),
                    width: bounds.width,
                    height: bounds.width,
                }
            }
        };

        let radius = bounds.width / 2.0;

        let start_angle =
            if angle_range.min() >= math::THREE_HALVES_PI {
                angle_range.min() - math::THREE_HALVES_PI
            } else {
                angle_range.min() + std::f32::consts::FRAC_PI_2
            };
        let angle_span = angle_range.max() - angle_range.min();
        let value_angle = start_angle + (self.normal_param.value.scale(angle_span));
    
        let knob_back_quad = Quad {
            bounds,
            border: Border {
                color: BORDER_COLOR,
                width: BORDER_WIDTH,
                radius: Radius::new(radius),
            },
            shadow: Shadow::default(),
        };
        //draw knob
        renderer.fill_quad(knob_back_quad, Background::Color(BACKGROUND_COLOR));
    

        //draw notch
        let value_angle = value_angle + std::f32::consts::FRAC_PI_2;

        let (dx, dy) = if !(-0.001..=0.001).contains(&value_angle) {
            value_angle.sin_cos()
        } else {
            (0.0, -1.0)
        };

        let notch_diameter = bounds.width * NOTCH_RELATIVE_DIAMETER;
        let notch_radius = notch_diameter / 2.0;

        let offset_radius = radius - NOTCH_RELATIVE_OFFSET * bounds.width;

        let border = Border {
            radius: Radius::new(notch_radius),
            width: BORDER_WIDTH,
            color: BORDER_COLOR,
        };

        let quad = Quad {
            bounds: Rectangle {
                x: bounds.center_x() + (dx * offset_radius) - notch_radius,
                y: bounds.center_y() - (dy * offset_radius) - notch_radius,
                width: notch_diameter,
                height: notch_diameter,
            },
            //background: Background::Color(style.color),
            border: border,
            shadow: Shadow::default(),
        };
        renderer.fill_quad(quad, Background::Color(BACKGROUND_COLOR));
    }

    fn on_event(
            &mut self,
            state: &mut iced::advanced::widget::Tree,
            event: iced::Event,
            layout: layout::Layout<'_>,
            cursor: iced::advanced::mouse::Cursor,
            _renderer: &Renderer,
            _clipboard: &mut dyn iced::advanced::Clipboard,
            shell: &mut iced::advanced::Shell<'_, Message>,
            _viewport: &iced::Rectangle,
        ) -> iced::advanced::graphics::core::event::Status 
    {   
        //get the state from the widget tree
        let state = state.state.downcast_mut::<KnobState>();

        //if the normals don't match, update to the newest one
        if state.dragging_status.is_none() 
            && state.previous_normal != self.normal_param.value 
        {
            state.previous_normal = self.normal_param.value;
            //state.current_normal = self.normal_param.value;
        }

        match event {
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                if state.dragging_status.is_some() {
                    let cursor_y_pos = cursor.position().and_then(|p| Some(p.y)).unwrap_or(state.previous_drag_y);
                    let normal_delta = (cursor_y_pos - state.previous_drag_y) * self.scalar;

                    state.previous_drag_y = cursor_y_pos;

                    if self.move_virtual_slider(state, normal_delta) == SliderStatus::Moved {
                        self.fire_on_change(shell);

                        state.dragging_status.as_mut().unwrap().set_moved();
                    }

                    return event::Status::Captured;
                }
            },
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let cursor_position = cursor.position().expect("failed to get cursor position");
                if layout.bounds().contains(cursor_position) {
                    let click = mouse::Click::new(cursor_position, state.previous_click);

                    match click.kind() {
                        mouse::click::Kind::Single => {
                            state.dragging_status = Some(Default::default());
                            state.previous_drag_y = cursor_position.y;
                        },
                        _ => {
                            //if right click, reset to default
                            let _previous_dragging_status = state.dragging_status.take();
                            
                            if self.normal_param.value != self.normal_param.default {
                                self.normal_param.value = self.normal_param.default;
                                self.fire_on_change(shell);
                            }
                        }
                    }
                    state.previous_click = Some(click);
                    return event::Status::Captured;
                }
            },
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                if let Some(_slider_status) = state.dragging_status.take() {
                    return event::Status::Captured;
                }
            }
            _ => ()
        }
        event::Status::Ignored
    }
}


impl<'a, Message: 'a, Theme, Renderer> From<Knob<Message>> for Element<'a, Message, Theme, Renderer> 
    where Renderer: iced::advanced::Renderer,
{
    fn from(knob: Knob<Message>) -> Self {
        Element::new(knob)
    }
}

/*    fn from(
        knob: Knob<'a, Message, Theme>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(knob)
    } */