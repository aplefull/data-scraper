use crate::structs::{
    AddressData, AutofillData, CookiesData, CreditCardData, DownloadHistoryData, HistoryData,
    KeywordsData, PasswordsData,
};
use crate::utils::{decrypt_sql_value, get_sql_value, get_sqlite_data};
use std::path::PathBuf;

pub fn get_cookies_data(path: &PathBuf) -> Vec<CookiesData> {
    let cookies = get_sqlite_data(&path, "SELECT * FROM cookies");
    let mut cookies_data = Vec::new();
    println!("{:?}", cookies);

    for cookie in cookies {
        let cookie_data = CookiesData {
            creation_utc: get_sql_value(cookie.get(0)),
            host_key: get_sql_value(cookie.get(1)),
            top_frame_site_key: get_sql_value(cookie.get(2)),
            name: get_sql_value(cookie.get(3)),
            value: decrypt_sql_value(cookie.get(5)),
            path: get_sql_value(cookie.get(6)),
            expires_utc: get_sql_value(cookie.get(7)),
            is_secure: get_sql_value(cookie.get(8)),
            is_http_only: get_sql_value(cookie.get(9)),
            last_access_utc: get_sql_value(cookie.get(10)),
            has_expires: get_sql_value(cookie.get(11)),
            is_persistent: get_sql_value(cookie.get(12)),
            priority: get_sql_value(cookie.get(13)),
            same_site: get_sql_value(cookie.get(14)),
            source_scheme: get_sql_value(cookie.get(15)),
            source_port: get_sql_value(cookie.get(16)),
            last_update_utc: get_sql_value(cookie.get(17)),
        };

        cookies_data.push(cookie_data);
    }

    cookies_data
}

pub fn get_passwords_data(path: &PathBuf) -> Vec<PasswordsData> {
    let passwords = get_sqlite_data(path, "SELECT * FROM logins");
    let mut passwords_data = Vec::new();

    for password in passwords {
        let password_data = PasswordsData {
            origin_url: get_sql_value(password.get(0)),
            action_url: get_sql_value(password.get(1)),
            username_value: get_sql_value(password.get(3)),
            password_value: decrypt_sql_value(password.get(5)),
            date_created: get_sql_value(password.get(8)),
            blacklisted_by_user: get_sql_value(password.get(9)),
            scheme: get_sql_value(password.get(10)),
            password_type: get_sql_value(password.get(11)),
            times_used: get_sql_value(password.get(12)),
            form_data: get_sql_value(password.get(13)),
            display_name: get_sql_value(password.get(14)),
            possible_username_pairs: get_sql_value(password.get(19)),
            date_last_used: get_sql_value(password.get(20)),
            date_password_modified: get_sql_value(password.get(22)),
            sender_email: get_sql_value(password.get(23)),
            sender_name: get_sql_value(password.get(24)),
            date_received: get_sql_value(password.get(25)),
        };

        passwords_data.push(password_data);
    }

    passwords_data
}

pub fn get_autofill_data(path: &PathBuf) -> Vec<AutofillData> {
    let autofill_data = get_sqlite_data(path, "SELECT * FROM autofill");
    let mut autofill_data_list = Vec::new();

    for autofill_entry in autofill_data {
        let autofill_data_entry = AutofillData {
            name: get_sql_value(autofill_entry.get(0)),
            value: get_sql_value(autofill_entry.get(1)),
            value_lower: get_sql_value(autofill_entry.get(2)),
            date_created: get_sql_value(autofill_entry.get(3)),
            date_last_used: get_sql_value(autofill_entry.get(4)),
            count: get_sql_value(autofill_entry.get(5)),
        };

        autofill_data_list.push(autofill_data_entry);
    }

    autofill_data_list
}

