#[derive(Clone, Debug)]
struct Fibonacci {
    fib_prev: usize,
    fib_curr: usize,
    limit: Option<usize>,
}

impl Fibonacci {
    fn new(limit: Option<usize>) -> Fibonacci {
        Fibonacci {
            fib_prev: 0,
            fib_curr: 1,
            limit
        }
    }
}

impl Iterator for Fibonacci {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next_fib = self.fib_curr + self.fib_prev;
        self.fib_prev = self.fib_curr;
        self.fib_curr = next_fib;

        if let None = self.limit {
            Some(next_fib)
        } else {
            if next_fib > self.limit.unwrap() {
                None
            } else {
                Some(next_fib)
            }
        }
    }
}

fn main() {
    let my_fibo = Fibonacci::new(Some(100));

}

#[test]
fn infinite_fibo() {
    let inf_fibo = Fibonacci::new(None);
    let first_values = inf_fibo.take(11).collect::<Vec<_>>();
    assert_eq!(first_values, vec![1,2,3,5,8,13,21,34,55,89,144]);
    // println!("{:?}", inf_fibo); inf_fibo consumed by take(11)
}

#[test]
fn finite_fibo() {
    let fin_fibo = Fibonacci::new(Some(10));
    assert_eq!(fin_fibo.clone().count(), 5); 
    assert_eq!(fin_fibo.clone().nth(23), None);
    assert_eq!(fin_fibo.clone().nth(4).unwrap(), 8); 
    let f5 = fin_fibo.clone().enumerate().nth(4).unwrap();
    assert_eq!(f5.0, 4);
    assert_eq!(f5.1, 8);
    assert_eq!(fin_fibo.clone().last().unwrap(), 8);

    let even: Vec<_> = fin_fibo.filter(|fib| fib%2 == 0).collect();
    assert_eq!(even, vec![2,8]);
}
