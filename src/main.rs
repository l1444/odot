use std::io::{stdout, stdin, Write};
use serde_derive::{Serialize, Deserialize};
use directories::{ProjectDirs};
use std::path::Path;
use std::fs::*;
use clap::{AppSettings, Clap};
use console::{style, Color, Term};
use dialoguer::{Select, theme::ColorfulTheme};

#[derive(Clap, Debug)]
#[clap(name = "Odot", about = "Une application qui sauvegarde les tâches de ce que tu dois faire!", version = "1.0", author = "L14 <l14ms1@outlook.fr>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Commands {
    ///
    /// Créer une tâche
    ///
    #[clap(short, long)]
    create_task: bool,
    ///
    /// Lit toutes les tâches insérées
    ///
    #[clap(short, long)]
    read_task: bool,
    ///
    /// Supprime une tâche
    ///
    #[clap(short, long)]
    delete_task: bool,
}


#[derive(Serialize, Deserialize)]
struct Todo {
    category: Vec<Category>
}

#[derive(Serialize, Deserialize)]
struct Category {
    id: u64,
    title: String,
    tasks: Vec<Task>
}

#[derive(Serialize, Deserialize)]
struct Task {
    id: u64,
    title: String,
    description: String,
}

trait JsonParser {
    fn parse(data: Todo) -> Option<String> {
        match serde_json::to_string(&data) {
            Ok(json) => {
                Some(json)
            },
            Err(_) => None
        }
    }

    fn unparse(json: String) -> Todo {
        let t: Todo = serde_json::from_str(&*json).unwrap();
        return t
    }
}

struct JSONParse;
impl JsonParser for JSONParse {}

