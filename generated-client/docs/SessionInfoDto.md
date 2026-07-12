# SessionInfoDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**play_state** | Option<[**models::PlayerStateInfo**](PlayerStateInfo.md)> | Gets or sets the play state. | [optional]
**additional_users** | Option<[**Vec<models::SessionUserInfo>**](SessionUserInfo.md)> | Gets or sets the additional users. | [optional]
**capabilities** | Option<[**models::ClientCapabilitiesDto**](ClientCapabilitiesDto.md)> | Gets or sets the client capabilities. | [optional]
**remote_end_point** | Option<**String**> | Gets or sets the remote end point. | [optional]
**playable_media_types** | Option<[**Vec<models::MediaType>**](MediaType.md)> | Gets or sets the playable media types. | [optional]
**id** | Option<**String**> | Gets or sets the id. | [optional]
**user_id** | Option<**uuid::Uuid**> | Gets or sets the user id. | [optional]
**user_name** | Option<**String**> | Gets or sets the username. | [optional]
**client** | Option<**String**> | Gets or sets the type of the client. | [optional]
**last_activity_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Gets or sets the last activity date. | [optional]
**last_playback_check_in** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Gets or sets the last playback check in. | [optional]
**last_paused_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Gets or sets the last paused date. | [optional]
**device_name** | Option<**String**> | Gets or sets the name of the device. | [optional]
**device_type** | Option<**String**> | Gets or sets the type of the device. | [optional]
**now_playing_item** | Option<[**models::BaseItemDto**](BaseItemDto.md)> | Gets or sets the now playing item. | [optional]
**now_viewing_item** | Option<[**models::BaseItemDto**](BaseItemDto.md)> | Gets or sets the now viewing item. | [optional]
**device_id** | Option<**String**> | Gets or sets the device id. | [optional]
**application_version** | Option<**String**> | Gets or sets the application version. | [optional]
**transcoding_info** | Option<[**models::TranscodingInfo**](TranscodingInfo.md)> | Gets or sets the transcoding info. | [optional]
**is_active** | Option<**bool**> | Gets or sets a value indicating whether this session is active. | [optional]
**supports_media_control** | Option<**bool**> | Gets or sets a value indicating whether the session supports media control. | [optional]
**supports_remote_control** | Option<**bool**> | Gets or sets a value indicating whether the session supports remote control. | [optional]
**now_playing_queue** | Option<[**Vec<models::QueueItem>**](QueueItem.md)> | Gets or sets the now playing queue. | [optional]
**has_custom_device_name** | Option<**bool**> | Gets or sets a value indicating whether this session has a custom device name. | [optional]
**playlist_item_id** | Option<**String**> | Gets or sets the playlist item id. | [optional]
**server_id** | Option<**String**> | Gets or sets the server id. | [optional]
**user_primary_image_tag** | Option<**String**> | Gets or sets the user primary image tag. | [optional]
**supported_commands** | Option<[**Vec<models::GeneralCommandType>**](GeneralCommandType.md)> | Gets or sets the supported commands. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


