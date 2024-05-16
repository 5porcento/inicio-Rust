enum EstadoSemaforo {
    Vermelho,
    Amarelo,
    Verde,
}

fn acao(semaforo: EstadoSemaforo) {
    match semaforo {
        EstadoSemaforo::Vermelho => println!("Parar"),
        EstadoSemaforo::Amarelo => println!("Atenção"),
        EstadoSemaforo::Verde => println!("Seguir"),
    }
}

fn main() {
    let semaforo = EstadoSemaforo::Verde;
    acao(semaforo);
}
