# MediaSegmentDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**uuid::Uuid**> | Gets or sets the id of the media segment. | [optional]
**item_id** | Option<**uuid::Uuid**> | Gets or sets the id of the associated item. | [optional]
**r#type** | Option<[**models::MediaSegmentType**](MediaSegmentType.md)> | Defines the types of content an individual Jellyfin.Database.Implementations.Entities.MediaSegment represents. | [optional][default to Unknown]
**start_ticks** | Option<**i64**> | Gets or sets the start of the segment. | [optional]
**end_ticks** | Option<**i64**> | Gets or sets the end of the segment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


