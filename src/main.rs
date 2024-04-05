mod cat;
use cat::cat::Cat;
use esp_idf_svc::sys::{gpio_set_direction, gpio_set_level, gpio_num_t_GPIO_NUM_2, gpio_mode_t_GPIO_MODE_OUTPUT};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Configurar el pin D2 (GPIO 2) como salida
    initialize_d2();

    let whiskers = Cat::new("Grillo".to_string(), 3, "tabbybrown".to_string());

    loop {
        whiskers.meow();
        blink_on();
        sleep(Duration::from_secs(1));
        blink_of();
        sleep(Duration::from_secs(1));
    }
}

fn blink_on(){
    unsafe {
        gpio_set_level(gpio_num_t_GPIO_NUM_2 as i32, 1);
    }
}

fn blink_of(){
    unsafe {
        gpio_set_level(gpio_num_t_GPIO_NUM_2 as i32, 0);
    }
}

fn initialize_d2(){
    unsafe {
        gpio_set_direction(gpio_num_t_GPIO_NUM_2 as i32, gpio_mode_t_GPIO_MODE_OUTPUT);
    }
}