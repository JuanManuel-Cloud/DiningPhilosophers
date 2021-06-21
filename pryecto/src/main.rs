/* macro_rules! compreh {
    ( $id1: ident | $id2: ident <- [ $start: expr; $end: expr ] , $cond:expr  ) => {
        {
        let mut v = Vec::new();
        for n in $start..$end+1 {
            if $cond(n) {
                v.push(n);
            }
        }
        v
        }
    };
}

macro_rules! compreh2 {
    ( ($id1: ident, $id2: ident) | $id3: ident <- [ $startx: expr; $endx: expr ] , 
                                   $id4: ident <- [ $starty: expr; $endy: expr ] ,
                                   $cond:expr  ) => {
        {
        let mut v = Vec::new();
        for n in $startx..$endx+1 {
            for m in $starty..$endy+1 {
                if $cond(n,m) {
                    v.push((n,m));
                }
                    
            }
        }
        v
        }
    };
} */

use std::collections::HashMap;
use rand::{thread_rng, Rng};


macro_rules! arc {
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

macro_rules! armar_int{
    ( $id1: ident | $id2: ident <- [ $startx: expr; $endx: expr ] , $id3:expr  ) => {
    {   
        let mut grid1 = [[0; 8] ; 12];
        for n in $startx..$endx+1 {
            for i in 0..$id3.get(&n).unwrap().len(){
                grid1[n][$id3.get(&n).unwrap()[i]]=1;
            }
        }
        grid1
    }   
    };
}

macro_rules! armar_out{
    ( $id1: ident | $id2: ident <- [ $startx: expr; $endx: expr ] , $id3:expr  ) => {
    {   
        let mut grid1 = [[0 ; 8] ; 12];
        for n in $startx..$endx+1 {
            for i in 0..$id3.get(&n).unwrap().len(){
                grid1[$id3.get(&n).unwrap()[i]][n]=1;
            }
        }
        grid1
    }   
    };
}

macro_rules! matriz_incidencia{
    ( $id1: ident , $id2: ident) => {
    {   
        let mut grid1 = [[0 ; 8] ; 12];
        for n in 0..8 {
            for i in 0..12{
                grid1[i][n]=$id1[i][n]-$id2[i][n];
            }
        }
        grid1
    }   
    };
}

macro_rules! trans_hab{
    ( $id1: ident , $id2: ident) => {
    {   
        let mut grid1 = [true ; 8];
        for n in 0..8 {
            for i in 0..12{
                if($id1[i][n]==-1){
                    if($id2[i]!=1){
                        grid1[n]=false;
                    }
                };
            }
        }
        grid1
    }   
    };
}

macro_rules! inner_prod {
    ($mat_inc: ident, $vec: ident) => {
        {
            let mut resultado = [0,0,0,0,0,0,0,0,0,0,0,0];
            for i in 0..12 {
                let mut elemento = 0;
                for j in 0..8 {
                    //println!("matriz = {:?}",$mat_inc[i][j]);
                    //println!("vector = {:?}",$vec[j]);
                    elemento += $mat_inc[i][j] * $vec[j];
                }
                resultado[i] = elemento;
            }          
            resultado
        }
    };
}

macro_rules! fire_empieza_n {
    ($transicion:ident, $trans_hab:ident, $curr_marc:ident, $mat_inc:ident) => {
        {
            let mut sensibilizada = false;
            for i in 0..8 {
                if $transicion[i] == 1 {
                    if $trans_hab[i] {
                        sensibilizada = true;
                        break;
                    } else{
                        break;
                    }
                }
            }
            if sensibilizada {
                let aux_marc = $curr_marc; 
                for i in 0..12 {
                    //new_marc[i] = $curr_marc[i] + inner_prod!($mat_inc, $transicion)[i];
                    $curr_marc[i] = aux_marc[i] + inner_prod!($mat_inc, $transicion)[i];
                }
                $trans_hab = trans_hab!($mat_inc,$curr_marc);
            } else {
                println!("La transición seleccionada no está sensibilizada por lo cuál no se puede disparar");
            }
        }
    };
}

macro_rules! fire_any {
    ($trans_hab:ident, $curr_marc:ident, $mat_inc:ident) => {
        {
            let mut sens: bool = false;
            let mut num: usize = 0;
            while !sens {
                num = thread_rng().gen_range(0..7);
                if $trans_hab[num] {
                    sens = true;
                }
            }

            let mut transicion = [0,0,0,0,0,0,0,0];
            for i in 0..8 {
                if i == num {
                    transicion[num] = 1;
                }
            }
            println!("Se disparo la transición T {:?}",num);
            fire_empieza_n!(transicion, $trans_hab, $curr_marc, $mat_inc);
        }
    };
}

fn listar_marc(curr_marc: [i32; 12]) {
    println!("{:?}",curr_marc);
}
fn listar_enables(trans_hab: [bool;8]) {
    println!("{:?}",trans_hab);
}

macro_rules! fire_all {
    ($trans_hab:ident, $curr_marc:ident, $mat_inc:ident) => {
        {
            let mut transicion = [0;8];
            for i in 0..8 {
                transicion[i] = 1;
                fire_empieza_n!(transicion, $trans_hab, $curr_marc, $mat_inc);
                listar_marc($curr_marc);
                listar_enables($trans_hab);
                transicion[i] = 0;
            }
        }
    };
}

fn main() {
    
    /* println!("Hello, world!");
    let v = compreh!(x | x <- [1;10],mayor5);
    let w = compreh2!( (x,y) | x <- [1;10],y <-[1;10],mayory);
    println!("{:?}",v);
    println!("{:?}",w); */

    let arc_int = arc!( 0   => vec![0,4],
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
    let arc_out = arc!( 0   => vec![1],
                        1   => vec![0,2,3],
                        2   => vec![9],
                        3   => vec![8,10,11],
                        4   => vec![5],
                        5   => vec![0,4,11],
                        6   => vec![6],
                        7   => vec![3,8,7]
                    );           

    //println!("{:?}",arc_int.get(&11).unwrap());
    //println!("{:?}",arc_out.get(&1).unwrap());
    //println!("{:?}",grid);
    //let mut grid1 = [0 as u8; 15];

    let mat_int = armar_int!(x | x <- [0;11],arc_int);
    let mat_out = armar_out!(y | y <- [0;7],arc_out);
    /*println!("Matriz de entradas:");
    for i in 0..12{
        println!("{:?}",mat_int[i]);
    }
    println!("Matriz de salidas:");
    for i in 0..12{
        println!("{:?}",mat_out[i]);
    }*/
    let mat_inc = matriz_incidencia!(mat_out,mat_int);
    /*println!("Matriz de Incidencia:");
    for i in 0..12{
        println!("{:?}",mat_inc[i]);
    }*/
    let marc=[1,0,1,1,1,0,0,1,1,0,1,1];
    let mut curr_marc = marc;
    let mut trans_hab= trans_hab!(mat_inc,marc);
    //println!("{:?}",trans_hab);

    //fire_empieza_n!(vect, trans_hab, curr_marc, mat_inc);
    //println!("initial marc:");
    //println!("{:?}",marc);
    //fire_any!(trans_hab, curr_marc, mat_inc);
    //println!("current marc:");
    //println!("{:?}",curr_marc);
    //[1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1]
    listar_marc(curr_marc);
    listar_enables(trans_hab);
    fire_all!(trans_hab, curr_marc, mat_inc);
}