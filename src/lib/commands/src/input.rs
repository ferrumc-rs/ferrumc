/// Very based on Cloud, this is gonna have to be changed up a bit probably.
#[derive(Clone)]
pub struct CommandInput {
    pub input: String,
    pub cursor: u32,
}

impl CommandInput {
    pub fn of(string: String) -> Self {
        Self {
            input: string,
            cursor: 0,
        }
    }
    pub fn append_string(&mut self, string: String) {
        self.input += &*string;
    }
    pub fn move_cursor(&mut self, chars: u32) {
        if self.cursor + chars > self.input.len() as u32 {
            return;
        }

        self.cursor += chars;
    }
    pub fn remaining_length(&self) -> u32 {
        self.input.len() as u32 - self.cursor
    }
    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.cursor as usize)
    }
    pub fn has_remaining_input(&self) -> bool {
        self.cursor < self.input.len() as u32
    }
    pub fn skip_whitespace(&mut self, max_spaces: u32, preserve_single: bool) {
        if preserve_single && self.remaining_length() == 1 && self.peek() == Some(' ') {
            return;
        }

        let mut i = 0;
        while i < max_spaces
            && self.has_remaining_input()
            && self.peek().is_some_and(|c| c.is_whitespace())
        {
            self.read(1);
            i += 1;
        }
    }
    pub fn remaining_input(&self) -> String {
        self.input[self.cursor as usize..].to_string()
    }
    pub fn peek_string_chars(&self, chars: u32) -> String {
        let remaining = self.remaining_input();
        if chars > remaining.len() as u32 {
            return "".to_string();
        }

        remaining[0..chars as usize].to_string()
    }
    pub fn read(&mut self, chars: u32) -> String {
        let read_string = self.peek_string_chars(chars);
        self.move_cursor(chars);
        read_string
    }
    pub fn remaining_tokens(&self) -> u32 {
        let count = self.remaining_input().split(' ').count() as u32;
        if self.remaining_input().ends_with(' ') {
            return count + 1;
        }
        count
    }
    pub fn read_string(&mut self) -> String {
        self.skip_whitespace(u32::MAX, false);
        let mut result = String::new();
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                break;
            }
            result.push(c);
            self.move_cursor(1);
        }
        result
    }
    pub fn peek_string(&self) -> String {
        let remaining = self.remaining_input();
        remaining
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_string()
    }
    pub fn read_until(&mut self, separator: char) -> String {
        self.skip_whitespace(u32::MAX, false);
        let mut result = String::new();
        while let Some(c) = self.peek() {
            if c == separator {
                self.move_cursor(1);
                break;
            }
            result.push(c);
            self.move_cursor(1);
        }
        result
    }
    pub fn read_string_skip_whitespace(&mut self, preserve_single: bool) -> String {
        let read_string = self.read_string();
        self.skip_whitespace(u32::MAX, preserve_single);
        read_string
    }
}
