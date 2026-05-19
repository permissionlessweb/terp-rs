use crate::{
    error::{NipError, NipResult},
    types::*,
    NipKind, NipMetadata, RawNostrEvent, Tag,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::Digest as _;

// ==================== NIP-52 Kinds ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Nip52Kind {
    DateEvent,
    TimeEvent,
    Calendar,
    RSVP,
}

impl NipKind for Nip52Kind {
    fn kind_value(&self) -> u16 {
        match self {
            Nip52Kind::DateEvent => 31922,
            Nip52Kind::TimeEvent => 31923,
            Nip52Kind::Calendar => 31924,
            Nip52Kind::RSVP => 31925,
        }
    }
}

// ==================== NIP-52 Common Types ====================

/// Reference to another calendar or event
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CalendarReference {
    pub kind: u32,
    pub author_pubkey: String,
    pub d_identifier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay_url: Option<String>,
}

impl CalendarReference {
    pub fn new(
        kind: u32,
        author_pubkey: impl Into<String>,
        d_identifier: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            author_pubkey: author_pubkey.into(),
            d_identifier: d_identifier.into(),
            relay_url: None,
        }
    }

    pub fn with_relay(mut self, relay: impl Into<String>) -> Self {
        self.relay_url = Some(relay.into());
        self
    }

    /// Convert to an "a" tag
    pub fn to_a_tag(&self) -> Tag {
        let a_value = format!("{}:{}:{}", self.kind, self.author_pubkey, self.d_identifier);
        if let Some(ref relay) = self.relay_url {
            Tag::a_with_relay(
                self.kind as u16,
                &self.author_pubkey,
                &self.d_identifier,
                relay,
            )
        } else {
            Tag::a(self.kind as u16, &self.author_pubkey, &self.d_identifier)
        }
    }

    /// Parse from an "a" tag
    pub fn from_a_tag(tag: &Tag) -> NipResult<Self> {
        let value = tag
            .value()
            .ok_or_else(|| NipError::Tag("Missing tag value".into()))?;
        let parts: Vec<&str> = value.split(':').collect();
        if parts.len() < 3 {
            return Err(NipError::Nip52("Invalid a-tag format".into()));
        }
        let kind: u32 = parts[0]
            .parse()
            .map_err(|_| NipError::Nip52("Invalid kind in a-tag".into()))?;
        let relay_url = tag.additional().first().cloned();
        Ok(Self {
            kind,
            author_pubkey: parts[1].to_string(),
            d_identifier: parts[2].to_string(),
            relay_url,
        })
    }
}

/// RSVP status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RSVPStatus {
    Accepted,
    Declined,
    Tentative,
}

impl RSVPStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            RSVPStatus::Accepted => "accepted",
            RSVPStatus::Declined => "declined",
            RSVPStatus::Tentative => "tentative",
        }
    }
}

/// Free/busy status
#[derive(Debug, Clone, Copy, PartialEq, JsonSchema, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FreeBusyStatus {
    Free,
    Busy,
}

impl FreeBusyStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            FreeBusyStatus::Free => "free",
            FreeBusyStatus::Busy => "busy",
        }
    }
}

/// Event type discriminator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema, Deserialize)]
pub enum EventType {
    #[serde(rename = "time")]
    TimeBased,
    #[serde(rename = "date")]
    DateBased,
}

impl EventType {
    pub fn kind(&self) -> Nip52Kind {
        match self {
            EventType::TimeBased => Nip52Kind::TimeEvent,
            EventType::DateBased => Nip52Kind::DateEvent,
        }
    }
}

// ==================== NIP-52 Calendar Event (kind:31922/31923) ====================

