-- Insert admin users
INSERT INTO admins (username, password_hash, created_at) VALUES
('admin1', 'password123', '2024-01-01 10:00:00'),
('admin2', 'adminpass456', '2024-01-02 11:30:00'),
('moderator1', 'modpass789', '2024-01-03 09:15:00');

-- Insert players
INSERT INTO players (username, password_hash, created_at) VALUES
('player1', 'playerpass123', '2024-01-05 14:20:00'),
('gamer2024', 'gamerpass456', '2024-01-06 16:45:00'),
('triviaking', 'kingpass789', '2024-01-07 12:30:00'),
('quizmaster', 'quizpass321', '2024-01-08 10:15:00'),
('brainiac', 'brainpass654', '2024-01-09 15:40:00');

-- Insert games
INSERT INTO games (title, description, created_by, created_at) VALUES
('Science Trivia', 'Test your knowledge of basic scientific facts!', 1, '2024-01-10 09:00:00'),
('History Facts', 'Journey through time with these historical questions', 1, '2024-01-11 10:30:00'),
('Tech Quiz', 'Modern technology and computing questions', 2, '2024-01-12 11:45:00'),
('Space Explorer', 'Questions about space and astronomy', 3, '2024-01-13 14:20:00');

-- Insert questions
INSERT INTO questions (question_text, correct_answer, created_by, created_at) VALUES
('The Earth revolves around the Sun', true, 1, '2024-01-15 09:00:00'),
('Water boils at 100 degrees Celsius at sea level', true, 1, '2024-01-15 09:01:00'),
('Humans only use 10% of their brains', false, 1, '2024-01-15 09:02:00'),
('The Great Wall of China is visible from space', false, 2, '2024-01-16 10:00:00'),
('World War II ended in 1945', true, 2, '2024-01-16 10:01:00'),
('The first programmable computer was the ENIAC', true, 2, '2024-01-16 10:02:00'),
('JavaScript is a compiled language', false, 3, '2024-01-17 11:00:00'),
('Mars is known as the Red Planet', true, 3, '2024-01-17 11:01:00'),
('Neptune is the closest planet to the Sun', false, 3, '2024-01-17 11:02:00'),
('The Moon has its own light source', false, 1, '2024-01-18 12:00:00');

-- Link questions to games
INSERT INTO game_questions (game_id, question_id, question_order) VALUES
-- Science Trivia questions
(1, 1, 1),  -- Earth revolves around Sun
(1, 2, 2),  -- Water boiling point
(1, 3, 3),  -- Brain usage myth

-- History Facts questions
(2, 4, 1),  -- Great Wall visibility
(2, 5, 2),  -- WWII end date

-- Tech Quiz questions
(3, 6, 1),  -- ENIAC
(3, 7, 2),  -- JavaScript

-- Space Explorer questions
(4, 8, 1),  -- Mars Red Planet
(4, 9, 2),  -- Neptune position
(4, 10, 3); -- Moon light source
