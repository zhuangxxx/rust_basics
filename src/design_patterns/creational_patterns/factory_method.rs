trait Button {
    fn render(&self);
    fn on_click(&self, f: Box<dyn Fn()>);
}

#[derive(Default)]
struct WindowsButton;

impl Button for WindowsButton {
    fn render(&self) {
        println!("展示一个Windows窗体的对话框。");
    }
    fn on_click(&self, f: Box<dyn Fn()>) {
        println!("Windows窗体对话框点击后触发：");
        f();
    }
}

#[derive(Default)]
struct HtmlButton;

impl Button for HtmlButton {
    fn render(&self) {
        println!("展示一个浏览器Web页面的对话框。");
    }
    fn on_click(&self, f: Box<dyn Fn()>) {
        println!("浏览器Web页面对话框点击后触发：");
        f();
    }
}

trait Dialog {
    fn create_button(&self) -> Box<dyn Button>;
    fn render_window(&self) {
        let button = self.create_button();
        button.render();
        button.on_click(Box::new(|| {
            println!("关闭对话框。");
        }));
    }
}

#[derive(Default)]
struct WindowsDialog;

impl Dialog for WindowsDialog {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton::default())
    }
}

#[derive(Default)]
struct HtmlDialog;

impl Dialog for HtmlDialog {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(HtmlButton::default())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Windows,
    WebBrowser,
}

pub struct Application {
    #[allow(unused)]
    platform: Platform,
    dialog: Box<dyn Dialog>,
}

impl Application {
    pub fn new(platform: Platform) -> Self {
        Self {
            platform,
            dialog: if platform == Platform::Windows {
                Box::new(WindowsDialog::default())
            } else {
                Box::new(HtmlDialog::default())
            },
        }
    }

    pub fn render_window(&self) {
        self.dialog.render_window();
    }
}

#[test]
fn test_factory_method() {
    println!("==============");
    Application::new(Platform::Windows).render_window();
    println!("==============");
    Application::new(Platform::WebBrowser).render_window();
    println!("==============");
}