/// NIP-52 Calendar Event metadata
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CalendarEventMetadata {
    /// d-tag: short unique identifier
    pub d_tag: String,
    /// Event title
    pub title: String,
    /// Brief description (summary)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Image URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Full description (content field)
    pub content: String,
    /// Location strings
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    /// Geohash for physical location (g-tag)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geohash: Option<String>,
    /// References/links (r-tags)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub references: Vec<String>,
    /// Hashtags for categorization (t-tags)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hashtags: Vec<String>,
    /// Start timezone (IANA identifier)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timezone: Option<String>,
    /// End timezone (IANA identifier)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timezone: Option<String>,
    /// Day-granularity timestamps (kind:31923 only)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub day_granularity: Vec<u64>,
    /// Collaborative calendar references (a-tags)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub calendar_requests: Vec<CalendarReference>,
    /// Whether this is a time-based or date-based event
    pub event_type: EventType,
    /// Start timestamp (unix seconds)
    pub start_time: u64,
    /// End timestamp (unix seconds)
    pub end_time: u64,
    /// Participants (p-tags)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub participants: Vec<Participant>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Participant {
    pub pubkey: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

impl CalendarEventMetadata {
    pub fn new_time_based(
        d_tag: impl Into<String>,
        title: impl Into<String>,
        content: impl Into<String>,
        start_time: u64,
        end_time: u64,
    ) -> Self {
        Self {
            d_tag: d_tag.into(),
            title: title.into(),
            summary: None,
            image: None,
            content: content.into(),
            locations: Vec::new(),
            geohash: None,
            references: Vec::new(),
            hashtags: Vec::new(),
            start_timezone: None,
            end_timezone: None,
            day_granularity: Vec::new(),
            calendar_requests: Vec::new(),
            event_type: EventType::TimeBased,
            start_time,
            end_time,
            participants: Vec::new(),
        }
    }

    pub fn new_date_based(
        d_tag: impl Into<String>,
        title: impl Into<String>,
        content: impl Into<String>,
        start_time: u64,
        end_time: u64,
    ) -> Self {
        Self {
            d_tag: d_tag.into(),
            title: title.into(),
            summary: None,
            image: None,
            content: content.into(),
            locations: Vec::new(),
            geohash: None,
            references: Vec::new(),
            hashtags: Vec::new(),
            start_timezone: None,
            end_timezone: None,
            day_granularity: Vec::new(),
            calendar_requests: Vec::new(),
            event_type: EventType::DateBased,
            start_time,
            end_time,
            participants: Vec::new(),
        }
    }

    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    pub fn with_image(mut self, image: impl Into<String>) -> Self {
        self.image = Some(image.into());
        self
    }

    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.locations.push(location.into());
        self
    }

    pub fn with_geohash(mut self, geohash: impl Into<String>) -> Self {
        self.geohash = Some(geohash.into());
        self
    }

    pub fn with_reference(mut self, reference: impl Into<String>) -> Self {
        self.references.push(reference.into());
        self
    }

    pub fn with_hashtag(mut self, hashtag: impl Into<String>) -> Self {
        self.hashtags.push(hashtag.into());
        self
    }

    pub fn with_calendar_request(mut self, reference: CalendarReference) -> Self {
        self.calendar_requests.push(reference);
        self
    }

    pub fn with_participant(mut self, pubkey: impl Into<String>, role: Option<String>) -> Self {
        self.participants.push(Participant {
            pubkey: pubkey.into(),
            relay_url: None,
            role,
        });
        self
    }
}

impl NipMetadata for CalendarEventMetadata {
    type Kind = Nip52Kind;

    fn kind(&self) -> Self::Kind {
        self.event_type.kind()
    }

    fn validate(&self) -> NipResult<()> {
        if self.d_tag.is_empty() {
            return Err(NipError::Nip52("d-tag cannot be empty".into()));
        }
        if self.title.is_empty() {
            return Err(NipError::Nip52("Title cannot be empty".into()));
        }
        if self.start_time > self.end_time {
            return Err(NipError::Nip52("Start time must be before end time".into()));
        }
        if self.event_type == EventType::TimeBased && self.day_granularity.is_empty() {
            // Time-based events should have day granularity timestamps
            // This is a warning, not an error
        }
        Ok(())
    }

