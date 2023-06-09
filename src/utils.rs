
use colored::Colorize;
use std::fmt::Write as _;
use std::collections::BTreeSet;

pub enum Modalidad {
    P,
    V,
    M,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub enum Hora {
    H07,H08,H09,H10,
    H11,H12,H13,H14,
    H15,H16,H17,H18,
    H19,H20,H21,H22,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub enum Dia {
    Lu,
    Ma,
    Mi,
    Ju,
    Vi,
    Sa,
    Do,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub enum Area{
    A,
    B,
    C,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub enum Bloque{
    B1,
    B2,
    B3,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct Horario {
    pub hora : Hora,
    pub dia : Dia,
}

impl Horario {
    pub fn new(hora : Hora, dia: Dia) -> Horario {
        Horario { hora, dia }
    }
}


#[derive(Debug,Copy,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct Tema {
    pub bloque : Bloque,
    pub area : Area,
}

impl Tema {
    pub fn new( bloque : Bloque, area : Area ) -> Tema {
        Tema { bloque, area }
    }
}

#[derive(Debug,Clone)]
pub struct Curso {
    pub tema : Tema,
    pub grupos : Vec<Grupo>,
    pub instructores_que_imparten : BTreeSet<usize>,
    pub militantes_que_tomaran : BTreeSet<usize>,
    pub militantes_sin_coincidencia : BTreeSet<usize>, 
}

impl Curso {
    pub fn new( tema : Tema , grupos : Vec<Grupo> ) -> Curso {
        Curso {
            tema,
            grupos,
            instructores_que_imparten : BTreeSet::new(),
            militantes_que_tomaran : BTreeSet::new(),
            militantes_sin_coincidencia : BTreeSet::new(),
        }
    }
}

#[derive(Debug,Clone)]
pub struct Grupo {
    pub horario : Horario,
    pub instructores : BTreeSet<usize>,
    pub militantes : BTreeSet<usize>,
}

impl Grupo {
    pub fn new( horario : Horario ) -> Grupo {
        Grupo {
            horario,
            instructores : BTreeSet::new(),
            militantes : BTreeSet::new(),
        }
    }
}

#[derive(Debug,Clone)]
pub struct Instructor {
    pub id : usize,
    pub nombre : String,
    pub temas_que_imparte : BTreeSet<Tema>,
    pub disponibilidad : BTreeSet<Horario>,
}

impl Instructor {
    pub fn new(
        id: usize,
        nombre : String,
        temas_que_imparte : BTreeSet<Tema>,
        disponibilidad : BTreeSet<Horario>,
    ) -> Instructor {
        Instructor {
            id,
            nombre,
            temas_que_imparte,
            disponibilidad,
        }
    }
}

#[derive(Debug,Clone)]
pub struct Militante {
    pub id : usize,
    pub nombre : String,
    pub temas_que_lleva :BTreeSet<Tema>,
    pub disponibilidad : BTreeSet<Horario>,
}

impl Militante {
    pub fn new(
        id : usize,
        nombre : String,
        temas_que_lleva : BTreeSet<Tema>,
        disponibilidad : BTreeSet<Horario>,
    ) -> Militante {

        Militante {
            id,
            nombre,
            temas_que_lleva,
            disponibilidad,
        }
    }
}


pub trait Afiliade {
    fn get_id(&self) -> usize;
    fn get_nombre(&self) -> &str;
    fn get_temas(&self) -> &BTreeSet<Tema>;
    fn get_mut_temas(&mut self) -> &mut BTreeSet<Tema>;
    fn get_disponibilidad(&self) -> &BTreeSet<Horario>;
    fn info(&self);
}

impl Afiliade for Instructor {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_nombre(&self) -> &str {
        self.nombre.as_str()
    }
    fn get_temas(&self) -> &BTreeSet<Tema> {
        &self.temas_que_imparte
    }
    fn get_mut_temas(&mut self) -> &mut BTreeSet<Tema> {
        &mut self.temas_que_imparte
    }
    fn get_disponibilidad(&self) -> &BTreeSet<Horario> {
        &self.disponibilidad
    }

    fn info(&self) {
        println!(" Nombre: {}", &self.nombre);
        
        println!(" Temas que imparte:");
        for tema in &self.temas_que_imparte {
            println!("\t -{}", tema.to_string() )
        } 
    }

}

impl Afiliade for Militante {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_nombre(&self) -> &str {
        self.nombre.as_str()
    }
    fn get_temas(&self) -> &BTreeSet<Tema> {
        &self.temas_que_lleva
    }
    fn get_mut_temas(&mut self) -> &mut BTreeSet<Tema> {
        &mut self.temas_que_lleva
    }
    fn get_disponibilidad(&self) -> &BTreeSet<Horario> {
        &self.disponibilidad
    }

    fn info(&self) {
        println!(" Nombre: {}", &self.nombre);
        
        println!(" Temas que requiere:");
        for tema in &self.temas_que_lleva {
            println!("\t- {}", tema.to_string() )
        } 
    }

}

impl Tema {

    fn indice(&self) -> usize {
        let x = match self.bloque {
            Bloque::B1 => 0,
            Bloque::B2 => 1,
            Bloque::B3 => 2,
        };

        let y = match self.area {
            Area::A => 0,
            Area::B => 1,
            Area::C => 2,
        };

        3*x + y
    }

    fn to_string(&self) -> String {

        let mut string = match &self.area {
            Area::A => String::from("Filsofía "),
            Area::B => String::from("Economía Política "),
            Area::C => String::from("Comunismo Científico "),
        };

        match &self.bloque {
            Bloque::B1 => string.push_str(" I"),
            Bloque::B2 => string.push_str(" II"),
            Bloque::B3 => string.push_str(" III"),
        }

        string
    }
}


impl Horario { 

    fn indice( &self ) -> usize {
        
        let x = match &self.hora {
            Hora::H07 =>  0, Hora::H08 =>  1, Hora::H09 =>  2, Hora::H10 =>  3,
            Hora::H11 =>  4, Hora::H12 =>  5, Hora::H13 =>  6, Hora::H14 =>  7,
            Hora::H15 =>  8, Hora::H16 =>  9, Hora::H17 => 10, Hora::H18 => 11,
            Hora::H19 => 12, Hora::H20 => 13, Hora::H21 => 14, Hora::H22 => 15,
        };
        
        let y = match &self.dia {
            Dia::Lu => 0,
            Dia::Ma => 1,
            Dia::Mi => 2,
            Dia::Ju => 3,
            Dia::Vi => 4,
            Dia::Sa => 5,
            Dia::Do => 6,
        };

        7*x + y
    }

    fn to_string( &self ) -> String {
        
        let mut dia = match &self.dia {
            Dia::Lu => String::from("Lunes "),
            Dia::Ma => String::from("Martes "),
            Dia::Mi => String::from("Miércoles "),
            Dia::Ju => String::from("Jueves "),
            Dia::Vi => String::from("Viernes "),
            Dia::Sa => String::from("Sábado "),
            Dia::Do => String::from("Domingo "),
        };

        dia.push_str( match &self.hora {
            Hora::H07 => "07:00 - 08:00", Hora::H08 => "08:00 - 09:00",
            Hora::H09 => "09:00 - 10:00", Hora::H10 => "10:00 - 11:00",
            Hora::H11 => "11:00 - 12:00", Hora::H12 => "12:00 - 13:00",
            Hora::H13 => "13:00 - 14:00", Hora::H14 => "14:00 - 15:00",
            Hora::H15 => "15:00 - 16:00", Hora::H16 => "16:00 - 17:00",
            Hora::H17 => "17:00 - 18:00", Hora::H18 => "18:00 - 19:00",
            Hora::H19 => "19:00 - 20:00", Hora::H20 => "20:00 - 21:00",
            Hora::H21 => "21:00 - 22:00", Hora::H22 => "22:00 - 23:00",
            }
        );

        dia
    }

}


pub fn mostrar_disponibilidad(disponibilidad : &BTreeSet<Horario>) {

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

    let horas_string = vec![
        " 07:00\t"," 08:00\t"," 09:00\t"," 10:00\t",
        " 11:00\t"," 12:00\t"," 13:00\t"," 14:00\t",
        " 15:00\t"," 16:00\t"," 17:00\t"," 18:00\t",
        " 19:00\t"," 20:00\t"," 21:00\t"," 22:00\t",
    ];

    println!(" Hora\t  Lu\t   Ma\t   Mi\t   Ju\t   Vi\t   Sa\t   Do");
    
    for (i,hora) in horas.iter().enumerate() {
        let mut linea = horas_string.get(i).unwrap().to_string();
        for dia in &dias {
            let horario = Horario {hora : hora.clone(), dia: dia.clone()};
            if disponibilidad.contains(&horario){
                linea.push_str("▮▮▮▮▮▮▮\t");
            }
            else{
                linea.push_str("    \t");
            }
        }
        print!("{}",linea);
        println!("\t");
    }
}


/*
// HELP
*/

pub fn help() {
    println!("
Militantes e instructores:
    listar [instructores | militantes]
    buscar [instructores | militantes] [nombre | id]
    info [instructores | militantes] [nombre | id]

Cursos y grupos:
    mostrar curso [id]
    mostrar grupo <curso_id> <grupo_id>
    crear grupo <curso_id> <horario> [-x <lista> | -i <lista>]
    inscribir [instructor | militante] [nombre | id] [curso id]
    desinscribir [instructor | militante] [nombre | id] [curso id]
    creados
    
Salir
    q
")
}


/*
// LISTAR
*/

pub fn listar_afiliades<T: Afiliade>(afiliades : &Vec<T>) -> () {
    for (i,afiliade) in afiliades.iter().enumerate() {
        println!("\t{}. {}", i, &afiliade.get_nombre());
    }
}

fn listar_afiliades_de_curso<T:Afiliade>( ids : &BTreeSet<usize>,  afiliades : &Vec<T>) -> () {
    
    for id in ids {
        println!("\t{}. {}", &afiliades[*id].get_id(), &afiliades[*id].get_nombre() );
    }

}

fn listar_afiliades_de_grupo<T:Afiliade>( ids : &BTreeSet<usize>,  afiliades : &Vec<T>) -> () {
    
    for id in ids {
        println!("\t{}. {}", &afiliades[*id].get_id(), &afiliades[*id].get_nombre() );
    }

}

/*
// BUSCAR
*/

fn buscar_afiliade_por_nombre<T: Afiliade>(nombre: &str, lista : &Vec<T>) -> Result<usize,()> {
    let mut resultados : Vec<(usize, &str)> = vec![];
    let nombre = nombre.to_lowercase();

    for afiliade in lista {
        if afiliade.get_nombre().to_lowercase().contains(&nombre) {
            resultados.push( (afiliade.get_id(), afiliade.get_nombre()) );
        }
    }

    if resultados.len() == 1 {
        return Ok(resultados.get(0).unwrap().0 as usize)
    }
    else if resultados.len() > 1  {
        println!("Múltiples coincidencias para: {}", nombre);
        for afiliade in resultados {
            println!("\t{}. {}", &afiliade.0, afiliade.1)
        }
        Err( () )
    }
    else {
        println!("Sin coincidencias para: {}", nombre );
        Err( () )
    }
}

fn buscar_afiliade<T: Afiliade>( query : &str, lista : &Vec<T> ) -> Result<usize,()> {
    
    if let Ok(id) = query.parse::<usize>() {

        if 0 < id && id < lista.len() {
            return Ok(id);
        }
        else {
            println!("El id que introdujiste no es válido.");
            return Err( () );
        }
    }
    else {

        match buscar_afiliade_por_nombre(query, lista) {
            Ok(id) => return Ok(id),
            Err( () ) => Err( () ),
        }

    }
}

pub fn buscar_id_afiliade<T: Afiliade>( nombre : &str, lista : &Vec<T> ) {

    match buscar_afiliade(nombre, lista) {
        Ok(id) => println!("\t{}. {}", lista[id as usize].get_id(), lista[id as usize].get_nombre() ),
        Err(()) => (),
    }

}



/*
// MOSTRAR
*/

pub fn mostrar_afiliade<T:Afiliade>( query : &str, afiliades : &Vec<T> ) {

    if let Ok(id) = buscar_afiliade(query,afiliades) {
        let afiliade = &afiliades[id];
        
        afiliade.info();
        println!("");
        
        mostrar_disponibilidad( afiliade.get_disponibilidad() );
    }

}

pub fn temas_por_cursar( temas_cursados : BTreeSet<Tema> ) -> BTreeSet<Tema> {
    
    let bloques = vec![Bloque::B1, Bloque::B2, Bloque::B3];
    let areas = vec![Area::A, Area::B, Area::C];
    
    for bloque in &bloques {
        let mut temas_por_cursar : BTreeSet<Tema> = BTreeSet::new();
        for area in &areas {
            let tema = Tema::new(bloque.clone(),area.clone());
            if !( temas_cursados.contains(&tema) ) {
                temas_por_cursar.insert(tema.clone()); 
            }   
        }
        if temas_por_cursar.len() != 0 {
            return temas_por_cursar;
        }
    }
    BTreeSet::new()
}


/*
// MOSTRAR CURSO
*/

pub fn buscar_curso_por_id( tema : &str ) -> Result<Tema, ()> {
    match tema {
        "A1"|"a1" => Ok(Tema::new(Bloque::B1,Area::A)),
        "B1"|"b1" => Ok(Tema::new(Bloque::B1,Area::B)),
        "C1"|"c1" => Ok(Tema::new(Bloque::B1,Area::C)),
        "A2"|"a2" => Ok(Tema::new(Bloque::B2,Area::A)),
        "B2"|"b2" => Ok(Tema::new(Bloque::B2,Area::B)),
        "C2"|"c2" => Ok(Tema::new(Bloque::B2,Area::C)),
        "A3"|"a3" => Ok(Tema::new(Bloque::B3,Area::A)),
        "B3"|"b3" => Ok(Tema::new(Bloque::B3,Area::B)),
        "C3"|"c3" => Ok(Tema::new(Bloque::B3,Area::C)),
        _ => { println!("Opción de tema inválido. id puede ser: A1, B1, C1, A2, B2, C2, A3, B3, C3."); Err(()) },
        }
}

pub fn buscar_horario_por_id( horario : &str ) -> Result<Horario, ()> {
    let horario = horario.to_lowercase();

    if horario.len() != 4 {
        println!("Opción inválida. Formato es: ddhh");
        return Err(());
    }

    let hora_str = &horario[2..4]; 
    let dia_str = &horario[0..2];

    let dia = match dia_str {
        "lu" => Dia::Lu,
        "ma" => Dia::Ma,
        "mi" => Dia::Mi,
        "ju" => Dia::Ju,
        "vi" => Dia::Vi,
        "sa" => Dia::Sa,
        "do" => Dia::Do,
        _ => { println!("Opción de horario inválido. Formato es: ddhh"); return Err(()); },
    };

    let hora = match hora_str {
        "07" => Hora::H07, "08" => Hora::H08, "09" => Hora::H09, "10" => Hora::H10,
        "11" => Hora::H11, "12" => Hora::H12, "13" => Hora::H13, "14" => Hora::H14,
        "15" => Hora::H15, "16" => Hora::H16, "17" => Hora::H17, "18" => Hora::H18,
        "19" => Hora::H19, "20" => Hora::H20, "21" => Hora::H21, "22" => Hora::H22,
        _ => { println!("Opción de horario inválido. Formato es: ddhh"); return Err(()); },
    };

    Ok( Horario{ hora, dia } )
}

pub fn mostrar_curso ( 
    cursos : &Vec<Curso>, 
    curso_id : &str, 
    instructores : &Vec<Instructor>, 
    militantes : &Vec<Militante> ) {

    let curso = match buscar_curso_por_id(curso_id) { 
        Ok(tema) => &cursos[tema.indice()],
        Err(()) => return,
    };

    println!("\n\tTema: {}\n", curso.tema.to_string() );
    horarios_curso(&curso.grupos);

    println!("\n\tInstructores que lo imparten: {}", curso.instructores_que_imparten.len());
    listar_afiliades_de_curso( &curso.instructores_que_imparten, instructores );

    let militantes_pueden_tomarlo : BTreeSet<usize> =
        curso.militantes_que_tomaran.difference(&curso.militantes_sin_coincidencia).cloned().collect();

    println!("\n\tMilitantes que pueden tomarlo: {}", militantes_pueden_tomarlo.len());
    listar_afiliades_de_curso( &militantes_pueden_tomarlo, militantes);

    println!("\n\tMilitantes sin coincidencias de horario: {}", &curso.militantes_sin_coincidencia.len());
    listar_afiliades_de_curso( &curso.militantes_sin_coincidencia, militantes);
}


#[cfg(target_os = "linux")]
fn horarios_curso( grupos : &Vec<Grupo> ) {

    let horas_string = vec![
        " 07:00"," 08:00"," 09:00"," 10:00",
        " 11:00"," 12:00"," 13:00"," 14:00",
        " 15:00"," 16:00"," 17:00"," 18:00",
        " 19:00"," 20:00"," 21:00"," 22:00",
    ];

    println!(" Hora\t  Lu\t   Ma\t   Mi\t   Ju\t   Vi\t   Sa\t   Do");

    for (i,hora) in horas_string.iter().enumerate() {
 
        let mut ins_color_str : Vec<colored::ColoredString> = vec![];
        let mut mil_color_str : Vec<colored::ColoredString> = vec![];

        for j in 0..7 {
            let grupo_dia = &grupos[7*i+j];
            let ( num_ins, num_mil ) = grupo_dia.to_str();
            ins_color_str.push(num_ins);
            mil_color_str.push(num_mil);
        }

        println!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            hora,
            ins_color_str[0], ins_color_str[1], ins_color_str[2],
            ins_color_str[3], ins_color_str[4], ins_color_str[5],
            ins_color_str[6]);

        println!("\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            mil_color_str[0], mil_color_str[1], mil_color_str[2],
            mil_color_str[3], mil_color_str[4], mil_color_str[5],
            mil_color_str[6]);
        
        println!("");
    }
}

#[cfg(target_os = "windows")]
fn horarios_curso( grupos : &Vec<Grupo> ) {

    let horas_string = vec![
        " 07:00"," 08:00"," 09:00"," 10:00",
        " 11:00"," 12:00"," 13:00"," 14:00",
        " 15:00"," 16:00"," 17:00"," 18:00",
        " 19:00"," 20:00"," 21:00"," 22:00",
    ];

    println!(" Hora\t  Lu\t   Ma\t   Mi\t   Ju\t   Vi\t   Sa\t   Do");

    for (i,hora) in horas_string.iter().enumerate() {
 
        let mut ins_str : Vec<String> = vec![];
        let mut mil_str : Vec<String> = vec![];

        for j in 0..7 {
            let grupo_dia = &grupos[7*i+j];
            let ( num_ins, num_mil ) = grupo_dia.to_str();
            ins_str.push(num_ins);
            mil_str.push(num_mil);
        }

        println!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            hora,
            ins_str[0], ins_str[1], ins_str[2],
            ins_str[3], ins_str[4], ins_str[5],
            ins_str[6]);

        println!("\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            mil_str[0], mil_str[1], mil_str[2],
            mil_str[3], mil_str[4], mil_str[5],
            mil_str[6]);
        
        println!("");
    }
}




impl Grupo {
    #[cfg(target_os = "linux")]
    fn to_str(&self) -> ( colored::ColoredString, colored::ColoredString )  {

        let instructores_size = self.instructores.len();
        let militantes_size = self.militantes.len();

        let mut instructores_str = String::new();
        let mut militantes_str = String::new();

        write!(&mut instructores_str, "  {}", instructores_size ).unwrap() ;
        write!(&mut militantes_str, "  {}", militantes_size ).unwrap() ;

        if instructores_size == 0 {
            return ( instructores_str.red(), militantes_str.red() );
        }

        if militantes_size >= 4 {
            return ( instructores_str.green(), militantes_str.green() );
        }

        ( instructores_str.yellow(), militantes_str.yellow() )

    }

    #[cfg(target_os = "windows")]
    fn to_str(&self) -> ( String, String )  {

        let instructores_size = self.instructores.len();
        let militantes_size = self.militantes.len();

        let mut instructores_str = String::new();
        let mut militantes_str = String::new();

        write!(&mut instructores_str, "  {}", instructores_size ).unwrap() ;
        write!(&mut militantes_str, "  {}", militantes_size ).unwrap() ;

        ( instructores_str, militantes_str )

    }
}


pub fn asignar_afiliades_a_cursos(
    cursos : &mut Vec<Curso>,
    instructores : &Vec<Instructor>,
    militantes : &Vec<Militante>) {

    for curso in cursos {

        let tema = &curso.tema;

        for instructor in instructores {
            if instructor.temas_que_imparte.contains(tema) {
                curso.instructores_que_imparten.insert( instructor.id );
            }
        }

        for militante in militantes {
            if militante.temas_que_lleva.contains(tema) {
                curso.militantes_que_tomaran.insert( militante.id );
            }
        }
    }
}


pub fn generar_grupos_de_curso(
    curso : &mut Curso,
    instructores : &Vec<Instructor>,
    militantes : &Vec<Militante>
) {

    curso.instructores_que_imparten = BTreeSet::new();
    curso.militantes_que_tomaran = BTreeSet::new();

    for instructor in instructores {
        if instructor.get_temas().contains(&curso.tema) {
            curso.instructores_que_imparten.insert(instructor.get_id());
        }
    }

    for militante in militantes {
        if militante.get_temas().contains(&curso.tema) {
            curso.militantes_que_tomaran.insert(militante.get_id());
        }
    }

    for mut grupo in &mut curso.grupos {
        grupo.instructores = BTreeSet::new();
        grupo.militantes = BTreeSet::new();
    }

    for id in &curso.instructores_que_imparten {
        for horario in instructores[*id].get_disponibilidad() {
            curso.grupos[ horario.indice() ].instructores.insert( *id );
        }
    }

    for id in &curso.militantes_que_tomaran { 
        let mut num_cursos = 0;
        for horario in militantes[*id].get_disponibilidad() {
            curso.grupos[ horario.indice() ].militantes.insert( *id );
            
            if curso.grupos[horario.indice()].instructores.len() != 0 {
                num_cursos += 1;
            }
        }

        if num_cursos == 0 {
            curso.militantes_sin_coincidencia.insert( *id );
        }

    }

}

pub fn generar_grupos_todos_cursos(
    cursos : &mut Vec<Curso>,
    instructores : &Vec<Instructor>,
    militantes : &Vec<Militante>
) {
    for curso in cursos {
        generar_grupos_de_curso(curso, instructores,militantes);
    }
}

/*
// Mostrar grupo
*/


pub fn mostrar_grupo(
    cursos : &Vec<Curso>,
    instructores : &Vec<Instructor>,
    militantes : &Vec<Militante>,
    curso_id : &str,
    horario_id : &str 
) {

    let tema = match buscar_curso_por_id(curso_id) { 
        Ok(tema) => tema,
        Err(()) => return,
    };

    let curso = &cursos[tema.indice()];
    
    let horario = match buscar_horario_por_id(horario_id) {
        Ok(horario) => horario,
        Err(()) => return,
    };

    let grupo = &curso.grupos[horario.indice()];

    println!("\tTema: {}", tema.to_string());
    println!("\tHorario: {}", horario.to_string());

    println!("\n\tInstructores con disponibilidad: {}", grupo.instructores.len());
    listar_afiliades_de_grupo( &grupo.instructores, instructores );
    println!("\n\tMilitantes con disponibilidad: {}", grupo.militantes.len());
    listar_afiliades_de_grupo( &grupo.militantes, militantes);

}


/*
// Crear grupo
*/

pub fn crear_grupo_help( error :  &str ) {
    println!("{}", error);
    println!("Uso:\n\tcrear <curso_id> <horario_id> [ <-x | -i> < [id | nombre][, ...] >")
}

pub fn crear_grupo(
    cursos : &mut Vec<Curso>,
    grupos_confirmados : &mut Vec<(Tema,Grupo)>,
    instructores : &Vec<Instructor>,
    militantes : &mut Vec<Militante>,
    curso_id : &str,
    horario_id : &str,
    flag : &str,
    args : &str
) {

    if flag != "-x" && flag != "-i" && flag != "" {
        crear_grupo_help("Opciones equivocadas.");
    }


    let tema = match buscar_curso_por_id(curso_id) { 
        Ok(tema) => tema,
        Err(()) => return,
    };

    let curso = &mut cursos[tema.indice()];
    
    let horario = match buscar_horario_por_id(horario_id) {
        Ok(horario) => horario,
        Err(()) => return,
    };

    let grupo = &curso.grupos[horario.indice()];


    let mut grupo_militantes : BTreeSet<usize> = BTreeSet::new();

    // Se crea con todes les militantes con disponibilidad
    if flag == "" && args == "" {
        grupo_militantes = grupo.militantes.clone();
    }
    else if flag != "" && args != ""
    {
        let lista = match procesar_lista( args, militantes ) {
            Ok(lista) => lista,
            Err(()) => return,
        };

        if !lista.is_subset(&grupo.militantes) { 
            println!("Hay militantes en la lista que no pertenecen a este grupo.");
            return 
        }

        // Se crea excluyendo a les militantes de la lista
        if flag == "-x" {
            grupo_militantes = grupo.militantes.difference(&lista).cloned().collect();
        }
        // Se crea incluyendo solo a les militantes de la lista
        else if flag == "-i" {
            grupo_militantes = lista; 
        }   
    }

    let grupo_creado = (
        tema.clone(),
        Grupo {
            horario : grupo.horario.clone(),
            instructores : grupo.instructores.clone(),
            militantes : grupo_militantes.clone(),
        }
    );

    println!("\nGrupo creado!\n");
    mostrar_grupo_creado(&grupo_creado, instructores, militantes);

    grupos_confirmados.push(grupo_creado);

    // Se retiran les militantes inscritos de la lista del curso
    curso.militantes_que_tomaran = curso.militantes_que_tomaran.difference(&grupo_militantes).cloned().collect();
    // Se retira la disponibilidad del horario inscrito de les militantes
    for militante_id in &grupo_militantes {
        let militante = &mut militantes[ *militante_id ];
        militante.disponibilidad.take(&grupo.horario);
    }

    generar_grupos_todos_cursos(cursos, instructores, militantes);
    

}


fn procesar_lista<T:Afiliade>( lista : &str, afiliades : &Vec<T> ) -> Result<BTreeSet<usize>,()> {

    let mut lista_afiliades : BTreeSet<usize> = BTreeSet::new();

    for split in lista.split(",") {
        match buscar_afiliade( split, &afiliades ) {
            Ok(id) => {lista_afiliades.insert(id);} ,
            Err(()) => return Err(()),
        }
    }

    Ok(lista_afiliades)
}


// Inscribir militantes/instructores a cursos

pub fn inscribir_help() {
    println!("Uso: 
    inscribir [instructores | militantes] [lista de afiliades] [curso_id]")
}

pub fn desinscribir_help() {
    println!("Uso: 
    desinscribir [instructores | militantes] [lista de afiliades] [curso_id]")
}

fn inscribir_afiliade_curso<T:Afiliade> (
    afiliade : &mut T,
    tema : Tema,
) -> bool {
    let temas = afiliade.get_mut_temas();
    temas.insert(tema)
}

fn desinscribir_afiliade_curso<T:Afiliade>(
    afiliade : &mut T,
    tema : Tema,
) -> bool {
    let temas = afiliade.get_mut_temas();
    temas.remove(&tema)
}

pub fn inscribir_afiliades_curso<T:Afiliade> (
    afiliades : &mut Vec<T>,
    lista_afiliades : &str,
    curso_id : &str 
){

    let tema = match buscar_curso_por_id(curso_id) { 
        Ok(tema) => tema,
        Err(()) => return,
    };

    let lista_afiliades = match procesar_lista( lista_afiliades, &afiliades ) {
        Ok(lista) => lista,
        Err(()) => return,
    };

    for id in lista_afiliades {
        let afiliade = &mut afiliades[id];
        if inscribir_afiliade_curso( afiliade, tema ) {
            println!("Se ha inscrito al afiliade {} a {}",
                afiliade.get_nombre(),
                tema.to_string());
        }
        else {
            println!("Afiliade {} ya estaba inscrito a {}",
                afiliade.get_nombre(),
                tema.to_string());
        }
    }
}

pub fn desinscribir_afiliades_curso<T:Afiliade> (
    afiliades : &mut Vec<T>,
    lista_afiliades : &str,
    curso_id : &str 
){

    let tema = match buscar_curso_por_id(curso_id) { 
        Ok(tema) => tema,
        Err(()) => return,
    };

    let lista_afiliades = match procesar_lista( lista_afiliades, &afiliades ) {
        Ok(lista) => lista,
        Err(()) => return,
    };

    for id in lista_afiliades {
        let afiliade = &mut afiliades[id];
        if desinscribir_afiliade_curso( afiliade, tema ) {
            println!("Se ha desinscrito al afiliade {} de {}",
                afiliade.get_nombre(),
                tema.to_string());
        }
        else {
            println!("Afiliade {} no estaba inscrito a {}",
                afiliade.get_nombre(),
                tema.to_string());
        }
    }
}

// Mostrar grupos creados

pub fn mostrar_grupos_creados(
    grupos_creados : &Vec<(Tema, Grupo)>,
    instructores : &Vec<Instructor>,
    militantes : &Vec<Militante>,
 ) {

    for grupo in grupos_creados {
        println!("");
        mostrar_grupo_creado(grupo, instructores, militantes);
    } 
}

pub fn mostrar_grupo_creado(
    grupo_creado : &(Tema, Grupo),
    instructores : &Vec<Instructor>,
    militantes : &Vec<Militante>,
) {
    println!("Tema: {}", grupo_creado.0.to_string());
    println!("Horario: {}", grupo_creado.1.horario.to_string());
    println!("Instructores");
    listar_afiliades_de_grupo(&grupo_creado.1.instructores, &instructores);
    println!("Militantes");
    listar_afiliades_de_grupo(&grupo_creado.1.militantes, &militantes);
}