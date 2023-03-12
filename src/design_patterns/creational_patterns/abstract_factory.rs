trait Button {
    fn paint(&self);
    fn click(&self, f: Box<dyn Fn()>);
}

#[derive(Default)]
struct WinButton;

impl Button for WinButton {
    fn paint(&self) {
        println!("展示一个Windows系统下的按钮。");
    }
    fn click(&self, f: Box<dyn Fn()>) {
        println!("按钮点击后触发：");
        f();
    }
}

#[derive(Default)]
struct MacButton;

impl Button for MacButton {
    fn paint(&self) {
        println!("展示一个Mac系统下的按钮。");
    }
    fn click(&self, _f: Box<dyn Fn()>) {
        println!("Mac鼠标没电了，充电时无法使用。");
        // f();
    }
}

trait Checkbox {
    fn paint(&self);
    fn check(&mut self);
}

#[derive(Default)]
struct WinCheckbox {
    checked: bool,
}

impl Checkbox for WinCheckbox {
    fn paint(&self) {
        println!("展示一个Windows系统下的复选框。");
    }
    fn check(&mut self) {
        self.checked = !self.checked;
        if self.checked {
            println!("复选框被选中");
        } else {
            println!("复选框取消选中");
        }
    }
}

#[derive(Default)]
struct MacCheckbox {
    checked: bool,
}

impl Checkbox for MacCheckbox {
    fn paint(&self) {
        println!("展示一个Mac系统下的复选框。");
    }
    fn check(&mut self) {
        self.checked = !self.checked;
        if self.checked {
            println!("复选框被选中。");
        } else {
            println!("复选框取消选中。");
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum System {
    Windows,
    Mac,
}

trait GuiFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}

#[derive(Default)]
struct WinFactory;

impl GuiFactory for WinFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WinButton::default())
    }
    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WinCheckbox::default())
    }
}

#[derive(Default)]
struct MacFactory;

impl GuiFactory for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton::default())
    }
    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacCheckbox::default())
    }
}

pub struct Application {
    #[allow(unused)]
    system: System,
    gui_factory: Box<dyn GuiFactory>,
    button: Option<Box<dyn Button>>,
    checkbox: Option<Box<dyn Checkbox>>,
}

impl Application {
    pub fn new(system: System) -> Self {
        Self {
            system,
            gui_factory: if system == System::Mac {
                Box::new(MacFactory::default())
            } else {
                Box::new(WinFactory::default())
            },
            button: None,
            checkbox: None,
        }
    }

    pub fn render_window(&mut self) {
        self.button = Some(self.gui_factory.create_button());
        self.button.iter().for_each(|button| button.paint());
        self.checkbox = Some(self.gui_factory.create_checkbox());
        self.checkbox.iter().for_each(|checkbox| checkbox.paint());
    }

    pub fn click(&self) {
        self.button.iter().for_each(|button| {
            button.click(Box::new(|| {
                println!("确认。");
            }));
        });
    }

    pub fn check(&mut self) {
        self.checkbox.iter_mut().for_each(|checkbox| {
            checkbox.check();
        });
    }
}

#[test]
fn test_abstract_factory() {
    println!("==============");
    let mut win_app = Application::new(System::Windows);
    win_app.render_window();
    win_app.click();
    win_app.check();
    win_app.check();
    println!("==============");
    let mut mac_app = Application::new(System::Mac);
    mac_app.render_window();
    mac_app.click();
    mac_app.check();
    mac_app.check();
    println!("==============");
}
