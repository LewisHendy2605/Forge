use fake::Fake;
use fake::faker::address::en::*;
use fake::faker::creditcard::en::CreditCardNumber;
use fake::faker::filesystem::ar_sa::FileExtension;
use fake::faker::filesystem::en::FileName;
use fake::faker::filesystem::en::FilePath;
use fake::faker::http::en::RfcStatusCode;
use fake::faker::internet::en::*;
use fake::faker::lorem::en::Word;
use fake::faker::name::en::*;
use fake::faker::phone_number::ar_sa::PhoneNumber;
use fake::faker::time::ar_sa::Date;
use fake::faker::time::ar_sa::DateTime;
use fake::faker::time::ar_sa::Time;
use fake::uuid::UUIDv4;

pub fn generate_data(data_type: &String, i: Option<u32>, delim: Option<u8>) -> String {
    return match data_type.as_str() {
        "index" => i.unwrap_or(0).to_string(),
        "uuid" => UUIDv4.fake(),

        "char" => {
            let w: String = Word().fake();
            w.chars().next().unwrap_or('a').to_string()
        }
        "quoted" => {
            Word().fake::<String>() + &(delim.unwrap_or(b',') as char).to_string() + Word().fake()
        }

        "post_code" => PostCode().fake(),
        "zip_code" => ZipCode().fake(),
        "country_name" => CountryName().fake(),
        "city_name" => CityName().fake(),
        "street_name" => StreetName().fake(),
        "building_number" => BuildingNumber().fake(),

        "title" => Title().fake(),
        "full_name" => Name().fake(),
        "first_name" => FirstName().fake(),
        "last_name" => LastName().fake(),
        "username" => Username().fake(),
        "password" => Password(8..20).fake(),

        "ip" => IPv4().fake(),
        "email" => SafeEmail().fake(),
        "phone" => PhoneNumber().fake(),
        "credit_card_number" => CreditCardNumber().fake(),

        "date" => Date().fake(),
        "time" => Time().fake(),
        "date_time" => DateTime().fake(),

        "file_path" => FilePath().fake(),
        "file_name" => FileName().fake(),
        "file_ext" => FileExtension().fake(),

        "http_status_code" => RfcStatusCode().fake(),
        "rfc_status_code" => RfcStatusCode().fake(),

        _ => String::new(),
    };
}
