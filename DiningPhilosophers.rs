use std::thread;
use std::sync::{Mutex, Arc}; //mutex para la exclusión mutua, arc para que la transición sea atómica 

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
    ($($empieza: tt),*,$($comiendo: tt),*) => {
        {
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
    };
}

macro_rules! arc {
    ($($k:expr => $v:expr),*) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($k,$v);
            )
            map
        }
    };
}

macro_rules! matriz_incidencia {
    ($arc_in:expr,$arc_out:expr,$plazas:expr,$transiciones:expr) => {
        {
            let mut matriz[plazas.len()][transiciones.len()];
            for p in plazas.len()
                for t in transiciones.len()
                    matriz[p][t]=0;
        
        }
    }
}

/*
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
*/
fn main() {
    let arc_int = arc!(  
        0   => vec![0,4],
        1   => vec![1],
        2   => vec![0],
        3   => vec![0,6],
        4   => vec![4],
        5   => vec![5],
        6   => vec![7],
        7   => vec![6],
        8   => vec![6,2],
        9   => vec![3],
        10  => vec![2],
        11  => vec![4,2]
    );

    let arc_out = arc!( 
            0   => vec![1],
            1   => vec![0,2,3],
            2   => vec![9],
            3   => vec![8,10,11],
            4   => vec![5],
            5   => vec![0,5,11],
            6   => vec![6],
            7   => vec![3,8,7]
        );

    println!("{:?}",arc_int.get(&11).unwrap());
    println!("{:?}",arc_out.get(&1).unwrap());
    
}