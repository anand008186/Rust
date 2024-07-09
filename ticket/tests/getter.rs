use ticket::ticket_module::Ticket;

#[test]
#[should_panic]
fn test_empty_title() {
    Ticket::new(
        "".to_string(),
        "Description".to_string(),
        "To-Do".to_string(),
    );
}
#[test]
#[should_panic]
fn test_empty_description() {
    Ticket::new("Title".to_string(), "".to_string(), "To-Do".to_string());
}
#[test]
#[should_panic]
fn test_long_title() {
    Ticket::new("This is a very long title that is more than 50 bytes long. This is going to be biggest title ever ".to_string(), "Description".to_string(), "To-Do".to_string());
}
#[test]
#[should_panic]
fn test_long_description() {
    Ticket::new("Title".to_string(), "This is a very long description that is more than 500 bytes long. This is going to be biggest description ever. This is a very long description that is more than 500 bytes long. This is going to be biggest description ever .This is a very long description that is more than 500 bytes long. This is going to be biggest description ever.This is a very long description that is more than 500 bytes long. This is going to be biggest description ever".to_string(), "To-Do".to_string());
}
#[test]
#[should_panic]
fn test_invalid_status() {
    Ticket::new(
        "Title".to_string(),
        "Description".to_string(),
        "Invalid".to_string(),
    );
}
#[test]
fn test_valid_ticket() {
    let ticket = Ticket::new(
        "Title".to_string(),
        "Description".to_string(),
        "To-Do".to_string(),
    );
    assert_eq!(ticket.title(), "Title");
    assert_eq!(ticket.description(), "Description");
    assert_eq!(ticket.status(), "To-Do");
}
