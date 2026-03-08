use backtrack::{solver, Problem}; // 'solver' এর বদলে অনেক সময় 'solve' ব্যবহার হয়

struct MyProblem {
    size: usize,
}

impl Problem for MyProblem {
    type State = Vec<usize>;
    type Value = usize;

    fn candidates(&self, state: &Self::State) -> Vec<Self::Value> {
        let mut options = Vec::new();
        for i in 0..self.size {
            if !state.contains(&i) {
                options.push(i);
            }
        }
        options
    }

    fn is_solution(&self, state: &Self::State) -> bool {
        state.len() == self.size
    }

    fn add(&self, mut state: Self::State, value: Self::Value) -> Self::State {
        state.push(value);
        state
    }
}

fn main() {
    let my_problem = MyProblem { size: 4 };
    let initial_state = Vec::new();

    if let Some(solution) = solve(&my_problem, initial_state).next() {
        println!("We got the solution: {:?}", solution);
    } else {
        println!("No solution found");
    }
}
