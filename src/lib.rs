use shrs::prelude::*;

pub struct Reedline {
    reedline: reedline::Reedline,
    prompt: Box<dyn reedline::Prompt>,
}

impl Reedline {
    pub fn new(reedline: reedline::Reedline, prompt: impl reedline::Prompt + 'static) -> Self {
        Self {
            reedline,
            prompt: Box::new(prompt),
        }
    }
}

impl Readline for Reedline {
    fn read_line(&mut self, sh: &Shell, ctx: &mut Context, rt: &mut Runtime) -> String {
        use reedline::Signal;
        match self.reedline.read_line(&*self.prompt) {
            Ok(Signal::Success(inner)) => inner,
            Ok(_) => String::new(),
            Err(_) => String::new(),
        }
    }
}
