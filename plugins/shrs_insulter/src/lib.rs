use rand::{rngs::ThreadRng, thread_rng, Rng};
use shrs::{
    anyhow,
    prelude::{AfterCommandCtx, Plugin, Print},
    Context, Runtime, Shell,
};

pub struct InsulterState {
    insults: Vec<String>,
    freq: f32,
    rand: ThreadRng,
}

impl InsulterState {
    pub fn new(insults: Vec<String>, freq: f32, include_default: bool) -> Self {
        let mut insults_c = insults.clone();
        if include_default {
            let mut def_insults: Vec<&str>  = vec![
                "Boooo!",
                "Don't you know anything?",
                "RTFM!",
                "Haha, n00b!",
                "Wow! That was impressively wrong!",
                "Pathetic",
                "The worst one today!",
                "n00b alert!",
                "Your application for reduced salary has been sent!",
                "lol",
                "u suk",
                "lol... plz",
                "plz uninstall",
                "And the Darwin Award goes to.... ${USER}!",
                "ERROR_INCOMPETENT_USER",
                "Incompetence is also a form of competence",
                "Bad.",
                "Fake it till you make it!",
                "What is this...? Amateur hour!?",
                "Come on! You can do it!",
                "Nice try.",
                "What if... you type an actual command the next time!",
                "What if I told you... it is possible to type valid commands.",
                "Y u no speak computer???",
                "This is not Windows",
                "Perhaps you should leave the command line alone...",
                "Please step away from the keyboard!",
                "error code: 1D10T",
                "ACHTUNG! ALLES TURISTEN UND NONTEKNISCHEN LOOKENPEEPERS! DAS KOMPUTERMASCHINE IST NICHT FÜR DER GEFINGERPOKEN UND MITTENGRABEN! ODERWISE IST EASY TO SCHNAPPEN DER SPRINGENWERK, BLOWENFUSEN UND POPPENCORKEN MIT SPITZENSPARKEN. IST NICHT FÜR GEWERKEN BEI DUMMKOPFEN. DER RUBBERNECKEN SIGHTSEEREN KEEPEN DAS COTTONPICKEN HÄNDER IN DAS POCKETS MUSS. ZO RELAXEN UND WATSCHEN DER BLINKENLICHTEN.",
                "Pro tip: type a valid command!",
                "Go outside.",
                "This is not a search engine.",
                "(╯°□°）╯︵ ┻━┻",
                "¯\\_(ツ)_/¯",
                "So, I'm just going to go ahead and run rm -rf / for you.",
                "Why are you so stupid?!",
                "Perhaps computers is not for you...",
                "Why are you doing this to me?!",
                "Don't you have anything better to do?!",
                "I am _seriously_ considering 'rm -rf /'-ing myself...",
                "This is why you get to see your children only once a month.",
                "This is why nobody likes you.",
                "Are you even trying?!",
                "Try using your brain the next time!",
                "My keyboard is not a touch screen!",
                "Commands, random gibberish, who cares!",
                "Typing incorrect commands, eh?",
                "Are you always this stupid or are you making a special effort today?!",
                "Dropped on your head as a baby, eh?",
                "Brains aren't everything. In your case they're nothing.",
                "I don't know what makes you so stupid, but it really works.",
                "You are not as bad as people say, you are much, much worse.",
                "Two wrongs don't make a right, take your parents as an example.",
                "You must have been born on a highway because that's where most accidents happen.",
                "If what you don't know can't hurt you, you're invulnerable.",
                "If ignorance is bliss, you must be the happiest person on earth.",
                "You're proof that god has a sense of humor.",
                "Keep trying, someday you'll do something intelligent!",
                "If shit was music, you'd be an orchestra.",
                "How many times do I have to flush before you go away?",
                "You can make a happy meal sad :(",
                "I don't think you can write out the alphabet let alone code"
            ];

            insults_c.append(&mut def_insults.iter().map(|i| i.to_string()).collect());
        }
        Self {
            insults: insults_c.iter().map(|i| i.to_string()).collect(),
            freq,
            rand: thread_rng(),
        }
    }

    fn should_insult(&mut self) -> bool {
        self.rand.gen::<f32>() <= self.freq
    }
    fn rand_insult(&mut self) -> &str {
        return self.insults[self.rand.gen_range(0..self.insults.len())].as_str();
    }
}
pub struct InsulterPlugin {
    insults: Vec<String>,
    freq: f32,
    include_default: bool,
}

impl InsulterPlugin {
    pub fn new(insults: Vec<String>, freq: f32, include_default: bool) -> Self {
        Self {
            insults,
            freq,
            include_default,
        }
    }
}

impl Plugin for InsulterPlugin {
    fn init(&self, shell: &mut shrs::ShellConfig) {
        shell.hooks.register(insult_hook);
        shell.state.insert(InsulterState::new(
            self.insults.clone(),
            self.freq,
            self.include_default,
        ));
    }
}
fn insult_hook(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &AfterCommandCtx,
) -> anyhow::Result<()> {
    if ctx.exit_code != 0 {
        if let Some(state) = sh_ctx.state.get_mut::<InsulterState>() {
            if state.should_insult() {
                println!("\n{}\n", state.rand_insult());
            }
        }
    }

    Ok(())
}
