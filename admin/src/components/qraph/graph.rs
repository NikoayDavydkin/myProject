//use gloo_console::info;
use gloo_render::{request_animation_frame, AnimationFrame};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::WebGlProgram;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext as GL};
use yew::prelude::*;
use yew::{html, Component, Context, Html, NodeRef};

static RADIUS: f64 = 0.02;

pub enum Msg {
    Render(f64),
    MouseMove(MouseEvent),
    MouseClick(MouseEvent),
}

pub struct Graph {
    gl: Option<GL>,
    node_ref: NodeRef,
    _render_loop: Option<AnimationFrame>,
    bg_shader_program: Option<WebGlProgram>,
    point_shader_program: Option<WebGlProgram>,
    selected_shader_program: Option<WebGlProgram>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub points: Vec<(f64, f64, Uuid)>,
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
    pub selected: Option<Uuid>,
    pub on_changed: Callback<Option<Uuid>>,
    pub on_hover: Callback<Option<Uuid>>,
}

pub fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    ((p2.0 - p1.0) * (p2.0 - p1.0) + (p2.1 - p1.1) * (p2.1 - p1.1)).sqrt()
}

pub fn scaled_point(point: &(f64, f64, Uuid), ctx: &Context<Graph>) -> (f64, f64) {
    (
        ((point.0 - ctx.props().min_x) / (ctx.props().max_x - ctx.props().min_x) * 2.0 - 1.0),
        ((point.1 - ctx.props().min_y) / (ctx.props().max_y - ctx.props().min_y) * 2.0 - 1.0),
    )
}

fn mouse_event(ctx: &Context<Graph>, event: &MouseEvent, callback: &Callback<Option<Uuid>>) {
    let elem = event
        .target()
        .unwrap()
        .dyn_into::<web_sys::Element>()
        .unwrap();
    let rect = elem.get_bounding_client_rect();
    let x = (event.x() as f64 - rect.x()) / rect.width() * 2.0 - 1.0;
    let y = (1.0 - (event.y() as f64 - rect.y()) / rect.height()) * 2.0 - 1.0;
    //info!(std::format!("Mouse Move: [{}, {}]", x, y));
    let mut selected: Option<usize> = None;
    for index in 0..ctx.props().points.len() {
        if let Some(selected_index) = selected {
            if distance((x, y), scaled_point(&ctx.props().points[index], ctx))
                < distance(
                    (x, y),
                    scaled_point(&ctx.props().points[selected_index], ctx),
                )
            {
                selected = Some(index);
            }
        } else {
            selected = Some(index);
        }
    }
    if let Some(selected) = selected {
        if distance((x, y), scaled_point(&ctx.props().points[selected], ctx)) <= RADIUS {
            callback.emit(Some(ctx.props().points[selected].2));
        } else {
            callback.emit(None);
        }
    }
}

impl Component for Graph {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            gl: None,
            node_ref: NodeRef::default(),
            _render_loop: None,
            bg_shader_program: None,
            point_shader_program: None,
            selected_shader_program: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Render(timestamp) => {
                // Render functions are likely to get quite large, so it is good practice to split
                // it into it's own function rather than keeping it inline in the update match
                // case. This also allows for updating other UI elements that may be rendered in
                // the DOM like a framerate counter, or other overlaid textual elements.
                self.render_gl(timestamp, ctx);
                false
            }
            Msg::MouseMove(event) => {
                mouse_event(ctx, &event, &ctx.props().on_hover);
                true
            }
            Msg::MouseClick(event) => {
                mouse_event(ctx, &event, &ctx.props().on_changed);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <canvas
                width=700
                height=700
                ref={self.node_ref.clone()}
                onmousemove={ctx.link().callback(Msg::MouseMove)}
                onclick={ctx.link().callback(Msg::MouseClick)}
            />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        // Once rendered, store references for the canvas and GL context. These can be used for
        // resizing the rendering area when the window or canvas element are resized, as well as
        // for making GL calls.

        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();

        let gl: GL = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        self.gl = Some(gl);

        // In a more complex use-case, there will be additional WebGL initialization that should be
        // done here, such as enabling or disabling depth testing, depth functions, face
        // culling etc.

        if first_render {
            // The callback to request animation frame is passed a time value which can be used for
            // rendering motion independent of the framerate which may vary.
            let handle = {
                let link = ctx.link().clone();
                request_animation_frame(move |time| link.send_message(Msg::Render(time)))
            };

            // A reference to the handle must be stored, otherwise it is dropped and the render won't
            // occur.
            self._render_loop = Some(handle);
        }
    }
}

