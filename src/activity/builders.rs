//! TODO
use crate::activity::models::{Activity, Assets, Button, Party, Secrets, Timestamps};

/// A struct for building `Activity` models
#[derive(Default)]
pub struct ActivityBuilder {
    state: Option<String>,
    details: Option<String>,
    timestamps: Option<Timestamps>,
    party: Option<Party>,
    assets: Option<Assets>,
    secrets: Option<Secrets>,
    buttons: Option<Vec<Button>>,
}

/// A struct for building an `Activity`'s timestamps
#[derive(Default)]
pub struct TimestampsBuilder {
    start: Option<i64>,
    end: Option<i64>,
}

/// A struct for building an `Activity`'s game party
#[derive(Default)]
pub struct PartyBuilder {
    id: Option<String>,
    size: Option<[i32; 2]>,
}

/// A struct for building the art assets and hover text used by an `Activity`
#[derive(Default)]
pub struct AssetsBuilder {
    large_image: Option<String>,
    large_text: Option<String>,
    small_image: Option<String>,
    small_text: Option<String>,
}

/// A struct for building the secrets used by an `Activity`
#[derive(Default)]
pub struct SecretsBuilder {
    join_secret: Option<String>,
    spectate_secret: Option<String>,
    match_secret: Option<String>,
}

impl ActivityBuilder {
    /// Sets the state of the activity
    pub fn state(mut self, state: impl ToString) -> Self {
        self.state = Some(state.to_string());
        self
    }

    /// Sets the details of the activity
    pub fn details(mut self, details: impl ToString) -> Self {
        self.details = Some(details.to_string());
        self
    }

    /// Add a `Timestamps` to this activity
    pub fn timestamps(mut self, timestamps: Timestamps) -> Self {
        self.timestamps = Some(timestamps);
        self
    }

    /// Add a `Party` to this activity
    pub fn party(mut self, party: Party) -> Self {
        self.party = Some(party);
        self
    }

    /// Add an `Assets` to this activity
    pub fn assets(mut self, assets: Assets) -> Self {
        self.assets = Some(assets);
        self
    }

    /// Add a `Secrets` to this activity
    pub fn secrets(mut self, secrets: Secrets) -> Self {
        self.secrets = Some(secrets);
        self
    }

    /// Add a `Vec` of `Button`s to this activity
    ///
    /// An activity may contain no more than 2 buttons
    ///
    /// Passing an empty `Vec` will clear the activity's buttons
    pub fn buttons(mut self, buttons: Vec<Button>) -> Self {
        // API call fails if the array is empty, so we skip serialization
        // entirely if this is the case
        if buttons.is_empty() {
            self.buttons = None;
            return self;
        }
        self.buttons = Some(buttons);
        self
    }

    /// Builds the `Activity` model
    pub fn build(self) -> Activity {
        Activity {
            state: self.state,
            details: self.details,
            timestamps: self.timestamps,
            party: self.party,
            assets: self.assets,
            secrets: self.secrets,
            buttons: self.buttons,
        }
    }
}

impl TimestampsBuilder {
    /// Sets the start time
    ///
    /// Returns `Self` for chaining
    pub fn start(mut self, start: i64) -> Self {
        self.start = Some(start);
        self
    }

    /// Sets the end time
    ///
    /// Returns `Self` for chaining
    pub fn end(mut self, end: i64) -> Self {
        self.end = Some(end);
        self
    }

    /// Builds the `Timestamps` model
    pub fn build(self) -> Timestamps {
        Timestamps {
            start: self.start,
            end: self.end,
        }
    }
}

impl PartyBuilder {
    /// Sets the ID of the party
    pub fn id(mut self, id: impl ToString) -> Self {
        self.id = Some(id.to_string());
        self
    }

    /// Sets the size of the party (current and maximum)
    ///
    /// # Example
    /// ```
    /// // Creates a party with a current size
    /// // of 1, and a max size of 3
    /// let party = PartyBuilder::new().size([1, 3]).build();
    /// ```
    pub fn size(mut self, size: [i32; 2]) -> Self {
        self.size = Some(size);
        self
    }

    /// Builds the `Party` model
    pub fn build(self) -> Party {
        Party {
            id: self.id.clone(),
            size: self.size,
        }
    }
}

impl AssetsBuilder {
    /// Sets the name of the art asset to be used as the large
    /// image
    ///
    /// Alternatively, the URL of the resource to be used as
    /// the large image
    pub fn large_image(mut self, large_image: impl ToString) -> Self {
        self.large_image = Some(large_image.to_string());
        self
    }

    /// Sets the text to be shown when hovering over the large
    /// image
    pub fn large_text(mut self, large_text: impl ToString) -> Self {
        self.large_text = Some(large_text.to_string());
        self
    }

    /// Sets the name of the art asset to be used as the small
    /// image
    ///
    /// Alternatively, the URL of the resource to be used as
    /// the small image
    pub fn small_image(mut self, small_image: impl ToString) -> Self {
        self.small_image = Some(small_image.to_string());
        self
    }

    /// Sets the text that is shown when hovering over the small
    /// image
    pub fn small_text(mut self, small_text: impl ToString) -> Self {
        self.small_text = Some(small_text.to_string());
        self
    }

    /// Builds the `Assets` model
    pub fn build(self) -> Assets {
        Assets {
            large_image: self.large_image,
            large_text: self.large_text,
            small_image: self.small_image,
            small_text: self.small_text,
        }
    }
}

impl SecretsBuilder {
    /// Sets the secret for joining a game party
    pub fn join_secret(mut self, join_secret: impl ToString) -> Self {
        self.join_secret = Some(join_secret.to_string());
        self
    }

    /// Sets the secret for spectating a match
    pub fn spectate_secret(mut self, spectate_secret: impl ToString) -> Self {
        self.spectate_secret = Some(spectate_secret.to_string());
        self
    }

    /// Sets the secret for a specific, instanced match
    pub fn match_secret(mut self, match_secret: impl ToString) -> Self {
        self.match_secret = Some(match_secret.to_string());
        self
    }

    /// Build the `Secrets` model
    pub fn build(self) -> Secrets {
        Secrets {
            join_secret: self.join_secret,
            spectate_secret: self.spectate_secret,
            match_secret: self.match_secret,
        }
    }
}
