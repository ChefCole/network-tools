use druid::
    {widget::{Widget,Flex, Button,Tabs,Label,Split, TextBox,Padding},
    menu::{Menu,MenuItem},
    Data, Lens, WindowDesc, AppLauncher,WindowId,Env, WidgetExt};

#[derive(Clone,Data,Lens)]
struct AppState{
    socket_url:String,
    btn_text:String,
    socket_content:String,
}

fn builder_ui() -> impl Widget<AppState> {
    //首先进行一个大的布局
    Flex::column()
        .with_child(main_ui())
}


//socket客户端界面
fn socket_client_ui() -> impl Widget<AppState> {

    let label = Label::new("socket连接:").padding((0.,0.));

    let text_box = TextBox::new().lens(AppState::socket_url).fix_width(200.).padding((10.,0.));

    let button = Button::new(|data: &AppState, _: &Env| format!("{}", data.btn_text)).on_click(|_,data,_|{
        data.btn_text = "断开".to_string();
    }).fix_width(100.).fix_height(30.).padding((10.,0.));

    let title = Flex::row()
        .with_child(label)
        .with_child(text_box)
        .with_child(button);

    let show_content = TextBox::multiline().lens(AppState::socket_content).disabled_if(|data,_|{true}).fix_width(400.).fix_height(300.);

    Flex::column()
        .with_child(Padding::new(10., title))
        .with_child(show_content)
}

//内容选项部分
fn main_ui() -> impl Widget<AppState> {
    let main_tabs = Tabs::new()
        .with_tab("socket客户端", socket_client_ui())
        .with_tab("待开发", Label::new(""))
        .with_tab("待开发", Label::new(""))
        .with_tab("待开发", Label::new(""))
        .with_tab_index(0);

    let label = Label::new("");

    Split::rows(main_tabs,label).split_point(0.8).draggable(true)
}

//菜单
fn make_menu(_: Option<WindowId>, _state: &AppState, _: &Env) -> Menu<AppState> {
    let mut base = Menu::empty();
    
    let about = MenuItem::new("关于").on_activate(|_ctx,_data,_env|{
        println!("关于");
    });
    base = base.entry(about);
    base 
}


fn main() {
    let win = WindowDesc::new(builder_ui())
        .menu(make_menu)
        .window_size((800.,500.))
        .title("网络工具")
        .resizable(false);

    let app_state = AppState {
        socket_url: "".to_string(),
        btn_text: "连接".to_string(),
        socket_content:"欢迎使用网络工具".to_string(),
    };

    let _ = AppLauncher::with_window(win)
        .launch(app_state);
}
