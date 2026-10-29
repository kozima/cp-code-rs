#![allow(non_snake_case)]

// Problem data that is provided before the first turn.
struct ProblemInput {}

// Reads the fixed problem data from the judge.
fn read_problem_input<R: std::io::BufRead>(
    _source: &mut proconio::source::line::LineSource<R>,
) -> ProblemInput {
    todo!("Read the fixed initial input")
}

// Judge input received before the program's action.
struct PreActionInput {}

// Judge input received after the program's action.
struct PostActionInput {}

// Action submitted to the judge at one turn.
struct Action {}

// Abstracts the interaction with either the official judge or a local tester.
trait Interaction {
    // Reads the input sent by the judge before the program's action.
    fn read_pre_action_input(&mut self) -> PreActionInput;

    // Sends the program's action to the judge.
    fn write_action(&mut self, action: &Action);

    // Reads the input sent by the judge after the program's action.
    fn read_post_action_input(&mut self) -> PostActionInput;

    fn terminate(&mut self);
}

// Communicates with the official interactive judge through stdin and stdout.
mod judge_interaction {
    use crate::*;

    pub struct JudgeInteraction<R: std::io::BufRead> {
        source: proconio::source::line::LineSource<R>,
    }

    impl<R: std::io::BufRead> JudgeInteraction<R> {
        pub fn new(
            source: proconio::source::line::LineSource<R>,
        ) -> JudgeInteraction<R> {
            JudgeInteraction { source }
        }
    }

    impl<R: std::io::BufRead> Interaction for JudgeInteraction<R> {
        fn read_pre_action_input(&mut self) -> PreActionInput {
            // Reads the input that precedes the program's action.
            todo!("Parse the input before the action")
        }

        fn write_action(&mut self, _action: &Action) {
            // Writes the action and flushes stdout before waiting for the judge.
            todo!("Write the action and flush stdout")
        }

        fn read_post_action_input(&mut self) -> PostActionInput {
            // Reads the input that follows the program's action.
            todo!("Parse the input after the action")
        }

        fn terminate(&mut self) {
            // Notifies the official judge that no further actions are sent.
        }
    }
}

// Simulates the judge's behavior using a local implementation.
#[allow(dead_code)]
mod simulation_interaction {
    use crate::*;

    pub struct SimulationInteraction {
        problem_input: ProblemInput,
    }

    // Loads the fixed input and turn data used by the local tester.
    impl SimulationInteraction {
        pub fn new(_file: std::fs::File) -> SimulationInteraction {
            todo!("Implement the local tester input format")
        }

        // Returns the fixed problem data loaded from the local file.
        pub fn problem_input(&self) -> &ProblemInput {
            &self.problem_input
        }
    }

    impl Interaction for SimulationInteraction {
        fn read_pre_action_input(&mut self) -> PreActionInput {
            // Reads the pre-action input from the local simulation.
            todo!("Read the input before the action")
        }

        fn write_action(&mut self, _action: &Action) {
            // Records or evaluates the action in the local simulation.
            todo!("Apply the action to the local simulation")
        }

        fn read_post_action_input(&mut self) -> PostActionInput {
            // Reads the post-action input from the local simulation.
            todo!("Read the input after the action")
        }

        fn terminate(&mut self) {}
    }
}

// Maintains the solution state and processes turns until the time limit.
fn solve<T: Interaction>(
    problem_input: &ProblemInput,
    timer: &Timer,
    interaction: &mut T,
) {
    let _ = problem_input;

    while timer.elapsed_millis() < TIME_LIMIT_MS {
        let before = interaction.read_pre_action_input();

        // Decide the next action from the current state and the pre-action input.
        let action = Action {};
        let _ = before;
        interaction.write_action(&action);

        // Update the current state from the post-action input.
        let after = interaction.read_post_action_input();
        let _ = after;
    }
    interaction.terminate();
}

fn main() {
    let timer = Timer::start();
    if let Some(file) = std::env::args()
        .nth(1)
        .and_then(|path| std::fs::File::open(path).ok())
    {
        // Use the local tester when an input file is specified.
        use simulation_interaction::*;
        let mut interaction = SimulationInteraction::new(file);
        let problem_input = interaction.problem_input();
        // `solve` mutates only tester, so clone or restructure this branch
        // after implementing Input if a local tester needs owned input.
        let _ = (problem_input, timer, interaction);
    } else {
        // Use the official interactive judge by default.
        use judge_interaction::*;
        type Reader = std::io::BufReader<std::io::Stdin>;
        let reader = std::io::BufReader::new(std::io::stdin().lock());
        let mut source = proconio::source::line::LineSource::new(reader);
        let problem_input = read_problem_input(&mut source);
        let mut interaction = JudgeInteraction::new(source);
        solve(&problem_input, &timer, &mut interaction);
    }
    eprintln!("elapsed {} ms", timer.elapsed_millis());
}

// Measures elapsed wall-clock time for time-limited search.
#[derive(Clone, Copy)]
struct Timer {
    start: std::time::Instant,
}

impl Timer {
    fn start() -> Timer {
        Timer {
            start: std::time::Instant::now(),
        }
    }

    fn elapsed_millis(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }
}

const TIME_LIMIT_MS: u64 = 1900;