pub fn get_credit_cards_data(path: &PathBuf) -> Vec<CreditCardData> {
    let credit_cards_data = get_sqlite_data(path, "SELECT * FROM credit_cards");
    let mut credit_cards_data_list = Vec::new();

    for credit_card_entry in credit_cards_data {
        let credit_card_data_entry = CreditCardData {
            guid: get_sql_value(credit_card_entry.get(0)),
            name_on_card: get_sql_value(credit_card_entry.get(1)),
            expiration_month: get_sql_value(credit_card_entry.get(2)),
            expiration_year: get_sql_value(credit_card_entry.get(3)),
            card_number: decrypt_sql_value(credit_card_entry.get(4)),
            date_modified: get_sql_value(credit_card_entry.get(5)),
            origin: get_sql_value(credit_card_entry.get(6)),
            use_count: get_sql_value(credit_card_entry.get(7)),
            use_date: get_sql_value(credit_card_entry.get(8)),
            billing_address_id: get_sql_value(credit_card_entry.get(9)),
            nickname: get_sql_value(credit_card_entry.get(10)),
        };

        credit_cards_data_list.push(credit_card_data_entry);
    }

    credit_cards_data_list
}

pub fn get_addresses_data(path: &PathBuf) -> Vec<AddressData> {
    let addresses_data = get_sqlite_data(path, "SELECT * FROM local_addresses");
    let mut addresses_data_list = Vec::new();

    for address_entry in addresses_data {
        let address_data_entry = AddressData {
            guid: get_sql_value(address_entry.get(0)),
            use_count: get_sql_value(address_entry.get(1)),
            use_date: get_sql_value(address_entry.get(2)),
            date_modified: get_sql_value(address_entry.get(3)),
            language_code: get_sql_value(address_entry.get(4)),
            label: get_sql_value(address_entry.get(5)),
        };

        addresses_data_list.push(address_data_entry);
    }

    addresses_data_list
}

pub fn get_history_data(path: &PathBuf) -> Vec<HistoryData> {
    let history = get_sqlite_data(path, "SELECT * FROM urls");
    let mut history_data = Vec::new();

    for history_entry in history {
        let history_data_entry = HistoryData {
            url: get_sql_value(history_entry.get(1)),
            title: get_sql_value(history_entry.get(2)),
            visit_count: get_sql_value(history_entry.get(3)),
            typed_count: get_sql_value(history_entry.get(4)),
            last_visit_time: get_sql_value(history_entry.get(5)),
            hidden: get_sql_value(history_entry.get(6)),
        };

        history_data.push(history_data_entry);
    }

    history_data
}

pub fn get_download_history_data(path: &PathBuf) -> Vec<DownloadHistoryData> {
    let download_history = get_sqlite_data(path, "SELECT * FROM downloads");
    let mut download_history_data = Vec::new();

    for download_entry in download_history {
        let download_history_data_entry = DownloadHistoryData {
            id: get_sql_value(download_entry.get(0)),
            guid: get_sql_value(download_entry.get(1)),
            current_path: get_sql_value(download_entry.get(2)),
            target_path: get_sql_value(download_entry.get(3)),
            referrer: get_sql_value(download_entry.get(15)),
            site_url: get_sql_value(download_entry.get(16)),
            mime_type: get_sql_value(download_entry.get(26)),
            original_mime_type: get_sql_value(download_entry.get(27)),
        };

        download_history_data.push(download_history_data_entry);
    }

    download_history_data
}

pub fn get_keywords_data(path: &PathBuf) -> Vec<KeywordsData> {
    let keywords_data = get_sqlite_data(path, "SELECT * FROM keyword_search_terms");
    let mut keywords_data_list = Vec::new();

    for keyword_entry in keywords_data {
        let keyword_data_entry = KeywordsData {
            keyword_id: get_sql_value(keyword_entry.get(0)),
            url_id: get_sql_value(keyword_entry.get(1)),
            term: get_sql_value(keyword_entry.get(2)),
            normalized_term: get_sql_value(keyword_entry.get(3)),
        };

        keywords_data_list.push(keyword_data_entry);
    }

    keywords_data_list
}
