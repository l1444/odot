use serde::{Deserialize, Serialize};
use directories::ProjectDirs;
use std::path::Path;
use std::fs::*;
use std::io::{stdout, stdin, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Categories {
    category: Vec<Category>
}

#[derive(Serialize, Deserialize, Debug)]
struct Category {
    id: i64,
    name: String,
    task: Vec<Task>, 
    finish: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: i64,
    title: String,
    description: String,
    finish: bool
}


pub mod json {
    use serde::{Serialize};

    // il faut que toutes les structs supportent..
    pub fn parse<T>(obj: &T) -> String
    where T: ?Sized + Serialize {
        return serde_json::to_string(&obj).unwrap()
    }

    pub fn unparse<'a, T>(str: &'a str) -> T
    where T: serde::de::Deserialize<'a> {
        return serde_json::from_str(&*str).unwrap()
    }
}


impl Categories {

    // Pouvoir recuperer l'id de la category et ses élements associés


}

impl Category {

    // Pouvoir recuperer l'id de la category et ses élements associés

    // Pouvoir recuperer toutes les catégories

    // Modifier une catégorie (titre indépendamment)

    // Supprimer une catégorie (avec une booléan, mettre finish = true)
    
}

impl Task {

    // Pouvoir recuperer l'id de la tache et ses élements associés

    // Pouvoir recuperer toutes les taches

    // Modifier une tache (titre indépendamment, description et etc...)

    // Supprimer une tache (avec une booléan, mettre finish = true)
}

// un moyen de sauvegarder, consulter, modifier et supprimer cela d'une très bonne façon

// faire de l'homme-machine efficace

// facultatif: mais de faire un moyen pour le GUI (oui)

// un système de networking avec un transfert de note face a un réseau

// un moyen de transfert de fichier, dossier ou de fichier compressé en réseau


fn main() {

    if let Some(proj_dirs) = ProjectDirs::from("dev", "L14",  "odot") {
        let folder = proj_dirs.config_dir().to_str().unwrap();
        let file   = format!("{}", Path::new(&folder).join("todo.json").to_str().unwrap());
        
        if !Path::new(&file).exists() {
            println!("{}\n{}", &folder, &file);
            create_dir(proj_dirs.config_dir());
            File::create(&file).unwrap();
        }    

    }


    let js = json::parse(&Categories{
        category: vec![
            Category {
                id: 1, 
                name: "oui".to_owned(),
                task: vec![
                    Task {
                        id: 1,
                        title: "oieee".to_owned(),
                        description: "odjivjv".to_owned(),
                        finish: true
                    }
                ],
                finish: false
            }
        ]
    });


    println!("{}", js);
    println!("---");

    let ob: Categories = json::unparse(&*js);
    println!("{:?}", ob)
}
