//! Provides Discord models as serializable structs.
use serde_derive::Serialize;

/// A struct representing a Discord rich presence activity
#[derive(Serialize, Clone)]
pub struct Activity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timestamps: Option<Timestamps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) party: Option<Party>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) assets: Option<Assets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) secrets: Option<Secrets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) buttons: Option<Vec<Button>>,
}

/// A struct representing an `Activity`'s timestamps
#[derive(Serialize, Clone)]
pub struct Timestamps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) start: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) end: Option<i64>,
}

/// A struct representing an `Activity`'s game party
#[derive(Serialize, Clone)]
pub struct Party {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) size: Option<[i32; 2]>,
}

/// A struct representing the art assets and hover text
/// used by an `Activity`
#[derive(Serialize, Clone)]
pub struct Assets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) large_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) large_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) small_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) small_text: Option<String>,
}

/// A struct representing the secrets used by an
/// `Activity`
#[derive(Serialize, Clone)]
pub struct Secrets {
    #[serde(skip_serializing_if = "Option::is_none", rename = "join")]
    pub(crate) join_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "spectate")]
    pub(crate) spectate_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "match")]
    pub(crate) match_secret: Option<String>,
}

/// A struct representing the buttons that are
/// attached to an [`Activity`]
///
/// An activity may have a maximum of 2 buttons
#[derive(Serialize, Clone)]
pub struct Button {
    pub(crate) label: String,
    pub(crate) url: String,
}

impl Activity {
    /// Changes the state of a mutable activity
    pub fn set_state(&mut self, state: impl ToString) {
        self.state = Some(state.to_string());
    }

    /// Changes the details of a mutable activity
    pub fn set_details(&mut self, details: impl ToString) {
        self.details = Some(details.to_string());
    }

    /// Changes the timestamps of a mutable activity
    pub fn set_timestamps(&mut self, timestamps: Timestamps) {
        self.timestamps = Some(timestamps);
    }

    /// Changes the party of a mutable activity
    pub fn set_party(&mut self, party: Party) {
        self.party = Some(party);
    }

    /// Changes the assets of a mutable activity
    pub fn set_assets(&mut self, assets: Assets) {
        self.assets = Some(assets);
    }

    /// Changes the secrets of a mutable activity
    pub fn set_secrets(&mut self, secrets: Secrets) {
        self.secrets = Some(secrets);
    }

    /// Changes the buttons of a mutable activity
    ///
    /// An activity may contain no more than 2 buttons
    ///
    /// Passing an empty `Vec` will clear the activity's buttons
    pub fn set_buttons(&mut self, buttons: Vec<Button>) {
        // API call fails if the array is empty, so we skip serialization
        // entirely if this is the case
        if buttons.is_empty() {
            self.buttons = None;
            return;
        }
        self.buttons = Some(buttons);
    }
}

impl Timestamps {
    /// Changes the start time
    pub fn set_start(&mut self, start: i64) {
        self.start = Some(start);
    }

    /// Changes the end time
    pub fn set_end(&mut self, end: i64) {
        self.end = Some(end);
    }

    /// Shorthand for creating a new `Timestamps`
    ///
    /// All parameters are `Option`-al and will be ignored if `None`
    /// is provided.
    pub fn new(start: Option<i64>, end: Option<i64>) -> Self {
        Self { start, end }
    }
}

impl Party {
    /// Sets the ID of the party
    pub fn set_id(&mut self, id: impl ToString) {
        self.id = Some(id.to_string());
    }

    /// Changes the size of the party (current and maximum)
    pub fn set_size(&mut self, size: [i32; 2]) {
        self.size = Some(size);
    }
}

impl Assets {
    /// Changes the name of the art asset to be used as the large
    /// image
    ///
    /// Alternatively, the URL of the resource to be used as
    /// the large image
    pub fn set_large_image(&mut self, large_image: impl ToString) {
        self.large_image = Some(large_image.to_string());
    }

    /// Changes the text to be shown when hovering over the large
    /// image
    pub fn set_large_text(&mut self, large_text: impl ToString) {
        self.large_text = Some(large_text.to_string());
    }

    /// Changes the name of the art asset to be used as the small
    /// image
    ///
    /// Alternatively, the URL of the resource to be used as
    /// the small image
    pub fn set_small_image(&mut self, small_image: impl ToString) {
        self.small_image = Some(small_image.to_string());
    }

    /// Changes the text that is shown when hovering over the small
    /// image
    pub fn set_small_text(&mut self, small_text: impl ToString) {
        self.small_text = Some(small_text.to_string());
    }
}

impl Secrets {
    /// Changes the secret for joining a game party
    pub fn set_join_secret(&mut self, join_secret: impl ToString) {
        self.join_secret = Some(join_secret.to_string());
    }

    /// Changes the secret for spectating a match
    pub fn set_spectate_secret(&mut self, spectate_secret: impl ToString) {
        self.spectate_secret = Some(spectate_secret.to_string());
    }

    /// Changes the secret for a specific, instanced match
    pub fn set_match_secret(&mut self, match_secret: impl ToString) {
        self.match_secret = Some(match_secret.to_string());
    }
}

impl Button {
    /// Creates a new `Button` with the given label and URL
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
