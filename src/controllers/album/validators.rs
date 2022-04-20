use async_graphql::CustomValidator;

const ALBUM_TITLE_MAX_LENGTH: usize = 32;
const ALBUM_TITLE_MIN_LENGTH: usize = 3;

const ALBUM_DESCRIPTION_MAX_LENGTH: usize = 200;

pub struct AlbumTitleValidator;
impl CustomValidator<String> for AlbumTitleValidator {
    fn check(&self, value: &String) -> Result<(), String> {
        if value.len() > ALBUM_TITLE_MAX_LENGTH {
            return Err("Album title must be less than 32 characters".to_string());
        }
        if value.len() < ALBUM_TITLE_MIN_LENGTH {
            return Err("Album title must be at least 3 characters".to_string());
        }
        Ok(())
    }
}

pub struct AlbumDescriptionValidator;
impl CustomValidator<String> for AlbumDescriptionValidator {
    fn check(&self, value: &String) -> Result<(), String> {
        if value.len() > ALBUM_DESCRIPTION_MAX_LENGTH {
            return Err("Album description must be less than 200 characters".to_string());
        }
        Ok(())
    }
}
