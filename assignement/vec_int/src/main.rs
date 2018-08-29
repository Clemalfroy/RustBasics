use std::collections::HashMap;

fn main() {
    let mut list = vec![1, 53, 26, 98, -1, 4 , 5, 1, 85, 1, 0, -65, 51];

    println!("The mean of list is: {:?}", mean(&list));
    println!("The mode of list is: {:?}", mode(&list));
    println!("The median of list is: {:?}", median(list));
}

fn mean(list: &[i32]) -> Option<isize> {
    if list.is_empty() { return None }
    let mut sum: isize = 0;
    for i in list {
        sum += *i as isize;
    }
    sum.checked_div(list.len() as isize)
}

fn median(mut list: Vec<i32>) -> Option<isize> {
    if list.is_empty() { return None }
    list.sort();
    Some(list[list.len() / 2] as isize)
}

fn mode(list: &[i32]) -> Option<i32> {
    if list.is_empty() { return None }

    let mut map = HashMap::new();
    for nb in list {
        let count = map.entry(*nb).or_insert(0);
        *count += 1;
    };
    let max_occurence_key = map.into_iter().max_by_key(|(_, y)| *y);
    max_occurence_key.map(|(x, _)| x)
}
