use std::{
    env,
    path::Path,
};

pub fn get_env_path<'a>() -> &'a Path {
    let current_dir = env::current_dir().unwrap();
    let workspace_dir = match current_dir.parent(){
        Some(path) => path,
        None => panic!("invalid directory"),
    };
    let workspace_env_str = workspace_dir.to_path_buf().into_os_string()
        .into_string().unwrap() + "/.env";
    let workspace_env = std::path::Path::new(
        &workspace_env_str
        );
    let workspace_env = std::path::Path::new(".env");
    println!("workspace env: {:?}", workspace_env);
    return workspace_env
}

