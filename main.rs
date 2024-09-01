use crossterm::{
QueueableCommand,
terminal::{Clear,ClearType},
cursor::{MoveTo,Hide,Show},
ExecutableCommand,
style::{ResetColor,SetForegroundColor,Color},
event::{self,KeyCode,KeyEvent}
};
use crossterm::terminal;
use rand::Rng;
use std::{
time::{Duration,Instant},
io::{stdout,Write},
thread,
error::{Error}
};

struct boss{
    health:i32,
    cooldown:i32,
    x:i32,
    y:i32,
    bullets: Vec<Bullet>,
    shoot_timer: u32,
    cooldown_skill:i32,
}

impl boss{
     fn skill1(&mut self, player: &mut player){
        
        
         let mut x_skill = 38;
         let mut y_skill = 6;
          for i in y_skill..51{
          show_character(x_skill, i, "xxxxxx", Color::Red);
          if player.y == i && player.x >= x_skill && player.x <= x_skill + 5 {
            player.hp -= 1; // Decrease player's health if hit by the skill
            } 
          thread::sleep(Duration::from_millis(20))
         }
         for i in y_skill..51{
            show_character(x_skill-10, i, "xxxxxx", Color::Red); 
            show_character(x_skill+10, i, "xxxxxx", Color::Red);
            if player.y == i && (player.x >= x_skill - 10 && player.x <= x_skill - 5 || player.x >= x_skill + 10 && player.x <= x_skill + 15) {
                player.hp -= 1; // Decrease player's health if hit by the skill
            } 
            thread::sleep(Duration::from_millis(20));
        }
           
         for i in y_skill..51{
            show_character(x_skill-20, i, "xxxxxx", Color::Red); 
            show_character(x_skill+20, i, "xxxxxx", Color::Red);
            if player.y == i && (player.x >= x_skill - 20 && player.x <= x_skill - 15 || player.x >= x_skill + 20 && player.x <= x_skill + 25) {
                player.hp -= 1;  
            
            }
            thread::sleep(Duration::from_millis(20));
        }
         for i in y_skill..51{
            show_character(x_skill-30, i, "xxxxxx", Color::Red); 
            show_character(x_skill+30, i, "xxxxxx", Color::Red);
            if player.y == i && (player.x >= x_skill - 30 && player.x <= x_skill - 25 || player.x >= x_skill + 30 && player.x <= x_skill + 35) {
                player.hp -= 1; 
            }
            thread::sleep(Duration::from_millis(20));
            }
       
        }
        fn shoot(&mut self) {
            if self.cooldown <= 0 {
                self.bullets.push(Bullet {
                    x: self.x as u16,
                    y: self.y as u16 - 1,
                    direction: 2, // Bullet downwards
                    color: Color::Red,
                });
                self.cooldown = 4; // Set cooldown
            }
        }
    
        fn update_bullets(&mut self) {
            self.bullets.retain(|bullet| bullet.y < 51); // Remove bullets off the screen
            for bullet in &mut self.bullets {
                bullet.move_bullet(); // Move the bullet
            }
        }
    
        fn draw_bullets(&self) {
            for bullet in &self.bullets {
                show_character(bullet.x, bullet.y, "x", bullet.color); // Draw the bullet
            }
        }
        
}


struct Bullet {
    x: u16,
    y: u16,
    direction: i16, // Positive for downwards, negative for upwards
    color: Color,
}

impl Bullet {
    fn move_bullet(&mut self) {
        self.y = (self.y as i16 + self.direction) as u16;
    }
}
struct player{
    hp:i32,
    cooldown:i32,
    bullets: Vec<Bullet>,
    shoot_timer: u32,
    cooldown_skill:i32,
    x:u16,
    y:u16

}

impl player {

    fn shoot(&mut self) {
        if self.cooldown <= 0 {
            self.bullets.push(Bullet {
                x: self.x,
                y: self.y + 1,
                direction: -1, // Bullet moves upwards
                color: Color::Blue,
            });
            self.cooldown = 2; // Set cooldown
        }
    }

    fn update_bullets(&mut self) {
        self.bullets.retain(|bullet| bullet.y > 2); // Remove bullets off the screen
        for bullet in &mut self.bullets {
            bullet.move_bullet(); // Move the bullet
        }
    }

    fn draw_bullets(&self) {
        for bullet in &self.bullets {
            show_character(bullet.x, bullet.y, "*", bullet.color); // Draw the bullet
        }
    }

    fn skill(&mut self,boss:&mut boss){

        show_character(33, 5, "(9@ ᗜˬᗜ @9)", Color::Red);
        show_character(27, 5, "x", Color::Blue);
        show_character(39, 5, "x", Color::Blue);

        for i in (2..51).rev() { // Loop to animate the skill
            show_character(28, i, "xxxxxx", Color::Blue); 
            show_character(38, i, "xxxxxx", Color::Blue); 
            show_character(48, i, "xxxxxx", Color::Blue);
            
            // Check if the skill hit the boss
            if i == 5 && boss.x >= 28 && boss.x <= 48 {
                boss.health -= 500;  // Decrease the boss health
            }

            thread::sleep(Duration::from_millis(50)); // Slow down the animation for better visual effect
        }

        // Reset cooldown and skillready status
        self.cooldown_skill = 30;
        show_character(5, 54, "Skill Ready Press E", Color::White);  // Indicate that the skill is ready
    }
    }

    
    