    fn to_tags(&self) -> Vec<Tag> {
        let mut tags = vec![Tag::d(&self.d_tag)];

        // Title tag
        tags.push(Tag::new(vec![String::from("title"), self.title.clone()]));

        // Summary tag (if present)
        if let Some(ref summary) = self.summary {
            tags.push(Tag::new(vec![String::from("summary"), summary.clone()]));
        }

        // Image tag (if present)
        if let Some(ref image) = self.image {
            tags.push(Tag::new(vec![String::from("image"), image.clone()]));
        }

        // Start/end time tags
        tags.push(Tag::new(vec![
            String::from("start"),
            self.start_time.to_string(),
        ]));
        if let Some(ref tz) = self.start_timezone {
            tags.push(Tag::new(vec![String::from("start_tzid"), tz.clone()]));
        }

        tags.push(Tag::new(vec![
            String::from("end"),
            self.end_time.to_string(),
        ]));
        if let Some(ref tz) = self.end_timezone {
            tags.push(Tag::new(vec![String::from("end_tzid"), tz.clone()]));
        }

        // Location tags
        for location in &self.locations {
            tags.push(Tag::new(vec![String::from("location"), location.clone()]));
        }

        // Geohash tag
        if let Some(ref geohash) = self.geohash {
            tags.push(Tag::g(geohash));
        }

        // Reference tags
        for reference in &self.references {
            tags.push(Tag::r(reference));
        }

        // Hashtag tags
        for hashtag in &self.hashtags {
            tags.push(Tag::t(hashtag));
        }

        // Day granularity tags (for time-based events)
        for day in &self.day_granularity {
            tags.push(Tag::new(vec![String::from("d"), day.to_string()]));
        }

        // Calendar request tags (a-tags)
        for request in &self.calendar_requests {
            tags.push(request.to_a_tag());
        }

        // Participant tags (p-tags)
        for participant in &self.participants {
            let mut p_tag = Tag::p(&participant.pubkey);
            if let Some(ref relay) = participant.relay_url {
                p_tag = Tag::p_with_relay(&participant.pubkey, relay);
            }
            tags.push(p_tag);
            if let Some(ref role) = participant.role {
                tags.push(Tag::new(vec![
                    String::from("role"),
                    participant.pubkey.clone(),
                    role.clone(),
                ]));
            }
        }

        tags
    }

    fn content(&self) -> String {
        self.content.clone()
    }

    fn d_tag(&self) -> Option<String> {
        Some(self.d_tag.clone())
    }

    fn from_raw_event(event: &RawNostrEvent) -> NipResult<Self> {
        let event_type = match event.kind {
            31922 => EventType::DateBased,
            31923 => EventType::TimeBased,
            _ => {
                return Err(NipError::Nip52(format!(
                    "Expected kind 31922 or 31923, got {}",
                    event.kind
                )))
            }
        };

        let d_tag = event
            .d_tag()
            .ok_or_else(|| NipError::Nip52("Calendar event must have a d-tag".into()))?
            .to_string();

        let mut title = String::new();
        let mut summary = None;
        let mut image = None;
        let mut start_time = 0u64;
        let mut end_time = 0u64;
        let mut start_timezone = None;
        let mut end_timezone = None;
        let mut locations = Vec::new();
        let mut geohash = None;
        let mut references = Vec::new();
        let mut hashtags = Vec::new();
        let mut day_granularity = Vec::new();
        let mut calendar_requests = Vec::new();
        let mut participants = Vec::new();

        for tag in &event.tags {
            if tag.is_empty() {
                continue;
            }
            match tag[0].as_str() {
                "title" => {
                    if let Some(v) = tag.get(1) {
                        title = v.clone();
                    }
                }
                "summary" => {
                    if let Some(v) = tag.get(1) {
                        summary = Some(v.clone());
                    }
                }
                "image" => {
                    if let Some(v) = tag.get(1) {
                        image = Some(v.clone());
                    }
                }
                "start" => {
                    if let Some(v) = tag.get(1) {
                        start_time = v
                            .parse()
                            .map_err(|_| NipError::Nip52("Invalid start time".into()))?;
                    }
                }
                "end" => {
                    if let Some(v) = tag.get(1) {
                        end_time = v
                            .parse()
                            .map_err(|_| NipError::Nip52("Invalid end time".into()))?;
                    }
                }
                "start_tzid" => {
                    if let Some(v) = tag.get(1) {
                        start_timezone = Some(v.clone());
                    }
                }
                "end_tzid" => {
                    if let Some(v) = tag.get(1) {
                        end_timezone = Some(v.clone());
                    }
                }
                "location" => {
                    if let Some(v) = tag.get(1) {
                        locations.push(v.clone());
                    }
                }
                "g" => {
                    if let Some(v) = tag.get(1) {
                        geohash = Some(v.clone());
                    }
                }
                "r" => {
                    if let Some(v) = tag.get(1) {
                        references.push(v.clone());
                    }
                }
                "t" => {
                    if let Some(v) = tag.get(1) {
                        hashtags.push(v.clone());
                    }
                }
                "a" => {
                    let tag_obj = Tag::new(tag.clone());
                    if let Ok(reference) = CalendarReference::from_a_tag(&tag_obj) {
                        calendar_requests.push(reference);
                    }
                }
                "p" => {
                    if let Some(pubkey) = tag.get(1) {
                        let relay_url = tag.get(2).cloned();
                        participants.push(Participant {
                            pubkey: pubkey.clone(),
                            relay_url,
                            role: None,
                        });
                    }
                }
                "role" => {
                    // role tags reference a participant pubkey with a role
                    if tag.len() >= 3 {
                        if let Some(pos) = participants.iter().position(|p| p.pubkey == tag[1]) {
                            participants[pos].role = Some(tag[2].clone());
                        }
                    }
                }
                _ => {}
            }
        }

        if title.is_empty() {
            return Err(NipError::Nip52("Calendar event must have a title".into()));
        }

        Ok(CalendarEventMetadata {
            d_tag,
            title,
            summary,
            image,
            content: event.content.clone(),
            locations,
            geohash,
            references,
            hashtags,
            start_timezone,
            end_timezone,
            day_granularity,
            calendar_requests,
            event_type,
            start_time,
            end_time,
            participants,
        })
    }
}

