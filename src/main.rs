use std::io;
use crossterm::cursor::{self, position, EnableBlinking, Hide, MoveLeft, MoveRight, MoveTo, RestorePosition, SavePosition, Show};
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::event::{KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::{
    execute,
    style::Print,
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::fs;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

const TAB_SIZE : usize = 4;

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

fn remove_char(start: usize, stop: usize, s: &String) -> String {
    let mut result = "".to_string();
    for (i, c) in s.chars().enumerate() {
        if start > i || stop < i + 1 {
            result.push(c);
        }
    }
    result
}

struct Editor{    
    content : Vec<String>,    
}

impl Drop for Editor{
    fn drop(&mut self) {
        _ = execute!(io::stdout(),Clear(ClearType::All),MoveTo(0,0));
    }
}

impl Editor {
    fn resize(&mut self, cols : usize ,rows: usize){
        if rows> self.content.len() {
            for _ in 0.. (rows - self.content.len()){
                self.content.push(String::from(""));
            }
        }
        
        if rows< self.content.len() {
            for _ in 0.. (self.content.len() - rows){
                self.content.pop();
            }
        }
    }

    fn run(&mut self) -> io::Result<()> {    
        match crossterm::terminal::size(){
            Err(error)=> println!("Failed to get terminal size: {}",error),
            Ok(terminal_size)=>{
                let (cols,rows) = terminal_size;
                self.resize(cols as usize, rows as usize);
                self.update();    
            }
        }
        
        loop {
            let event = read()?;
            match event {
                Event::Resize(cols,rows) =>{
                    self.resize(cols as usize, rows as usize)
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

    fn update(&self){
        let s= self.content.join(LINE_ENDING);
        _ = execute!(io::stdout(),SavePosition,Hide,Clear(ClearType::All),MoveTo(0,0),Print(s),RestorePosition,Show)
    }

    fn save(&self){
        match fs::write("",self.content.join(LINE_ENDING)) {
            Err(error)=>println!("Unable to save file, Error: {}",error),
            Ok(_)=> println!("{}","Saving ...")                                        
        } ;                                                                      
    }

    fn handle_key(&mut self,event:KeyEvent) -> bool{
        if event.modifiers == KeyModifiers::CONTROL {
            match event.code {
                KeyCode::Char(k) =>{
                    match k.to_ascii_uppercase() {                                
                        'S' => {
                            self.save()
                        },                               
                        _ =>()
                    }
                } 
                _ => ()
            }
        }else if event.modifiers != KeyModifiers::ALT {
            match event.code {
                KeyCode::Char(c) =>{
                    self.insert_string(&c.to_string());
                    self.update()
                },
                KeyCode::Right => {
                    let (cols,_) = terminal::size().unwrap();
                    let (cursor_column,_) = position().unwrap();
                    if  cursor_column< cols{
                        _ = execute!(io::stdout(),MoveRight(1));
                    }                    
                },
                KeyCode::Left => {                    
                    let (cursor_column,_) = position().unwrap();
                    if  cursor_column>0 {
                        _ = execute!(io::stdout(),MoveLeft(1));
                    }                    
                },
                KeyCode::Up => {                    
                    let (cursor_col,cursor_row) = position().unwrap();
                    if cursor_row > 0{        
                        let line_below_length = self.content[(cursor_row - 1) as usize].len() as u16;
                        if cursor_col > line_below_length {
                            _ = execute!(io::stdout(),MoveTo(line_below_length,cursor_row-1));
                        }else{
                            _ = execute!(io::stdout(),MoveTo(line_below_length,cursor_row-1));
                        }                        
                    }                        
                },
                KeyCode::Down => {                        
                    let (_,rows) = terminal::size().unwrap();
                    let (_,cursor_row) = position().unwrap();
                    if  cursor_row< rows {
                        _ = execute!(io::stdout(),MoveTo(self.content[(cursor_row + 1) as usize].len() as u16,cursor_row + 1));
                    }                    
                },
                KeyCode::Enter => {                    
                    let (cursor_col,cursor_row) = position().unwrap();                                                            
                    match self.get_line_leading_spaces(cursor_row as usize){
                        Some(spaces) if spaces>0 => {
                            self.split_line(cursor_row as usize,cursor_col as usize," ".repeat(spaces as usize).as_ref());
                            _ = execute!(io::stdout(),MoveTo(spaces,cursor_row + 1))
                        },
                        _ => {
                            self.split_line(cursor_row as usize,cursor_col as usize,"");
                            _ = execute!(io::stdout(),MoveTo(0,cursor_row + 1))                            
                        }
                    }                    
                    self.update();
                },
                KeyCode::Tab =>{                    
                    self.insert_string(&" ".repeat(TAB_SIZE));
                    self.update()
                }
                KeyCode::Backspace =>{
                    let (cursor_col,cursor_row) = position().unwrap();
                    let mut delete_length = 0;
                    if cursor_col == 0 {
                        if cursor_row > 0 {
                            let previous_line = cursor_row - 1;
                            self.merge_lines(previous_line );
                            _ = execute!(io::stdout(),MoveTo(self.content[previous_line as usize].len() as u16 + 1, previous_line));
                        }                        
                    }else{
                        match self.get_line_leading_spaces(cursor_row as usize) {
                            Some(spaces_count) if cursor_col <= spaces_count =>{
                                let r = spaces_count % TAB_SIZE as u16 ;
                                if r > 0{
                                    delete_length = r
                                }else{
                                    delete_length = TAB_SIZE as u16
                                }
                            },
                            _ =>delete_length = 1
                        } 
                        self.content[cursor_row as usize] = remove_char((cursor_col - delete_length) as usize,cursor_col as usize,&self.content[cursor_row as usize]);                                                
                    }
                    
                    _ = execute!(io::stdout(),MoveLeft(delete_length));
                    self.update();
                    
                                                        
                },
                KeyCode::Delete =>{               
                    let (cursor_col,cursor_row) = position().unwrap();                          
                    if cursor_col + 1 > self.content[cursor_row as usize].len() as u16 {
                        self.merge_lines(cursor_row);
                    }else{                        
                        self.content[cursor_row as usize] = remove_char(cursor_col as usize,(cursor_col+1) as usize,&self.content[cursor_row as usize]);
                    }
                    self.update();
                },                    
                KeyCode::Home =>{
                    let (_,cursor_row) = position().unwrap();
                    match self.get_line_leading_spaces(cursor_row as usize){
                        None => (),
                        Some (spaces_count)=>_ = execute!(io::stdout(),MoveTo(spaces_count ,cursor_row as u16))
                    }
                },
                KeyCode::End =>{
                    let (_,cursor_row) = position().unwrap();
                    let line: &String = &self.content[cursor_row as usize];
                    let mut iter = line.chars();                    
                    let mut i: usize = 0;
                    loop{                                                                        
                        match iter.nth_back(i){
                            Some(c) if ! char::is_whitespace(c)=>{                                
                                _ = execute!(io::stdout(),MoveTo((line.len() - i) as u16 ,cursor_row as u16));
                                break;
                            },        
                            None=> break,                    
                            _=>()
                        }                  
                        i += 1;       
                    }                    
                },
                _ => ()
            }         
        }               

        if event.code == KeyCode::Esc {
            return false
        }
        true
    }    

    fn get_line_leading_spaces(&mut self,line_index: usize) -> Option<u16>{
        let line = &self.content[line_index];
        let mut iter = line.chars();
        let mut i = 0;
        loop{                                                                                                
            match iter.next(){
                Some(c) if ! char::is_whitespace(c)=>{                                
                    return Some(i)
                },                            
                None=> return Some(i),
                _=>()
            }                  
            i += 1;       
        }
    }

    fn merge_lines(&mut self,line_index: u16){
        let mut this_line = self.content[(line_index) as usize].clone();
        let next_line = self.content[(line_index + 1) as usize].as_str();                        
        this_line.push_str(next_line);
        self.content[line_index as usize] = this_line;                                               
        self.content.remove((line_index + 1) as usize);
    }

    fn insert_string(&mut self, ch: &str){
        let (cursor_col,cursor_row) = position().unwrap();     
        let current_line = cursor_row as usize;
        if self.content[current_line].len() == 0{
            self.content[current_line].push_str(ch);
        }else{
            if cursor_col  < self.content[current_line].len() as u16 {
                self.content[current_line].insert_str(cursor_col as usize,ch);
            }else{
                self.content[current_line].push_str(ch);
            }            
        }                
        _ = execute!(io::stdout(),MoveRight(ch.len() as u16));                
    }

    fn split_line(&mut self,line_index: usize,character_index: usize,new_line_prefix: &str ){        
        let this_line = self.content[line_index].clone();                
        if this_line.len() == 0 {
            self.content.insert(line_index + 1,String::from(""));    
        }else{
            let (first_part,second_part) = this_line.split_at(character_index);
            self.content[line_index] = String::from(first_part);
            let mut s = String::from(second_part);
            s.insert_str(0,new_line_prefix);
            self.content.insert(line_index + 1,s);
            self.content.pop();
        }
    }
}


