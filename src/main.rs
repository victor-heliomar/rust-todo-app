use std::{
    io,
    io::{ BufRead, Error, Read, Write },
    fs::{ OpenOptions, write },
};

#[derive(Debug)]
struct Todo {
    task: String,
    done: bool
}

impl Todo {
    fn create(task: String, done: bool) -> Todo {
        Todo { task, done }
    }

    fn save(&self) -> Result<(), Error> {
        let task = format!("{}:{}", self.task, self.done);
        let mut file = OpenOptions::new()
                                        .write(true)
                                        .append(true)
                                        .open("todo.txt")
                                        .expect("Algo ha fallado al abrir el archivo");
        writeln!(file, "{}", task).expect("Algo ha fallado al guardar el texto.");
        Ok(())
    }
}

const EXIT: &str = "break";
const CREATE: &str = "create";
const COMPLETE: &str = "complete";
const DELETE: &str = "delete";
const SHOW: &str = "show";

const SEE_ALL: &str = "all";
const SEE_COMPLETED: &str = "completed";
const SEE_TODO: &str = "todo";

fn main() {
    let mut todo: Vec<Todo> = all_todo().expect("Ha ocurrido un error al obtener la lista de tareas");

    loop {
        let mut task = String::new();
        let stdin = io::stdin();

        println!("\nQué acción deseas realizar?\ncommands: [ {} | {} | {} | {} ]\n\nEscribe 'break' para salir\n", SHOW, CREATE, COMPLETE, DELETE);
        let action = stdin.lock().lines().next().unwrap().unwrap();

        if action == EXIT {
            break
        };

        if (action != COMPLETE && action != DELETE) && (action == SHOW || action == CREATE) {
            let mut message = String::new();

            if action == SHOW { 
                message = format!("Que tipo de tareas quieres ver?\ncommands: [ {} | {} | {} ]", SEE_ALL, SEE_COMPLETED, SEE_TODO);
            } else if action == CREATE { 
                message = "Cual es el nombre de tu tarea?".to_string()
            } else if action == COMPLETE {
                message = "Ingresa el nombre de la tarea que quieres completar".to_string() 
            } else if action == DELETE { 
                message = "Ingresa el nombre de la tarea que quieres eliminar".to_string()
            }

            println!("{}", message);
            task = stdin.lock().lines().next().unwrap().unwrap();
        }
        
        match action.as_ref() {
            "show" => show_todo(&mut todo, task),
            "create" => create_todo(&mut todo, task),
            "complete" => complete_or_delete_task(&mut todo, action),
            "delete" => complete_or_delete_task(&mut todo, action),
            _ => println!("La opción elegida es invalida")
        }
    }
}

fn show_todo(todo: &mut Vec<Todo>, status_filter: String) {
    println!("Lista de tareas por hacer\n");
    for task in todo {
        let status = if task.done { "Completada" } else { "Por hacer" };

        if status_filter == SEE_COMPLETED && task.done || status_filter == SEE_TODO && !task.done || status_filter == SEE_ALL {
            println!("{} - {}", task.task, status);
        }
    }
}

fn create_todo(todo: &mut Vec<Todo>, task: String) {
    let todo_instance = Todo::create(task, false);
    match todo_instance.save() {
        Ok(_) => {
            todo.push(todo_instance);
            println!("La tarea ha sido guardada correctamente");
        }
        Err(_) => { println!("Ha ocurrido un error"); }
    }
}

fn complete_or_delete_task(todo: &mut Vec<Todo>, action: String) {
    let mut count = 0u32;
    let stdin = io::stdin();

    let mut body = String::new();
    let mut undone_todo_tasks = String::new();
    let mut all_todo_tasks = String::new();

    for task in todo.iter().clone() {
        count += 1;

        let current_task = format!("{}:{}\n", &task.task, &task.done);
        
        if action == DELETE {
            all_todo_tasks.push_str(&format!("{}. {} - {}\n", count, &task.task, &task.done));
        } else if action == COMPLETE && !&task.done {
            undone_todo_tasks.push_str(&format!("{}. {} - {}\n", count, &task.task, &task.done));
        } 
        
        body.push_str(&current_task);
    }

    println!("Selecciona la tarea por el ID:");
    loop {
        println!("{}", if action == DELETE { &all_todo_tasks } else { &undone_todo_tasks });

        let task_id = stdin.lock().lines().next().unwrap().unwrap().trim().to_string();

        match task_id.parse::<usize>() {
            Ok(id) => {
                if id > 0 && id <= count.try_into().unwrap() {
                    let ref selected_task: Todo = todo[id - 1];
                    let start_line_break: &str = if id > 1 { "\n" } else { "" };
                    let end_line_break: &str = if id == 1 && action == DELETE { "\n" } else { "" };
                    let formatted_task = format!("{}{}:{}{}", start_line_break, selected_task.task, selected_task.done, end_line_break);

                    if action == COMPLETE {
                        let completed_task = format!("{}:true", selected_task.task);
                        
                        body = body.replace(&formatted_task, &completed_task);
                        
                        match write("todo.txt", &body) {
                            Ok(_) => {
                                todo[id - 1].done = true;
                                println!("La tarea fue marcada como completada.\nBuen trabajo!")
                            },
                            Err(error) => println!("Ha ocurrido un error: {}", error),
                        }
                    }

                    if action == DELETE {
                        body = body.replace(&formatted_task, "");

                        match write("todo.txt", &body) {
                            Ok(_) => {
                                todo.remove(id - 1);
                                println!("La tarea fue eliminada con exito")
                            },
                            Err(error) => println!("Ha ocurrido un error: {}", error),
                        }
                    }
                    break;
                } else {
                    println!("El número que elegiste es inválido, por favor intenta con alguno de los siguientes números:")
                }
            },
            Err(..) => println!("Por favor ingresa un número, en lugar de: {}", task_id),
        };
    }

}

fn all_todo() -> Result<Vec<Todo>, Error> {
    let mut file = OpenOptions::new()
                                    .write(true)
                                    .create(true)
                                    .read(true)
                                    .open("todo.txt")
                                    .expect("Ha ocurrido un error al intentar abrir el archivo");
    let mut body = String::new();
    file.read_to_string(&mut body).expect("No se ha podido leer el archivo.");
    let mut list: Vec<Todo> = Vec::new();
    for line in body.lines() {
        // task:[false|true]
        let task = line.split(':').collect::<Vec<&str>>();
        list.push(
            Todo::create(task[0].to_string(), task[1].parse().unwrap())
        );
    }

    Ok(list)
}