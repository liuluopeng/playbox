use std::collections::HashMap;

struct FrequencyTracker {
    map: HashMap<i32, usize>,
    freq_list: Vec<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrequencyTracker {
    fn new() -> Self {
        FrequencyTracker {
            map: HashMap::new(),
            freq_list: vec![0; 100005],
        }
    }

    fn add(&mut self, number: i32) {
        let count = self.map.entry(number).or_insert(0);
        *count += 1;

        self.freq_list[*count] += 1;

        if *count != 1 {
            self.freq_list[*count - 1] -= 1;
        }
    }

    fn delete_one(&mut self, number: i32) {
        if !self.map.contains_key(&number) {
            return;
        }

        let count = self.map.entry(number).or_insert(0);
        if *count == 0 {
            return;
        }
        *count -= 1;

        self.freq_list[*count + 1] -= 1; //  比如有3个2   现在有2个2了  2要加一  3要减一

        if *count != 0 {
            self.freq_list[*count] += 1;
        }
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        self.freq_list[frequency as usize] > 0
    }
}

/**
 * Your FrequencyTracker object will be instantiated and called as such:
 * let obj = FrequencyTracker::new();
 * obj.add(number);
 * obj.delete_one(number);
 * let ret_3: bool = obj.has_frequency(frequency);
 */

fn main() {
    let mut obj = FrequencyTracker::new();
    // obj.add(1);

    // obj.delete_one(1);
    // obj.delete_one(1);
    // obj.delete_one(1);
    // let ret_3: bool = obj.has_frequency(1);

    /*

        ["FrequencyTracker","deleteOne","hasFrequency","hasFrequency","deleteOne","hasFrequency","hasFrequency","add","deleteOne","deleteOne"]
    [[],[5],[1],[1],[3],[1],[1],[7],[7],[7]] */
    obj.delete_one(5);
    obj.has_frequency(1);
    obj.has_frequency(1);
    obj.delete_one(3);
    obj.has_frequency(1);
    obj.has_frequency(1);
    obj.add(7);
    obj.add(7);
    obj.add(7);
}
