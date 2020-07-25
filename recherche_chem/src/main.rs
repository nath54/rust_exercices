use rand::Rng;

fn generate_map() -> std::vec::Vec<std::vec::Vec<u32>> {
    //STRUCTURE D'UNE MAP : 
    // -0:vide, on peut y aller
    // -1:mur, on ne peut pas y aller
    // -2:entrée, il ne peut pas y avoir de murs
    // -3:sortie, pareil, pas de murs

    let mut rng = rand::thread_rng();
    //on définit la taille de la grille
    let tx: usize=10;
    let ty: usize=10;
    let nbmurs=10;

    //on définit les coordonnées de l'entrée et de la sortie
    let entree=[0,0];
    let sortie=[tx-1,ty-1];

    //on crée la map
    let mut array = vec![vec![0; tx]; ty];

    //on y place l'entrée et la sortie
    array[entree[0]][entree[1]]=2;
    array[sortie[0]][sortie[1]]=3;

    //on y place les murs
    let mut nm=0;
    while nm<nbmurs{
        let cx=rng.gen_range(0, tx);
        let cy=rng.gen_range(0, ty);
        if !([cx,cy]==entree) && !([cx,cy]==sortie) {
            nm+=1;
            array[cx][cy]=1;
        }
    }

    println!("map : {:?}", array);
    return array;
}

fn isincases(case: std::vec::Vec<u32>, cases_explored: std::vec::Vec<std::vec::Vec<u32>>) -> bool{

    return false;
}

fn explore_case(x: u32, y: u32, cases_explored: std::vec::Vec<std::vec::Vec<u32>>, map: std::vec::Vec<std::vec::Vec<u32>>) -> std::vec::Vec<std::vec::Vec<u32>>{
    for xx in &[0, 2]{
        for yy in &[0, 2]{
            let cx: usize = (x+xx-1) as usize;
            let cy: usize = (y+yy-1) as usize;            
            if map[cx][cy]==0 && !(isincases( [cx as u32, cy as u32].to_vec() , cases_explored)){

            }
        }
    }
    return cases_explored;
}

//Fonction principale
fn main(){
    let map = generate_map();

}
