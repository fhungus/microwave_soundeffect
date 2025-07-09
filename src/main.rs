use mlua::{FromLua, Lua, MultiValue, ObjectLike, Value};
use std::{env, fs::read_to_string};

struct IterationData<'a> {
    iteration: u64,
    data: &'a str,
    result: Option<MultiValue>,
}

fn run_file(iteration: &mut IterationData) {
    let lua = Lua::new();

    let iteration_table = lua.create_table().unwrap();
    _ = iteration_table.set("iteration", iteration.iteration);
    _ = iteration_table.set("data", iteration.data);

    let globals = lua.globals();
    _ = globals.set("iteration_data", iteration_table);

    let result = match lua.load(iteration.data).eval::<MultiValue>() {
        Ok(value) => value,
        Err(e) => {
            panic!("An error occured: {}", e);
        }
    };

    iteration.result = Some(result)
}

enum FileOperation {
    Add(u64, String),
    Replace(u64, String),
    Remove(u64),
}

fn evaluate_next_file(iteration: &IterationData) -> String {
    match iteration.result.clone() {
        Some(deque) => {
            // parse until we get a table
            match deque.iter().find(|v| v.is_table()) {
                Some(v) => {
                    let v = v.as_table().unwrap();
                    for pair in v.pairs::<mlua::Value, mlua::Value>() {
                        let (key, value): (mlua::Value, mlua::Value) = pair.unwrap();

                        let key = key.as_string().unwrap().to_str().unwrap();
                        let value = value.as_string().unwrap().to_str().unwrap();

                        let line = &key[1..].parse::<u64>().unwrap();
                        let operation = match &key[0..1] {
                            "+" => {}
                            "!" => {}
                            "-" => {}
                            _ => {
                                panic!("uh oh");
                            }
                        };
                    }

                    return String::new();
                }
                None => {
                    panic!(
                        "Iteration {} produced no data.", // should exit gracefully but to be honest i don't know how to do that
                        iteration.iteration
                    );
                }
            };
        }
        None => {
            panic!("not supposed to happen...")
        }
    }
}

fn main() {
    let args = env::args();
    if let Some(path) = args.last() {
        let mut iterations: u64 = 0;
        loop {
            iterations += 1;
            let contents = read_to_string(&path).expect("Could not find file in path");

            let mut iteration = IterationData {
                iteration: iterations,
                data: &contents[0..],
                result: None,
            };

            run_file(&mut iteration);
        }
    }
}
