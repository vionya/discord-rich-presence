//! Provides an interface for building activities to send
//! to Discord via [`DiscordIpc::set_activity`](crate::DiscordIpc::set_activity).
use serde_derive::Serialize;

/// A struct representing a Discord rich presence activity
///
/// Note that all methods return `Self`, and can be chained
/// for fluency
#[derive(Serialize, Clone)]
pub struct Activity<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamps: Option<Timestamps>,

    #[serde(skip_serializing_if = "Option::is_none")]
    party: Option<Party<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    assets: Option<Assets<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    secrets: Option<Secrets<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    buttons: Option<Vec<Button<'a>>>,
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
pub struct Party<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<[i32; 2]>,
}

/// A struct representing the art assets and hover text
/// used by an `Activity`
///
/// Note that all methods return `Self`, and can be chained
/// for fluency
#[derive(Serialize, Clone)]
pub struct Assets<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    large_image: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    large_text: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    small_image: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    small_text: Option<&'a str>,
}

/// A struct representing the secrets used by an
/// `Activity`
///
/// Note that all methods return `Self`, and can be chained
/// for fluency
#[derive(Serialize, Clone)]
pub struct Secrets<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    join: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    spectate: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    r#match: Option<&'a str>,
}

/// A struct representing the buttons that are
/// attached to an `Activity`
///
/// An activity may have a maximum of 2 buttons
#[derive(Serialize, Clone)]
pub struct Button<'a> {
    label: &'a str,
    url: &'a str,
}

impl<'a> Activity<'a> {
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
    pub fn state(mut self, state: &'a str) -> Self {
        self.state = Some(state);
        self
    }

    /// Sets the details of the activity
    pub fn details(mut self, details: &'a str) -> Self {
        self.details = Some(details);
        self
    }

    /// Add a `Timestamps` to this activity
    pub fn timestamps(mut self, timestamps: Timestamps) -> Self {
        self.timestamps = Some(timestamps);
        self
    }

    /// Add a `Party` to this activity
    pub fn party(mut self, party: Party<'a>) -> Self {
        self.party = Some(party);
        self
    }

    /// Add an `Assets` to this activity
    pub fn assets(mut self, assets: Assets<'a>) -> Self {
        self.assets = Some(assets);
        self
    }

    /// Add a `Secrets` to this activity
    pub fn secrets(mut self, secrets: Secrets<'a>) -> Self {
        self.secrets = Some(secrets);
        self
    }

    /// Add a `Vec` of `Button`s to this activity
    ///
    /// An activity may contain no more than 2 buttons
    pub fn buttons(mut self, buttons: Vec<Button<'a>>) -> Self {
        // API call fails if the array is empty, so we skip serialization
        // entirely if this is the case
        if buttons.is_empty() {
            return self;
        }

        self.buttons = Some(buttons);
        self
    }
}

impl<'a> Default for Activity<'a> {
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

impl<'a> Party<'a> {
    /// Creates a new `Party`
    pub fn new() -> Self {
        Party {
            id: None,
            size: None,
        }
    }

    /// Sets the ID of the party
    pub fn id(mut self, id: &'a str) -> Self {
        self.id = Some(id);
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

impl<'a> Default for Party<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Assets<'a> {
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
    pub fn large_image(mut self, large_image: &'a str) -> Self {
        self.large_image = Some(large_image);
        self
    }

    /// Sets the text to be shown when hovering over the large
    /// image
    pub fn large_text(mut self, large_text: &'a str) -> Self {
        self.large_text = Some(large_text);
        self
    }

    /// Sets the name of the art asset to be used as the small
    /// image
    ///
    /// Alternatively, the URL of the resource to be used as
    /// the small image
    pub fn small_image(mut self, small_image: &'a str) -> Self {
        self.small_image = Some(small_image);
        self
    }

    /// Sets the text that is shown when hovering over the small
    /// image
    pub fn small_text(mut self, small_text: &'a str) -> Self {
        self.small_text = Some(small_text);
        self
    }
}

impl<'a> Default for Assets<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Secrets<'a> {
    /// Creates a new `Secrets`
    pub fn new() -> Self {
        Secrets {
            join: None,
            spectate: None,
            r#match: None,
        }
    }

    /// Sets the secret for joining a game party
    pub fn join(mut self, join: &'a str) -> Self {
        self.join = Some(join);
        self
    }

    /// Sets the secret for spectating a match
    pub fn spectate(mut self, spectate: &'a str) -> Self {
        self.spectate = Some(spectate);
        self
    }

    /// Sets the secret for a specific, instanced match
    pub fn r#match(mut self, r#match: &'a str) -> Self {
        self.r#match = Some(r#match);
        self
    }
}

impl<'a> Default for Secrets<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Button<'a> {
    /// Creates a new `Button` with the given label and
    /// URL
    ///
    /// The label must be 1-32 characters long
    ///
    /// The URL must be 1-512 characters long
    pub fn new(label: &'a str, url: &'a str) -> Self {
        Button { label, url }
    }
}
