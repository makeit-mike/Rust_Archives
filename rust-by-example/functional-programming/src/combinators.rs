
// Functional Combinators (FC for short) are anonymous fns to massage collections.
//      think map fn in Haskell/Scala/AnyGoodLanguage

// FC are based on the iterator trait in Rust, Arrays/Vectors/Etc implement the iterator.
//      think how most collections implement IEnumerable in C#. Remember how AMAZING linq is? okay, now we are talkin.

// Rust Iterators are LAZY! Haskell gods have blessed us again.

pub fn use_map_combinator() {
    let my_vector = vec!(1,2,3,4,5);

    let mapped_vec = my_vector
                        .iter()
                        .map(|&x| x + 1) // think lamba syntax. Lazy
                        .collect::<Vec<i32>>();

    println!("{:?}", mapped_vec);
}

pub fn use_filter_combinator() {
    let my_vector = vec!(1,2,3,4,5);

    let filtered_vec = my_vector
                        .iter()
                        .filter(|&x| x % 2 == 0) // only even's
                        .collect::<Vec<&i32>>(); // why do i need & here? wtf

    println!("{:?}", filtered_vec);
}

pub fn use_count_combinator() {
    let my_vector = vec!(1,2,3,4,5);
    let vec_count = my_vector.iter().count();

    println!("Count of {:?} = {}", my_vector, vec_count);
}

pub fn use_zip_with_index() {
    let my_vector = vec!(10,20,30,40,50);
    let index_vec = 0..my_vector.iter().count();
    let indexed_zipped_vector = my_vector
                                    .iter()
                                    .zip(index_vec)
                                    .collect::<Vec<(&i32,usize)>>();

    println!("NonZipped vector.");
    for i in my_vector.iter() {
        print!("{}", i);
    }
    
    println!("\r\nZipped vector.");
    for (i, j) in indexed_zipped_vector.iter() {
        print!("({}, {}), ", i, j);
    }
    println!("\r\n");
}

pub fn use_max_combinator() {
    let my_vector = vec!(10,20,30,40,50);
    let max = my_vector
                .iter()
                .max()
                .unwrap();

    println!("Max of {:?} = {}", my_vector, max);
}

pub fn use_fold_combinator() {
    let my_vector = vec!(10,20,30,40,50);
    let sum = my_vector
                .iter()
                .fold(0, |sum, value| sum + value);

    println!("Sum of {:?} = {}", my_vector, sum);
}

pub fn use_forall_combinator() {
    let my_vector = vec!(10,20,30,40,50);
    let gr_than_zero = my_vector
                        .iter()
                        .all(|&x| x > 20);

    println!("All Gr than 0 {:?} -> {:?}", my_vector, gr_than_zero);
}

pub fn use_flatmap_combinator() { 
    let lines_vec = vec!("hello,how","are,you");
    let words_vec = lines_vec
                        .iter()
                        .flat_map(|&x| x.split(","))
                        .collect::<Vec<&str>>();                      
    println!("All Gr than 0 {:?} -> {:?}", lines_vec, words_vec);
}