#[derive(Debug)]
pub enum Path {
    Root,
    Up,
    Name(String),
}

#[derive(Debug)]
pub enum LsOutput {
    File(String, usize),
    Dir(String),
}

#[derive(Debug)]
pub enum Cmd {
    Ls(Vec<LsOutput>),
    Cd(Path)
}
