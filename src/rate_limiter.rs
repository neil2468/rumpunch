use dashmap::DashMap;
use std::time::{Duration, Instant};
use tracing::{enabled, Level};

struct TokenBucket {
    max_tokens: usize,
    reset_duration: Duration,
    last_reset: Instant,
    token_count: usize,
    expire_duration: Duration,
}

impl TokenBucket {
    fn new(max_tokens: usize, reset_duration: Duration, expire_duration: Duration) -> Self {
        Self {
            max_tokens,
            reset_duration,
            last_reset: Instant::now(),
            token_count: max_tokens,
            expire_duration,
        }
    }

    fn is_expired(&self) -> bool {
        self.last_reset.elapsed() >= self.expire_duration
    }

    /// If a token is available, uses it and returns `true`.
    /// Otherwise, returns `false`.
    fn try_use_token(&mut self) -> bool {
        // If no tokens are available, try a reset first
        if self.token_count == 0 {
            self.try_reset();
        }

        // Try to use a token
        match self.token_count {
            0 => false,
            _ => {
                self.token_count -= 1;
                tracing::debug!("Token used. {} remain.", self.token_count);
                true
            }
        }
    }

    /// If it is time for a reset, reset the bucket
    fn try_reset(&mut self) {
        let elapsed = self.last_reset.elapsed();
        if elapsed >= self.reset_duration {
            tracing::debug!(?elapsed, "Resetting bucket");
            self.token_count = self.max_tokens;
            self.last_reset = Instant::now();
        }
    }
}

pub struct RateLimiter {
    limit_count: usize,
    limit_duration: Duration,
    expire_duration: Duration,
    data: DashMap<u32, TokenBucket>,
}

const EXPIRE_MULTIPLIER: u32 = 3;

impl RateLimiter {
    pub fn new(rate_limit_count: usize, rate_limit_duration: Duration) -> Self {
        Self {
            limit_count: rate_limit_count,
            limit_duration: rate_limit_duration,
            expire_duration: rate_limit_duration * EXPIRE_MULTIPLIER,
            data: DashMap::new(),
        }
    }

    pub fn can_service(&self, id: u32) -> bool {
        // Get token bucket
        // If logging enabled, log when we are waiting on a lock
        let val = if enabled!(Level::TRACE) {
            match self.data.try_entry(id) {
                Some(entry) => entry,
                None => {
                    tracing::trace!("Waiting on lock for id {}", id);
                    self.data.entry(id)
                }
            }
        } else {
            self.data.entry(id)
        };

        // If token bucket does not exist, create it
        let mut val = val.or_insert_with(|| {
            TokenBucket::new(self.limit_count, self.limit_duration, self.expire_duration)
        });

        // Try to use a token
        val.try_use_token()
    }

    pub fn maintain(&self) {
        // Delete expired token buckets and shrink
        tracing::trace!("Pre-maintain token bucket count: {}", self.data.len());
        self.data.retain(|_, v| v.is_expired() == false);
        self.data.shrink_to_fit();
        tracing::trace!("Post-maintain token bucket count: {}", self.data.len());
    }
}
