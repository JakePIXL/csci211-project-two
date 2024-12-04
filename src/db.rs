use crate::models::{Admin, Game, GameQuestionFull, Player, Question};
use sqlx::mysql::MySqlPool;

pub struct Database {
    pool: MySqlPool,
}

impl Database {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn get_games(&self) -> Result<Vec<Game>, sqlx::Error> {
        sqlx::query_as!(Game, "SELECT game_id, title, description FROM games")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn login_admin(&self, username: &str, password: &str) -> Result<Admin, sqlx::Error> {
        let result = sqlx::query_as!(
            Admin,
            r#"
            SELECT admin_id, username, password_hash, created_at
            FROM admins
            WHERE username = ? AND password_hash = ?
            "#,
            username,
            password
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn login_player(
        &self,
        username: &str,
        password: &str,
    ) -> Result<Player, sqlx::Error> {
        let result = sqlx::query_as!(
            Player,
            r#"
            SELECT player_id, username, password_hash, created_at
            FROM players
            WHERE username = ? AND password_hash = ?
            "#,
            username,
            password
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_all_players(&self) -> Result<Vec<Player>, sqlx::Error> {
        sqlx::query_as!(
            Player,
            "SELECT player_id, username, password_hash, created_at FROM players"
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn delete_player(&self, player_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM players
            WHERE player_id = ?
            "#,
            player_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn create_new_player(
        &self,
        username: &str,
        password: &str,
    ) -> Result<(), sqlx::Error> {
        match sqlx::query!(
            r#"
            INSERT INTO players (username, password_hash)
            VALUES (?, ?)
            "#,
            username,
            password
        )
        .execute(&self.pool)
        .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(sqlx::Error::RowNotFound),
        }
    }

    pub async fn get_game_questions(
        &self,
        game_id: i32,
    ) -> Result<Vec<GameQuestionFull>, sqlx::Error> {
        sqlx::query_as!(
            GameQuestionFull,
            r#"
            SELECT q.question_id, q.question_text, q.correct_answer, gq.question_order
            FROM questions q
            JOIN game_questions gq ON q.question_id = gq.question_id
            WHERE gq.game_id = ?
            ORDER BY gq.question_order
            "#,
            game_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_all_questions(&self) -> Result<Vec<Question>, sqlx::Error> {
        sqlx::query_as!(
            Question,
            "SELECT question_id, question_text, correct_answer FROM questions"
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn add_question_to_game(
        &self,
        game_id: i32,
        question_id: i32,
        order: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO game_questions (game_id, question_id, question_order)
                VALUES (?, ?, ?)
                "#,
            game_id,
            question_id,
            order
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_question(&self, question_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM questions
            WHERE question_id = ?
            "#,
            question_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_game_question(
        &self,
        game_id: i32,
        question_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM game_questions
            WHERE game_id = ? AND question_id = ?
            "#,
            game_id,
            question_id
        )
        .execute(&self.pool)
        .await?;

        self.reorder_game_questions(game_id).await?;

        Ok(())
    }

    async fn reorder_game_questions(&self, game_id: i32) -> Result<(), sqlx::Error> {
        let questions = self.get_game_questions(game_id).await?;

        let mut tx = self.pool.begin().await?;

        for (new_order, question) in questions.iter().enumerate() {
            let new_order = (new_order + 1) as i32;
            if new_order != question.question_order {
                sqlx::query!(
                    r#"
                    UPDATE game_questions
                    SET question_order = ?
                    WHERE game_id = ? AND question_id = ?
                    "#,
                    new_order,
                    game_id,
                    question.question_id
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn update_question_order(
        &self,
        game_id: i32,
        question_id: i32,
        order: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE game_questions
            SET question_order = ?
            WHERE game_id = ? AND question_id = ?
            "#,
            order,
            game_id,
            question_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_max_question_order(&self, game_id: i32) -> Result<i32, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                SELECT COALESCE(MAX(question_order), 0) as max_order
                FROM game_questions
                WHERE game_id = ?
                "#,
            game_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.max_order.expect("No max order found"))
    }

    pub async fn create_question(
        &self,
        text: &str,
        answer: bool,
        admin_id: i32,
    ) -> Result<i32, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            INSERT INTO questions (question_text, correct_answer, created_by)
            VALUES (?, ?, ?)
            "#,
            text,
            answer,
            admin_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_id() as i32)
    }

    pub async fn create_game(
        &self,
        title: &str,
        description: Option<&str>,
        admin_id: i32,
    ) -> Result<i32, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            INSERT INTO games (title, description, created_by)
            VALUES (?, ?, ?)
            "#,
            title,
            description,
            admin_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_id() as i32)
    }
}
