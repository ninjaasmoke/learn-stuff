// fn main() {
//     let col: Vec<i32> = [1,2,3,9].to_vec();
//     const TARGET: i32 = 10;

//     println!("{:?}", get_collections(col, TARGET, 0));
// }

// fn get_collections(l: Vec<i32>, t: i32, s: i32) -> Vec<Vec<i32>> {

//     let mut sum: i32 = 0;
//     let mut arr: Vec<i32> = vec![];
//     let mut res: Vec<Vec<i32>> = vec![];

//     for i in &l {
//         sum += *i;
//         arr.push(*i);

//         if sum == t {
//             res.push(arr.clone());
//         } else if sum < t {
//             let sub_col: Vec<Vec<i32>> = get_collections(l.clone(), t - sum, i + 1);
//             for sub in sub_col {
//                 let mut new_sub = vec![s];
//                 new_sub.extend(sub);
//                 res.push(new_sub);
//             }
//         }

//         sum -= arr.pop().unwrap_or(0);
//     }

//     return res;
// }

fn main() {
    let col: Vec<i32> = vec![1, 2, 3, 6, 9];
    const TARGET: i32 = 10;

    println!("{:?}", get_collections(&col, TARGET, 0));
}

fn get_collections(l: &[i32], t: i32, s: usize) -> Vec<Vec<i32>> {
    let mut sum: i32 = 0;
    let mut arr: Vec<i32> = vec![];
    let mut res: Vec<Vec<i32>> = vec![];

    for i in l {
        sum += *i;
        arr.push(*i);

        if sum == t {
            res.push(arr.clone());
        } else if sum < t {
            let sub_col: Vec<Vec<i32>> = get_collections(&l[s + 1..], t - sum, s + 1);
            for sub in sub_col {
                let mut new_sub = vec![*i];
                new_sub.extend(sub);
                res.push(new_sub);
            }
        }

        sum -= arr.pop().unwrap_or(0);
    }

    res
}
