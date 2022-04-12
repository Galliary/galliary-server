table! {
    Album (id) {
        id -> Text,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        sourceId -> Text,
        colors -> Nullable<Array<Int4>>,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
        authorId -> Text,
        groupId -> Nullable<Text>,
        lockStatus -> LockingStatus,
        rating -> SafetyRating,
        userFavouriteIds -> Nullable<Array<Text>>,
    }
}

table! {
    Category (slug) {
        slug -> Text,
        display -> Text,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        albumId -> Text,
        imageId -> Text,
        lockStatus -> LockingStatus,
        rating -> SafetyRating,
    }
}

table! {
    Group (id) {
        id -> Text,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        name -> Text,
        displayName -> Text,
        userId -> Nullable<Text>,
    }
}

table! {
    GroupMember (id) {
        id -> Text,
        role -> GroupMemberRole,
        groupId -> Text,
        userId -> Text,
        invitedAt -> Timestamp,
    }
}

table! {
    Image (id) {
        id -> Text,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        sourceId -> Text,
        colors -> Nullable<Array<Int4>>,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
        authorId -> Text,
        groupId -> Nullable<Text>,
        albumId -> Text,
        lockStatus -> LockingStatus,
        rating -> SafetyRating,
        userFavouriteIds -> Nullable<Array<Text>>,
    }
}

table! {
    ModeratorNotifications (id) {
        id -> Text,
        createdAt -> Timestamp,
        #[sql_name = "type"]
        type_ -> ModeratorNotificationsType,
        title -> Text,
        message -> Text,
        userId -> Nullable<Text>,
    }
}

table! {
    Notification (id) {
        id -> Text,
        createdAt -> Timestamp,
        #[sql_name = "type"]
        type_ -> NotificationType,
        title -> Text,
        message -> Text,
        userId -> Nullable<Text>,
    }
}

table! {
    Report (id) {
        id -> Text,
        createdAt -> Timestamp,
        albumId -> Nullable<Text>,
        imageId -> Nullable<Text>,
        userId -> Nullable<Text>,
        reporteeId -> Text,
        reason -> Text,
    }
}

table! {
    Session (id) {
        id -> Text,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        expiresAt -> Nullable<Timestamp>,
        handle -> Text,
        hashedSessionToken -> Nullable<Text>,
        antiCSRFToken -> Nullable<Text>,
        publicData -> Nullable<Text>,
        privateData -> Nullable<Text>,
        userId -> Nullable<Text>,
    }
}

table! {
    Token (id) {
        id -> Text,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        hashedToken -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        expiresAt -> Timestamp,
        sentTo -> Text,
        userId -> Text,
    }
}

table! {
    User (id) {
        id -> Text,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        username -> Text,
        nickname -> Nullable<Text>,
        bio -> Nullable<Text>,
        role -> UserRole,
        email -> Text,
        hashedPassword -> Nullable<Text>,
        avatarUrl -> Nullable<Text>,
        avatarSourceId -> Nullable<Text>,
        bannerSourceId -> Nullable<Text>,
        badges -> Nullable<Array<UserBadge>>,
        lockStatus -> LockingStatus,
        premiumFeatures -> Nullable<Array<PremiumFeature>>,
        userFavouriteIds -> Nullable<Array<Text>>,
    }
}

table! {
    UserConnection (userId, email) {
        #[sql_name = "type"]
        type_ -> UserConnectionType,
        email -> Text,
        handle -> Nullable<Text>,
        accessToken -> Nullable<Text>,
        refreshToken -> Nullable<Text>,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        expiresAt -> Nullable<Timestamp>,
        userId -> Text,
    }
}

table! {
    _UserFavouriteAlbums (A, B) {
        A -> Text,
        B -> Text,
    }
}

table! {
    _UserFavouriteImages (A, B) {
        A -> Text,
        B -> Text,
    }
}

table! {
    _UserFavouriteUsers (A, B) {
        A -> Text,
        B -> Text,
    }
}

joinable!(Album -> Group (groupId));
joinable!(Album -> User (authorId));
joinable!(Category -> Album (albumId));
joinable!(Category -> Image (imageId));
joinable!(Group -> User (userId));
joinable!(GroupMember -> Group (groupId));
joinable!(GroupMember -> User (userId));
joinable!(Image -> Album (albumId));
joinable!(Image -> Group (groupId));
joinable!(Image -> User (authorId));
joinable!(ModeratorNotifications -> User (userId));
joinable!(Notification -> User (userId));
joinable!(Report -> Album (albumId));
joinable!(Report -> Image (imageId));
joinable!(Session -> User (userId));
joinable!(Token -> User (userId));
joinable!(UserConnection -> User (userId));
joinable!(_UserFavouriteAlbums -> Album (A));
joinable!(_UserFavouriteAlbums -> User (B));
joinable!(_UserFavouriteImages -> Image (A));
joinable!(_UserFavouriteImages -> User (B));

allow_tables_to_appear_in_same_query!(
    Album,
    Category,
    Group,
    GroupMember,
    Image,
    ModeratorNotifications,
    Notification,
    Report,
    Session,
    Token,
    User,
    UserConnection,
    _UserFavouriteAlbums,
    _UserFavouriteImages,
    _UserFavouriteUsers,
);
