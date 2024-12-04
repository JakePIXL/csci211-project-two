use crate::{
    clear_screen,
    db::Database,
    models::{Admin, Player},
};
use std::io::{self, Write};

pub struct GameManager {
    db: Database,
    current_admin: Option<Admin>,
    current_player: Option<Player>,
}

impl GameManager {
    pub fn new(db: Database) -> Self {
        Self {
            db,
            current_admin: None,
            current_player: None,
        }
    }

    pub fn get_user_input(&self, prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            clear_screen();
            println!("\n=== Trivia Game ===");
            println!("1. Admin Mode");
            println!("2. Player Mode");
            println!("3. Exit");
            let input = self.get_user_input("Choose an option: ");

            match input.trim() {
                "1" => self.admin_menu().await?,
                "2" => self.player_menu().await?,
                "3" => break,
                _ => println!("Invalid option!"),
            }
        }
        Ok(())
    }

    async fn admin_menu(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.current_admin.is_none() {
            clear_screen();
            println!("Must login to access!");
            let username = self.get_user_input("Enter username: ");

            let password = self.get_user_input("Enter password: ");

            match self.db.login_admin(username.trim(), password.trim()).await {
                Ok(admin) => {
                    self.current_admin = Some(admin);
                    self.current_player = None;
                }
                Err(_) => {
                    println!("Invalid username or password!");
                    return Ok(());
                }
            }
        }

        loop {
            clear_screen();
            println!("\n=== Admin Menu ===");
            println!("1. Create Question");
            println!("2. Create Game");
            println!("3. Edit Game");
            println!("4. Back");
            let input = self.get_user_input("Choose an option: ");

            match input.trim() {
                "1" => self.create_question().await?,
                "2" => self.create_game().await?,
                "3" => self.edit_games().await?,
                "4" => break,
                _ => println!("Invalid option!"),
            }
        }
        Ok(())
    }

    async fn player_menu(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.current_player.is_none() {
            clear_screen();
            let username = self.get_user_input("Enter username: ");
            let password = self.get_user_input("Enter password: ");

            match self.db.login_player(username.trim(), password.trim()).await {
                Ok(player) => {
                    self.current_player = Some(player);
                    self.current_admin = None;
                }
                Err(_) => {
                    println!("Invalid username or password!");
                    return Ok(());
                }
            }
        }

        let games = self.db.get_games().await?;

        clear_screen();
        println!("\nAvailable Games:");
        for game in &games {
            println!("{}. {}", game.game_id, game.title);
        }

        let input = self.get_user_input("Select a game (enter game ID): ");

        if let Ok(game_id) = input.trim().parse::<i32>() {
            self.play_regular_game(game_id).await?;
        } else {
            println!("Invalid game ID!");
        }
        Ok(())
    }

    async fn create_question(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(admin) = self.current_admin.clone() {
            let text = self.get_user_input("Enter question text: ");

            let answer = self.get_user_input("Enter correct answer (true/false): ");

            let answer = answer.trim().to_lowercase() == "true";

            self.db
                .create_question(text.trim(), answer, admin.admin_id)
                .await?;
            println!("Question created successfully!");
            Ok(())
        } else {
            println!("Not a valid admin account.");
            Ok(())
        }
    }

    async fn create_game(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(admin) = self.current_admin.clone() {
            let title = self.get_user_input("Enter game title: ");

            let description =
                self.get_user_input("Enter game description (optional, press Enter to skip): ");

            let description = if description.trim().is_empty() {
                None
            } else {
                Some(description.trim())
            };

            let game_id = self
                .db
                .create_game(title.trim(), description, admin.admin_id)
                .await?;
            println!("Game created! Now let's add questions.");

            loop {
                println!("\nAdd question to game:");
                println!("1. Create new question");
                println!("2. Add existing question");
                println!("3. Finish adding questions");
                let input = self.get_user_input("Choose an option: ");

                match input.trim() {
                    "1" => {
                        let question_id = self.create_question_inner().await?;
                        self.add_question_to_game(game_id, question_id).await?;
                    }
                    "2" => {
                        self.add_existing_question(game_id).await?;
                    }
                    "3" => break,
                    _ => println!("Invalid option!"),
                }
            }

            println!("Game created successfully!");
            Ok(())
        } else {
            println!("Not a valid admin account.");
            Ok(())
        }
    }

    async fn create_question_inner(&self) -> Result<i32, Box<dyn std::error::Error>> {
        if let Some(admin) = self.current_admin.clone() {
            let text = self.get_user_input("Enter question text: ");

            let answer = self.get_user_input("Enter correct answer (true/false): ");

            let answer = answer.trim().to_lowercase() == "true";

            self.db
                .create_question(text.trim(), answer, admin.admin_id)
                .await
                .map_err(|e| e.into())
        } else {
            println!("Not a valid admin account.");
            Err("Not a valid admin account.".into())
        }
    }

    async fn add_question_to_game(
        &self,
        game_id: i32,
        question_id: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let order_input =
            self.get_user_input("Enter question order (press Enter for next available): ");

        let order = if order_input.trim().is_empty() {
            let max_order = self.db.get_max_question_order(game_id).await?;
            max_order + 1
        } else {
            order_input.trim().parse()?
        };

        self.db
            .add_question_to_game(game_id, question_id, order)
            .await?;
        println!("Question added to game successfully!");
        Ok(())
    }

    async fn add_existing_question(&self, game_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let questions = self.db.get_all_questions().await?;

        println!("\nAvailable questions:");
        for question in &questions {
            println!("{}. {}", question.question_id, question.question_text);
        }

        let input = self.get_user_input("Enter question ID to add: ");

        if let Ok(question_id) = input.trim().parse::<i32>() {
            if questions.iter().any(|q| q.question_id == question_id) {
                self.add_question_to_game(game_id, question_id).await?;
            } else {
                println!("Invalid question ID!");
            }
        } else {
            println!("Invalid input!");
        }
        Ok(())
    }

    async fn edit_games(&self) -> Result<(), Box<dyn std::error::Error>> {
        let games = self.db.get_games().await?;

        clear_screen();
        println!("All available games:");

        for game in &games {
            println!("{}. {}", game.game_id, game.title);
        }

        let input = self.get_user_input("Select a game (enter game ID): ");

        if let Ok(game_id) = input.trim().parse::<i32>() {
            self.edit_game(game_id).await?;
        } else {
            println!("Invalid game ID!");
        }

        Ok(())
    }

    async fn edit_game(&self, game_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            clear_screen();
            let questions = self.db.get_game_questions(game_id).await?;

            println!("\nCurrent Questions:");
            for question in &questions {
                println!(
                    "{}. [ID: {}] {}",
                    question.question_order, question.question_id, question.question_text
                );
            }

            println!("\nEdit Game Options:");
            println!("1. Add new question");
            println!("2. Remove question");
            println!("3. Reorder questions");
            println!("4. Back");

            let choice = self.get_user_input("Enter your choice: ");

            match choice.trim() {
                "1" => {
                    self.create_question_inner().await?;
                }
                "2" => {
                    clear_screen();
                    let question_id = self.get_user_input("Enter question ID to remove: ");
                    let question_id: i32 = question_id.trim().parse()?;

                    self.db.delete_game_question(game_id, question_id).await?;

                    println!("Question removed successfully!");
                }
                "3" => {
                    clear_screen();
                    let questions = self.db.get_game_questions(game_id).await?;

                    println!("\nCurrent Questions:");
                    for question in &questions {
                        println!(
                            "{}. [ID: {}] {}",
                            question.question_order, question.question_id, question.question_text
                        );
                    }

                    println!(
                        "\nEnter new order for questions (comma-separated list of question IDs)"
                    );
                    println!("Example: 3,1,4,2");
                    let new_order = self.get_user_input("New order: ");

                    let order_ids: Vec<i32> = new_order
                        .split(',')
                        .filter_map(|s| s.trim().parse().ok())
                        .collect();

                    for (new_order, &question_id) in order_ids.iter().enumerate() {
                        self.db
                            .update_question_order(game_id, question_id, new_order as i32 + 1)
                            .await?;
                    }

                    println!("Questions reordered successfully!");
                }
                "4" => break,
                _ => println!("Invalid choice, please try again."),
            }
        }

        Ok(())
    }

    async fn play_regular_game(&self, game_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let mut correct_answers = 0;

        let questions = self.db.get_game_questions(game_id).await?;

        for (i, question) in questions.iter().enumerate() {
            clear_screen();
            println!("\nQuestion {} of {}", i + 1, questions.len());
            println!("{}", question.question_text);
            print!("Your answer (true/false): ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let answer = input.trim().to_lowercase() == "true";
            let correct = answer == question.correct_answer;

            if correct {
                correct_answers += 1;
                println!("Correct!");
            } else {
                println!(
                    "Incorrect! Correct answer is: {}",
                    question.correct_answer.to_bool()
                );
            }
        }

        clear_screen();
        println!("\nGame Over!\n\n");
        println!(
            "You got {} out of {} questions correct!",
            correct_answers,
            questions.len()
        );
        let percentage = (correct_answers as f64 / questions.len() as f64) * 100.0;
        println!("Score: {:.1}%", percentage);

        if percentage == 100.0 {
            println!("Perfect score! Congratulations!");
        } else if percentage >= 80.0 {
            println!("Great job!");
        } else if percentage >= 60.0 {
            println!("Not bad!");
        } else {
            println!("Keep practicing!");
        };

        println!("\nPress Enter to continue...");
        io::stdout().flush()?;
        let mut temp = String::new();
        io::stdin().read_line(&mut temp)?;
        Ok(())
    }
}
