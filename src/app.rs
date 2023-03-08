//use gloo::{self, console}; //, file};
use lazy_static::lazy_static;
use rfd::AsyncFileDialog;
//use std::fs::File;
//use std::io::{BufReader, Empty, Read};
//use std::path::Path;
use std::str;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::{mpsc::channel, Mutex};
//use std::{thread, time};
use wasm_bindgen::JsValue;
//use web_sys::console;
use web_sys::console::log_1;

type StaticChanPair = Mutex<(Sender<Vec<u8>>, Receiver<Vec<u8>>)>;

lazy_static! {
    static ref MYCHAN: StaticChanPair = Mutex::new(channel());
}

//use eframe::{egui,epi};
//use std::io::prelude::*;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    #[serde(skip)]
    result: String,
    // this how you opt-out of serialization of a member
    //#[serde(skip)]
}

const TAPE_SIZE: usize = 30000;

fn interpret(contents: String) -> String {
    let mut tape = [0; TAPE_SIZE];
    let mut tape_ptr = 0;
    let mut code_ptr = 0;

    let mut output = String::new();

    while code_ptr < contents.len() {
        let instruction = contents.chars().nth(code_ptr).unwrap();
        match instruction {
            '>' => tape_ptr += 1,
            '<' => tape_ptr -= 1,
            '+' => tape[tape_ptr] += 1,
            '-' => tape[tape_ptr] -= 1,
            '.' => output.push(tape[tape_ptr] as char), //print!("{}", tape[tape_ptr] as char),
            ',' => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                tape[tape_ptr] = input.chars().next().unwrap() as u8;
            }
            '[' => {
                if tape[tape_ptr] == 0 {
                    let mut nest_level = 1;
                    while nest_level > 0 {
                        code_ptr += 1;
                        let char = contents.chars().nth(code_ptr).unwrap();
                        if char == '[' {
                            nest_level += 1;
                        }
                        if char == ']' {
                            nest_level -= 1;
                        }
                    }
                }
            }
            ']' => {
                if tape[tape_ptr] != 0 {
                    let mut nest_level = 1;
                    while nest_level > 0 {
                        code_ptr -= 1;
                        let char = contents.chars().nth(code_ptr).unwrap();
                        if char == ']' {
                            nest_level += 1;
                        }
                        if char == '[' {
                            nest_level -= 1;
                        }
                    }
                }
            }
            _ => {
                output = "Invalid file".to_string();
                break;
            }
        }
        code_ptr += 1;
    }

    output
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "+".to_owned(),
            result: "".to_owned(),
            //value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label, result } = self; //, value } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Super basic Rust/WASM Brain* Interpreter");

            if ui.button("Upload BF File").clicked() {
                let _contents = label.clone();

                let task = AsyncFileDialog::new().pick_file();
                wasm_bindgen_futures::spawn_local(async move {
                    let txclone = MYCHAN.lock().unwrap().0.clone();
                    let file = task.await;
                    if let Some(file) = file {
                        log_1(&"gotfile".into());
                        // If you care about wasm support you just read() the file
                        let mystring = file.read().await;
                        txclone.send(mystring).unwrap();
                    } else {
                        log_1(&"not file".into());
                    }
                });
            }

            if ui.button("Run BF File").clicked() {
                if let Ok(msg) = MYCHAN.lock().unwrap().1.try_recv() {
                    let stmsg = String::from_utf8(msg).unwrap();
                    *result = interpret(stmsg);
                    let jsmsg: JsValue = result.clone().into();
                    log_1(&jsmsg);
                    log_1(&"something".into());
                };
            }

            

            //ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("Interpreter Output:");
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label(result.as_str());
            });
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
