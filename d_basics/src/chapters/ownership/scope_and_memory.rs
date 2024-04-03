pub fn scope_and_memory() {
    // this goes to stack memory...
    {
        // s is not valid here, it’s not yet declared
        let _s = "hello";
        // s can be accessed here.
    } // this scope is now over, and s is no longer valid

    // this goes to heap memory...
    {
        // s is not valid here, it’s not yet declared
        let _s = String::from("hello");

        // s can be accessed here.
    } // this scope is now over, and s is no longer valid
}
