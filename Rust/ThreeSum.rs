impl Solution {
    pub fn three_sum(mut a: Vec<i32>) -> Vec<Vec<i32>> {
      let mut res = Vec::new();
    a.sort();

    let ra = a.len();
    for i in 0..ra - 2 {
        if i > 0 && a[i] == a[i - 1] {
            continue;   }
        let mut le = i + 1;
        let mut ri = ra - 1;

        while le < ri {
            let s = a[i] + a[le] + a[ri];
            if s == 0 {
                res.push(vec![a[i], a[le], a[ri]]);
                le += 1;
                ri -= 1;

                while le < ri && a[le] == a[le - 1] {
                    le += 1;
                }
                while le < ri && a[ri] == a[ri + 1] {
                    ri -= 1;
                }
            } else if s < 0 {
                le += 1;
            } else {
                ri -= 1;
            }
        }
    }
    // println!("{:?}", res);
    return res;   
    }
}