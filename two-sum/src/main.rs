// Two Sum.
//      Given a list of ints and a target
//      return the indices of the two numbers that add up to the target.
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let result = two_sum_bruteforce(vec![1,2,3,4],5);
    println!("{:?}", result);

    let result = two_sum_hash(vec![1,2,3,4],5);
    println!("{:?}", result);

    let result = two_sum_hashmatch(vec![1,2,3,4],5);
    println!("{:?}", result);
}

fn two_sum_bruteforce(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[j] == target - nums[i] {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

/*
pseudocode.

map = new HashMap 
for i = 0 to length of nums 
    map[nums[i]] = i 
for i = 0 to length of nums 
    complement = target - nums[i] 
    if map.contains(complement) and map[complement] != i 
        return [i, map[complement]]
*/
fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    for (i, item) in nums.iter().enumerate() {
        hash_map.insert(*item, i as i32);
    }

    for (i, item) in nums.iter().enumerate() {
        let compliment = target - item;
        if let Some(&index) = hash_map.get(&compliment) {
            if index != i as i32 {
                return vec![i as i32, index];
            }
        } 
    }
    vec![]
}

fn two_sum_hashmatch(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    for (i, item) in nums.iter().enumerate() {
        match hash_map.get(&(target - item)) {
            Some(value) => {
                return vec![i.try_into().unwrap(), *value];
            }
            None => {
                hash_map.insert(*item, i.try_into().unwrap());
            }
        }
    }

    vec![]
}