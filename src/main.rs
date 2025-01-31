use rand::Rng;
use std::io;

fn main() {
    let mut replay: bool = false;
    loop {

      if replay == true {
        let mut start = String::new();
        println!("Want to play again? Y/[N]");
        io::stdin().read_line(&mut start).expect("Failed to read line");
        start = start.trim().to_string();
        if start=="Y" || start=="y"{
            //continue to play
        } else {
            println!("Thanks for playing :-) and see you again...");
            break;
        }
      }

      let mut summe_player = random_card();
      let mut summe_bank = random_card();
      let mut player_win = true;

      println!("Player {} Bank {}", summe_player, summe_bank);

      let mut card_player = random_card();
      loop {
          println!("Card for Player {}", card_player);
          summe_player += card_player;
          println!("Player {} Bank {}", summe_player, summe_bank);
          if summe_player > 21 {
              player_win = false;
              break;
          }
          let mut answer = String::new();
          println!("Card? Y/[N]");
          io::stdin().read_line(&mut answer).expect("Failed to read line");
          answer = answer.trim().to_string();
          if answer=="Y" || answer=="y"{
              card_player = random_card();
          } else {
              break;
          }
      }

      let mut card_bank = random_card();
      loop {
          if player_win == false || summe_bank > 21 {
              break
          }
          println!("Card for Bank {}", card_bank);
          summe_bank += card_bank;
          println!("Player {} Bank {}", summe_player, summe_bank);
          if summe_bank < 17 {
              card_bank = random_card();
          } else {
              break;
          }
      }

      if summe_bank > 21 {
          player_win = true;
      } else if summe_bank > summe_player {
          player_win = false;
      }
      if player_win {
          println!("Player win")
      } else {
          println!("Bank win")
      }

      replay = true;
    }
}

fn random_card() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=11)
}