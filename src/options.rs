pub struct FuzzyPassOptions<'a> {
    pub bind: Vec<&'a str>,
    pub prompt: Option<&'a str>,
    pub cmd_prompt: Option<&'a str>,
    pub cmd: Option<&'a str>,
    pub interactive: bool,
    pub query: Option<&'a str>,
    pub cmd_query: Option<&'a str>,
    pub regex: bool,
    pub color: Option<&'a str>,
    pub no_height: bool,
    pub no_clear: bool,
    pub no_clear_start: bool,
    pub min_height: Option<&'a str>,
    pub height: Option<&'a str>,
    pub preview: Option<&'a str>,
    pub select1: bool,
    pub exit0: bool,
    pub sync: bool,
    pub no_mouse: bool,
    pub expect: Option<String>,
}

impl<'a> Default for FuzzyPassOptions<'a> {
    fn default() -> Self {
        Self {
            expect: None,
            bind: vec![],
            prompt: Some("> "),
            cmd_prompt: Some("c> "),
            cmd: None,
            interactive: false,
            query: None,
            cmd_query: None,
            regex: false,
            color: None,
            no_height: false,
            no_clear: false,
            no_clear_start: false,
            min_height: Some("10"),
            height: Some("100%"),
            preview: None,
            select1: false,
            exit0: false,
            sync: false,
            no_mouse: false,
        }
    }
}
