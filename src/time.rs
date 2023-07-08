pub(crate) struct Name<'a> {
    first_name: &'a str,
    last_name: &'a str,
}

impl<'a> Name<'a> {
    pub(crate) fn new(full_name: &'a str) -> Self {
        let mut parts = full_name.trim().split(" ");
        let first_name = parts.next().unwrap();
        let last_name = parts.next().unwrap();
        Self {
            first_name,
            last_name,
        }
    }
}
