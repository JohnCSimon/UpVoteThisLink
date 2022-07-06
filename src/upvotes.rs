pub use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug)]
pub struct VoteEvent {
    pub url_id: String,
    pub user_id: String,
    pub event: Event,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Event {
    UpVote,
    DownVote,
}

// enum UpvoteError {
//     InvalidUrlId,
//     InvalidUserId,
//     DuplicateUpvote,
// }

// enum Emotes {
//     Smile,
//     Frown,
//     Meh,
//     Wink,
//     Tongue,
//     Grin,
//     Sad,
//     Cry,
//     Surprise,
//     Angry,
//     Confused,
//     Cool,
//     Heart,
//     Diamond,
//     Star,
//     Bomb,
//     Skull,
//     Coffee,
//     Ghost,
//     Soccer,
//     Diamond2,
//     Anchor,
//     Soccer2,
//     Bomb2,
//     Skull2,
//     Coffee2,
//     Ghost2,
//     Diamond3,
// }

#[cfg(test)]
mod tests {
    use crate::urlparser::parse_url_local;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn deserialize() -> Result<(), String> {
        let point = VoteEvent {
            url_id: "https://www.rust-lang.org/".to_string(),
            user_id: "12345".to_string(),
            event: Event::DownVote,
        };

        let serialized = serde_json::to_string(&point).unwrap();
        println!("serialized = {}", serialized);

        let deserialized: VoteEvent = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);
        Ok(())
    }

    #[test]
    fn test_parse_url_fail() -> Result<(), String> {
        let badurl = "httpss//www.rust-lang.org/";
        match parse_url_local(badurl) {
            Ok(_) => Err("Should have failed".to_string()),
            Err(x) => {
                println!("Error: {}", &x);
                Ok(())
            }
        }
    }
}
