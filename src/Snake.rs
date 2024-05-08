struct Snake {
    body: Vec<(usize, usize)>,
    direction: Direction,
}

impl Snake {
    fn new(start_x: usize, start_y: usize) -> Snake {
        let initial_body = vec![(start_x, start_y)];
        Snake {
            body: initial_body,
            direction: Direction::Right,
        }
    }
    fn move_forward(&mut self) {
        let (head_x, head_y) = self.body[0];
        let new_head = match self.direction {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        };
        self.body.insert(0, new_head);
        self.body.pop();
    }
    fn change_direction(&mut self, new_direction: Direction) {
        if new_direction.opposite() != self.direction {
            self.direction = new_direction;
        }
    }
    fn grow(&mut self) {
        let last_segment = *self.body.last().unwrap();
        self.body.push(last_segment);
    }
}