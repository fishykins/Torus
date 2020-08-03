use crate::error::ObjResult;
use std::io::BufRead;

pub fn lex<T, F>(input: T, mut callback: F) -> ObjResult<()>
where
    T: BufRead,
    F: FnMut(&str, &[&str]) -> ObjResult<()>,
{
    // This is a buffer of the "arguments" for each line, it uses raw pointers
    // in order to allow it to be re-used across iterations.
    let mut args: Vec<*const str> = Vec::new();

    // This is a buffer for continued lines joined by '\'.
    let mut multi_line = String::new();

    for line in input.lines() {
        let line = line?;
        let line = line.split('#').next().unwrap(); // Remove comments

        if line.ends_with('\\') {
            multi_line.push_str(&line[..line.len() - 1]);
            multi_line.push(' '); // Insert a space to delimit the following lines
            continue;
        }

        multi_line.push_str(line); // Append the current line

        {
            let mut words = multi_line.split_whitespace();

            if let Some(stmt) = words.next() {
                // Add the rest of line to the args buffer, the &str coerces to *const str
                for w in words {
                    args.push(w);
                }
                // Transmute the slice we get from args (&[*const str]) to the type
                // we want (&[&str]), this is safe because the args vector is
                // cleared after the callback returns, meaning the raw pointers don't
                // outlive the data they're pointing to.
                callback(stmt, unsafe {
                    &*(&args[..] as *const [*const str] as *const [&str])
                })?;
                // Clear the args buffer for reuse on the next iteration
                args.clear();
            }
        }

        multi_line.clear();
    }

    Ok(())
}