// ==================== NIP-52 Calendar (kind:31924) ====================

/// NIP-52 Calendar metadata
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CalendarMetadata {
    /// d-tag: UUID for this calendar
    pub d_tag: String,
    /// Calendar title
    pub title: String,
    /// Calendar description (content field)
    pub description: String,
    /// Calendar owner pubkey
    pub owner_pubkey: String,
    /// Included event references
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event_references: Vec<CalendarReference>,
    /// Calendar hashtags
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hashtags: Vec<String>,
}

impl CalendarMetadata {
    pub fn new(
        d_tag: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        owner_pubkey: impl Into<String>,
    ) -> Self {
        Self {
            d_tag: d_tag.into(),
            title: title.into(),
            description: description.into(),
            owner_pubkey: owner_pubkey.into(),
            event_references: Vec::new(),
            hashtags: Vec::new(),
        }
    }

    pub fn with_event_reference(mut self, reference: CalendarReference) -> Self {
        self.event_references.push(reference);
        self
    }

    pub fn with_hashtag(mut self, hashtag: impl Into<String>) -> Self {
        self.hashtags.push(hashtag.into());
        self
    }
}

impl NipMetadata for CalendarMetadata {
    type Kind = Nip52Kind;

    fn kind(&self) -> Self::Kind {
        Nip52Kind::Calendar
    }

    fn validate(&self) -> NipResult<()> {
        if self.d_tag.is_empty() {
            return Err(NipError::Nip52("Calendar d-tag cannot be empty".into()));
        }
        if self.title.is_empty() {
            return Err(NipError::Nip52("Calendar title cannot be empty".into()));
        }
        if self.owner_pubkey.is_empty() {
            return Err(NipError::Nip52(
                "Calendar owner pubkey cannot be empty".into(),
            ));
        }
        // Validate pubkey format
        PubKey::new(&self.owner_pubkey)?;
        Ok(())
    }

    fn to_tags(&self) -> Vec<Tag> {
        let mut tags = vec![Tag::d(&self.d_tag)];

        // Title tag
        tags.push(Tag::new(vec![String::from("title"), self.title.clone()]));

        // Description tag (or use content field)
        tags.push(Tag::new(vec![
            String::from("description"),
            self.description.clone(),
        ]));

        // Event references (a-tags)
        for reference in &self.event_references {
            tags.push(reference.to_a_tag());
        }

        // Hashtag tags
        for hashtag in &self.hashtags {
            tags.push(Tag::t(hashtag));
        }

        tags
    }

