use crossterm::{
QueueableCommand,
terminal::{Clear,ClearType,SetSize},
cursor::{MoveTo,Hide,Show},
ExecutableCommand,
style::{ResetColor,SetForegroundColor,Color},
event::{self,KeyCode,KeyEvent}
};
use crossterm::terminal;
use rand::Rng;
use std::{
time::{Duration},
io::{stdout,Write},
thread,
error::{Error}
};


struct boss{
    health:i32,
    cooldown:i32
}

impl boss{
     fn skill1(&mut self){
        

     }

}
struct player{
    hp:i32,
    cooldown:i32,

}

fn main()-> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    stdout.execute(Hide)?;
    terminal::enable_raw_mode()?;
    
    stdout.queue(Clear(ClearType::All));
    terminal::SetSize(76,61);

    let mut pos_x = 45;
    let mut pos_y = 45;
    let life = "♡"; 
    let life2 = "♡  ♡";
    let life3 = "♡  ♡  ♡";
    let player = player{hp:3,cooldown:0};

    let boss = "(9@ ᗜˬᗜ @9)";
    let fumo = "ᗜ_ᗜ";

    stdout.queue(MoveTo(10,10));
    
    
    loop{

        stdout.execute(Clear(ClearType::All))?;
        
        for i in 0..51{
            
            stdout.queue(MoveTo(0,i));
            stdout.write(b"*");
            stdout.queue(MoveTo(76,i));
            stdout.write(b"*");
            
        }
        for i in 0..76{
            stdout.queue(MoveTo(i,0));
            stdout.write(b"*");
            stdout.queue(MoveTo(i,51));
            stdout.write(b"*");

        }
        
        if player.hp == 3{
            show_character(5,53,&format!("Life: {}", life3),Color::Red);
        }
        if player.hp == 2{
            show_character(5,53,&format!("Life: {}", life2),Color::Red);
        }
        if player.hp == 1{
            show_character(5,53,&format!("Life: {}", life),Color::Red);
        }
        if player.hp == 0{
            show_character(5,53,&format!("Life:XXXXXXXXXX", ),Color::Red);
        }

        

        show_character(pos_x,pos_y,fumo,Color::Blue);
        

        let boss_stat = boss{health:2000,cooldown:10};
        let mut rng = rand::thread_rng();
        let rem = rng.gen_range(0..10);
        let mut rem_x = 33;
        let delay = Duration::from_millis(20);
        if rem_x > 1 || rem_x < 74 {
            if rem < 5 {
              for i  in 0..rng.gen_range(0..20){
                rem_x -= 1;
                thread::sleep(delay);
                if boss_stat.cooldown == 0 {
                    rem_x = 33;
                    
                }
                
              }
            }
            else {
               for i  in 0..rng.gen_range(0..20){
                    rem_x += 1;
                    thread::sleep(delay);
                  }   
            }
        }


        
        show_character(rem_x,2,boss,Color::Red);
        stdout.flush()?;

        if let Ok(true) = event::poll(Duration::from_millis(50)) {
            if let Ok(event::Event::Key(KeyEvent { code, .. })) = event::read() {
                match code {
                    KeyCode::Char('a') => {
                       if pos_x > 1 {
                        pos_x -= 1;
                       }
                        
                    }
                    KeyCode::Char('d') => {    
                       if pos_x < 74{
                         pos_x += 1;
                       }
                         
                    }
                    KeyCode::Char('w') => {   
                        if pos_y > 1{
                          pos_y -= 1;  
                        }
                        
                    
                    }
                   KeyCode::Char('s') => {
                        if pos_y < 50{
                          pos_y += 1;  
                        }
                        
                
                    }
                   KeyCode::Char('e') => {
                        
                        
                        }
                        
                
                    
                    KeyCode::Esc => break Ok(()),
                    _ => {}

                }
            }
         }
         thread::sleep(Duration::from_millis(50));
  }
  
} 
fn show_character(x: u16, y: u16, character: &str, color: Color) {
    let mut stdout = stdout();
    stdout.execute(MoveTo(x, y)).unwrap();
    stdout.execute(SetForegroundColor(color)).unwrap();
    print!("{}", character);
    stdout.execute(ResetColor).unwrap();
}