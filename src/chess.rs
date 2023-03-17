
// converts a FEN string to ASCII art
pub fn fen_to_str(fen: String) -> String {
    let mut ascii = "".to_string();
    for c in fen.chars() {
        match c.to_digit(10) {
            Some(i) => {
                for _ in 0..i {
                    ascii.push('.');
                    ascii.push(' ');
                }
            }
            None => {
                if c == ' ' {
                    return ascii
                }
                if c == '/' {
                    ascii.push(' ');
                    ascii.push('\n');
                } else {
                    ascii.push(c);
                    ascii.push(' ')
                }
            }
        }
    }
    ascii
}