use std::cmp::Ordering::*;
use std::collections::*;

struct User {
    id: i32,
    following: HashSet<i32>,
    tweets: Vec<Tweet>,
}

impl User {
    fn new(id: i32) -> Self {
        let mut following = HashSet::new();
        following.insert(id);
        Self {
            id,
            following,
            tweets: vec![],
        }
    }
}

struct Tweet {
    id: i32,
    timestamp: usize,
}

struct Twitter {
    users: HashMap<i32, User>,
    timestamp: usize,
}

impl Twitter {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            users: HashMap::new(),
            timestamp: 0,
        }
    }

    /** Compose a new tweet. */
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.users.entry(user_id)
            .or_insert_with(|| User::new(user_id))
            .tweets.push(Tweet {
            id: tweet_id,
            timestamp: self.timestamp,
        });
        self.timestamp += 1;
    }

    /** Retrieve the 10 most recent tweet ids in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user herself. Tweets must be ordered from most recent to least recent. */
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        if let Some(user) = self.users.get(&user_id) {
            let mut tweets = vec![];
            for following_user_id in &user.following {
                if let Some(following_user) = self.users.get(following_user_id) {
                    for t in &following_user.tweets {
                        tweets.push(t);
                    }
                }
            }
            tweets.sort_by_key(|t| -(t.timestamp as isize));
            tweets.iter().take(10).map(|t| t.id).collect()
        } else {
            vec![]
        }
    }

    /** Follower follows a followee. If the operation is invalid, it should be a no-op. */
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.users.entry(followee_id)
            .or_insert_with(|| User::new(followee_id));
        self.users.entry(follower_id)
            .or_insert_with(|| User::new(follower_id))
            .following.insert(followee_id);
    }

    /** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id == followee_id {
            return;
        }
        self.users.entry(followee_id)
            .or_insert_with(|| User::new(followee_id));
        self.users.entry(follower_id)
            .or_insert_with(|| User::new(follower_id))
            .following.remove(&followee_id);
    }
}