fn main() -> std::io::Result<()> {

    let opt = Commands::parse();
    let term = Term::stdout();
    term.set_title("odot");

    if let Some(proj_dirs) = ProjectDirs::from("dev", "L14",  "odot") {
        let file = format!("{}/todo.json", proj_dirs.config_dir().to_str().unwrap());
        if !Path::new(&file).exists() {
            let _ = create_dir(proj_dirs.config_dir());
            let _ = File::create(&file).unwrap();
        }
        if read_to_string(&file).unwrap() == String::from("") {
            let json = JSONParse::parse(Todo {
                category: vec![
                    Category {
                        id: 1,
                        title: "À faire".to_string(),
                        tasks: vec![Task {
                            id: 1,
                            title: "Hello World".to_string(),
                            description: "Ceci est une note qui a été généré par le logiciel afin de te remercier de l'avoir installé et de l'avoir utilisé une fois!!".to_string()
                        }]
                    }
                ]
            }).unwrap();
            let _ = write(&file, &json).unwrap();
        }
        fn show_tasks() {
            if let Some(proj_dirs) = ProjectDirs::from("dev", "L14",  "odot") {
                let mut categories = JSONParse::unparse(read_to_string(format!("{}/todo.json", proj_dirs.config_dir().to_str().unwrap())).unwrap()).category;
                println!("{}", style("[~] Sélectionne une liste : ").bold());
                let mut items = Vec::new();

                for category in &categories {
                    items.push(&*category.title)
                }

                items.push("Créer une nouvelle liste");
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .items(&items)
                    .default(0)
                    .interact_on_opt(&Term::stderr()).unwrap();

                match selection {
                    Some(index) => {
                         if items.len() - 1 == index {
                            let title = Input::new("[~] Le titre de la nouvelle liste : ");
                            categories.append(&mut vec![
                                Category {
                                    id: items.len() as u64,
                                    title,
                                    tasks: vec![Task {
                                        id: 1,
                                        title: "Hello World".to_string(),
                                        description: "Ceci est une tâche créait automatiquement par le logiciel dès la création d'une nouvelle liste!!".to_string()
                                    }]
                                }
                            ]);
                            let json = JSONParse::parse(Todo { category: categories }).unwrap();

                            let _ = write(format!("{}/todo.json", proj_dirs.config_dir().to_str().unwrap()), &json).unwrap();
                        }
                        let categories = JSONParse::unparse(read_to_string(format!("{}/todo.json", proj_dirs.config_dir().to_str().unwrap())).unwrap()).category;
                        let tasks = &categories[index].tasks;
                        if tasks.is_empty() == false {
                            for task in tasks {
                                println!("\n{}", style(&task.title).bold().fg(Color::Green));
                                for _ in 0..task.title.clone().len() {
                                    print!("-");
                                }
                                println!("\n{}\n", &task.description);
                            }
                        } else {
                            println!("{}", style("[!] Il n'y a aucune tâche à faire dans cette liste").fg(Color::Yellow));
                        }

                    }
                    None => println!("{}", style("[!] Une erreur s'est produite").fg(Color::Red))

                }
            }
        }
        // Le code est vraiment deguelasse, il est à refaire car je jongle avec la mémoire
        if opt.create_task {
            let mut categories = JSONParse::unparse(read_to_string(&file).unwrap()).category;
            println!("{}", style("[~] Sélectionne une liste : ").bold());
            let mut items = Vec::new();
            for category in &categories {
                items.push(&*category.title)
            }
            items.push("Créer une nouvelle liste");

            let selection = Select::with_theme(&ColorfulTheme::default())
                .items(&items)
                .default(0)
                .interact_on_opt(&Term::stderr()).unwrap();


            match selection {
                Some(index) => {
                    if items.len() - 1 == index {
                        println!("---- Création de la liste des tâches ----");
                        let title_list = Input::new("[~] Le titre de la nouvelle liste : ");
                        categories.append(&mut vec![
                            Category {
                                id: items.len() as u64,
                                title: title_list,
                                tasks: vec![Task {
                                    id: 1,
                                    title: "Hello World :)".to_string(),
                                    description: "Ceci est une tâche créait automatiquement par le logiciel dès la création d'une nouvelle liste!!".to_string()
                                }]
                            }
                        ]);
                        let json = JSONParse::parse(Todo { category: categories }).unwrap();
                        let _ = write(&file, &json).unwrap();
                        println!("---- Création de la tâche ----");
                    }

                    let title = Input::new("[~] Le titre de la tâche : ");
                    if title == String::from("") {
                        println!("{}", style("[!] Aïe, vous devez mettre un titre...").fg(Color::Yellow));
                        std::process::exit(1);
                    }
                    let description = Input::new("[~] La description de la tâche : ");

                    let mut categories = JSONParse::unparse(read_to_string(&file).unwrap()).category;
                    let mut task = swap(&mut categories.get_mut(index).unwrap().tasks);
                    let cat_info = categories.get(index).unwrap();

                    {
                        let mut cat = JSONParse::unparse(read_to_string(&file).unwrap()).category;
                        cat.retain(|x| x.id != cat_info.id);
                        let _ = write(&file, &JSONParse::parse(Todo { category : cat }).unwrap()).unwrap();
                    }
                    let mut categories = JSONParse::unparse(read_to_string(&file).unwrap()).category;

                    task.append(&mut vec![Task {
                        id: (task.len() + 1) as u64,
                        title,
                        description
                    }]);
                    categories.append(&mut vec![
                        Category {
                            id: cat_info.id,
                            title: (&*cat_info.title).to_string(),
                            tasks: task
                        }]);

                    let json = JSONParse::parse(Todo { category: categories } ).unwrap();
                    let _ = write(&file, &json).unwrap();
                    println!("{}", style("[!] La tâche a bien été ajouté!!").fg(Color::Green))
                }
                None => println!("{}", style("[!] Une erreur s'est produite dès lors de la création de la tâche!!").fg(Color::Red))
            }

        } else if opt.read_task {
            show_tasks();
        } else if opt.delete_task {
            let categories = JSONParse::unparse(read_to_string(format!("{}/todo.json", proj_dirs.config_dir().to_str().unwrap())).unwrap()).category;
            let mut items_cat = Vec::new();
            for category in categories  {
                items_cat.push(category.title)
            }
            println!("");
            println!("{}", style("[~] Sélectionne une liste de tâche : ").bold());
            let selection = Select::with_theme(&ColorfulTheme::default())
                .items(&items_cat)
                .default(0)
                .interact_on_opt(&Term::stderr()).unwrap();

             match selection {
                Some(index) => {
                    let mut categories = JSONParse::unparse(read_to_string(&file).unwrap()).category;
                    let tasks = swap(&mut categories.get_mut(index).unwrap().tasks);
                    let mut items = Vec::new();
                    for task in tasks.into_iter() {
                        items.push(task.title)
                    }
                    let mut selection;
                    if items.is_empty() != true {
                        println!("");
                        println!("{}", style("[~] Sélectionne la tâche que tu veux supprimer : ").bold());
                        selection = Select::with_theme(&ColorfulTheme::default())
                            .items(&items)
                            .default(0)
                            .interact_on_opt(&Term::stderr()).unwrap().unwrap();
                    } else {
                        println!("{}", style("[!] Oh bah, cette tâche est vide.. Pas de chance :')").fg(Color::Red));
                        std::process::exit(1);
                    }

                    let cat_info = categories.get(index).unwrap();

                    let mut categories = JSONParse::unparse(read_to_string(&file).unwrap()).category;
                    let mut tasks = swap(&mut categories.get_mut(index).unwrap().tasks);

                    {
                        let mut cat = JSONParse::unparse(read_to_string(&file).unwrap()).category;
                        cat.retain(|x| x.id != cat_info.id);
                        let _ = write(&file, &JSONParse::parse(Todo { category : cat }).unwrap()).unwrap();
                    }

                    tasks.retain(|x| x.title != items[selection]);

                    let mut categories = JSONParse::unparse(read_to_string(&file).unwrap()).category;

                    categories.append(&mut vec![
                        Category {
                            id: cat_info.id,
                            title: (&*cat_info.title).to_string(),
                            tasks
                        }]);

                    let json = JSONParse::parse(Todo { category: categories }).unwrap();
                    let _ = write(&file, &json).unwrap();
                    println!("{}", style("[!] La tâche a été supprimé!!").fg(Color::Green));

                    let mut categories = JSONParse::unparse(read_to_string(&file).unwrap()).category;
                    let mut tasks = swap(&mut categories.get_mut(index).unwrap().tasks);
                    if &tasks.is_empty() == &true {
                        let boolean = Input::new("[~] La liste de tâche est vide, veux-tu supprimer la liste? [Y/n] ");
                        if boolean.to_uppercase() == String::from("Y") {
                            let mut categories = JSONParse::unparse(read_to_string(&file).unwrap()).category;
                            categories.retain(|c| c.title != items_cat[index]);
                            let json = JSONParse::parse(Todo { category: categories }).unwrap();
                            let _ = write(&file, &json).unwrap();
                            println!("{}", style("[!] C'est bon, la liste a été supprimé!!").fg(Color::Green));
                        } else {
                            println!("{}", style("[!] Ok très bien :), la liste ne sera pas supprimer!!").fg(Color::Green));
                        }
                    }

                },
                None => println!("{}", style("[!] Une erreur s'est produite").fg(Color::Red))
            }
           } else {
            show_tasks();
        }
    }
    Ok(())
}

fn swap<T>(mut vec: &mut Vec<T>) -> Vec<T> {
    std::mem::replace(&mut vec, Vec::new())
}

struct Input;
impl Input {
    pub fn new(str: &str) -> String {
        let mut input = String::new();
        print!("{}", style(str).bold());
        let _ = stdout().flush();
        let _ = stdin().read_line(&mut input).unwrap();
        return input.replace("\r", "").replace("\n", "")
    }
}

