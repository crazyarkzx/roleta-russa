use std::process::Command;
use std::io::{self, Write};

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    print!("Tem certeza que quer continuar? (s/n): ");
    io::stdout().flush().unwrap();

    let mut sorn = String::new();
    io::stdin().read_line(&mut sorn).unwrap();

    if sorn.trim().to_lowercase() != "s" {
        print!("\x1B[2J\x1B[1;1H");
        println!("Covarde 🤣");
        return;
    }

    let mut round = 1;

    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("Round {}", round);
        std::thread::sleep(std::time::Duration::from_secs(2));

        print!("Preparando");
        io::stdout().flush().unwrap();

        for _ in 0..5 {
            print!(".");
            io::stdout().flush().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        println!();

        let bala = rounds();

        if bala == 1 {
            print!("\x1B[2J\x1B[1;1H");
            println!("Se Fudeu!\n");
            println!("3");
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("2");
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("1");
            std::thread::sleep(std::time::Duration::from_secs(1));
            print!("\x1B[2J\x1B[1;1H");
            killsys();
        } else {
            print!("\x1B[2J\x1B[1;1H");
            println!("\nSortudo, Não foi dessa vez...");
            println!();

            if round >= 4 {
                print!("\x1B[2J\x1B[1;1H");
                println!("Nossa, muito bom ein! você passou por {} rounds sem ir de F", round);
                println!("Mas sabe o que é pior?");
                std::thread::sleep(std::time::Duration::from_secs(2));
                println!("A próxima bala, pode ser a última...");
                println!();
            }
            
            print!("Quer continuar? (s/n): ");
            io::stdout().flush().unwrap();

            let mut sorn2 = String::new();
            io::stdin().read_line(&mut sorn2).unwrap();

            if sorn2.trim().to_lowercase() != "s" {
                print!("\x1B[2J\x1B[1;1H");
                println!("Tmj Frango");
                break;
            }
            round += 1;
            rounds();
            println!();
        }
    }
}

fn rounds() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let tmp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    ((tmp % 6) + 1) as u32
}

fn killsys() -> ! {
    #[cfg(target_os = "windows")] { Command::new("shutdown").args(&["/s", "/f", "/t", "0"]).spawn().ok(); }
    #[cfg(target_os = "linux")] { Command::new("shutdown").args(&["-h", "now"]).spawn().ok(); }
    #[cfg(target_os = "macos")] { Command::new("shutdown").args(&["-h", "now"]).spawn().ok(); }
    #[cfg(target_os = "android")] { Command::new("su").args(&["-c", "reboot -p"]).spawn().ok(); }
    std::process::exit(0);
}
