use std::io;
use std::str::Split;
use crate::utils::*;
use crate::utils::{Instructor,Militante};



pub fn cmd_line(
    cursos : &mut Vec<Curso>,
    instructores : &mut Vec<Instructor>,
    militantes : &mut Vec<Militante>,
) {
    loop {
        let mut input = String::new(); 
        println!("");
        std::io::stdin().read_line(&mut input).unwrap();
        input.pop().unwrap();
        let mut split = input.split(' ');
        let args : Vec<&str> = split.collect();

        match args[0] {

            "listar" => {

                if args.len() != 2 {
                    println!("Número de argumentos equivocado.");
                    println!("Opciones:\n\tlistar [instructores | militantes]");
                    continue
                }

                match args[1] {
                    "instructores" => listar_afiliades(instructores),
                    "militantes" => listar_afiliades(militantes),
                    _ => println!("Opciones:\n\tlistar [instructores | militantes]"),
                }

            },

            "buscar" => {

                if args.len() != 3 {
                    println!("Número de argumentos equivocado.");
                    println!("Opciones:\n\tbuscar [instructores | militantes] [nombre]");
                    continue
                }

                match args[1] {
                    "instructores" => { buscar_id_afiliade( args[2], instructores); },
                    "militantes" => { buscar_id_afiliade( args[2], militantes); },
                    _ => println!("Opciones:\n\tbuscar [instructores | militantes] id "),
                }
                
            },

            "info" => {
                if args.len() != 3 {
                    println!("Número de argumentos equivocado.");
                    println!("Opciones:\n\tinfo [instructores | militantes] [nombre|id]");
                    continue
                }

                match args[1] {
                    "instructores" => mostrar_afiliade(args[2],instructores),
                    "militantes" => mostrar_afiliade(args[2],militantes),
                    _ => println!("Opciones\n\tinfo [instructores | militantes] [nombre | id]"),
                }

            },

            "mostrar" => {
                if args.len() != 3 {
                    println!("Número de argumentos equivocado.");
                    println!("Opciones:\n\tmostrar curso [id]");
                    continue
                }
                mostrar_curso(cursos, args[2], instructores, militantes);
            },

            "help" => {
                help();
            },

            "q" => {
                break;
            },
            _ => println!("Comando no reconocido."),
        }
    }
}