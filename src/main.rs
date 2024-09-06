use std::io;
use std::io::stdout;
use crossterm::cursor::{position, DisableBlinking, EnableBlinking, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp};
use crossterm::terminal::{Clear, ClearType};
use crossterm::event::{KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor},
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::fs;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn main() -> io::Result<()> {
    _ = execute!(io::stdout(),Clear(ClearType::All),MoveTo(0,0));
    _ = execute!(
        io::stdout(),    
        EnableBlinking,    
    );

    let mut editor = Editor{ content : vec![]};
    
        
    enable_raw_mode()?;
    if let Err(e) = editor.run() {
        println!("Error: {:?}\r", e);
    }
    disable_raw_mode()?;

    Ok(())
}

struct Editor{
    content : Vec<String>,    
}

impl Editor {
    fn run(&mut self) -> io::Result<()> {
        
        loop {
            let event = read()?;
            match event {
                Event::Resize(cols,rows) =>{
                    
                },
    
                Event::Key(event) if event.kind == KeyEventKind::Press =>{
                    if ! self.handle_key(event){
                        break;
                    }                        
                } ,
                _ => {}
            }
        }
        Ok(())
    }    

    fn handle_key(&mut self,event:KeyEvent) -> bool{
        if event.modifiers == KeyModifiers::CONTROL {
            match event.code {
                KeyCode::Char(k) =>{
                    match k.to_ascii_uppercase() {                                
                        'S' => {
                            match fs::write("",self.content.join(LINE_ENDING)) {
                                Err(error)=>println!("Unable to save file, Error: {}",error),
                                Ok(_)=> println!("{}","Saving ...")                                        
                            } ;                                                                        
                        },                               
                        _ =>()
                    }
                } 
                _ => ()
            }
        }else if event.modifiers == KeyModifiers::NONE {
            match event.code {
                KeyCode::Char(c) =>{
                    _ = execute!(
                        stdout(),
                        SetBackgroundColor(Color::DarkGrey),
                        Print(c),
                        ResetColor
                    );
                },
                KeyCode::Right => {
                    _ = execute!(io::stdout(),MoveRight(1))
                },
                KeyCode::Left => {
                    _ = execute!(io::stdout(),MoveLeft(1))
                },
                KeyCode::Up => {
                    _ = execute!(io::stdout(),MoveUp(1))
                },
                KeyCode::Down => {                        
                    _ = execute!(io::stdout(),MoveDown(1))
                },
                KeyCode::Enter => {
                    println!();
                    self.content.push(String::from(""));
                },
                KeyCode::Backspace =>{
                    _ = execute!(io::stdout(),DisableBlinking,MoveLeft(1),Print(' '),EnableBlinking,MoveLeft(1))
                },
                KeyCode::Delete =>{

                },                    
                KeyCode::Home =>{
                    match position() {
                        Ok(pos) =>{
                            let (col,row) = pos;
                            _ = execute!(io::stdout(),MoveTo(0,row));
                        },
                        _=>()
                    }
                    
                },
                KeyCode::End =>{

                },
                _ => ()
            }         
        }
               

        if event.code == KeyCode::Esc {
            return false
        }
        true
    }
}


