mod config;
mod setup;
mod launcher;

fn show_err_stack(top_err: &dyn std::error::Error) {
    eprintln!("{}", top_err);
    #[cfg(debug_assertions)]
    {
        let mut err_obj : &dyn std::error::Error = top_err;
        while let Some(source) = err_obj.source() {
            eprintln!("  Because: {}", source);
            err_obj = source;
        }
    }
}

fn main() {

    if let Err(e) = launcher::launch_app() {
        //println!("{}", e);   // show only top err message
        //println!("{:?}", e); // show all the errors
        show_err_stack(&e);  // show all the errors (customized)
        std::process::exit(1);
    }
   
}
