fn main() {
    println!("Hello, world!");

    //  Your Codec object will be instantiated and called as such:
    let mut obj = Codec::new();

    let strs = "https://www.baidu.com/hello".to_string();
    let s: String = obj.encode(strs);
    let ans: String = obj.decode(s.clone());

    println!("{:?} {:?}", s, ans);
}

use std::collections::HashMap;

use rand::Rng;
struct Codec {
    memo_size: usize,
    rand_memo: HashMap<String, String>, // key:long url  v: short url
    url_map: HashMap<String, String>,   // k:short url  v: long url
    short_prefix: String,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {
            memo_size: 0,
            rand_memo: HashMap::new(),
            url_map: HashMap::new(),
            short_prefix: "https://short/".to_string(),
        }
    }

    // Encodes a URL to a shortened URL.
    fn encode(&mut self, longURL: String) -> String {
        //
        let dict = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
        let dict_list: Vec<char> = dict.chars().collect();

        while !self.rand_memo.contains_key(&longURL) {
            let mut rng = rand::thread_rng();
            let mut candi = String::new();
            for i in 0..=6 {
                // println!("Integer: {}", );

                let candi_char = dict_list[rng.gen_range(0..(26 + 26 + 10))];
                candi.push_str(&candi_char.to_string());
            }
            println!("{:?}", candi);

            // 查重
            if self.url_map.contains_key(&candi) {
                continue;
            } else {
                self.rand_memo.insert(longURL.clone(), candi.clone());
                self.url_map.insert(candi, longURL.clone());
            }
        }

        let res = self.rand_memo.get(&longURL);
        res.unwrap().to_string()
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, shortURL: String) -> String {
        let res = self.url_map.get(&shortURL);

        return res.unwrap().to_string();

        // shortURL
    }
}
