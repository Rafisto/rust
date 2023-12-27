mod todo;

fn main() {
    println!("I. Create a TODO list.");
    let mut todo_list = todo::create_todo();
    println!("{}", todo_list);
    println!("II. Add three elements to the TODO list.");
    todo_list.add_element("Add an element", true);
    todo_list.add_element("Add an unfinished element", false);
    todo_list.add_element("Add an unintentional element", false);
    println!("{}", todo_list);
    println!("III. Remove the 4th element.");
    todo_list.remove_element(4);
    println!("{}", todo_list);
    println!("IV. Mark the 3rd element as finished.");
    todo_list.mark(3, true);
    println!("{}", todo_list);
    println!("V. Mark the 3rd element as unfinished.");
    todo_list.mark(3, false);
    println!("{}", todo_list);
}
