# PlayQueueUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reason** | Option<[**models::PlayQueueUpdateReason**](PlayQueueUpdateReason.md)> | Gets the request type that originated this update. | [optional]
**last_update** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Gets the UTC time of the last change to the playing queue. | [optional]
**playlist** | Option<[**Vec<models::SyncPlayQueueItem>**](SyncPlayQueueItem.md)> | Gets the playlist. | [optional]
**playing_item_index** | Option<**i32**> | Gets the playing item index in the playlist. | [optional]
**start_position_ticks** | Option<**i64**> | Gets the start position ticks. | [optional]
**is_playing** | Option<**bool**> | Gets a value indicating whether the current item is playing. | [optional]
**shuffle_mode** | Option<[**models::GroupShuffleMode**](GroupShuffleMode.md)> | Gets the shuffle mode. | [optional]
**repeat_mode** | Option<[**models::GroupRepeatMode**](GroupRepeatMode.md)> | Gets the repeat mode. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


