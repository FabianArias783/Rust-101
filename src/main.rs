fn main() {
    //let x = 5;
    //x=10; ← Esto no va a funcionar porque x es inmutable

    //let mut y = 10;
    //y = 20; // Esto si funciona porque y es mutable

    let x= 5;
    let x= x+1; //Nueva variable
    print!("El valor de x es: {}", x);

    println!("Tupla forma 1: ");

    //Variables
    let entero: i32 = 42;
    let flotante: f64 = 3.1416;
    let booleano: bool = true;
    let caracter: char = 'a';
    //Tupla → Structs || Creación de tupla llamada firulais
    let _firulais: (i32, f64, char) = (42, 3.1416, 'a');
    let arreglo: [i32; 3] = [1, 2, 3];
    println!("Tupla(firulais) forma1: {:?}", _firulais);
    println!("Tupla(firulais) forma2:  {} {} {}", _firulais.0, _firulais.1, _firulais.2);
}