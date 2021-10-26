use rand::seq::SliceRandom;
use rand::Rng;

const STACK_REQUIRED: [&dyn Fn(usize) -> String; 5] = [
    &|_| "add".to_string(),
    &|_| "sub".to_string(),
    &|_| "mul".to_string(),
    &|_| "pop".to_string(),
    &|_| "copy".to_string(),
];

fn generate_instruction(stack_size: usize) -> (String, usize) {
    if stack_size == 0 {
        (format!("pi {}", rand::random::<i32>()), stack_size + 1)
    } else if stack_size == 1 {
        let s = rand::thread_rng().gen_range(0..3);
        if s == 0 {
            (format!("pi {}", rand::random::<i32>()), stack_size + 1)
        } else if s == 1 {
            ("copy".to_string(), stack_size + 1)
        } else {
            ("pop".to_string(), stack_size - 1)
        }
    } else {
        let s = rand::thread_rng().gen_range(0..5);
        if s == 8 {
            (format!("pi {}", rand::random::<i32>()), stack_size + 1)
        } else if s == 7 && stack_size > 1 {
            (
                format!("rot {}", rand::thread_rng().gen_range(1..stack_size)),
                stack_size,
            )
        } else {
            let inst = STACK_REQUIRED.choose(&mut rand::thread_rng()).unwrap()(stack_size);
            if &inst == "copy" {
                (inst, stack_size + 1)
            } else {
                (inst, stack_size - 1)
            }
        }
    }
}

fn main() {
    let mut res = "".to_string();
    let mut stack_size = 0;
    for _ in 0..203 {
        let (instruction, s) = generate_instruction(stack_size);
        stack_size = s;
        res = format!("{}\n{}", res, instruction);
    }
    println!("{}\nexit", res)
}
