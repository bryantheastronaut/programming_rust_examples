use std::fmt::Display;
use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("Works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn smallest(nums: &Vec<i32>) -> &i32 {
    let mut s = &nums[0];
    for r in &nums[0..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

fn first_third(point: &[i32; 3]) -> (&i32, &i32) {
    (&point[0], &point[2])
}

struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0 .. self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i])
            }
        }
        None
    }
}

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for el in slice {
        vec.push(*el);
    }
}

// #[derive(Debug)]
fn main() {
    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];
    extend(&mut wave, &head);
    extend(&mut wave, &tail);
    extend(&mut wave, &wave);
    println!("{:?}", wave);
    // let st = StringTable {
    //     elements: vec!["Hello".to_string(), "World".to_string(), "Goodbye".to_string(), "wow".to_string()],
    // };
    // let word = st.find_by_prefix("pot");
    // println!("{}", word.unwrap());
    // let points = [12, 22, 322];
    // let ft = first_third(&points);
    // println!("{}, {}", ft.0, ft.1);
    // struct S<'a> {
    //     r: &'a i32
    // }
    // let nums = vec![1222, 123123123, 1231, 52, 22347, 223, 112];
    // let smallest = smallest(&nums);
    // println!("{} is the smallest number here", smallest);
    // struct Anime { name: &'static str, betchel_pass: bool };
    // let mha = Anime { name: "My Hero Academia", betchel_pass: true};
    // let anime_ref = &mha;
    // assert_eq!(anime_ref.name, "My Hero Academia");
    // let mut v = vec![123, 9, 11, 1, 42];
    // v.sort();
    // println!("{:?}", v);
    // let mut table = Table::new();
    // table.insert("Gesuldo".to_string(), vec!["many things".to_string(), "other things".to_string()]);
    // table.insert("Steve".to_string(), vec!["nothing".to_string(), "no really nothing".to_string()]);
    // show(&table);
    // sort_works(&mut table);
    // show(&table);
    // let mut v = vec!["Hello", "Apple", "World"];
    // v.sort();
    // for word in &v {
    //     println!("{}", word);
    // }
    // let mut v = vec![2,3,4,5,6];
    // let total = &v.iter().fold(0, |a, b| a + b);
    // println!("Total: {}", total);

    // let languages: Vec<String> = std::env::args().skip(1).collect();
    // for l in languages {
    //     println!("{}", l);
    // }
    // struct Person { name: String, birth: i32 }
    // struct Person { name: Option<String>, birth: i32 }
    // let mut composers = Vec::new();
    // // composers.push(Person { name: "Beethoven".to_string(), birth: 1854});
    // composers.push(Person { name: "Bryan".to_string(), birth: 1987});
    // for composer in &composers {
    //     println!("{}, born in {}", composer.name, composer.birth);
    // }
    // composers.push(Person { name: Some("Palestina".to_string()), birth: 1999 });
    // for composer in composers {
    //     println!("{}, born in {}", composer.name.unwrap(), composer.birth);
    // }
}
