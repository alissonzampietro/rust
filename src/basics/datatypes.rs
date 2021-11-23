mod mutables_immutables;
mod string;

mod datatypes {
    pub fn run() {
        integers();
        strings::run();
        mutables_immutables::run();
    }
    
    #[allow(unused_variables)]
    fn integers() {
        let integer8bits_unsigned: u8 = 255;
        let integer16bits_unsigned: u16 = 65535;
        let integer32bits_unsigned: u32 = 4294967295;
        let integer64bits_unsigned: u64 = 5656564294967296;
    }
}