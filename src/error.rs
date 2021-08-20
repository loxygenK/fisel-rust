pub trait UnwrapOrError<T, E> {
    fn unwrap_or_error(self, message: fn(E) -> String) -> T;
}

fn error_and_panic(message: &str) -> ! {
    println!("[!] Fatal error:");
    println!("    {}", message);

    panic!("Cannot continue running: {}", message);
}

impl<T, E> UnwrapOrError<T, E> for Result<T, E> {
    fn unwrap_or_error(self, message: fn(E) -> String) -> T {
        if let Ok(t) = self { return t }
        error_and_panic(&message(self.err().unwrap()));
    }
}

impl<T> UnwrapOrError<T, ()> for Option<T> {
    fn unwrap_or_error(self, message: fn(()) -> String) -> T {
        if let Some(t) = self { return t }
        error_and_panic(&message(()));
    }
}