    fn content(&self) -> String {
        self.description.clone()
    }

    fn d_tag(&self) -> Option<String> {
        Some(self.d_tag.clone())
    }

    fn from_raw_event(event: &RawNostrEvent) -> NipResult<Self> {
        if event.kind != Nip52Kind::Calendar.kind_value() {
            return Err(NipError::Nip52(format!(
                "Expected kind 31924, got {}",
                event.kind
            )));
        }

        let d_tag = event
            .d_tag()
            .ok_or_else(|| NipError::Nip52("Calendar must have a d-tag".into()))?
            .to_string();

        let mut title = String::new();
        let mut description = event.content.clone();
        let mut event_references = Vec::new();
        let mut hashtags = Vec::new();
        let mut owner_pubkey = event.pubkey.clone();

        for tag in &event.tags {
            if tag.is_empty() {
                continue;
            }
            match tag[0].as_str() {
                "title" => {
                    if let Some(v) = tag.get(1) {
                        title = v.clone();
                    }
                }
                "description" => {
                    if let Some(v) = tag.get(1) {
                        description = v.clone();
                    }
                }
                "a" => {
                    let tag_obj = Tag::new(tag.clone());
                    if let Ok(reference) = CalendarReference::from_a_tag(&tag_obj) {
                        event_references.push(reference);
                    }
                }
                "t" => {
                    if let Some(v) = tag.get(1) {
                        hashtags.push(v.clone());
                    }
                }
                _ => {}
            }
        }

        if title.is_empty() {
            return Err(NipError::Nip52("Calendar must have a title".into()));
        }

        Ok(CalendarMetadata {
            d_tag,
            title,
            description,
            owner_pubkey,
            event_references,
            hashtags,
        })
    }
}

// ==================== NIP-52 RSVP (kind:31925) ====================

/// NIP-52 RSVP metadata
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RSVPMetadata {
    /// d-tag: unique identifier for this RSVP
    pub d_tag: String,
    /// Reference to the calendar event
    pub event_reference: CalendarReference,
    /// Optional specific event revision id (e-tag)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// RSVP status
    pub status: RSVPStatus,
    /// Free/busy indicator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_busy: Option<FreeBusyStatus>,
    /// Optional note (content field)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Public key of event author (for easy querying)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_author_pubkey: Option<String>,
}

impl RSVPMetadata {
    pub fn new(
        d_tag: impl Into<String>,
        event_reference: CalendarReference,
        status: RSVPStatus,
    ) -> Self {
        Self {
            d_tag: d_tag.into(),
            event_reference,
            event_id: None,
            status,
            free_busy: None,
            note: None,
            event_author_pubkey: None,
        }
    }

    pub fn with_event_id(mut self, event_id: impl Into<String>) -> Self {
        self.event_id = Some(event_id.into());
        self
    }

    pub fn with_free_busy(mut self, free_busy: FreeBusyStatus) -> Self {
        self.free_busy = Some(free_busy);
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    pub fn with_event_author(mut self, pubkey: impl Into<String>) -> Self {
        self.event_author_pubkey = Some(pubkey.into());
        self
    }
}

impl NipMetadata for RSVPMetadata {
    type Kind = Nip52Kind;

    fn kind(&self) -> Self::Kind {
        Nip52Kind::RSVP
    }

    fn validate(&self) -> NipResult<()> {
        if self.d_tag.is_empty() {
            return Err(NipError::Nip52("RSVP d-tag cannot be empty".into()));
        }
        // Validate event reference
        if self.event_reference.author_pubkey.is_empty() {
            return Err(NipError::Nip52(
                "Event reference author pubkey cannot be empty".into(),
            ));
        }
        if self.event_reference.d_identifier.is_empty() {
            return Err(NipError::Nip52(
                "Event reference d-identifier cannot be empty".into(),
            ));
        }
        Ok(())
    }

