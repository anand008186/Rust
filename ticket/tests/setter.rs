use ticket::ticket_module::Ticket;


#[test]
fn test_setters() {
    let mut ticket = Ticket::new(
        "Title".to_string(),
        "Description".to_string(),
        "To-Do".to_string(),
    );
    ticket.set_title("New Title".to_string());
    ticket.set_description("New Description".to_string());
    ticket.set_status("Done".to_string());

    assert_eq!(ticket.title(), "New Title");
    assert_eq!(ticket.description(), "New Description");
    assert_eq!(ticket.status(), "Done");
}
