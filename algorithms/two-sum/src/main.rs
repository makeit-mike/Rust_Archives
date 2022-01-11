// Two Sum.
//      Given a list of ints and a target
//      return the indices of the two numbers that add up to the target.
use std::collections::HashMap;

fn main() {
    let result = two_sum_bruteforce(vec![1,2,3,4],5);
    println!("two_sum_bruteforce:");
    print_list_ints(result);
    
    let result = two_sum_hashmap_twopass(vec![1,2,3,4],5);
    println!("two_sum_hashmap_twopass:");
    print_list_ints(result);

    let result = two_sum_hashmap_onepass(vec![1,2,3,4],5);
    println!("two_sum_hashmap_onepass:");
    print_list_ints(result);
}

fn print_list_ints(items: Vec<i32>){
    println!("{:?}", items);
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
fn two_sum_hashmap_twopass(nums: Vec<i32>, target: i32) -> Vec<i32> {
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


/*
map = new HashMap 
for i in length of nums 
    if map.contains(num) 
        return [map[num], i] 
    else 
        map.insert(target - num, i)
*/
fn two_sum_hashmap_onepass(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut complements: HashMap<i32, i32> = HashMap::new();
    for (i, item) in nums.iter().enumerate() {
        match complements.get(item) {
            Some(&index) => return vec![index, i as i32],
            None => complements.insert(target - item, i as i32),
        };
    }
    vec![]
}
