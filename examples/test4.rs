use std::io::{self, Write};

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn ask_yes_no(question: &str) -> bool {
    loop {
        print!("{} (y/n): ", question);
        io::stdout().flush().unwrap();
        let answer = read_line().to_lowercase();
        match answer.as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please answer with 'y' or 'n'."),
        }
    }
}

fn test_unicode_support() -> bool {
    println!("Unicode Character Test for rpian-terminal");
    println!("Please check if you can see the following characters correctly:");
    println!();

    let test_chars = [
        ('█', "Full Block"),
        ('▀', "Upper Half Block"),
        ('▄', "Lower Half Block"),
        ('▌', "Left Half Block"),
        ('▐', "Right Half Block"),
        ('░', "Light Shade"),
        ('▒', "Medium Shade"),
        ('▓', "Dark Shade"),
        ('╭', "Top-Left Rounded Corner"),
        ('╮', "Top-Right Rounded Corner"),
        ('╰', "Bottom-Left Rounded Corner"),
        ('╯', "Bottom-Right Rounded Corner"),
        ('─', "Horizontal Line"),
        ('│', "Vertical Line"),
        ('┌', "Top-Left Corner"),
        ('┐', "Top-Right Corner"),
        ('└', "Bottom-Left Corner"),
        ('┘', "Bottom-Right Corner"),
    ];

    for (char, description) in &test_chars {
        println!("{} - {} ({})", char, char, description);
    }

    println!();
    ask_yes_no("Can you see all the above characters correctly?")
}

fn main() {
    let unicode_supported = test_unicode_support();

    if unicode_supported {
        println!(
            "\nGreat! Your terminal supports the necessary Unicode characters for rpian-terminal."
        );
        println!("You can use the Unicode block mode for the best visual experience.");
    } else {
        println!(
            "\nIt seems your terminal might not fully support the necessary Unicode characters."
        );
        println!("You may want to use the ASCII block mode for better compatibility.");
        println!("Alternatively, try changing your terminal font to one that supports Unicode block characters.");
    }

    println!("\nBased on this test, we recommend using:");
    println!(
        "BlockMode::{}",
        if unicode_supported {
            "Unicode"
        } else {
            "Ascii"
        }
    );
}
