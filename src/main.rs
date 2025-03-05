use cranberry::Scheme;
use glib::clone;
use gtk::prelude::*;
use gtk::{ Button, ComboBoxText, HeaderBar, Label, ScrolledWindow, TextView, Window, WindowType };
use std::cell::RefCell;
use std::rc::Rc;
use gtk::Clipboard;

fn main() {
    gtk::init().expect("Failed to initialize GTK");

    let main_window = Window::new(WindowType::Toplevel);
    main_window.set_title("Cranberry");
    main_window.set_default_size(800, 600);

    let header_bar = HeaderBar::new();
    header_bar.set_show_close_button(true);
    header_bar.set_title(Some("Cranberry"));

    main_window.set_titlebar(Some(&header_bar));

    let main_grid = gtk::Grid::new();
    main_grid.set_column_homogeneous(true);
    main_grid.set_row_homogeneous(false);
    main_grid.set_vexpand(true);
    main_grid.set_hexpand(true);
    main_grid.set_margin_top(16);
    main_grid.set_margin_bottom(16);
    main_grid.set_margin_start(16);
    main_grid.set_margin_end(16);
    main_grid.set_row_spacing(16);
    main_grid.set_column_spacing(16);
    main_window.add(&main_grid);

    let create_text_area = || {
        let scrolled_window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        scrolled_window.set_vexpand(true);
        scrolled_window.set_hexpand(true);
        let text_view = TextView::new();
        text_view.set_wrap_mode(gtk::WrapMode::WordChar);
        scrolled_window.add(&text_view);
        (scrolled_window, text_view)
    };

    let (input_area, input_text_view) = create_text_area();
    let (output_area, output_text_view) = create_text_area();
    output_text_view.set_editable(false);

    let input_control_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let paste_button = Button::from_icon_name(Some("edit-paste-symbolic"), gtk::IconSize::Button);
    let character_count_label = Label::new(None);
    input_control_box.pack_start(&paste_button, false, false, 0);
    input_control_box.pack_end(&character_count_label, false, false, 0);

    let output_control_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let copy_button = Button::from_icon_name(Some("edit-copy-symbolic"), gtk::IconSize::Button);

    let scheme_combo_box = ComboBoxText::new();
    scheme_combo_box.append_text("Cranberry");
    scheme_combo_box.append_text("Soviet 1");
    scheme_combo_box.append_text("Soviet 2");
    scheme_combo_box.append_text("Soviet 3");
    scheme_combo_box.append_text("ISO 1954");
    scheme_combo_box.append_text("ISO 1968 Base");
    scheme_combo_box.append_text("ISO 1968 Alt 1");
    scheme_combo_box.append_text("ISO 1968 Alt 2");
    scheme_combo_box.append_text("ISO 1995");
    scheme_combo_box.append_text("ALALC");
    scheme_combo_box.set_active(Some(0));

    output_control_box.pack_start(&scheme_combo_box, false, false, 0);
    output_control_box.pack_end(&copy_button, false, false, 0);

    main_grid.attach(&input_area, 0, 1, 1, 1);
    main_grid.attach(&output_area, 1, 1, 1, 1);
    main_grid.attach(&input_control_box, 0, 2, 1, 1);
    main_grid.attach(&output_control_box, 1, 2, 1, 1);

    let transliteration_engine = Rc::new(RefCell::new(Scheme::Cranberry.init()));

    let update_transliteration = {
        let engine_copy = transliteration_engine.clone();
        let input_text = input_text_view.clone();
        let output_text = output_text_view.clone();
        let char_count_label = character_count_label.clone();

        move || {
            let buffer = input_text.buffer().unwrap();
            let (start, end) = buffer.bounds();
            let input_text_string = buffer.text(&start, &end, false).unwrap().to_string();
            let translated_text = engine_copy.borrow().process(&input_text_string);

            char_count_label.set_text(&format!("{} ch.", input_text_string.chars().count()));
            output_text.buffer().unwrap().set_text(&translated_text);
        }
    };

    {
        let update_transliteration_copy = update_transliteration.clone();
        scheme_combo_box.connect_changed(move |combo| {
            let selected_scheme = match combo.active() {
                Some(0) => Scheme::Cranberry,
                Some(1) => Scheme::Soviet1,
                Some(2) => Scheme::Soviet2,
                Some(3) => Scheme::Soviet3,
                Some(4) => Scheme::ISO1954,
                Some(5) => Scheme::ISO1968Base,
                Some(6) => Scheme::ISO1968Alt1,
                Some(7) => Scheme::ISO1968Alt2,
                Some(8) => Scheme::ISO1995,
                Some(9) => Scheme::ALALC,
                _ => Scheme::Cranberry,
            };
            *transliteration_engine.borrow_mut() = selected_scheme.init();
            update_transliteration_copy();
        });
    }

    input_text_view.buffer().unwrap().connect_changed(clone!(@strong update_transliteration => move |_| {
        update_transliteration();
    }));

    paste_button.connect_clicked(clone!(@strong input_text_view => move |_| {
        let clipboard = Clipboard::default(&gtk::gdk::Display::default().unwrap()).unwrap();
        if let Some(text) = clipboard.wait_for_text() {
            input_text_view.buffer().unwrap().set_text(&text);
        }
    }));

    copy_button.connect_clicked(clone!(@strong output_text_view => move |_| {
        let clipboard = Clipboard::default(&gtk::gdk::Display::default().unwrap()).unwrap();
        let buffer = output_text_view.buffer().unwrap();
        let (start, end) = buffer.bounds();
        clipboard.set_text(&buffer.text(&start, &end, false).unwrap());
    }));

    main_window.show_all();
    main_window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

