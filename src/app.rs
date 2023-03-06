//use gloo::{self, console}; //, file};
use std::fs::File;
use std::io::{BufReader, Read, Empty};
use std::str;
use std::path::Path;
use std::sync::mpsc::{Receiver, TryRecvError};
use rfd::AsyncFileDialog;
use wasm_bindgen::JsValue;
use web_sys::console;
use web_sys::console::log_1;
use std::sync::{Arc,Mutex,mpsc::channel};
use std::sync::mpsc::Sender;
use std::{thread, time};
use lazy_static::lazy_static;

lazy_static!{
    static ref MYCHAN: Mutex<(Sender<Vec<u8>>,Receiver<Vec<u8>>)> = Mutex::new(channel());
}

//use eframe::{egui,epi};
//use std::io::prelude::*;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
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
            _ => (),
        }
        code_ptr += 1;
    }

    output
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: ">++++++++++++++++++++++++++++++++++++++++++++++++<++++++++++[>.+<-]".to_owned(),
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

            ui.horizontal(|ui| {
                ui.label("Enter some Brain* code: ");
                ui.text_edit_singleline(label);
            });
            
            //let mut result = String::new();
            static mut CHECKED: bool = false;
            

            if ui.button("Run").clicked() {
                let contents = label.clone();
                
                unsafe {
                    CHECKED = true;
                }
                //let txclone = tx.clone();
                //let mut answer:Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![]));
                //let answerb = answer.clone();
                let task = rfd::AsyncFileDialog::new().pick_file();
                wasm_bindgen_futures::spawn_local(async move{
                    //let mut answerclone = answer.clone();
                    //let mut stringy = answerclone.lock().unwrap();
                    let txclone = MYCHAN.lock().unwrap().0.clone();
                    let file = task.await;
                    if let Some(file) = file {
                        log_1(&"gotfile".into());
                        // If you care about wasm support you just read() the file
                        let mystring = file.read().await;
                        txclone.send(mystring).unwrap();
                        //*stringy = mystring.clone();//= file.read().await.into();
                        //let mystring = String::from_utf8(file.read().await).unwrap();
                        
                        //let js: JsValue = String::from_utf8(mystring).unwrap().into();
                        //log_1(&"inner stuff".into());
                        //log_1(&js);      
                        //drop(txclone);
                    } else {
                        log_1(&"not file".into());
                    }
               });

            //    if let Ok(msg) = rx.recv() {
            //     //let jsmsg: JsValue = String::from_utf8(msg).unwrap().into();
            //     log_1(&"something".into());
            //     //log_1(&jsmsg);
            //    } else {
            //     log_1(&"oops".into());
            //    }

               *result = interpret(contents);

                //let tmp = answerb.lock().unwrap();
                // let myvec = match tmp {
                //     Ok(vec) => {
                //         log_1(&"vector".into());
                //         let mtx = *vec.lock().unwrap();
                //         mtx
                //     },
                //     Err(arc) => {
                //         let vect: Vec<u8> = vec![];
                //         log_1(&"arc".into());
                //         vect
                //     },
                // };
                //let myvec = tmp.to_vec();//(&*tmp.lock().unwrap()).to_vec();
                //log_1(&"tried some stuff".into());
                //let js: JsValue = String::from_utf8(myvec).unwrap().into();
                //log_1(&js);
                //*result = String::from_utf8(myvec).unwrap();
                
                //if let Some(picked_path) = picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace("lol");
                });
                //}
            }

            //let mut stringprint = String::new();

            unsafe {
                if CHECKED {
                    log_1(&"checkum".into());
                    match MYCHAN.lock().unwrap().1.try_recv() {
                        Ok(msg) => {
            
                            let stmsg = String::from_utf8(msg).unwrap();
                            *result = stmsg.clone();
                            let jsmsg: JsValue = stmsg.into();
                            log_1(&jsmsg);
                            log_1(&"something".into());
                            //log_1(&jsmsg);
                            CHECKED = false;
                        },
                        Err(TryRecvError::Empty) => log_1(&"no".into()),
                        Err(TryRecvError::Disconnected) => log_1(&"no2".into()),
                    }        
                }
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
            //});
        });

        // match RXG.try_recv() {
        //     Ok(msg) => {
            
        //     //let jsmsg: JsValue = String::from_utf8(msg).unwrap().into();
        //     //log_1(&msg.to_string().into());
        //     log_1(&"something".into());
        //     //log_1(&jsmsg);
        //     },
        //     Err(TryRecvError::Empty) => log_1(&"no".into()),
        //     Err(TryRecvError::Disconnected) => log_1(&"no2".into()),
        // }

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
