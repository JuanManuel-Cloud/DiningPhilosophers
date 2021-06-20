use std::thread;
use std::sync::{Mutex, Arc}; //mutex para la exclusión mutua, arc para que la transición sea atómica 

//Estas dos structurs no se si la vamos a poder usar, las saque del doc de rust

struct Transicion {
    nombre:str;
    entradas: Vec<Plaza>;
    salidas: Vec<Plaza>;
}

impl Transicion {
    fn new(nombre: &str, entradas: Vec<Plaza>, salidas: Vec<Plaza>) -> Transicion {
        nombre:nombre.to_string,
        entradas: entradas,
        salidas: salidas,
    }
}

//https://alecmooreutdms.wordpress.com/2015/10/01/petri-nets/ -> explicación del problema
//https://doc.rust-lang.org/1.2.0/book/dining-philosophers.html -> documentación de rust

//los filosofos o bien pueden estar pensando (cuando no tienen los dos tenedores), o pueden estar comiendo (cuando tienen los dos tenedores)
//por lo que la cantidad de tokens para poder comer es 2.

//Vamos a tener que usar un mapa para decir la plaza y la cantidad de tokens que tiene 
//otro mapa vamos a tener los arcos que van de las transiciones a las plazas o de las plazas a las transiciones, en función de eso podemos
//ir diciendo si está o no sensibilizada una determianada transición, y hacer un disparo de una transición y hacer que se muevan los tokens.

/*
MAP EXAMPLE

macro_rules! my_map {
    ($($k:expr => $v:expr),*) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($k,$v);
            )*
            map
        }
    };
}

fn main() {

    let m = my_map!("coco" => vec![2,1],
                    "luis" => vec![3],
                    "ana"  => vec![6,5,72]);
    println!("{:?}",m.get("coco").unwrap());

}
*/

//no se bien que es el tipo tt, lo usan en la macro de println! asi que me pareció oportuno xD.
//te dejo un link por si lo entendes: https://stackoverflow.com/questions/40302026/what-does-the-tt-metavariable-type-mean-in-rust-macros
macro_rules! place { 
    ($($plaza: tt),*) => {
        {
            let mut plazas = Vec::new();
            $(
                plazas.push($plaza);
            )*
            plazas
        }  
    };
}

/*
macro_rules! trans {
    ($($transicion: tt),*) => {
        {
            let mut transiciones = Vec::new();
            $(
                transiciones.push($transicion);
            )*
            transiciones
        }
    };    
}
*/

macro_rules! trans {
    ($($empieza),*,$($comiendo),*) => {
        let mut empieza = Vec::new();
        let mut comiendo = Vec::new();
        let mut transiciones = Vec::new();

        $(
            empieza.push($empieza);
            comiendo.push($comiendo);
        )*
        for aux in &comiendo {
            empieza.push(val);
        }
        transiciones = empieza
    }
}

macro_rules! arc {
    ($($k: expr => $v: exp),*) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($k,$v);
            )*
            map
        }
    }
}

macro_rules! matriz_incidencia {
    ($plazas: expr, $transiciones: expr, $arcos: expr) => {
        {
            let mut matriz [plazas.len()][transiciones.len()];
            for p in plazas {
                for t in transiciones {
                    matriz[p][t] = 0;                 
                }
            }

        }
    }
}

macro_rules! init {

}

fn empiezan() {

}

fn cualquiera() {

}

fn todas() {

}

fn marcas() -> Vec<i32> {

}

fn habilitadas() -> Vec<trans> {

}

fn main() {
    let p = place!("p0","p1","p2","p3","p4")

    while(!p.is_empty()) {
        println!(p.pop())
    }
}