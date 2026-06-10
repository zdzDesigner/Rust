#[derive(Debug, Clone)]
pub struct TodoItem {
    pub text: String,
    pub completed: bool,
}

pub struct App {
    pub todos: Vec<TodoItem>,
    pub selected: usize,
    pub add_mode: bool,
    pub input: String,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            todos: vec![
                TodoItem {
                    text: "Learn Rust".into(),
                    completed: false,
                },
                TodoItem {
                    text: "Build TUI app".into(),
                    completed: false,
                },
                TodoItem {
                    text: "Ship it!".into(),
                    completed: false,
                },
            ],
            selected: 0,
            add_mode: false,
            input: String::new(),
            should_quit: false,
        }
    }

    pub fn next(&mut self) {
        if !self.todos.is_empty() {
            self.selected = (self.selected + 1) % self.todos.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.todos.is_empty() {
            self.selected = if self.selected == 0 {
                self.todos.len() - 1
            } else {
                self.selected - 1
            };
        }
    }

    pub fn toggle(&mut self) {
        if let Some(todo) = self.todos.get_mut(self.selected) {
            todo.completed = !todo.completed;
        }
    }

    pub fn delete(&mut self) {
        if !self.todos.is_empty() {
            self.todos.remove(self.selected);
            if self.selected >= self.todos.len() && !self.todos.is_empty() {
                self.selected = self.todos.len() - 1;
            }
        }
    }

    pub fn confirm_add(&mut self) {
        if !self.input.is_empty() {
            self.todos.push(TodoItem {
                text: self.input.clone(),
                completed: false,
            });
            self.input.clear();
            self.add_mode = false;
            self.selected = self.todos.len() - 1;
        }
    }
}
