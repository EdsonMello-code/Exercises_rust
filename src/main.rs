fn main() {
    // Funcion for get name
    fn get_name(name: &str) {
        return println!("Your name is {}", name);
    }

    get_name("Edson");

    // Funtion for plus
    fn plus(value1: i64, value2: i64) {
        let result: i64 = value1 + value2;
        println!(
            "O primeiro valor digitado é {}, já o se o segundo é {}, portanto o resultado é {}",
            value1, value2, result
        );
    }

    plus(7, 2);

    // Function for verify if the number is par

    fn verify_is_par_or_impar(value: i64) {
        let number: i64 = value % 2;

        let mut response: &str = "";

        if number == 0 {
            response = "Par";
        } else {
            response = "Impar";
        }

        return println!("The value {} is {}", value, response);
    }

    verify_is_par_or_impar(9);
}
