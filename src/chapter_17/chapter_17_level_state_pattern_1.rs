// As suggested by the bookd, add the following capabilites to your answer from state_pattern_0:
//      - reject method: changes the post state from PendingReview back to Draft
//      - in order for the state to be changed to Published the function approve need to be called twice
//      - allow users to add text content only when a post is in the Draft state
//
// Uncomment the main function and add the code so that the main function can compile



// fn main() {
//     let mut post =  Post::new();

//     post.add_text("I love Rust");
//     asset_eq!("", post.content());

//     post.request_review();
//     asset_eq!("", post.content());

//     post.approve();
//     asset_eq!("", post.content());
//     post.approve();
//     asset_eq!("I love Rust", post.content());
// }

