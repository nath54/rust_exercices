fn main() {
    let first_word="nail";
    let liste=["shoe", "horse", "rider", "message", "battle", "kingdom"];
    let mut last_word = first_word;
    for actual_word in &liste{
        println!("For want of a {} the {} was lost",last_word,actual_word);
        last_word=actual_word;
    }
    println!("And all for the want of a {}.", first_word);
}
