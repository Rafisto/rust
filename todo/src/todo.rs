use std::fmt;

pub struct TodoList {
    list: Vec<TodoElement>,
}

pub struct TodoElement {
    description: String,
    done: bool,
}

impl TodoList {
    pub fn add_element(&mut self, description: &str, done: bool) {
        self.list.push(TodoElement {
            description: description.to_string(),
            done: done,
        })
    }
    pub fn remove_element(&mut self, index: usize) {
        if index > 0 && index <= self.list.iter().len() {
            self.list.remove(index - 1);
        }
    }
    pub fn mark(&mut self, index: usize, finished: bool) {
        if index > 0 && index <= self.list.iter().len() {
            let old_description = self.list.get(index - 1).unwrap();
            self.list[index - 1] = TodoElement {
                description: old_description.description.clone(),
                done: finished,
            };
        }
    }
}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut number = 1;
        for item in &self.list {
            if item.done == true {
                writeln!(f, "{}. [x] {}", number, item.description)?;
            } else {
                writeln!(f, "{}. [ ] {}", number, item.description)?;
            }
            number += 1;
        }
        Ok(())
    }
}

impl fmt::Display for TodoElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.done == true {
            return write!(f, "- [x] {}", self.description);
        } else {
            return write!(f, "- [ ] {}", self.description);
        }
    }
}

pub fn create_todo() -> TodoList {
    let mut todo_list = vec![];
    todo_list.push(TodoElement {
        description: "Create TODO list".to_string(),
        done: true,
    });
    return TodoList { list: todo_list };
}
