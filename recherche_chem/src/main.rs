use rand::Rng;

fn generate_map() -> (std::vec::Vec<std::vec::Vec<u32>>, [usize; 2], [usize; 2]) {
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
    return (array,entree,sortie);
}

fn isincases(case: [u32; 2], cases_explored: &std::vec::Vec<[u32; 2]>) -> bool{
    for cc in cases_explored{
        if cc==&case{
            return true;
        } 
    }
    return false;
}

fn explore_case(x: u32, y: u32, cases_explored: &std::vec::Vec<[u32; 2]>, map: &std::vec::Vec<std::vec::Vec<u32>>) -> (std::vec::Vec<[u32; 2]>, bool) {
    let mut cexp=cases_explored.to_vec();
    cexp.push([x,y]);
    for xx in &[0, 2]{
        for yy in &[0, 2]{
            let dx:u32=*xx;
            let dy:u32=*yy;
            if !(x+dx==0) && !(y+dy==0) {
                let cx: u32 = (x+dx-1) as u32;
                let cy: u32 = (y+dy-1) as u32;  
                if cx>=0 && cy>=0 && cx<map.len() as u32 && cy < map[0].len() as u32 {
                    if map[cx as usize][cy as usize]== 0 as u32 && !(isincases( [cx as u32, cy as u32] , &cexp)){
                        let (chem,bon)=explore_case(cx as u32, cy as u32, &cexp, map);
                        if bon{
                            return (chem,bon);
                        }
                    }
                    if map[cx as usize][cy as usize]== 3 as u32{
                        let case=[cx, cy];
                        cexp.push(case);
                        return (cexp, true);
                    }
                }
            }            
        }
    }
    return (cexp, false);
}

fn get_chem(map: &std::vec::Vec<std::vec::Vec<u32>>, entree: &[usize; 2], sortie: &[usize; 2]) -> (std::vec::Vec<[u32; 2]>, bool){
    let start_point_x=entree[0] as u32;
    let start_point_y=entree[1] as u32;
    let (chem,bon)=explore_case(start_point_x, start_point_y, &Vec::new(), &map);
    return (chem,bon);
}

//Fonction principale
fn main(){
    let (map, entree, sortie) = generate_map();
    
    let result=get_chem(&map, &entree, &sortie);
    println!("\nresult : {:?}", result);
}
