//! Provides an interface for building activities to send
//! to Discord via [`DiscordIpc::set_activity`](crate::DiscordIpc::set_activity).
use serde_derive::Serialize;

/// A struct representing a Discord rich presence activity
///
/// Note that all methods return `Self`, and can be chained
/// for fluency
#[derive(Serialize, Clone)]
pub struct Activity {
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamps: Option<Timestamps>,

    #[serde(skip_serializing_if = "Option::is_none")]
    party: Option<Party>,

    #[serde(skip_serializing_if = "Option::is_none")]
    assets: Option<Assets>,

    #[serde(skip_serializing_if = "Option::is_none")]
    secrets: Option<Secrets>,

    #[serde(skip_serializing_if = "Option::is_none")]
    buttons: Option<Vec<Button>>,
}

/// A struct representing an `Activity`'s timestamps
///
/// Note that all methods return `Self`, and can be chained
/// for fluency
#[derive(Serialize, Clone)]
pub struct Timestamps {
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<i64>,
}

/// A struct representing an `Activity`'s game party
///
/// Note that all methods return `Self`, and can be chained
/// for fluency
#[derive(Serialize, Clone)]
pub struct Party {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<[i32; 2]>,
}

/// A struct representing the art assets and hover text
/// used by an `Activity`
///
/// Note that all methods return `Self`, and can be chained
/// for fluency
#[derive(Serialize, Clone)]
pub struct Assets {
    #[serde(skip_serializing_if = "Option::is_none")]
    large_image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    large_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    small_image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    small_text: Option<String>,
}

/// A struct representing the secrets used by an
/// `Activity`
///
/// Note that all methods return `Self`, and can be chained
/// for fluency
#[derive(Serialize, Clone)]
pub struct Secrets {
    #[serde(skip_serializing_if = "Option::is_none")]
    join: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    spectate: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    r#match: Option<String>,
}

/// A struct representing the buttons that are
/// attached to an `Activity`
///
/// An activity may have a maximum of 2 buttons
#[derive(Serialize, Clone)]
pub struct Button {
    label: String,
    url: String,
}

impl Activity {
    /// Creates a new `Activity`
    pub fn new() -> Self {
        Activity {
            state: None,
            details: None,
            assets: None,
            buttons: None,
            party: None,
            secrets: None,
            timestamps: None,
        }
    }

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
    pub fn buttons(mut self, buttons: Vec<Button>) -> Self {
        // API call fails if the array is empty, so we skip serialization
        // entirely if this is the case
        if buttons.is_empty() {
            return self;
        }
        self.buttons = Some(buttons);
        self
    }
}

impl Default for Activity {
    fn default() -> Self {
        Self::new()
    }
}

impl Timestamps {
    /// Creates a new `Timestamps`
    pub fn new() -> Self {
        Timestamps {
            start: None,
            end: None,
        }
    }

    /// Sets the start time
    pub fn start(mut self, start: i64) -> Self {
        self.start = Some(start);
        self
    }

    /// Sets the end time
    pub fn end(mut self, end: i64) -> Self {
        self.end = Some(end);
        self
    }
}

impl Default for Timestamps {
    fn default() -> Self {
        Self::new()
    }
}

impl Party {
    /// Creates a new `Party`
    pub fn new() -> Self {
        Party {
            id: None,
            size: None,
        }
    }

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
    /// let party = Party::new().size([1, 3])
    /// ```
    pub fn size(mut self, size: [i32; 2]) -> Self {
        self.size = Some(size);
        self
    }
}

impl Default for Party {
    fn default() -> Self {
        Self::new()
    }
}

impl Assets {
    /// Creates a new `Assets`
    pub fn new() -> Self {
        Assets {
            large_image: None,
            large_text: None,
            small_image: None,
            small_text: None,
        }
    }

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
}

impl Default for Assets {
    fn default() -> Self {
        Self::new()
    }
}

impl Secrets {
    /// Creates a new `Secrets`
    pub fn new() -> Self {
        Secrets {
            join: None,
            spectate: None,
            r#match: None,
        }
    }

    /// Sets the secret for joining a game party
    pub fn join(mut self, join: impl ToString) -> Self {
        self.join = Some(join.to_string());
        self
    }

    /// Sets the secret for spectating a match
    pub fn spectate(mut self, spectate: impl ToString) -> Self {
        self.spectate = Some(spectate.to_string());
        self
    }

    /// Sets the secret for a specific, instanced match
    pub fn r#match(mut self, r#match: impl ToString) -> Self {
        self.r#match = Some(r#match.to_string());
        self
    }
}

impl Default for Secrets {
    fn default() -> Self {
        Self::new()
    }
}

impl Button {
    /// Creates a new `Button` with the given label and
    /// URL
    ///
    /// The label must be 1-32 characters long
    ///
    /// The URL must be 1-512 characters long
    pub fn new(label: impl ToString, url: impl ToString) -> Self {
        Button {
            label: label.to_string(),
            url: url.to_string(),
        }
    }
}
