fn main() {
    let my_name: String = String::from("Riccardo");
    //let my_emoji = "🥰".to_string();
    let s = string_to_hex(&my_name);
    println!("{:?}", s);

    let integer_checker: u32 = "1".parse().expect("Not a integer number!");
}
/*
**  Questa funzione riceve come parametro un riferimento "&" ad una Stringa immutabile (String)
**  Restituisce un Array (Vettore) di String (Vec<String>) che rappresenta ogni singolo byte della stringa
**  convertita in Hex
*/
fn string_to_hex(s: &String) -> Vec<String>{
    // restituisce un array di interi che rappresenta il bytecode di ogni carattere della stringa in ingresso
    let bytes: &[u8] = s.as_bytes();
    // rappresenta l'array vuoto da popolare
    let mut my_ex_string: Vec<String> = vec![];
    // cicliamo i byte, convertiamo in hex e inseriamo in array
    for byte in bytes {
        my_ex_string.push(format!("{:X}", byte));
    }
    //ritorniamo l'array al termine del ciclo
    my_ex_string
}
