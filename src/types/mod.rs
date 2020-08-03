//! API types.

pub use self::{
    allowed_update::*, animation::*, audio::*, callback_game::*, callback_query::*, chat::*,
    chat_action::*, chat_id::*, chat_member::*, chat_permissions::*, chat_photo::*,
    chosen_inline_result::*, contact::*, document::*, encrypted_credentials::*,
    encrypted_passport_element::*, file::*, force_reply::*, game::*, game_high_score::*,
    inline_keyboard_button::*, inline_keyboard_markup::*, inline_query::*, inline_query_result::*,
    inline_query_result_article::*, inline_query_result_audio::*,
    inline_query_result_cached_audio::*, inline_query_result_cached_document::*,
    inline_query_result_cached_gif::*, inline_query_result_cached_mpeg4_gif::*,
    inline_query_result_cached_photo::*, inline_query_result_cached_sticker::*,
    inline_query_result_cached_video::*, inline_query_result_cached_voice::*,
    inline_query_result_contact::*, inline_query_result_document::*, inline_query_result_game::*,
    inline_query_result_gif::*, inline_query_result_location::*, inline_query_result_mpeg4_gif::*,
    inline_query_result_photo::*, inline_query_result_venue::*, inline_query_result_video::*,
    inline_query_result_voice::*, input_file::*, input_media::*, input_message_content::*,
    invoice::*, keyboard_button::*, label_price::*, location::*, login_url::*, mask_position::*,
    message::*, message_entity::*, order_info::*, parse_mode::*, passport_data::*,
    passport_file::*, photo_size::*, poll::*, pre_checkout_query::*, reply_keyboard_markup::*,
    reply_keyboard_remove::*, reply_markup::*, response_parameters::*, send_invoice::*,
    shipping_address::*, shipping_option::*, shipping_query::*, sticker::*, sticker_set::*,
    successful_payment::*, unit_false::*, unit_true::*, update::*, user::*, user_profile_photos::*,
    venue::*, video::*, video_note::*, voice::*, webhook_info::*,
};

mod allowed_update;
mod animation;
mod audio;
mod callback_game;
mod callback_query;
mod chat;
mod chat_action;
mod chat_id;
mod chat_member;
mod chat_permissions;
mod chat_photo;
mod chosen_inline_result;
mod contact;
mod document;
mod file;
mod force_reply;
mod game;
mod game_high_score;
mod inline_keyboard_button;
mod inline_keyboard_markup;
mod input_file;
mod input_media;
mod input_message_content;
mod invoice;
mod keyboard_button;
mod label_price;
mod location;
mod login_url;
mod mask_position;
mod message;
mod message_entity;
mod order_info;
mod parse_mode;
mod photo_size;
mod poll;
mod pre_checkout_query;
mod reply_keyboard_markup;
mod reply_keyboard_remove;
mod reply_markup;
mod response_parameters;
mod send_invoice;
mod shipping_address;
mod shipping_option;
mod shipping_query;
mod sticker;
mod sticker_set;
mod successful_payment;
mod unit_false;
mod unit_true;
mod update;
mod user;
mod user_profile_photos;
mod venue;
mod video;
mod video_note;
mod voice;
mod webhook_info;

mod inline_query;
mod inline_query_result;
mod inline_query_result_article;
mod inline_query_result_audio;
mod inline_query_result_cached_audio;
mod inline_query_result_cached_document;
mod inline_query_result_cached_gif;
mod inline_query_result_cached_mpeg4_gif;
mod inline_query_result_cached_photo;
mod inline_query_result_cached_sticker;
mod inline_query_result_cached_video;
mod inline_query_result_cached_voice;
mod inline_query_result_contact;
mod inline_query_result_document;
mod inline_query_result_game;
mod inline_query_result_gif;
mod inline_query_result_location;
mod inline_query_result_mpeg4_gif;
mod inline_query_result_photo;
mod inline_query_result_venue;
mod inline_query_result_video;
mod inline_query_result_voice;

mod encrypted_credentials;
mod encrypted_passport_element;
mod passport_data;
mod passport_file;
