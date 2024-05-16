struct Pessoa{
    nome:String,
    idade:u8,
}
fn main(){
    let pessoa=Pessoa{
        nome:String::from("Kauan"),
        idade:19,
    };
    println!(" Ola {} voce tem {} anos :)", pessoa.nome,pessoa.idade);
}
