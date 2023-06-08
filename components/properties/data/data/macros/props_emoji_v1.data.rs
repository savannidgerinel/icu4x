// @generated
/// Implement [`DataProvider<EmojiV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_emoji_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_EMOJI_V1: &'static <icu_properties::provider::EmojiV1Marker as icu_provider::DataMarker>::Yokeable = &icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu_collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"#\0\0\0$\0\0\0*\0\0\0+\0\0\x000\0\0\0:\0\0\0\xA9\0\0\0\xAA\0\0\0\xAE\0\0\0\xAF\0\0\0< \0\0= \0\0I \0\0J \0\0\"!\0\0#!\0\09!\0\0:!\0\0\x94!\0\0\x9A!\0\0\xA9!\0\0\xAB!\0\0\x1A#\0\0\x1C#\0\0(#\0\0)#\0\0\xCF#\0\0\xD0#\0\0\xE9#\0\0\xF4#\0\0\xF8#\0\0\xFB#\0\0\xC2$\0\0\xC3$\0\0\xAA%\0\0\xAC%\0\0\xB6%\0\0\xB7%\0\0\xC0%\0\0\xC1%\0\0\xFB%\0\0\xFF%\0\0\0&\0\0\x05&\0\0\x0E&\0\0\x0F&\0\0\x11&\0\0\x12&\0\0\x14&\0\0\x16&\0\0\x18&\0\0\x19&\0\0\x1D&\0\0\x1E&\0\0 &\0\0!&\0\0\"&\0\0$&\0\0&&\0\0'&\0\0*&\0\0+&\0\0.&\0\x000&\0\08&\0\0;&\0\0@&\0\0A&\0\0B&\0\0C&\0\0H&\0\0T&\0\0_&\0\0a&\0\0c&\0\0d&\0\0e&\0\0g&\0\0h&\0\0i&\0\0{&\0\0|&\0\0~&\0\0\x80&\0\0\x92&\0\0\x98&\0\0\x99&\0\0\x9A&\0\0\x9B&\0\0\x9D&\0\0\xA0&\0\0\xA2&\0\0\xA7&\0\0\xA8&\0\0\xAA&\0\0\xAC&\0\0\xB0&\0\0\xB2&\0\0\xBD&\0\0\xBF&\0\0\xC4&\0\0\xC6&\0\0\xC8&\0\0\xC9&\0\0\xCE&\0\0\xD0&\0\0\xD1&\0\0\xD2&\0\0\xD3&\0\0\xD5&\0\0\xE9&\0\0\xEB&\0\0\xF0&\0\0\xF6&\0\0\xF7&\0\0\xFB&\0\0\xFD&\0\0\xFE&\0\0\x02'\0\0\x03'\0\0\x05'\0\0\x06'\0\0\x08'\0\0\x0E'\0\0\x0F'\0\0\x10'\0\0\x12'\0\0\x13'\0\0\x14'\0\0\x15'\0\0\x16'\0\0\x17'\0\0\x1D'\0\0\x1E'\0\0!'\0\0\"'\0\0('\0\0)'\0\x003'\0\x005'\0\0D'\0\0E'\0\0G'\0\0H'\0\0L'\0\0M'\0\0N'\0\0O'\0\0S'\0\0V'\0\0W'\0\0X'\0\0c'\0\0e'\0\0\x95'\0\0\x98'\0\0\xA1'\0\0\xA2'\0\0\xB0'\0\0\xB1'\0\0\xBF'\0\0\xC0'\0\x004)\0\x006)\0\0\x05+\0\0\x08+\0\0\x1B+\0\0\x1D+\0\0P+\0\0Q+\0\0U+\0\0V+\0\x0000\0\x0010\0\0=0\0\0>0\0\0\x972\0\0\x982\0\0\x992\0\0\x9A2\0\0\x04\xF0\x01\0\x05\xF0\x01\0\xCF\xF0\x01\0\xD0\xF0\x01\0p\xF1\x01\0r\xF1\x01\0~\xF1\x01\0\x80\xF1\x01\0\x8E\xF1\x01\0\x8F\xF1\x01\0\x91\xF1\x01\0\x9B\xF1\x01\0\xE6\xF1\x01\0\0\xF2\x01\0\x01\xF2\x01\0\x03\xF2\x01\0\x1A\xF2\x01\0\x1B\xF2\x01\0/\xF2\x01\x000\xF2\x01\x002\xF2\x01\0;\xF2\x01\0P\xF2\x01\0R\xF2\x01\0\0\xF3\x01\0\"\xF3\x01\0$\xF3\x01\0\x94\xF3\x01\0\x96\xF3\x01\0\x98\xF3\x01\0\x99\xF3\x01\0\x9C\xF3\x01\0\x9E\xF3\x01\0\xF1\xF3\x01\0\xF3\xF3\x01\0\xF6\xF3\x01\0\xF7\xF3\x01\0\xFE\xF4\x01\0\xFF\xF4\x01\0>\xF5\x01\0I\xF5\x01\0O\xF5\x01\0P\xF5\x01\0h\xF5\x01\0o\xF5\x01\0q\xF5\x01\0s\xF5\x01\0{\xF5\x01\0\x87\xF5\x01\0\x88\xF5\x01\0\x8A\xF5\x01\0\x8E\xF5\x01\0\x90\xF5\x01\0\x91\xF5\x01\0\x95\xF5\x01\0\x97\xF5\x01\0\xA4\xF5\x01\0\xA6\xF5\x01\0\xA8\xF5\x01\0\xA9\xF5\x01\0\xB1\xF5\x01\0\xB3\xF5\x01\0\xBC\xF5\x01\0\xBD\xF5\x01\0\xC2\xF5\x01\0\xC5\xF5\x01\0\xD1\xF5\x01\0\xD4\xF5\x01\0\xDC\xF5\x01\0\xDF\xF5\x01\0\xE1\xF5\x01\0\xE2\xF5\x01\0\xE3\xF5\x01\0\xE4\xF5\x01\0\xE8\xF5\x01\0\xE9\xF5\x01\0\xEF\xF5\x01\0\xF0\xF5\x01\0\xF3\xF5\x01\0\xF4\xF5\x01\0\xFA\xF5\x01\0P\xF6\x01\0\x80\xF6\x01\0\xC6\xF6\x01\0\xCB\xF6\x01\0\xD3\xF6\x01\0\xD5\xF6\x01\0\xD8\xF6\x01\0\xDC\xF6\x01\0\xE6\xF6\x01\0\xE9\xF6\x01\0\xEA\xF6\x01\0\xEB\xF6\x01\0\xED\xF6\x01\0\xF0\xF6\x01\0\xF1\xF6\x01\0\xF3\xF6\x01\0\xFD\xF6\x01\0\xE0\xF7\x01\0\xEC\xF7\x01\0\xF0\xF7\x01\0\xF1\xF7\x01\0\x0C\xF9\x01\0;\xF9\x01\0<\xF9\x01\0F\xF9\x01\0G\xF9\x01\0\0\xFA\x01\0p\xFA\x01\0}\xFA\x01\0\x80\xFA\x01\0\x89\xFA\x01\0\x90\xFA\x01\0\xBE\xFA\x01\0\xBF\xFA\x01\0\xC6\xFA\x01\0\xCE\xFA\x01\0\xDC\xFA\x01\0\xE0\xFA\x01\0\xE9\xFA\x01\0\xF0\xFA\x01\0\xF9\xFA\x01\0") }, 1424usize)
            });
        }
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_properties::provider::EmojiV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_properties::provider::EmojiV1Marker>, icu_provider::DataError> {
                let locale = req.locale;
                match {
                    if locale.is_empty() {
                        Ok(Self::SINGLETON_PROPS_EMOJI_V1)
                    } else {
                        Err(icu_provider::DataErrorKind::ExtraneousLocale)
                    }
                } {
                    Ok(payload) => Ok(icu_provider::DataResponse { metadata: Default::default(), payload: Some(icu_provider::DataPayload::from_static_ref(payload)) }),
                    Err(e) => Err(e.with_req(<icu_properties::provider::EmojiV1Marker as icu_provider::KeyedDataMarker>::KEY, req)),
                }
            }
        }
    };
}
