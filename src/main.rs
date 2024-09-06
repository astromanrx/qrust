use std::io;
use std::io::stdout;
use crossterm::cursor::{position, DisableBlinking, EnableBlinking, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp};
use crossterm::terminal::{Clear, ClearType};
use crossterm::event::{KeyEventKind, KeyModifiers};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use crossterm::style::Stylize;


fn main() -> io::Result<()> {
    _ = execute!(io::stdout(),Clear(ClearType::All),MoveTo(0,0));
    _ = execute!(
        io::stdout(),    
        EnableBlinking,    
    );
    
        
    enable_raw_mode()?;
    if let Err(e) = print_events() {
        println!("Error: {:?}\r", e);
    }
    disable_raw_mode()?;

    Ok(())
}

fn print_events() -> io::Result<()> {
    
    loop {
        let event = read()?;
        match event {
            Event::Resize(cols,rows) =>{
                
            },

            Event::Key(event) if event.kind == KeyEventKind::Press => {                
                if event.modifiers != KeyModifiers::NONE {
                    print!("{}+", event.modifiers);
                }  
                match event.code {
                    KeyCode::Char(c) =>{
                        execute!(
                            stdout(),
                            SetBackgroundColor(Color::DarkGrey),
                            Print(c),
                            ResetColor
                        )?;
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
                        println!()
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

                if event.code == KeyCode::Esc {
                    break;
                }
            }
            _ => {}
        }
    }
    Ok(())
}