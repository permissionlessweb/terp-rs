use crate::{
    error::{NipError, NipResult},
    types::*,
    NipKind, NipMetadata, RawNostrEvent, Tag,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ==================== NIP-15 Kinds ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Nip15Kind {
    SetStall,
    SetProduct,
    MarketplaceUI,
    AuctionProduct,
    Bid,
    BidConfirmation,
}

impl NipKind for Nip15Kind {
    fn kind_value(&self) -> u16 {
        match self {
            Nip15Kind::SetStall => 30017,
            Nip15Kind::SetProduct => 30018,
            Nip15Kind::MarketplaceUI => 30019,
            Nip15Kind::AuctionProduct => 30020,
            Nip15Kind::Bid => 1021,
            Nip15Kind::BidConfirmation => 1022,
        }
    }
}

// ==================== NIP-15 Metadata Types ====================

/// Kind 30017: Create or update a stall
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StallMetadata {
    /// Unique stall ID (also used as d-tag)
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub currency: String,
    pub shipping: Vec<ShippingZone>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ShippingZone {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub cost: f64,
    pub regions: Vec<String>,
}

impl NipMetadata for StallMetadata {
    type Kind = Nip15Kind;

    fn kind(&self) -> Self::Kind {
        Nip15Kind::SetStall
    }

    fn validate(&self) -> NipResult<()> {
        if self.id.is_empty() {
            return Err(NipError::Nip15("Stall ID cannot be empty".into()));
        }
        if self.name.is_empty() {
            return Err(NipError::Nip15("Stall name cannot be empty".into()));
        }
        if self.currency.is_empty() {
            return Err(NipError::Nip15("Currency cannot be empty".into()));
        }
        if self.shipping.is_empty() {
            return Err(NipError::Nip15(
                "At least one shipping zone is required".into(),
            ));
        }
        for zone in &self.shipping {
            if zone.id.is_empty() {
                return Err(NipError::Nip15("Shipping zone ID cannot be empty".into()));
            }
            if zone.regions.is_empty() {
                return Err(NipError::Nip15(
                    "Shipping zone must have at least one region".into(),
                ));
            }
        }
        Ok(())
    }

    fn to_tags(&self) -> Vec<Tag> {
        vec![Tag::d(&self.id)]
    }

    fn content(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    fn d_tag(&self) -> Option<String> {
        Some(self.id.clone())
    }

    fn from_raw_event(event: &RawNostrEvent) -> NipResult<Self> {
        if event.kind != Nip15Kind::SetStall.kind_value() {
            return Err(NipError::Nip15(format!(
                "Expected kind 30017, got {}",
                event.kind
            )));
        }

        let d_tag = event
            .d_tag()
            .ok_or_else(|| NipError::Nip15("Stall event must have a d-tag".into()))?;

        let mut stall: StallMetadata =
            serde_json::from_str(&event.content).map_err(NipError::Serialization)?;

        // Ensure d-tag matches stall ID
        if stall.id != d_tag {
            return Err(NipError::Nip15(format!(
                "Stall ID {} does not match d-tag {}",
                stall.id, d_tag
            )));
        }

        Ok(stall)
    }
}

/// Kind 30018: Create or update a product
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProductMetadata {
    /// Unique product ID (also used as d-tag)
    pub id: String,
    pub stall_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<String>,
    pub currency: String,
    pub price: f64,
    pub quantity: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specs: Vec<Vec<String>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shipping: Vec<ProductShipping>,
    /// Product categories (t-tags)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProductShipping {
    pub id: String,
    pub cost: f64,
}

impl NipMetadata for ProductMetadata {
    type Kind = Nip15Kind;

    fn kind(&self) -> Self::Kind {
        Nip15Kind::SetProduct
    }

    fn validate(&self) -> NipResult<()> {
        if self.id.is_empty() {
            return Err(NipError::Nip15("Product ID cannot be empty".into()));
        }
        if self.stall_id.is_empty() {
            return Err(NipError::Nip15("Stall ID cannot be empty".into()));
        }
        if self.name.is_empty() {
            return Err(NipError::Nip15("Product name cannot be empty".into()));
        }
        if self.currency.is_empty() {
            return Err(NipError::Nip15("Currency cannot be empty".into()));
        }
        if self.price < 0.0 {
            return Err(NipError::Nip15("Price cannot be negative".into()));
        }
        Ok(())
    }

    fn to_tags(&self) -> Vec<Tag> {
        let mut tags = vec![Tag::d(&self.id)];

        // Add category t-tags
        for category in &self.categories {
            tags.push(Tag::t(category));
        }

        tags
    }

    fn content(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    fn d_tag(&self) -> Option<String> {
        Some(self.id.clone())
    }

    fn from_raw_event(event: &RawNostrEvent) -> NipResult<Self> {
        if event.kind != Nip15Kind::SetProduct.kind_value() {
            return Err(NipError::Nip15(format!(
                "Expected kind 30018, got {}",
                event.kind
            )));
        }

        let d_tag = event
            .d_tag()
            .ok_or_else(|| NipError::Nip15("Product event must have a d-tag".into()))?;

        let mut product: ProductMetadata =
            serde_json::from_str(&event.content).map_err(NipError::Serialization)?;

        // Ensure d-tag matches product ID
        if product.id != d_tag {
            return Err(NipError::Nip15(format!(
                "Product ID {} does not match d-tag {}",
                product.id, d_tag
            )));
        }

        // Extract categories from t-tags
        let categories: Vec<String> = event
            .tags
            .iter()
            .filter(|tag| tag.first().map(|n| n.as_str()) == Some("t"))
            .filter_map(|tag| tag.get(1).cloned())
            .collect();

        product.categories = categories;

        Ok(product)
    }
}

/// Kind 30019: Marketplace UI/UX configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MarketplaceUIMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui: Option<MarketplaceUI>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub merchants: Vec<String>,
    /// Unique identifier for this marketplace config
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MarketplaceUI {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_mode: Option<bool>,
}

impl NipMetadata for MarketplaceUIMetadata {
    type Kind = Nip15Kind;

    fn kind(&self) -> Self::Kind {
        Nip15Kind::MarketplaceUI
    }

    fn validate(&self) -> NipResult<()> {
        if self.id.is_empty() {
            return Err(NipError::Nip15("Marketplace ID cannot be empty".into()));
        }
        Ok(())
    }

    fn to_tags(&self) -> Vec<Tag> {
        let mut tags = vec![Tag::d(&self.id)];

        // Add merchant p-tags
        for merchant in &self.merchants {
            tags.push(Tag::p(merchant));
        }

        tags
    }

    fn content(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    fn d_tag(&self) -> Option<String> {
        Some(self.id.clone())
    }

    fn from_raw_event(event: &RawNostrEvent) -> NipResult<Self> {
        if event.kind != Nip15Kind::MarketplaceUI.kind_value() {
            return Err(NipError::Nip15(format!(
                "Expected kind 30019, got {}",
                event.kind
            )));
        }

        let d_tag = event
            .d_tag()
            .ok_or_else(|| NipError::Nip15("Marketplace UI event must have a d-tag".into()))?;

        let mut ui: MarketplaceUIMetadata =
            serde_json::from_str(&event.content).map_err(NipError::Serialization)?;

        ui.id = d_tag.to_string();

        // Extract merchants from p-tags
        ui.merchants = event
            .tags
            .iter()
            .filter(|tag| tag.first().map(|n| n.as_str()) == Some("p"))
            .filter_map(|tag| tag.get(1).cloned())
            .collect();

        Ok(ui)
    }
}

/// Kind 30020: Auction product
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuctionProductMetadata {
    pub id: String,
    pub stall_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<String>,
    pub starting_bid: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<u64>,
    pub duration: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub specs: Vec<Vec<String>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shipping: Vec<ProductShipping>,
}

impl NipMetadata for AuctionProductMetadata {
    type Kind = Nip15Kind;

    fn kind(&self) -> Self::Kind {
        Nip15Kind::AuctionProduct
    }

    fn validate(&self) -> NipResult<()> {
        if self.id.is_empty() {
            return Err(NipError::Nip15("Auction ID cannot be empty".into()));
        }
        if self.name.is_empty() {
            return Err(NipError::Nip15("Auction name cannot be empty".into()));
        }
        if self.duration == 0 {
            return Err(NipError::Nip15("Duration must be greater than 0".into()));
        }
        Ok(())
    }

    fn to_tags(&self) -> Vec<Tag> {
        vec![Tag::d(&self.id)]
    }

    fn content(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    fn d_tag(&self) -> Option<String> {
        Some(self.id.clone())
    }

    fn from_raw_event(event: &RawNostrEvent) -> NipResult<Self> {
        if event.kind != Nip15Kind::AuctionProduct.kind_value() {
            return Err(NipError::Nip15(format!(
                "Expected kind 30020, got {}",
                event.kind
            )));
        }

        let d_tag = event
            .d_tag()
            .ok_or_else(|| NipError::Nip15("Auction event must have a d-tag".into()))?;

        let mut auction: AuctionProductMetadata =
            serde_json::from_str(&event.content).map_err(NipError::Serialization)?;

        if auction.id != d_tag {
            return Err(NipError::Nip15(format!(
                "Auction ID {} does not match d-tag {}",
                auction.id, d_tag
            )));
        }

        Ok(auction)
    }
}

/// Kind 1021: Bid
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BidMetadata {
    /// The bid amount in sats
    pub amount: u64,
    /// The event ID of the auction being bid on
    pub auction_event_id: String,
}

impl NipMetadata for BidMetadata {
    type Kind = Nip15Kind;

    fn kind(&self) -> Self::Kind {
        Nip15Kind::Bid
    }

    fn validate(&self) -> NipResult<()> {
        if self.auction_event_id.is_empty() {
            return Err(NipError::Nip15("Auction event ID cannot be empty".into()));
        }
        Ok(())
    }

    fn to_tags(&self) -> Vec<Tag> {
        vec![Tag::e(&self.auction_event_id)]
    }

    fn content(&self) -> String {
        self.amount.to_string()
    }

    fn from_raw_event(event: &RawNostrEvent) -> NipResult<Self> {
        if event.kind != Nip15Kind::Bid.kind_value() {
            return Err(NipError::Nip15(format!(
                "Expected kind 1021, got {}",
                event.kind
            )));
        }

        let auction_event_id = event
            .tags
            .iter()
            .find(|tag| tag.first().map(|n| n.as_str()) == Some("e"))
            .and_then(|tag| tag.get(1).cloned())
            .ok_or_else(|| NipError::Nip15("Bid must reference an auction event".into()))?;

        let amount: u64 = event
            .content
            .parse()
            .map_err(|_| NipError::Nip15("Bid content must be a numeric amount".into()))?;

        Ok(BidMetadata {
            amount,
            auction_event_id,
        })
    }
}

/// Kind 1022: Bid confirmation
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BidConfirmationMetadata {
    pub status: BidStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_extended: Option<u64>,
    /// The event ID of the bid being confirmed
    pub bid_event_id: String,
    /// The event ID of the auction
    pub auction_event_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BidStatus {
    Accepted,
    Rejected,
    Pending,
    Winner,
}

impl NipMetadata for BidConfirmationMetadata {
    type Kind = Nip15Kind;

    fn kind(&self) -> Self::Kind {
        Nip15Kind::BidConfirmation
    }

    fn validate(&self) -> NipResult<()> {
        if self.bid_event_id.is_empty() {
            return Err(NipError::Nip15("Bid event ID cannot be empty".into()));
        }
        if self.auction_event_id.is_empty() {
            return Err(NipError::Nip15("Auction event ID cannot be empty".into()));
        }
        Ok(())
    }

    fn to_tags(&self) -> Vec<Tag> {
        vec![Tag::e(&self.bid_event_id), Tag::e(&self.auction_event_id)]
    }

    fn content(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    fn from_raw_event(event: &RawNostrEvent) -> NipResult<Self> {
        if event.kind != Nip15Kind::BidConfirmation.kind_value() {
            return Err(NipError::Nip15(format!(
                "Expected kind 1022, got {}",
                event.kind
            )));
        }

        let e_tags: Vec<&Vec<String>> = event
            .tags
            .iter()
            .filter(|tag| tag.first().map(|n| n.as_str()) == Some("e"))
            .collect();

        if e_tags.len() < 2 {
            return Err(NipError::Nip15(
                "Bid confirmation must reference both bid and auction events".into(),
            ));
        }

        let bid_event_id = e_tags[0]
            .get(1)
            .ok_or_else(|| NipError::Nip15("Missing bid event ID".into()))?
            .clone();
        let auction_event_id = e_tags[1]
            .get(1)
            .ok_or_else(|| NipError::Nip15("Missing auction event ID".into()))?
            .clone();

        let mut confirmation: BidConfirmationMetadata =
            serde_json::from_str(&event.content).map_err(NipError::Serialization)?;

        confirmation.bid_event_id = bid_event_id;
        confirmation.auction_event_id = auction_event_id;

        Ok(confirmation)
    }
}

// ==================== Checkout Message Types ====================

/// Customer order message (type 0)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CustomerOrder {
    pub id: String,
    #[serde(rename = "type")]
    pub message_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<OrderContact>,
    pub items: Vec<OrderItem>,
    pub shipping_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OrderContact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nostr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OrderItem {
    pub product_id: String,
    pub quantity: u64,
}

impl CustomerOrder {
    pub fn new(
        id: impl Into<String>,
        items: Vec<OrderItem>,
        shipping_id: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            message_type: 0,
            name: None,
            address: None,
            message: None,
            contact: None,
            items,
            shipping_id: shipping_id.into(),
        }
    }
}

/// Merchant payment request message (type 1)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PaymentRequest {
    pub id: String,
    #[serde(rename = "type")]
    pub message_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub payment_options: Vec<PaymentOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PaymentOption {
    #[serde(rename = "type")]
    pub payment_type: PaymentType,
    pub link: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentType {
    Url,
    Btc,
    Ln,
    Lnurl,
}

impl PaymentRequest {
    pub fn new(id: impl Into<String>, payment_options: Vec<PaymentOption>) -> Self {
        Self {
            id: id.into(),
            message_type: 1,
            message: None,
            payment_options,
        }
    }
}

/// Merchant order status update message (type 2)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OrderStatusUpdate {
    pub id: String,
    #[serde(rename = "type")]
    pub message_type: u8,
    pub message: String,
    pub paid: bool,
    pub shipped: bool,
}

impl OrderStatusUpdate {
    pub fn new(
        id: impl Into<String>,
        message: impl Into<String>,
        paid: bool,
        shipped: bool,
    ) -> Self {
        Self {
            id: id.into(),
            message_type: 2,
            message: message.into(),
            paid,
            shipped,
        }
    }
}