fn main()-> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    stdout.execute(Hide)?;
    terminal::enable_raw_mode()?;
    
    stdout.queue(Clear(ClearType::All));
    

    let mut pos_x = 45;
    let mut pos_y = 45;
    let life = "♡"; 
    let life2 = "♡  ♡";
    let life3 = "♡  ♡  ♡";
    

    let boss = "(9@ ᗜˬᗜ @9)";
    let fumo = "ᗜ_ᗜ";

    let now = Instant::now();
    
    let mut player = player{hp:3,cooldown:2,bullets:Vec::new(),shoot_timer:0,cooldown_skill:30,x:0,y:0};
    let mut boss_stat = boss{health:2000,cooldown:2,x:0,y:0,shoot_timer:0,cooldown_skill:20,bullets:Vec::new()};
    

    stdout.queue(MoveTo(10,10));
    let mut last_skill_time = Instant::now();
    

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
        

        match player.hp {
            3 => show_character(5, 53, &format!("Life: {}", life3), Color::Red),
            2 => show_character(5, 53, &format!("Life: {}", life2), Color::Red),
            1 => show_character(5, 53, &format!("Life: {}", life), Color::Red),
            0 => {
                show_character(5, 53, "Life:XXXXXXXXXX", Color::Red);
                show_character(25, 30, "Sorrow Has Fade, Void Awake, Freedom At Last.. . .", Color::Red);
                break Ok(());
            }
            _ => {}
        }

        if player.cooldown_skill > 0 {
            player.cooldown -= 1; 
        } 

        if player.cooldown_skill == 0 {
                show_character(5,54,&format!("Skill Ready Press E" ),Color::White);
  
         }
        
        if player.cooldown > 0 {
            player.cooldown -= 1; // Decrease cooldown in every loop
        }
        player.update_bullets();
        player.draw_bullets();
        if player.shoot_timer % 2 == 0{
            player.shoot();
        }
        player.shoot_timer += 1;

        show_character(pos_x,pos_y,fumo,Color::Blue);
        player.x = pos_x;
        player.y = pos_y;
        
        // Boss 
        
        let mut rng = rand::thread_rng();
        let rem = rng.gen_range(0..10);
        boss_stat.x = 33;
        boss_stat.y = 5;
        let delay = Duration::from_millis(5);
        if boss_stat.x > 1 || boss_stat.x < 74 {
            if rem < 5 {
              for i  in 0..rng.gen_range(0..20){
                boss_stat.x -= 1;
                thread::sleep(delay);
              }
            }
            else {
               for i  in 0..rng.gen_range(0..20){
                    boss_stat.x += 1;
                    thread::sleep(delay);
                  }   
            }
        }
        
        
        if now.elapsed().as_secs() % boss_stat.cooldown_skill as u64 == 0 && now.elapsed().as_secs() != 0 {
            boss_stat.skill1(&mut player);
 
        }

        if boss_stat.health == 0{
            show_character(3, 30, &format!("My Fallen Don't Mean My Essence Is Perished But Always Be In The Room Waiting"), Color::Blue);
            show_character(25, 31, &format!("For VENGEANCE'S CALL"), Color::Red);
            break (Ok(()));
        }
        
        
        show_character(boss_stat.x as u16, boss_stat.y as u16,boss,Color::Red);
        player.bullets.retain(|bullet| {
            let hit = bullet.y >= boss_stat.y as u16 - 1 && bullet.y <= boss_stat.y as u16 + 1 &&
                      bullet.x >= boss_stat.x as u16 - 4 && bullet.x <= boss_stat.x as u16 + 4;
            if hit {
                boss_stat.health -= 20; // Decrease boss health on hit
            }
            !hit // Remove bullet if it hits
        });

        if boss_stat.cooldown > 0 {
            boss_stat.cooldown -= 1; // Decrease cooldown in every loop
        }

        boss_stat.update_bullets();
        boss_stat.draw_bullets();
        
        if boss_stat.shoot_timer % 2 == 0{
            boss_stat.shoot();
        }
        boss_stat.shoot_timer += 1;

        boss_stat.bullets.retain(|bullet| {
            // Calculate the player's width based on the length of the `fumo` string
            let player_width = fumo.len() as u16;
        
            let hit = (bullet.y == player.y - 1) && (bullet.x >= player.x && bullet.x < player.x + player_width);
            
            if hit {
                player.hp -= 1; // Decrease player health on hit
            }
        
            !hit // Remove bullet if it hits
        });

        show_character(35,53,&format!("Boss HP:{}",boss_stat.health ),Color::Red);


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
                    KeyCode::Esc => break Ok(()),
                    _ => {}

                }
            }
         }
         thread::sleep(Duration::from_millis(50));

         if player.cooldown_skill == 0 && last_skill_time.elapsed().as_secs() >= 5 {
            if let Ok(true) = event::poll(Duration::from_millis(50)) {
                if let Ok(event::Event::Key(KeyEvent { code, .. })) = event::read() {
                    match code {
                        KeyCode::Char('e') => {
                            player.skill(&mut boss_stat);
                            last_skill_time = Instant::now(); // Reset last skill time
                            player.cooldown_skill = 30; // Reset the skill cooldown after activation
                        }
                        _ => {}
                    }
                }
            }
        }
   
         
         
  }
  
} 
fn show_character(x: u16, y: u16, character: &str, color: Color) {
    let mut stdout = stdout();
    stdout.execute(MoveTo(x, y)).unwrap();
    stdout.execute(SetForegroundColor(color)).unwrap();
    print!("{}", character);
    stdout.execute(ResetColor).unwrap();
}
