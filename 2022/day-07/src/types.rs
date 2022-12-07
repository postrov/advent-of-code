#[derive(Debug)]
pub enum Path<'a> {
    Root,
    Up,
    Name(&'a str),
}

#[derive(Debug)]
pub enum LsOutput<'a> {
    File(&'a str, usize),
    Dir(&'a str),
}

#[derive(Debug)]
pub enum Cmd<'a> {
    Ls(Vec<LsOutput<'a>>),
    Cd(Path<'a>)
}
