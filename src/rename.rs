use crate::AnimeEntry;

const TOKEN_EPISODE_NUMBER: &str = "episode";

impl AnimeEntry {
    pub fn get_new_name(&self) -> String {
        let pattern = self
            .get_rename_pattern()
            .expect("Rename pattern shouldn't be None here");

        substitute(&pattern, &self)
    }
}

fn substitute(pattern: &str, entry: &AnimeEntry) -> String {
    let mut new_name = String::new();
    let mut expression = String::new();
    let mut in_expression = false;

    for char in pattern.chars() {
        if char == '{' {
            in_expression = true;
            continue;
        }
        if char == '}' {
            in_expression = false;
            new_name.push_str(&evaluate_expression(&expression, entry));
            continue;
        }
        if in_expression {
            expression.push(char);
            continue;
        }
        new_name.push(char);
    }

    new_name
}

// Should be either episode_number or episode_number+x
fn evaluate_expression(expression: &str, entry: &AnimeEntry) -> String {
    if expression.starts_with(TOKEN_EPISODE_NUMBER) {
        if expression.contains("+") {
            // TODO
            let mut buffer = String::new();
            for char in expression.chars() {
                if char.is_ascii_digit() {
                    buffer.push(char);
                }
            }
            let evalutated = entry.get_current_episode() as i32
                + buffer.parse::<i32>().expect("Unable to parse number");
            return evalutated.to_string();
        }
        return entry.get_current_episode().to_string();
    }

    println!("Warning: invalid expression, defaulting to episode_number");
    entry.get_current_episode().to_string()
}
