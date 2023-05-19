use crate::utils::*;
use crate::utils::{Instructor,Militante};



pub fn cmd_line(
    cursos : &mut Vec<Curso>,
    grupos_confirmados : &mut Vec<(Tema, Grupo)>,
    instructores : &mut Vec<Instructor>,
    militantes : &mut Vec<Militante>,
) {
    loop {
        let mut input = String::new(); 
        println!("");
        std::io::stdin().read_line(&mut input).unwrap();
        let trimmed_input = input.trim();
        let split = trimmed_input.split(' ');
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
                if args.len() < 3 || args.len() > 4 {
                    println!("Número de argumentos equivocado.");
                    println!("\tmostrar curso [id]");
                    println!("\tmostrar grupo [curso_id] [horario_id]");
                    continue
                }

                if args[1] == "curso" {
                    if args.len() == 3 {
                        mostrar_curso(cursos, args[2], instructores, militantes);
                    }
                    else {
                        println!("Número de argumentos equivocado.");
                        println!("Opciones:\n\tmostrar curso [id]");
                    }
                    continue
                }
                else if args[1] == "grupo" {
                    if args.len() == 4 {
                        mostrar_grupo(cursos, instructores, militantes, args[2], args[3]);
                    }
                    else {
                        println!("Número de argumentos equivocado.");
                        println!("Opciones:\n\tmostrar grupos [grupo_id] [horario_id]");
                    }
                    continue
                }

                println!("Argumento equivocado.");
                println!("Opciones:\n\tmostrar [curso | grupo] ...");  
            },

            "crear" => {
                if args.len() == 3 {
                    crear_grupo(cursos, grupos_confirmados, instructores, militantes, args[1], args[2], "", "");
                }
                else if args.len() == 5 {
                    crear_grupo(cursos, grupos_confirmados, instructores, militantes, args[1], args[2], args[3], args[4]);
                }
                else {
                    crear_grupo_help("Número de argumentos equivocado.");
                }

            },

            "inscribir" => {
                if args.len() != 4 {
                    println!("Número de argumentos equivocado.");
                    inscribir_help();
                    continue
                }

                match args[1] {
                    "instructores" => inscribir_afiliades_curso(instructores, args[2], args[3]),
                    "militantes" => inscribir_afiliades_curso(militantes, args[2], args[3]),
                    _ => inscribir_help(),
                }

                generar_grupos_todos_cursos(cursos, instructores, militantes);

            },

            "desinscribir" => {
                if args.len() != 4 {
                    println!("Número de argumentos equivocado.");
                    desinscribir_help();
                    continue
                }

                match args[1] {
                    "instructores" => desinscribir_afiliades_curso(instructores, args[2], args[3]),
                    "militantes" => desinscribir_afiliades_curso(militantes, args[2], args[3]),
                    _ => desinscribir_help(),
                }

                generar_grupos_todos_cursos(cursos, instructores, militantes);

            },


            "ayuda" => {
                help();
            },

            "creados" => {
                mostrar_grupos_creados(grupos_confirmados, instructores, militantes);
            },

            "q" => {
                mostrar_grupos_creados(grupos_confirmados, instructores, militantes);
                break;
            },
            _ => println!("Comando no reconocido."),
        }
    }
}