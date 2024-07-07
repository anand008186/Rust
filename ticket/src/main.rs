use ticket::ticket_module::Ticket;
 mod tests;
fn main() {
    
    let ticket: Ticket = Ticket::new("Title".to_string(), "Description".to_string(), "To-Do".to_string());
    println!("{:?}", ticket);
    println!("Title: {}", ticket.title());
    println!("Description: {}", ticket.description());
    println!("Status: {}", ticket.status());
    
}
