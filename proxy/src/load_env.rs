use std::{env, path::Path};

pub fn load_env() {
    let mut dir = env::current_dir().unwrap();
    recursive_env_func(&mut dir);
}

fn recursive_env_func<'a>(dir: &'a Path) {
    let workspace_env = dir.to_path_buf().into_os_string().into_string().unwrap() + "/.env";

    match dotenvy::from_path(workspace_env) {
        Ok(()) => (),
        Err(_) => match dir.parent() {
            Some(new_dir) => recursive_env_func(new_dir),
            None => panic!("invalid directory"),
        },
    }
}
