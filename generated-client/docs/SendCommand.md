# SendCommand

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group_id** | Option<**uuid::Uuid**> | Gets the group identifier. | [optional]
**playlist_item_id** | Option<**uuid::Uuid**> | Gets the playlist identifier of the playing item. | [optional]
**when** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Gets or sets the UTC time when to execute the command. | [optional]
**position_ticks** | Option<**i64**> | Gets the position ticks. | [optional]
**command** | Option<[**models::SendCommandType**](SendCommandType.md)> | Gets the command. | [optional]
**emitted_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Gets the UTC time when this command has been emitted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


