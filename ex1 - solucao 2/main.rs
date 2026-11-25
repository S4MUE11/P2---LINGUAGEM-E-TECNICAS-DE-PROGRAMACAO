use ex1_solucao_2::plants;

fn main() {
    let jardim = "VRCG\nVRCG";

    let resultado = plants(jardim, "Alice");

    println!("Plantas da Alice: {:?}", resultado);
}