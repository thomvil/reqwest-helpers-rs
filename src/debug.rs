use crate::prelude::*;

impl Client {
    pub fn print_cookies(&self, context: &str) {
        println!("Cookies ({context}):");
        let store = self
            .cookies()
            .lock()
            .expect("Don't print cookies in weird concurrent context");
        let count = store.iter_any().count();
        if count == 0 {
            println!("  Cookie store is empty");
        } else {
            for c in store.iter_any() {
                println!("  {:?}", c);
            }
        }
        println!();
    }
}
