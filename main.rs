/* Author: Daria Gerasimenko */
/* Creation date: 04.11.24 */

use std::fmt;
use std::io;
use std::str::FromStr;

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ± {:.2}i", self.real, self.imag)
    }
}

#[derive(Debug)]
struct Equation {
    a: f64,
    b: f64,
    c: f64,
    roots: [f64; 2],
    croot: Complex
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign_a = if self.a >= 0.0 { "" } else { "-" };
        let sign_b = if self.b >= 0.0 { "+" } else { "-" };
        let sign_c = if self.c >= 0.0 { "+" } else { "-" };
        
        write!(f, "{}{:.2}x^2 {} {:.2}x {} {:.2} = 0", 
                sign_a, self.a.abs(),
                sign_b, self.b.abs(),
                sign_c, self.c.abs()
        )
    }
}

impl Equation {
    fn calculate_d (&self) -> f64 {
        let d: f64 = self.b * self.b - 4.0 * self.a * self.c;
        return d;
    }
    
    fn number_of_roots (&self) -> i32 {
        if self.calculate_d() > 0.0 { return 2 };
        if self.calculate_d() == 0.0 { return 1 };
        if self.calculate_d() < 0.0 { return -1 };
        
        return -1;
    }
    
    fn find_roots (&mut self) -> () {
        if self.number_of_roots() > 0 {
            self.roots[0] = (self.b * (-1.0) - self.calculate_d().sqrt() ) / (2.0 * self.a);
            self.roots[1] = (self.b * (-1.0) + self.calculate_d().sqrt() ) / (2.0 * self.a);
            
            if self.roots[0] != self.roots[1] {
                println!("x1 ≈ {:.2}\nx2 ≈ {:.2}", self.roots[0], self.roots[1]);
            }
            else {
                println!("x ≈ {:.2}", self.roots[0]);
            }
        }
        else if self.number_of_roots() < 0 {
            self.croot = Complex{ real: self.b * (-1.0) / (2.0 * self.a), imag: self.calculate_d().abs().sqrt() / (2.0 * self.a) };
            println!("x = {}", self.croot);
        }
    }
}

fn parse_user_input(user_input: &str) -> Result<Equation, String> {
    let binding = user_input.replace(',', ".");
    let parts: Vec<&str> = binding.split_whitespace().collect();

    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    let mut c: f64 = 0.0;

    if parts.len() != 7 && parts.len() != 5 {
        return Err("Неправильный формат.\nСоблюдайте формат ввода:\n 
                    ax^2 +/- bx +/- c = 0\n
                    или  ax^2 +/- bx = 0\n
                    или ax^2 +/- с = 0\n".to_string());
    }    
        
    for (i, part) in parts.iter().enumerate() {
        match i {
          0 => { // Обработка коэффициента 'a'
            if part.contains("x^2") {
              let a_str = part.split("x^2").next().unwrap_or("");
              a = match a_str {
                "-" => -1.0,
                "" => 1.0,
                _ => f64::from_str(a_str).map_err(|_| "Неверный коэффициент 'a'".to_string())?,
              }
            } else {
              return Err("Отсутствует 'x^2'. Введите пожалуйста квадратное уравнение".to_string());
            }
          }
          2 => { // Обработка коэффициента 'b'
            if parts.len() == 7 { // Проверяем, есть ли элемент 'bx'
              let b_str = parts[i - 1]; // Получаем знак из предыдущего элемента
              let b_value = parts[i];
              if !b_value.contains("x") {
                return Err("В члене 'bx' должен присутствовать 'x'".to_string());
              }
              b = match b_str {
                "-" => -f64::from_str(b_value.split("x").next().unwrap_or("0")).map_err(|_| "Неверный коэффициент 'b'".to_string())?,
                _ => f64::from_str(b_value.split("x").next().unwrap_or("0")).map_err(|_| "Неверный коэффициент 'b'".to_string())?,
              }
             } else if parts.len() == 5 {
                let b_str = parts[i - 1];
                let b_value = parts[i];
                if !b_value.contains("x") {
                    c = match b_str {
                    "-" => -f64::from_str(b_value).map_err(|_| "Неверный коэффициент 'c'".to_string())?,
                    _ => f64::from_str(b_value).map_err(|_| "Неверный коэффициент 'c'".to_string())?,
                    }
                } else {
                    b = match b_str {
                    "-" => -f64::from_str(b_value.split("x").next().unwrap_or("0")).map_err(|_| "Неверный коэффициент 'b'".to_string())?,
                    _ => f64::from_str(b_value.split("x").next().unwrap_or("0")).map_err(|_| "Неверный коэффициент 'b'".to_string())?,
                    }
                }
              }
           }
           4 => { // Обработка коэффициента 'c'   
             if parts.len() == 7 {
               let c_str = parts[i - 1];
               let c_value = parts[i]; 
                c = match c_str {
                 "-" => -f64::from_str(c_value).map_err(|_| "Неверный коэффициент 'c'".to_string())?,
                  _ => f64::from_str(c_value).map_err(|_| "Неверный коэффициент 'c'".to_string())?,
                }
              }
            }
            _ => {} // Игнорируем другие элементы
        }
    }
    
    println!("Полученные коэффициенты: a = {:.2}, b = {:.2}, c = {:.2}", a, b, c); 

    Ok(Equation {
        a,
        b,
        c,
        roots: [0.0, 0.0],
        croot: Complex { real: 0.0, imag: 0.0 },
    })
}


fn main() {
    print!("\x1B[2J\x1B[1;1H"); // Очистка терминала

    loop {
        println!("\nВведите квадратное уравнение в формате ax^2 +/- bx +/- c = 0:");
        let mut user_input = String::new();
        
        io::stdin().read_line(&mut user_input).expect("Ошибка при чтении строки");
        match parse_user_input(user_input.trim()) {
            Ok(mut equation) => {
                equation.find_roots();
                break;
            } Err(error) => {
                println!("Ошибка: {}", error);
            }
        }   
    }
}
