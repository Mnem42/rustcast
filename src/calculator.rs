//! This handle the logic for the calculator in rustcast

/// A struct that represents an expression
#[derive(Debug, Clone, Copy)]
pub struct Expression {
    pub first_num: f64,
    pub operation: Operation,
    pub second_num: f64,
}

/// An enum that represents the different operations that can be performed on an expression
#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Power,
}

impl Expression {
    /// This evaluates the expression
    pub fn eval(&self) -> f64 {
        match self.operation {
            Operation::Addition => self.first_num + self.second_num,
            Operation::Subtraction => self.first_num - self.second_num,
            Operation::Multiplication => self.first_num * self.second_num,
            Operation::Division => self.first_num / self.second_num,
            Operation::Power => self.first_num.powf(self.second_num),
        }
    }

    /// This parses an expression from a string (and is public)
    ///
    /// This function is public because it is used in the `handle_search_query_changed` function,
    /// and the parse expression function, while doing the same thing, should not be public due to
    /// the function name, not portraying the intention of the function.
    pub fn from_str(s: &str) -> Option<Expression> {
        Self::parse_expression(s)
    }

    /// This is the function that parses an expression from a string
    fn parse_expression(s: &str) -> Option<Expression> {
        let s = s.trim();

        // 1. Parse first (possibly signed) number with manual scan
        let (first_str, rest) = Self::parse_signed_number_prefix(s)?;

        // 2. Next non‑whitespace char must be the binary operator
        let rest = rest.trim_start();
        let (op_char, rest) = rest.chars().next().map(|c| (c, &rest[c.len_utf8()..]))?;

        let operation = match op_char {
            '+' => Operation::Addition,
            '-' => Operation::Subtraction,
            '*' => Operation::Multiplication,
            '/' => Operation::Division,
            '^' => Operation::Power,
            _ => return None,
        };

        // 3. The remainder should be the second (possibly signed) number
        let rest = rest.trim_start();
        let (second_str, tail) = Self::parse_signed_number_prefix(rest)?;
        // Optionally ensure nothing but whitespace after second number:
        if !tail.trim().is_empty() {
            return None;
        }

        let first_num: f64 = first_str.parse().ok()?;
        let second_num: f64 = second_str.parse().ok()?;

        Some(Expression {
            first_num,
            operation,
            second_num,
        })
    }

    /// Returns (number_lexeme, remaining_slice) for a leading signed float.
    /// Very simple: `[+|-]?` + "anything until we hit whitespace or an operator".
    fn parse_signed_number_prefix(s: &str) -> Option<(&str, &str)> {
        let s = s.trim_start();
        if s.is_empty() {
            return None;
        }

        let mut chars = s.char_indices().peekable();

        // Optional leading sign
        if let Some((_, c)) = chars.peek()
            && (*c == '+' || *c == '-')
        {
            chars.next();
        }

        // Now consume until we hit an operator or whitespace
        let mut end = 0;
        while let Some((idx, c)) = chars.peek().cloned() {
            if c.is_whitespace() || "+-*/^".contains(c) {
                break;
            }
            end = idx + c.len_utf8();
            chars.next();
        }

        if end == 0 {
            return None; // nothing that looks like a number
        }

        let (num, rest) = s.split_at(end);
        Some((num, rest))
    }
}
