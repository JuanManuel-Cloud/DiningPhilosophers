use std::thread;
use std::sync::{Mutex, Arc}; //mutex para la exclusión mutua, arc para que la transición sea atómica 

//Estas dos structurs no se si la vamos a poder usar, las saque del doc de rust

struct Filosofo {
    nombre: String,
    izq: usize, //es un entero sin signo, su tamaño depende de la arquitectura del SO, en mi caso 64bits, por ende u64
    der: usize, //de esta manera te da la garantía que va a ser lo suficientemente grande para almacenar cualquier puntero
}

struct Mesa {
    tenedores: Vec<Mutex<()>>, //es el recurso compartido, por ende va a ser un vector de mutex
}

//https://alecmooreutdms.wordpress.com/2015/10/01/petri-nets/ -> explicación del problema
//https://doc.rust-lang.org/1.2.0/book/dining-philosophers.html -> documentación de rust

//los filosofos o bien pueden estar pensando (cuando no tienen los dos tenedores), o pueden estar comiendo (cuando tienen los dos tenedores)
//por lo que la cantidad de tokens para poder comer es 2.

//Vamos a tener que usar un mapa para decir la plaza y la cantidad de tokens que tiene 

macro_rules! place {

}

macro_rules! trans {

}

macro_rules! arc {

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

    //creo los 5 tenedores

    let mesa = Arc::new(Mesa { tenedores: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    //creo los 5 filósofos

    let filosofos = vec![
        Filosofo::new("Judith Butler", 0, 1),
        Filosofo::new("Gilles Deleuze", 1, 2),
        Filosofo::new("Karl Marx", 2, 3),
        Filosofo::new("Emma Goldman", 3, 4),
        Filosofo::new("Michel Foucault", 0, 4), //notar que en lugar de ir de 4 a 0, toma de 0 a 4, esto es para evitar el deadlock
    ];

}