impl Graph {
    fn render_gl(&mut self, timestamp: f64, ctx: &Context<Self>) {
        let gl = self.gl.as_ref().expect("GL Context not initialized!");

        let bg_vert_code = include_str!("./solid.vert");
        let bg_frag_code = include_str!("./solid.frag");

        let vertices: Vec<f32> = vec![
            -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        ];
        let vertex_buffer = gl.create_buffer().unwrap();
        let verts = js_sys::Float32Array::from(vertices.as_slice());

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

        let bg_shader_program = match &self.bg_shader_program {
            Some(bg_shader_program) => bg_shader_program,
            _ => {
                let bg_vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
                gl.shader_source(&bg_vert_shader, bg_vert_code);
                gl.compile_shader(&bg_vert_shader);

                let bg_frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
                gl.shader_source(&bg_frag_shader, bg_frag_code);
                gl.compile_shader(&bg_frag_shader);

                let bg_shader_program = gl.create_program().unwrap();
                gl.attach_shader(&bg_shader_program, &bg_vert_shader);
                gl.attach_shader(&bg_shader_program, &bg_frag_shader);
                gl.link_program(&bg_shader_program);

                self.bg_shader_program = Some(bg_shader_program);
                self.bg_shader_program.as_ref().unwrap()
            }
        };

        gl.use_program(Some(bg_shader_program));

        // Attach the position vector as an attribute for the GL context.
        let position = gl.get_attrib_location(bg_shader_program, "a_position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);

        // Attach the time as a uniform for the GL context.
        let time = gl.get_uniform_location(bg_shader_program, "u_time");
        gl.uniform1f(time.as_ref(), timestamp as f32);

        gl.draw_arrays(GL::TRIANGLES, 0, 6);

        let points: Vec<(f64, f64)> = ctx
            .props()
            .points
            .iter()
            .filter(|point| {
                if let Some(selected) = ctx.props().selected {
                    point.2 != selected
                } else {
                    true
                }
            })
            .map(|point| {
                (
                    (point.0 - ctx.props().min_x) / (ctx.props().max_x - ctx.props().min_x) * 2.0
                        - 1.0,
                    (point.1 - ctx.props().min_y) / (ctx.props().max_y - ctx.props().min_y) * 2.0
                        - 1.0,
                )
            })
            .collect();
        let mut vertices: Vec<f32> = Vec::new();
        for point in &points {
            vertices.append(&mut [point.0 as f32, point.1 as f32, -1.0, -1.0].to_vec());
            vertices.append(&mut [point.0 as f32, point.1 as f32, 1.0, -1.0].to_vec());
            vertices.append(&mut [point.0 as f32, point.1 as f32, -1.0, 1.0].to_vec());
            vertices.append(&mut [point.0 as f32, point.1 as f32, -1.0, 1.0].to_vec());
            vertices.append(&mut [point.0 as f32, point.1 as f32, 1.0, -1.0].to_vec());
            vertices.append(&mut [point.0 as f32, point.1 as f32, 1.0, 1.0].to_vec());
        }

        let point_vert_code = include_str!("./point.vert");
        let point_frag_code = include_str!("./point.frag");

        let vertex_buffer = gl.create_buffer().unwrap();
        let verts = js_sys::Float32Array::from(vertices.as_slice());

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

        let point_shader_program = match &self.point_shader_program {
            Some(point_shader_program) => point_shader_program,
            _ => {
                let point_vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
                gl.shader_source(&point_vert_shader, point_vert_code);
                gl.compile_shader(&point_vert_shader);

                let point_frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
                gl.shader_source(&point_frag_shader, point_frag_code);
                gl.compile_shader(&point_frag_shader);

                let point_shader_program = gl.create_program().unwrap();
                gl.attach_shader(&point_shader_program, &point_vert_shader);
                gl.attach_shader(&point_shader_program, &point_frag_shader);
                gl.link_program(&point_shader_program);

                self.point_shader_program = Some(point_shader_program);
                self.point_shader_program.as_ref().unwrap()
            }
        };

        gl.use_program(Some(point_shader_program));

        let position = gl.get_attrib_location(point_shader_program, "a_position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 4, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);

        let radius = gl.get_uniform_location(point_shader_program, "radius");
        gl.uniform1f(radius.as_ref(), RADIUS as f32);

        gl.enable(GL::BLEND);
        gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);

        gl.draw_arrays(GL::TRIANGLES, 0, vertices.len() as i32);

        let points: Vec<(f64, f64)> = ctx
            .props()
            .points
            .iter()
            .filter(|point| {
                if let Some(selected) = ctx.props().selected {
                    point.2 == selected
                } else {
                    false
                }
            })
            .map(|point| {
                (
                    (point.0 - ctx.props().min_x) / (ctx.props().max_x - ctx.props().min_x) * 2.0
                        - 1.0,
                    (point.1 - ctx.props().min_y) / (ctx.props().max_y - ctx.props().min_y) * 2.0
                        - 1.0,
                )
            })
            .collect();
        let mut vertices: Vec<f32> = Vec::new();
        for point in &points {
            vertices.append(&mut [point.0 as f32, point.1 as f32, -1.0, -1.0].to_vec());
            vertices.append(&mut [point.0 as f32, point.1 as f32, 1.0, -1.0].to_vec());
            vertices.append(&mut [point.0 as f32, point.1 as f32, -1.0, 1.0].to_vec());
            vertices.append(&mut [point.0 as f32, point.1 as f32, -1.0, 1.0].to_vec());
            vertices.append(&mut [point.0 as f32, point.1 as f32, 1.0, -1.0].to_vec());
            vertices.append(&mut [point.0 as f32, point.1 as f32, 1.0, 1.0].to_vec());
        }

        let selected_vert_code = include_str!("./selected.vert");
        let selected_frag_code = include_str!("./selected.frag");

        let vertex_buffer = gl.create_buffer().unwrap();
        let verts = js_sys::Float32Array::from(vertices.as_slice());

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

        let selected_shader_program = match &self.selected_shader_program {
            Some(selected_shader_program) => selected_shader_program,
            _ => {
                let selected_vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
                gl.shader_source(&selected_vert_shader, selected_vert_code);
                gl.compile_shader(&selected_vert_shader);

                let selected_frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
                gl.shader_source(&selected_frag_shader, selected_frag_code);
                gl.compile_shader(&selected_frag_shader);

                let selected_shader_program = gl.create_program().unwrap();
                gl.attach_shader(&selected_shader_program, &selected_vert_shader);
                gl.attach_shader(&selected_shader_program, &selected_frag_shader);
                gl.link_program(&selected_shader_program);

                self.selected_shader_program = Some(selected_shader_program);
                self.selected_shader_program.as_ref().unwrap()
            }
        };

        gl.use_program(Some(selected_shader_program));

        let position = gl.get_attrib_location(selected_shader_program, "a_position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 4, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);

        let radius = gl.get_uniform_location(selected_shader_program, "radius");
        gl.uniform1f(radius.as_ref(), RADIUS as f32);

        gl.draw_arrays(GL::TRIANGLES, 0, vertices.len() as i32);

        let handle = {
            let link = ctx.link().clone();
            request_animation_frame(move |time| link.send_message(Msg::Render(time)))
        };

        // A reference to the new handle must be retained for the next render to run.
        self._render_loop = Some(handle);
    }
}
