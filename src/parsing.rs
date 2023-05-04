use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use std::str::Split;
use std::collections::BTreeSet;

use crate::utils::*;


pub fn parsear_militantes() -> Vec<Militante> {
    
    let mut militantes : Vec<Militante> = vec![];

    if let Ok(lineas) = read_lines("./militantes.tsv"){
        for (i, linea_result) in lineas.enumerate() {
            let militante = match linea_result {
                Ok(linea) => parsear_militante(i, linea),
                Err(_) => panic!("Error leyendo archivo militantes.csv"), 
            };
            militantes.push(militante);
        }
    }
    println!("{} militantes procesados!", militantes.len() - 1);
    militantes
}

fn parsear_militante(id : usize, linea : String) -> Militante {
    let mut split = linea.split("\t");
    split.next();
    let nombre = parsear_nombre(&mut split);
    let disponibilidad = parsear_horarios(&mut split);
    split.next(); // Modalidad
    let temas_que_lleva = 
        temas_por_cursar(parsear_temas_militantes(&mut split));

    Militante::new(id, nombre, temas_que_lleva,disponibilidad)
}


pub fn parsear_instructores() -> Vec<Instructor> {
    let mut instructores : Vec<Instructor> = vec![];

    if let Ok(lineas) = read_lines("./instructores.tsv"){
        for (i, linea_result) in lineas.enumerate() {
            let instructor = match linea_result {
                Ok(linea) => parsear_instructor(i, linea),
                Err(_) => panic!("Error leyendo archivo instructores.tsv"), 
            };
            instructores.push(instructor);
        }
    }
    println!("{} instructores procesados!", instructores.len() - 1);
    instructores
}

fn parsear_instructor(id : usize, linea : String) -> Instructor {
    let mut split = linea.split("\t");
    split.next();
    let nombre = parsear_nombre(&mut split);
    let disponibilidad = parsear_horarios(&mut split);
    split.next(); // Modalidad
    let temas_que_imparte = 
        parsear_temas_instructores(&mut split);

    Instructor::new(id, nombre, temas_que_imparte, disponibilidad)
}



fn parsear_nombre(split : &mut Split<'_,&str>) -> String {
    match split.next() {
        Some(nombre) => nombre.to_string(),
        None => panic!("Error parseando nombre de militante.")
    }
}

fn parsear_horarios(split : &mut Split<'_,&str>) -> BTreeSet<Horario> {

    let mut disponibilidad : BTreeSet<Horario> = BTreeSet::new();

    let horas : Vec<Hora> = vec![
        Hora::H07,Hora::H08,Hora::H09,Hora::H10,
        Hora::H11,Hora::H12,Hora::H13,Hora::H14,
        Hora::H15,Hora::H16,Hora::H17,Hora::H18,
        Hora::H19,Hora::H20,Hora::H21,Hora::H22,
    ];

    let dias : Vec<(&str,Dia)> = vec![
            ("Lunes",       Dia::Lu),
            ("Martes",      Dia::Ma),
            ("Miércoles",   Dia::Mi),
            ("Jueves",      Dia::Ju),
            ("Viernes",     Dia::Vi),
            ("Sábado",      Dia::Sa),
            ("Domingo",     Dia::Do),
    ];

    for hora in &horas {
        let dias_string = match split.next() {
            Some(s) => s,
            None => panic!("Error parseando archivo.")
        };

        for dia in &dias {
            if dias_string.contains(dia.0){
                disponibilidad.insert(
                    Horario {
                        hora : hora.clone(),
                        dia : dia.1.clone(),
                    }
                );
            }
        }
    }

    disponibilidad
}


fn parsear_temas_militantes(split : &mut Split<'_,&str>) -> BTreeSet<Tema> {
    let bloques = vec![Bloque::B1, Bloque::B2, Bloque::B3];
        
    let mut temas_cursados : BTreeSet<Tema> = BTreeSet::new();

    for bloque in &bloques {

        let areas = match split.next() {
            Some(s) => s,
            None => panic!("Error parseando los temas de militantes.")
        };

        if areas.contains("Filosofía") {
            temas_cursados.insert(Tema::new(bloque.clone(), Area::A));
        }

        if areas.contains("Economía Política") {
            temas_cursados.insert(Tema::new(bloque.clone(), Area::B));
        }

        if areas.contains("Comunismo Cientifico") {
            temas_cursados.insert(Tema::new(bloque.clone(), Area::C));
        }
    }

    temas_cursados
}

fn parsear_temas_instructores(split : &mut Split<'_,&str>) -> BTreeSet<Tema> {
    
    let area = match split.next() {
        Some(s) => s,
        None => panic!("Error parseando las áreas de Instructores"),
    };

    let area = match area {
        s if s.contains("Filosofía") => Area::A,
        s if s.contains("Economía Política") => Area::B,
        s if s.contains("Comunismo Científico") => Area::C,
        _ => Area::A, // default
    };
    
    let mut bloques : BTreeSet<Tema> = BTreeSet::new();

    let bloques_str = match split.next() {
        Some(s) => s,
        None => panic!("Error parseando las áreas de Instructores"),
    };

    if bloques_str.contains("1") {
        bloques.insert(Tema::new(Bloque::B1,area.clone()));
    }
    
    if bloques_str.contains("2") {
        bloques.insert(Tema::new(Bloque::B2,area.clone()));
    }
    
    if bloques_str.contains("3") {
        bloques.insert(Tema::new(Bloque::B3,area.clone()));
    }

    bloques
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file_result = File::open(filename);
    match file_result {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(_) => panic!("No se pudo leer el archivo militante.tsv o instructor.tsv"),
    }
}
