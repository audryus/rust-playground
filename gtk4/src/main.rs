use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{glib, Align, Application, ApplicationWindow, Box as GBox, Button, Entry, Label, Orientation};

// Exemplo pequeno de GTK4 em Rust, recriando o skald-example (Odin).
//
// GTK é retido (widgets vivem e são mutados), não imediato como o Skald.
// Mesmo assim dá pra manter a "arquitetura Elm" na mão:
//   State  -> os dados da app
//   Msg    -> tudo que o usuário pode fazer
//   update -> (estado antigo, msg) -> estado novo
//   render -> estado -> sincroniza os widgets (em vez de reconstruir a árvore)

const APP_ID: &str = "dev.playground.Gtk4Example";

#[derive(Clone, Default)]
struct State {
    name: String,
    waves: u32,
}

enum Msg {
    NameChanged(String),
    Wave,
    Unwave,
    Reset,
}

fn update(mut s: State, m: Msg) -> State {
    match m {
        Msg::NameChanged(name) => s.name = name,
        Msg::Wave => s.waves += 1,
        Msg::Unwave => s.waves = s.waves.saturating_sub(1),
        Msg::Reset => s = State::default(),
    }
    s
}

// Os widgets que dependem do estado.
struct Widgets {
    greeting: Label,
    entry: Entry,
}

fn render(s: &State, w: &Widgets) {
    let who = if s.name.is_empty() { "estranho" } else { &s.name };
    w.greeting.set_text(&format!("Olá, {who}! 👋 x{}", s.waves));

    // Só mexe no Entry se mudou de fora (ex.: Reset), senão o cursor pula.
    if w.entry.text() != s.name {
        w.entry.set_text(&s.name);
    }
}

// Estado + widgets compartilhados entre os callbacks.
struct App {
    state: RefCell<State>,
    widgets: Widgets,
}

impl App {
    fn dispatch(&self, m: Msg) {
        let new = update(self.state.borrow().clone(), m);
        *self.state.borrow_mut() = new.clone();
        // O borrow já foi solto: render pode disparar sinais (ex.: `changed`
        // do Entry) que voltam a chamar dispatch sem dar pânico no RefCell.
        render(&new, &self.widgets);
    }
}

const CSS: &str = "
.title    { font-size: 14pt; opacity: 0.6; }
.greeting { font-size: 32pt; font-weight: bold; }
";

fn build_ui(gtk_app: &Application) {
    // Tema escuro, como o `skald.theme_dark()`.
    if let Some(settings) = gtk::Settings::default() {
        settings.set_gtk_application_prefer_dark_theme(true);
    }

    let provider = gtk::CssProvider::new();
    provider.load_from_string(CSS);
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("sem display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let title = Label::builder().label("GTK4 — Exemplo").css_classes(["title"]).build();
    let greeting = Label::builder().css_classes(["greeting"]).build();
    let entry = Entry::builder()
        .placeholder_text("Seu nome...")
        .width_request(320)
        .halign(Align::Center)
        .build();

    let unwave = Button::builder().label("−").width_request(64).build();
    let reset = Button::builder().label("Reset").width_request(96).build();
    let wave = Button::builder()
        .label("Acenar")
        .width_request(120)
        .css_classes(["suggested-action"]) // cor "primary" do tema
        .build();

    let buttons = GBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .halign(Align::Center)
        .build();
    buttons.append(&unwave);
    buttons.append(&reset);
    buttons.append(&wave);

    let root = GBox::builder()
        .orientation(Orientation::Vertical)
        .valign(Align::Center)
        .halign(Align::Center)
        .margin_top(32)
        .margin_bottom(32)
        .margin_start(32)
        .margin_end(32)
        .build();
    root.append(&title);
    root.append(&spacer(8));
    root.append(&greeting);
    root.append(&spacer(32));
    root.append(&entry);
    root.append(&spacer(16));
    root.append(&buttons);

    let app = Rc::new(App {
        state: RefCell::new(State::default()),
        widgets: Widgets { greeting, entry: entry.clone() },
    });

    // Cada sinal vira uma Msg. Cada closure guarda um Rc<App> (o App vive
    // enquanto a janela existir, então o ciclo widget -> closure -> App é ok).
    let a = app.clone();
    entry.connect_changed(move |e| a.dispatch(Msg::NameChanged(e.text().into())));
    let a = app.clone();
    wave.connect_clicked(move |_| a.dispatch(Msg::Wave));
    let a = app.clone();
    unwave.connect_clicked(move |_| a.dispatch(Msg::Unwave));
    let a = app.clone();
    reset.connect_clicked(move |_| a.dispatch(Msg::Reset));

    render(&app.state.borrow(), &app.widgets);

    let window = ApplicationWindow::builder()
        .application(gtk_app)
        .title("GTK4 Example")
        .default_width(640)
        .default_height(420)
        .child(&root)
        .build();
    window.present();
}

fn spacer(height: i32) -> GBox {
    GBox::builder().height_request(height).build()
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
