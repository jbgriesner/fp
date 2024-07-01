use fp::app::App;
use fp::prelude::*;
use fp::run;

fn main() -> Result<()> {
    let app = App::new();
    run(app)
}
