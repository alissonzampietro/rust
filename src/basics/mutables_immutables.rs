mod mutables_immutables {
    pub fn run() {
        immutable();
        mutable();
    }
    
    // if you want to change the value of a variable, you have to use the prefix mut
    fn mutable() {
        let mut name: &str = "My name is Alisson Zampietro Rodrigues";
        printing::show_message(name);
    }
}