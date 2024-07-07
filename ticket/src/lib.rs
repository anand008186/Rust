
pub mod ticket_module {
    // Define the `Order` struct and its implementation
    #[derive(Debug)]
    pub struct Ticket {
        title: String,
        description: String,
        status: String,
    }

    impl Ticket {

        // private method to check the validity of the title
        fn check_title_validity(title: &str) {
            if title.is_empty() {
                panic!("Title cannot be empty");
            }
            if title.len() > 50 {
                panic!("Title cannot be more than 50 bytes long");
            }
        }

        // private method to check the validity of the description
        fn check_description_validity(description: &str) {
            if description.is_empty() {
                panic!("Description cannot be empty");
            }
            if description.len() > 100 {
                panic!("Description cannot be more than 100 bytes long");
            }
        }

        // private method to check the validity of the status
        fn check_status_validity(status: &str) {
            if status != "To-Do" && status != "In-Progress" && status != "Done" {
                panic!("Invalid status");
            }
        }

        // public method to create a new ticket
        pub fn new(title: String, description: String, status: String) -> Ticket {
            // check the validity all the fields
            Ticket::check_title_validity(&title);
            Ticket::check_description_validity(&description);
            Ticket::check_status_validity(&status);
            Ticket {
                title,
                description,
                status,
            }
        }

        // public method to set the title
        pub fn set_title(&mut self, title: String) {
            Ticket::check_title_validity(&title);
            self.title = title;
        }

        // public method to set the description
        pub fn set_description(&mut self, description: String) {
            Ticket::check_description_validity(&description);
            self.description = description;
        }

        // public method to set the status
        pub fn set_status(&mut self, status: String) {
            Ticket::check_status_validity(&status);
            self.status = status;
        }

        // public method to get the title
        pub fn title(&self) -> &str {
            &self.title
        }

        // public method to get the description
        pub fn description(&self) -> &str {
            &self.description
        }

        // public method to get the status
        pub fn status(&self) -> &str {
            &self.status
        }
    }
}
