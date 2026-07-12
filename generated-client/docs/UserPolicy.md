# UserPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**is_administrator** | Option<**bool**> | Gets or sets a value indicating whether this instance is administrator. | [optional]
**is_hidden** | Option<**bool**> | Gets or sets a value indicating whether this instance is hidden. | [optional]
**enable_collection_management** | Option<**bool**> | Gets or sets a value indicating whether this instance can manage collections. | [optional][default to false]
**enable_subtitle_management** | Option<**bool**> | Gets or sets a value indicating whether this instance can manage subtitles. | [optional][default to false]
**enable_lyric_management** | Option<**bool**> | Gets or sets a value indicating whether this user can manage lyrics. | [optional][default to false]
**is_disabled** | Option<**bool**> | Gets or sets a value indicating whether this instance is disabled. | [optional]
**max_parental_rating** | Option<**i32**> | Gets or sets the max parental rating. | [optional]
**max_parental_sub_rating** | Option<**i32**> |  | [optional]
**blocked_tags** | Option<**Vec<String>**> |  | [optional]
**allowed_tags** | Option<**Vec<String>**> |  | [optional]
**enable_user_preference_access** | Option<**bool**> |  | [optional]
**access_schedules** | Option<[**Vec<models::AccessSchedule>**](AccessSchedule.md)> |  | [optional]
**block_unrated_items** | Option<[**Vec<models::UnratedItem>**](UnratedItem.md)> |  | [optional]
**enable_remote_control_of_other_users** | Option<**bool**> |  | [optional]
**enable_shared_device_control** | Option<**bool**> |  | [optional]
**enable_remote_access** | Option<**bool**> |  | [optional]
**enable_live_tv_management** | Option<**bool**> |  | [optional]
**enable_live_tv_access** | Option<**bool**> |  | [optional]
**enable_media_playback** | Option<**bool**> |  | [optional]
**enable_audio_playback_transcoding** | Option<**bool**> |  | [optional]
**enable_video_playback_transcoding** | Option<**bool**> |  | [optional]
**enable_playback_remuxing** | Option<**bool**> |  | [optional]
**force_remote_source_transcoding** | Option<**bool**> |  | [optional]
**enable_content_deletion** | Option<**bool**> |  | [optional]
**enable_content_deletion_from_folders** | Option<**Vec<String>**> |  | [optional]
**enable_content_downloading** | Option<**bool**> |  | [optional]
**enable_sync_transcoding** | Option<**bool**> | Gets or sets a value indicating whether [enable synchronize]. | [optional]
**enable_media_conversion** | Option<**bool**> |  | [optional]
**enabled_devices** | Option<**Vec<String>**> |  | [optional]
**enable_all_devices** | Option<**bool**> |  | [optional]
**enabled_channels** | Option<**Vec<uuid::Uuid>**> |  | [optional]
**enable_all_channels** | Option<**bool**> |  | [optional]
**enabled_folders** | Option<**Vec<uuid::Uuid>**> |  | [optional]
**enable_all_folders** | Option<**bool**> |  | [optional]
**invalid_login_attempt_count** | Option<**i32**> |  | [optional]
**login_attempts_before_lockout** | Option<**i32**> |  | [optional]
**max_active_sessions** | Option<**i32**> |  | [optional]
**enable_public_sharing** | Option<**bool**> |  | [optional]
**blocked_media_folders** | Option<**Vec<uuid::Uuid>**> |  | [optional]
**blocked_channels** | Option<**Vec<uuid::Uuid>**> |  | [optional]
**remote_client_bitrate_limit** | Option<**i32**> |  | [optional]
**authentication_provider_id** | **String** |  | 
**password_reset_provider_id** | **String** |  | 
**sync_play_access** | Option<[**models::SyncPlayUserAccessType**](SyncPlayUserAccessType.md)> | Enum SyncPlayUserAccessType. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


