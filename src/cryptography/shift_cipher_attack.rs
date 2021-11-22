struct Sovler {
    original_probs: Vec<f32>,
    new_probs: Vec<f32>,
    ciphertext: String,
}

impl Sovler {
    fn new(data: &Vec<f32>, text: String) -> Sovler {
        let mut probs: Vec<f32> = vec![0.0; 26];
        for c in text.chars() {
            let pos = ((c as u8) - ('A' as u8)) as usize;
            probs[pos] += 1.0;
        }
        let count = text.len() as f32;
        probs = probs.iter().map(|x| x * 100.0 / count).collect();
        println!("[Sovler-init] The distribution of ciphertext: {:?}", probs);
        Sovler{ original_probs: data.to_owned(), new_probs: probs, ciphertext: text}
    }
    
    fn sum_original_prob_2(&self) -> f32 {
        self.original_probs.iter().map(|x| x*x).sum::<f32>()
    }

    fn sum_shift_prob_2(&self, k: usize) -> f32 {
        let mut sum = 0f32;
        self.original_probs.iter().enumerate().for_each(|(pos, prob)| sum += prob * self.new_probs[(pos + k) % 26]);
        sum
    }

    fn find_k(&self) -> usize {
        let mut res = 0;
        let mut delta = 10000f32;
        println!("[Sovler-find_k] Started finding k.");
        for k in 0..25 {
            let temp_delta = (650.0 - self.sum_shift_prob_2(k)).abs();
            println!("[Sovler-find_k] k: {}, plaintext in k: {}, delta: {}", 26 - k, self.compute_plaintext(26 - k), temp_delta);
            if temp_delta < delta {
                delta = temp_delta;
                res = 26 - k
            }
        }
        println!("[Sovler-find_k] Finished finding k...");
        res
    }

    fn compute_plaintext(&self, k: usize) -> String {
        self.ciphertext.chars().map(|c| ((c as u8 - 'A' as u8 + k as u8) % 26 + 'a' as u8) as char).collect::<String>()
    }
}

fn main() {

    let ciphertext = "OVDTHUFWVZZPISLRLFZHYLAOLYL";
    let probs = vec![8.2, 1.5, 2.8, 4.3, 12.7, 2.2, 2.0, 
                    6.1, 7.0, 0.2, 0.8, 4.0, 2.4, 6.7,
                    1.5, 1.9, 0.1, 6.0, 6.3, 9.1,
                    2.8, 1.0, 2.4, 0.2, 2.0, 0.1];

    let sovler = Sovler::new(&probs, String::from(ciphertext));
    println!("[Init] The original probs sum: {}", sovler.sum_original_prob_2());
    let k = sovler.find_k();
    println!("[Result] The key is {}, and the plaintext is {:?}", k, sovler.compute_plaintext(k));
}