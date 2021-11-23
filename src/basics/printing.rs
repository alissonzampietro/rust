mod printing {
    pub fn run() {
        show_message("Testing show message");
    }
    
    pub fn show_message(message:&str) {
        print!("{}", message);
    }
}