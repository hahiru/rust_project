use std::collections::HashMap;

fn mean(v: &Vec<i32>) -> i32 {
   let mut sum = 0;
   for n in v {
       sum += n;
   }
   sum/v.len() as i32
}

fn mediam(v: &Vec<i32>) -> i32 {
   let mut v2 = v.clone();
   v2.sort();
   v2[v2.len()/2]
}

fn mode(v: &Vec<i32>) -> i32 {
   let mut ct = HashMap::new();
   for n in v { // 参照
       let count = ct.entry(n).or_insert(0);
       *count += 1;
   }
   let mut v2: Vec<(_, _)> = ct.iter().collect(); // 参照
   v2.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
   **v2[0].0
}

fn pig_latin_consonants(s: &str) -> String {
   let mut t =  "".to_string();
   let mut head_ch = None;
   for ch in s.chars() {
       match head_ch {
       	     Some(_) => t.push(ch),
	     None => head_ch = Some(ch),
       }
   }
   t.push(head_ch.unwrap());
   t.push('a');
   t.push('y');
   t
}

fn pig_latin_vowels(s: &str) -> String{
   let mut t = s.to_string().clone();
   t.push_str("hay");
   t
}

fn pig_latin(s: &str) -> String {
   match s.chars().next().unwrap() {
   	 'a' | 'i' | 'u' | 'e' | 'o' | 'y' => return pig_latin_vowels(s),
	 _ => pig_latin_consonants(s)
   }
}

fn add_person(persons: &mut HashMap<String, Vec<String>>, person: &str, department: &str) {
   let person = person.to_string();
   let department = department.to_string();
   let e = persons.entry(department).or_insert(Vec::new());
   e.push(person);
   e.dedup();
}

fn main() {
   /*
   let v = vec![4, 3, 2, 5, 1, 1];
   let mean = mean(&v);
   let mediam = mediam(&v);
   let mode = mode(&v);
   
   println!("{}", mean);
   println!("{}", mediam);
   println!("{:?}", mode);

   let s = "first";
   let s2 = "apple";
   println!("{} => {}", s, pig_latin(s));
   println!("{} => {}", s, pig_latin(s2));
   */

   let mut persons = HashMap::new();
   add_person(&mut persons, "Sally", "Engineering");
   add_person(&mut persons, "Sally", "Engineering");   
   add_person(&mut persons, "Amir", "Sales");
   add_person(&mut persons, "Hatsune", "Sales");

   println!("{:?}", persons);
}
