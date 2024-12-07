use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use gpui::*;
use std::fs;

struct Assets {
    base: PathBuf,
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        fs::read(self.base.join(path))
            .map(|data| Some(std::borrow::Cow::Owned(data)))
            .map_err(|e| e.into())
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|e| e.into())
    }
}

#[derive(IntoElement)]
struct ImageContainer {
    src: ImageSource,
}

impl ImageContainer {
    pub fn new( src: impl Into<ImageSource>) -> Self {
        Self {
            src: src.into(),
        }
    }
}

impl RenderOnce for ImageContainer {
    fn render(self, _: &mut WindowContext) -> impl IntoElement {
        div().child(
            div()
                .flex_row()
                .size_full()
                .gap_4()
                .child(img(self.src).w(px(256.0)).h(px(256.0))),
        )
    }
}

struct ImageShowcase {
    local_resource: Arc<std::path::Path>,
}

impl Render for ImageShowcase {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .gap_8()
            .bg(rgb(0xFFFFFF))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_center()
                    .items_center()
                    .gap_8()
                    .child(ImageContainer::new(
                        self.local_resource.clone(),
                    ))
            )
    }
}

actions!(image, [Quit]);

fn main() {
    env_logger::init();

    App::new()
        .with_assets(Assets {
            base: PathBuf::from("test_images"),
        })
        .run(|cx: &mut AppContext| {
            cx.activate(true);
            cx.on_action(|_: &Quit, cx| cx.quit());
            cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
            cx.set_menus(vec![Menu {
                name: "Image".into(),
                items: vec![MenuItem::action("Quit", Quit)],
            }]);

            let window_options = WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::from("Chitra Viewer")),
                    appears_transparent: false,
                    ..Default::default()
                }),

                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    size: size(px(1100.), px(600.)),
                    origin: Point::new(px(200.), px(200.)),
                })),

                ..Default::default()
            };

            cx.open_window(window_options, |cx| {
                cx.new_view(|_cx| ImageShowcase {
                    // Relative path to your root project path
                    local_resource: PathBuf::from_str("test_images/krypton.jpg")
                        .unwrap()
                        .into(),
                })
            })
            .unwrap();
        });
}
