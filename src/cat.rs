pub mod cat {
    use esp_idf_svc::log::EspLogger;

    pub struct Cat {
        pub name: String,
        pub age: u8,
        pub color: String,
        output : () // EspLogger::initialize_default(
    }

    impl Cat {
        pub fn new(name: String, age: u8, color: String) -> Cat {
            Cat {
                name,
                age,
                color,
                output: EspLogger::initialize_default()
            }
        }

        pub fn meow(&self) {
            log::info!("{} says: Meow! im {} and i am {} years old", self.name,self.color,self.age);
        }
    }
}