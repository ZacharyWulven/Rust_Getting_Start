pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Vec 里的元素是实现了 Draw trait 的类型
    //pub components: Vec<Box<dyn Draw>>,
    pub components: Vec<Box<dyn Clone>>,

}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("绘制一个 Button");
    }
}