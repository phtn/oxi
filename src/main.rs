
mod types;
mod ios;
mod loops;

fn main() {
    println!("Hello, world!");
    println!("Sanity Check");

    print!("u8 MAX: ");
    println!("{}", std::u8::MAX);
    types::run();
    ios::run();  
    loops::run() 
}
