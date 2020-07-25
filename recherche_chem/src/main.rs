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
    let cexp=cases_explored.to_vec();
    for cc in cexp{
        if cc==case{
            return true;
        } 
    }
    return false;
}

fn get_shortest_chem(chems: &std::vec::Vec<std::vec::Vec<[u32; 2]>>) -> &std::vec::Vec<[u32; 2]>{
    let mut shortest=0;
    for i in 0..chems.len(){
        if chems[i]<chems[shortest]{
            shortest=i;
        }
    }
    let chemshort=&chems[shortest];
    return chemshort;
}

fn explore_case(x: u32, y: u32, cases_explored: &std::vec::Vec<[u32; 2]>, map: &std::vec::Vec<std::vec::Vec<u32>>) -> (std::vec::Vec<[u32; 2]>, bool) {
    let mut cexp=cases_explored.to_vec();
    let mut chems: std::vec::Vec<std::vec::Vec<[u32; 2]>>=Vec::new();

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
                            chems.push(chem);
                        }
                    }
                    if map[cx as usize][cy as usize]== 3 as u32{
                        let case=[cx, cy];
                        cexp.push(case);
                        chems.push(cexp);
                    }
                }
            }            
        }
    }

    if chems.len()>0{
        let chem=get_shortest_chem(&chems);
        return (chem.to_vec(), true);
    }
    else{
        return (cexp, false);
    }
}

fn get_chem(map: &std::vec::Vec<std::vec::Vec<u32>>, entree: &[usize; 2], sortie: &[usize; 2]) -> (std::vec::Vec<[u32; 2]>, bool){
    let start_point_x=entree[0] as u32;
    let start_point_y=entree[1] as u32;
    let (chem,bon)=explore_case(start_point_x, start_point_y, &Vec::new(), &map);
    return (chem,bon);
}

fn aff_result(map: &std::vec::Vec<std::vec::Vec<u32>>, chem: &std::vec::Vec<[u32; 2]>){
    let txt="";
    for x in 0..map.len(){
        for y in 0..map[x].len(){
            let mut case="";
            if map[x][y]==0{
                if(isincases([x as u32,y as u32], chem)){
                    case="x";
                }
                else{
                    case=".";
                }
            }
            else if map[x][y]==1{
                case="O";
            }
            if map[x][y]==2{
                case="E";
            }
            if map[x][y]==3{
                case="S";
            }
            let txt = format!("{} {} ",txt,case);
        }
        let txt = format!("{}{}",txt,"\n");
    }
    println!("result : {}",txt);
}

//Fonction principale
fn main(){
    let (map, entree, sortie) = generate_map();
    
    let result=get_chem(&map, &entree, &sortie);
    println!("\nresult : {:?}", result);
}
