fn first_task(stacks: &mut[Vec<char>], moves: &[Vec<usize>]){
    for m in moves {
        let mut elems: Vec<char> = Vec::new();
        {
            let from = stacks.get_mut(m[1] - 1).unwrap();
            let mut amount = m[0];
            while amount > 0 {
                amount -= 1;
                elems.push(from.pop().unwrap());
            }
        }
        let to = stacks.get_mut(m[2] - 1).unwrap();
        to.append(&mut elems);
    }

    let mut result = String::new();
    for s in stacks {
        result.push(*s.last().unwrap());
    }
    println!("first task: {result}")
}

fn second_task(stacks: &mut[Vec<char>], moves: &[Vec<usize>]){
    for m in moves {
        let mut elems: Vec<char> = Vec::new();
        {
            let from = stacks.get_mut(m[1] - 1).unwrap();
            let mut amount = m[0];
            while amount > 0 {
                amount -= 1;
                elems.push(from.pop().unwrap());
            }
            elems.reverse();
        }
        let to = stacks.get_mut(m[2] - 1).unwrap();
        to.append(&mut elems);
    }

    let mut result = String::new();
    for s in stacks {
        result.push(*s.last().unwrap());
    }
    println!("second task: {result}");
}

fn main() {
    let mut stacks = vec![
        vec!['J', 'H', 'P', 'M', 'S', 'F', 'N', 'V'],
        vec!['S', 'R', 'L', 'M', 'J', 'D', 'Q'],
        vec!['N', 'Q', 'D', 'H', 'C', 'S', 'W', 'B'],
        vec!['R', 'S', 'C', 'L'],
        vec!['M', 'V', 'T', 'P', 'F', 'B'],
        vec!['T', 'R', 'Q', 'N', 'C'],
        vec!['G', 'V', 'R'],
        vec!['C', 'Z', 'S', 'P', 'D', 'L', 'R'],
        vec!['D', 'S', 'J', 'V', 'G', 'P', 'B', 'F'],
    ];

    let moves = vec![
        vec![1, 8, 4],
        vec![1, 7, 8],
        vec![1, 6, 3],
        vec![2, 6, 5],
        vec![8, 5, 1],
        vec![5, 3, 8],
        vec![1, 7, 8],
        vec![8, 1, 2],
        vec![3, 3, 9],
        vec![13, 8, 7],
        vec![2, 1, 2],
        vec![1, 6, 2],
        vec![2, 1, 7],
        vec![4, 4, 2],
        vec![10, 9, 4],
        vec![7, 4, 1],
        vec![1, 6, 7],
        vec![2, 4, 5],
        vec![1, 5, 2],
        vec![1, 5, 8],
        vec![3, 1, 5],
        vec![2, 4, 6],
        vec![2, 6, 4],
        vec![2, 4, 5],
        vec![5, 1, 5],
        vec![1, 9, 5],
        vec![1, 8, 5],
        vec![14, 2, 6],
        vec![12, 7, 4],
        vec![4, 6, 7],
        vec![3, 6, 4],
        vec![4, 4, 9],
        vec![2, 4, 6],
        vec![2, 9, 3],
        vec![9, 4, 3],
        vec![2, 1, 6],
        vec![5, 7, 3],
        vec![4, 7, 8],
        vec![2, 6, 7],
        vec![3, 6, 7],
        vec![10, 5, 8],
        vec![8, 8, 9],
        vec![10, 9, 7],
        vec![12, 7, 5],
        vec![1, 1, 5],
        vec![3, 7, 2],
        vec![10, 3, 7],
        vec![6, 5, 7],
        vec![2, 6, 1],
        vec![12, 2, 7],
        vec![3, 3, 1],
        vec![1, 6, 5],
        vec![10, 5, 7],
        vec![3, 3, 4],
        vec![34, 7, 1],
        vec![2, 6, 9],
        vec![1, 6, 3],
        vec![3, 1, 3],
        vec![1, 7, 3],
        vec![3, 3, 6],
        vec![1, 4, 3],
        vec![22, 1, 6],
        vec![2, 9, 7],
        vec![2, 4, 9],
        vec![12, 6, 8],
        vec![1, 7, 6],
        vec![4, 8, 2],
        vec![1, 7, 1],
        vec![6, 8, 9],
        vec![1, 2, 5],
        vec![1, 2, 7],
        vec![13, 1, 2],
        vec![2, 3, 1],
        vec![4, 7, 5],
        vec![3, 9, 4],
        vec![1, 1, 8],
        vec![4, 5, 2],
        vec![12, 6, 2],
        vec![3, 1, 3],
        vec![1, 4, 1],
        vec![9, 8, 5],
        vec![6, 5, 7],
        vec![2, 4, 5],
        vec![5, 9, 6],
        vec![5, 3, 7],
        vec![30, 2, 6],
        vec![11, 7, 9],
        vec![36, 6, 3],
        vec![10, 9, 3],
        vec![1, 6, 5],
        vec![3, 5, 2],
        vec![2, 5, 2],
        vec![28, 3, 4],
        vec![6, 4, 1],
        vec![1, 2, 3],
        vec![2, 5, 2],
        vec![6, 1, 7],
        vec![1, 1, 6],
        vec![6, 3, 5],
        vec![6, 7, 2],
        vec![1, 6, 4],
        vec![2, 2, 6],
        vec![8, 2, 1],
        vec![3, 2, 4],
        vec![2, 3, 4],
        vec![4, 3, 4],
        vec![1, 6, 1],
        vec![2, 1, 8],
        vec![1, 6, 4],
        vec![1, 9, 3],
        vec![2, 5, 4],
        vec![1, 8, 7],
        vec![1, 7, 9],
        vec![1, 3, 5],
        vec![1, 8, 6],
        vec![34, 4, 9],
        vec![13, 9, 8],
        vec![1, 8, 2],
        vec![1, 2, 1],
        vec![4, 5, 1],
        vec![9, 8, 7],
        vec![11, 1, 3],
        vec![1, 4, 2],
        vec![1, 6, 7],
        vec![1, 9, 4],
        vec![1, 4, 1],
        vec![1, 5, 3],
        vec![5, 7, 8],
        vec![1, 2, 5],
        vec![1, 5, 1],
        vec![21, 9, 5],
        vec![19, 3, 4],
        vec![17, 4, 6],
        vec![2, 8, 4],
        vec![2, 6, 8],
        vec![2, 6, 9],
        vec![2, 7, 6],
        vec![1, 4, 9],
        vec![6, 5, 6],
        vec![1, 9, 8],
        vec![8, 5, 7],
        vec![15, 6, 2],
        vec![1, 9, 7],
        vec![2, 1, 6],
        vec![3, 4, 7],
        vec![1, 1, 6],
        vec![3, 5, 4],
        vec![2, 5, 6],
        vec![2, 4, 1],
        vec![13, 7, 8],
        vec![2, 6, 4],
        vec![3, 2, 4],
        vec![2, 7, 6],
        vec![5, 4, 6],
        vec![4, 2, 6],
        vec![1, 1, 9],
        vec![18, 8, 3],
        vec![1, 4, 5],
        vec![1, 2, 7],
        vec![15, 3, 1],
        vec![1, 5, 1],
        vec![3, 3, 4],
        vec![1, 5, 4],
        vec![1, 5, 6],
        vec![1, 6, 8],
        vec![2, 8, 2],
        vec![3, 1, 8],
        vec![6, 2, 8],
        vec![1, 7, 6],
        vec![12, 8, 5],
        vec![2, 9, 6],
        vec![6, 1, 5],
        vec![9, 5, 3],
        vec![1, 2, 8],
        vec![20, 6, 9],
        vec![3, 6, 7],
        vec![1, 7, 1],
        vec![7, 3, 4],
        vec![2, 7, 2],
        vec![1, 8, 7],
        vec![8, 4, 1],
        vec![11, 1, 7],
        vec![10, 7, 6],
        vec![2, 4, 9],
        vec![21, 9, 3],
        vec![6, 5, 9],
        vec![6, 3, 2],
        vec![1, 4, 5],
        vec![1, 7, 9],
        vec![8, 3, 2],
        vec![9, 2, 1],
        vec![14, 1, 6],
        vec![1, 1, 7],
        vec![4, 3, 8],
        vec![3, 8, 7],
        vec![5, 7, 4],
        vec![3, 6, 9],
        vec![2, 3, 7],
        vec![3, 5, 6],
        vec![1, 5, 6],
        vec![2, 7, 9],
        vec![1, 8, 3],
        vec![22, 6, 5],
        vec![3, 9, 4],
        vec![3, 6, 1],
        vec![5, 4, 6],
        vec![9, 2, 8],
        vec![4, 6, 1],
        vec![1, 3, 2],
        vec![1, 2, 3],
        vec![6, 8, 1],
        vec![2, 4, 3],
        vec![10, 1, 7],
        vec![2, 8, 7],
        vec![1, 9, 6],
        vec![4, 3, 5],
        vec![1, 8, 3],
        vec![4, 9, 8],
        vec![1, 4, 3],
        vec![1, 3, 8],
        vec![3, 7, 6],
        vec![1, 1, 5],
        vec![10, 5, 9],
        vec![5, 6, 4],
        vec![5, 8, 5],
        vec![4, 9, 8],
        vec![3, 3, 9],
        vec![2, 8, 6],
        vec![5, 7, 5],
        vec![1, 4, 1],
        vec![1, 1, 2],
        vec![2, 8, 6],
        vec![1, 2, 1],
        vec![1, 7, 2],
        vec![1, 1, 5],
        vec![28, 5, 9],
        vec![3, 6, 1],
        vec![1, 6, 9],
        vec![1, 2, 9],
        vec![2, 1, 2],
        vec![2, 7, 5],
        vec![1, 7, 5],
        vec![1, 2, 5],
        vec![3, 1, 9],
        vec![1, 5, 8],
        vec![15, 9, 2],
        vec![11, 9, 4],
        vec![11, 4, 7],
        vec![2, 4, 1],
        vec![7, 7, 8],
        vec![1, 1, 4],
        vec![20, 9, 1],
        vec![2, 7, 8],
        vec![1, 4, 6],
        vec![1, 6, 2],
        vec![2, 7, 5],
        vec![1, 9, 6],
        vec![1, 4, 9],
        vec![4, 5, 2],
        vec![1, 6, 8],
        vec![1, 4, 9],
        vec![11, 8, 3],
        vec![1, 1, 9],
        vec![1, 5, 9],
        vec![1, 2, 6],
        vec![4, 9, 8],
        vec![4, 8, 7],
        vec![10, 1, 6],
        vec![7, 1, 5],
        vec![8, 3, 4],
        vec![2, 3, 5],
        vec![3, 7, 4],
        vec![1, 4, 5],
        vec![2, 1, 6],
        vec![9, 2, 6],
        vec![1, 7, 9],
        vec![1, 3, 2],
        vec![7, 4, 3],
        vec![3, 3, 7],
        vec![5, 2, 3],
        vec![1, 1, 9],
        vec![2, 2, 7],
        vec![1, 4, 6],
        vec![3, 5, 6],
        vec![4, 7, 6],
        vec![1, 7, 4],
        vec![1, 4, 7],
        vec![1, 2, 8],
        vec![1, 7, 1],
        vec![27, 6, 2],
        vec![1, 4, 1],
        vec![7, 5, 7],
        vec![1, 4, 1],
        vec![1, 8, 3],
        vec![3, 7, 3],
        vec![2, 1, 6],
        vec![2, 9, 1],
        vec![18, 2, 1],
        vec![2, 7, 5],
        vec![12, 3, 4],
        vec![1, 5, 6],
        vec![3, 6, 1],
        vec![24, 1, 8],
        vec![9, 2, 4],
        vec![3, 2, 1],
        vec![2, 6, 3],
        vec![1, 6, 9],
        vec![1, 5, 6],
        vec![1, 6, 2],
        vec![1, 1, 7],
        vec![1, 2, 1],
        vec![1, 1, 2],
        vec![3, 7, 2],
        vec![2, 1, 4],
        vec![8, 4, 5],
        vec![22, 8, 1],
        vec![1, 8, 1],
        vec![13, 4, 1],
        vec![1, 8, 5],
        vec![3, 3, 1],
        vec![1, 2, 7],
        vec![38, 1, 6],
        vec![27, 6, 1],
        vec![2, 2, 9],
        vec![3, 9, 8],
        vec![2, 8, 6],
        vec![1, 8, 3],
        vec![1, 2, 1],
        vec![1, 3, 6],
        vec![1, 2, 3],
        vec![1, 7, 6],
        vec![7, 6, 3],
        vec![20, 1, 4],
        vec![6, 1, 6],
        vec![17, 4, 7],
        vec![3, 6, 5],
        vec![14, 7, 9],
        vec![8, 5, 7],
        vec![3, 1, 6],
        vec![3, 3, 1],
        vec![2, 4, 1],
        vec![4, 5, 1],
        vec![9, 6, 2],
        vec![3, 6, 4],
        vec![4, 7, 8],
        vec![4, 1, 6],
        vec![2, 3, 1],
        vec![6, 6, 7],
        vec![4, 8, 7],
        vec![4, 2, 1],
        vec![4, 2, 3],
        vec![4, 9, 5],
        vec![8, 9, 5],
        vec![1, 9, 5],
        vec![1, 2, 1],
        vec![16, 7, 2],
        vec![10, 2, 9],
        vec![11, 9, 8],
        vec![4, 3, 5],
        vec![3, 1, 4],
        vec![13, 5, 7],
        vec![10, 8, 5],
        vec![2, 1, 5],
        vec![11, 7, 4],
        vec![2, 3, 6],
        vec![3, 7, 6],
        vec![1, 3, 2],
        vec![1, 1, 8],
        vec![2, 8, 4],
        vec![3, 1, 2],
        vec![4, 6, 1],
        vec![7, 1, 9],
        vec![1, 6, 7],
        vec![2, 5, 8],
        vec![1, 2, 9],
        vec![1, 7, 8],
        vec![5, 5, 8],
        vec![1, 2, 3],
        vec![4, 2, 5],
        vec![17, 4, 1],
        vec![10, 5, 9],
        vec![2, 4, 2],
        vec![2, 4, 1],
        vec![1, 4, 9],
        vec![1, 3, 7],
        vec![1, 7, 8],
        vec![12, 9, 2],
        vec![1, 2, 4],
        vec![1, 4, 1],
        vec![1, 1, 9],
        vec![1, 8, 1],
        vec![8, 8, 3],
        vec![2, 5, 1],
        vec![3, 1, 9],
        vec![1, 2, 6],
        vec![4, 3, 7],
        vec![1, 7, 6],
        vec![10, 9, 2],
        vec![1, 5, 9],
        vec![1, 9, 3],
        vec![17, 1, 6],
        vec![2, 1, 2],
        vec![11, 6, 7],
        vec![2, 2, 9],
        vec![2, 9, 5],
        vec![12, 7, 9],
        vec![20, 2, 7],
        vec![5, 9, 5],
        vec![21, 7, 1],
        vec![2, 6, 4],
        vec![11, 1, 4],
        vec![5, 4, 6],
        vec![1, 7, 8],
        vec![5, 9, 3],
        vec![5, 2, 8],
        vec![3, 9, 3],
        vec![2, 8, 7],
        vec![2, 1, 7],
        vec![10, 6, 3],
        vec![1, 2, 6],
        vec![2, 8, 5],
        vec![1, 6, 5],
        vec![2, 4, 9],
        vec![1, 4, 5],
        vec![8, 1, 6],
        vec![4, 4, 8],
        vec![6, 8, 4],
        vec![21, 3, 9],
        vec![5, 9, 2],
        vec![4, 7, 9],
        vec![22, 9, 3],
        vec![9, 6, 4],
        vec![2, 2, 6],
        vec![2, 2, 1],
        vec![2, 5, 7],
        vec![7, 5, 4],
        vec![22, 4, 2],
        vec![2, 5, 4],
        vec![16, 2, 5],
        vec![2, 6, 2],
        vec![13, 3, 4],
        vec![5, 5, 7],
        vec![15, 4, 7],
        vec![3, 2, 3],
        vec![3, 2, 5],
        vec![1, 1, 2],
        vec![1, 2, 4],
        vec![6, 5, 9],
        vec![4, 3, 6],
        vec![2, 5, 9],
        vec![1, 2, 7],
        vec![1, 1, 9],
        vec![2, 4, 5],
        vec![19, 7, 8],
        vec![1, 6, 5],
        vec![1, 5, 1],
        vec![1, 9, 4],
        vec![5, 8, 1],
        vec![3, 8, 1],
        vec![7, 5, 6],
        vec![3, 7, 1],
        vec![1, 2, 5],
        vec![4, 9, 8],
        vec![2, 5, 6],
        vec![10, 1, 4],
        vec![1, 7, 2],
        vec![6, 3, 4],
        vec![9, 4, 3],
        vec![2, 2, 8],
        vec![2, 9, 5],
        vec![5, 8, 3],
        vec![1, 1, 5],
        vec![2, 5, 6],
        vec![1, 1, 7],
        vec![2, 9, 7],
        vec![8, 4, 7],
        vec![3, 3, 9],
        vec![4, 6, 3],
        vec![1, 5, 3],
        vec![1, 7, 2],
        vec![1, 2, 1],
        vec![1, 6, 5],
        vec![1, 5, 2],
        vec![10, 7, 4],
        vec![10, 4, 1],
        vec![10, 1, 8],
        vec![1, 9, 6],
        vec![1, 1, 4],
        vec![11, 8, 1],
        vec![2, 9, 5],
        vec![5, 6, 3],
        vec![1, 3, 8],
        vec![4, 1, 3],
        vec![5, 3, 8],
        vec![1, 4, 7],
        vec![1, 7, 2],
        vec![13, 3, 5],
        vec![2, 2, 1],
        vec![4, 3, 1],
        vec![4, 5, 6],
        vec![3, 6, 2],
        vec![4, 5, 4],
        vec![8, 8, 7],
        vec![1, 3, 9],
    ];

    first_task(stacks.clone().as_mut_slice(), &moves);
    second_task(stacks.as_mut_slice(), &moves);
}
