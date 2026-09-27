fn main() {
    let mut scores = vec![10, 20, 30, 40, 50];
    // eyni tipdə olan elementləri saxlayır
    println!("Scores: {:?}", scores);

    scores.push(60);
    println!("Scores: {:?}", scores);

    for score in &scores {
        println!("Score: {}", score+1);
    }

    for score in &mut scores {
        *score += 10;
    }
    println!("Scores: {:?}", scores);
   
   let last_score = scores.pop();
   // pop metodu vektordan son elementi çıxarır və onu geri qaytarır
   println!("Last Score: {:?}", last_score);

      let last_score = scores.pop().unwrap_or(0);
// unwrap_or metodu pop metodunun geri qaytardığı Option tipini unwrap edir və əgər vektor boşdursa, 0 dəyərini qaytarır
   println!("Last Score: {}", last_score);

   let codes : Vec<i32> = (1..=20).collect(); //1-20 arası ədədləri vektora çevirir
   println!("Codes: {:?}", codes);

   
}