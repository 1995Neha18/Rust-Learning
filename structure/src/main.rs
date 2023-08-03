struct User {
 active: bool,
 username: String,
 email: String,
 sign_in_count: u64,
}

fn main() {
 // Creating instances of the User struct
 let user1 = User {
     active: true,
     username: String::from("user123"),
     email: String::from("user123@example.com"),
     sign_in_count: 3,
 };

 let user2 = User {
     active: false,
     username: String::from("anotheruser"),
     email: String::from("anotheruser@example.com"),
     sign_in_count: 1,
 };

 println!("User 1's username: {}", user1.username);
 println!("User 2's email: {}", user2.email);

 let mut user1 = user1; // Convert user1 to mutable
 user1.sign_in_count += 1;


 display_user_info(&user1);
 display_user_info(&user2);
}

// Function to display user information
fn display_user_info(user: &User) {
 println!("Username: {}", user.username);
 println!("Email: {}", user.email);
 println!("Active: {}", user.active);
 println!("Sign-in Count: {}", user.sign_in_count);
 println!();
}
