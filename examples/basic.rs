use shrs::prelude::*;
use shrs_reedline::Reedline;

fn main() {
    let reedline = reedline::Reedline::create();
    let prompt = reedline::DefaultPrompt::default();

    let myline = Reedline::new(reedline, prompt);

    let myshell = ShellBuilder::default()
        .with_readline(myline)
        .build()
        .unwrap();

    myshell.run();
}