    fn to_tags(&self) -> Vec<Tag> {
        let mut tags = vec![Tag::d(&self.d_tag)];

        // Event reference (a-tag)
        tags.push(self.event_reference.to_a_tag());

        // Specific event ID (e-tag) if present
        if let Some(ref event_id) = self.event_id {
            tags.push(Tag::e(event_id));
        }

        // Event author pubkey (p-tag)
        if let Some(ref author) = self.event_author_pubkey {
            tags.push(Tag::p(author));
        }

        // Status tag (l-tag with namespace)
        tags.push(Tag::new(vec![
            String::from("l"),
            self.status.as_str().to_string(),
            String::from("status"),
        ]));

        // Free/busy tag (l-tag with namespace)
        if let Some(ref free_busy) = self.free_busy {
            tags.push(Tag::new(vec![
                String::from("l"),
                free_busy.as_str().to_string(),
                String::from("freebusy"),
            ]));
        }

        tags
    }

    fn content(&self) -> String {
        self.note.clone().unwrap_or_default()
    }

    fn d_tag(&self) -> Option<String> {
        Some(self.d_tag.clone())
    }

    fn from_raw_event(event: &RawNostrEvent) -> NipResult<Self> {
        if event.kind != Nip52Kind::RSVP.kind_value() {
            return Err(NipError::Nip52(format!(
                "Expected kind 31925, got {}",
                event.kind
            )));
        }

        let d_tag = event
            .d_tag()
            .ok_or_else(|| NipError::Nip52("RSVP must have a d-tag".into()))?
            .to_string();

        let mut event_reference = None;
        let mut event_id = None;
        let mut event_author_pubkey = None;
        let mut status = None;
        let mut free_busy = None;

        for tag in &event.tags {
            if tag.is_empty() {
                continue;
            }
            match tag[0].as_str() {
                "a" => {
                    let tag_obj = Tag::new(tag.clone());
                    if let Ok(reference) = CalendarReference::from_a_tag(&tag_obj) {
                        event_reference = Some(reference);
                    }
                }
                "e" => {
                    if let Some(v) = tag.get(1) {
                        event_id = Some(v.clone());
                    }
                }
                "p" => {
                    if let Some(v) = tag.get(1) {
                        event_author_pubkey = Some(v.clone());
                    }
                }
                "l" => {
                    if tag.len() >= 3 {
                        let value = tag[1].as_str();
                        let namespace = tag[2].as_str();
                        match namespace {
                            "status" => {
                                status = match value {
                                    "accepted" => Some(RSVPStatus::Accepted),
                                    "declined" => Some(RSVPStatus::Declined),
                                    "tentative" => Some(RSVPStatus::Tentative),
                                    _ => None,
                                };
                            }
                            "freebusy" => {
                                free_busy = match value {
                                    "free" => Some(FreeBusyStatus::Free),
                                    "busy" => Some(FreeBusyStatus::Busy),
                                    _ => None,
                                };
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        let event_reference = event_reference
            .ok_or_else(|| NipError::Nip52("RSVP must reference a calendar event".into()))?;

        let status = status.ok_or_else(|| NipError::Nip52("RSVP must have a status".into()))?;

        Ok(RSVPMetadata {
            d_tag,
            event_reference,
            event_id,
            status,
            free_busy,
            note: if event.content.is_empty() {
                None
            } else {
                Some(event.content.clone())
            },
            event_author_pubkey,
        })
    }
}

// ==================== NIP-52 Helper Functions ====================

/// Generate a d-tag for a calendar event
pub fn generate_event_d_tag(title: &str, timestamp: u64) -> String {
    let hash = sha2::Sha256::digest(format!("{}:{}", title, timestamp).as_bytes());
    hex::encode(hash)[..16].to_string()
}

/// Generate a d-tag for an RSVP
pub fn generate_rsvp_d_tag(event_ref: &CalendarReference, responder_pubkey: &str) -> String {
    let hash = sha2::Sha256::digest(
        format!(
            "{}:{}:{}",
            event_ref.kind, event_ref.d_identifier, responder_pubkey
        )
        .as_bytes(),
    );
    hex::encode(hash)[..16].to_string()
}

/// Calculate the actual end time of an auction considering duration extensions
pub fn calculate_auction_end_time(start_time: u64, base_duration: u64, extensions: &[u64]) -> u64 {
    let total_extensions: u64 = extensions.iter().sum();
    start_time + base_duration + total_extensions
}
