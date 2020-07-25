fn main() {
    for x in 0..98{
        println!("{x} bottles of beer on the wall, {x} bottles of beer.\nTake one down and pass it around, {xx} bottles of beer on the wall.\n",x=99-x,xx=98-x);
    }
    println!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    println!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.");
}
