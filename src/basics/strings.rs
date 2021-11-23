mod strings {
    pub fn run() {
        str_example();
    }
    
    // most primitive type of string, you can only use it with "&"
    fn str_example() {
        let example1: &str = "Tento";
        println!("{}", example1);
    }
}