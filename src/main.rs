use std::collections::BTreeSet;


pub mod utils;
pub mod parsing;
pub mod cmd_line;

use crate::utils::*;
use crate::utils::{
    Hora,Dia,Bloque,Area,
    Horario,Curso
};


fn main() {

    println!("
    _       ____               ____     ______                                _           
    | |     / / /_  ___  ____  /  _/____/ ____/___  _________ ___  ____ ______(_)___  ____ 
    | | /| / / __ \\/ _ \\/ __ \\ / // ___/ /_  / __ \\/ ___/ __ `__ \\/ __ `/ ___/ / __ \\/ __ \\
    | |/ |/ / / / /  __/ / / // /(__  ) __/ / /_/ / /  / / / / / / /_/ / /__/ / /_/ / / / /
    |__/|__/_/ /_/\\___/_/ /_/___/____/_/    \\____/_/  /_/ /_/ /_/\\__,_/\\___/_/\\____/_/ /_/ 

    Escribe \"help\" para ver los comandos del programa.
");


    let horas : Vec<Hora> = vec![
        Hora::H07,Hora::H08,Hora::H09,Hora::H10,
        Hora::H11,Hora::H12,Hora::H13,Hora::H14,
        Hora::H15,Hora::H16,Hora::H17,Hora::H18,
        Hora::H19,Hora::H20,Hora::H21,Hora::H22,
    ];

    let dias : Vec<Dia> = vec![
        Dia::Lu,
        Dia::Ma,
        Dia::Mi,
        Dia::Ju,
        Dia::Vi,
        Dia::Sa,
        Dia::Do,
    ];

    let mut grupos : Vec<Grupo> = vec![];

    for hora in &horas{
        for dia in &dias{
            grupos.push(
                Grupo {
                    horario : Horario {
                        hora : hora.clone(),
                        dia : dia.clone(),
                    },
                    instructores : vec![],
                    militantes : vec![],
                }
            );
        }
    }

    let bloques : Vec<Bloque> = vec![
        Bloque::B1,
        Bloque::B2,
        Bloque::B3,
    ];

    let areas : Vec<Area> = vec![
        Area::A,
        Area::B,
        Area::C,
    ];

    let mut cursos : Vec<Curso> = vec![];

    for bloque in &bloques {
        for area in &areas {
            cursos.push(
                Curso {
                    tema : Tema {
                        bloque : bloque.clone(),
                        area : area.clone()
                    },
                    grupos : grupos.clone(),
                    instructores_que_imparten : BTreeSet::new(),
                    militantes_que_tomaran : BTreeSet::new(),
                }
            );
        }
    }

    let mut instructores = parsing::parsear_instructores();
    let mut militantes = parsing::parsear_militantes();
    

    utils::asignar_afiliades_a_cursos(&mut cursos, &instructores, &militantes);

    for mut curso in &mut cursos {
        generar_grupos_de_curso(&mut curso, &instructores, &militantes)
    }


    let grupos_confirmados : Vec<(Tema, Grupo)> = vec![];


    cmd_line::cmd_line(
        &mut cursos,
        &mut instructores, 
        &mut militantes,
    );

}








