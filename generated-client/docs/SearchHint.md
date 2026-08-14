# SearchHint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_id** | Option<**uuid::Uuid**> | Gets or sets the item id. | [optional]
**id** | Option<**uuid::Uuid**> | Gets or sets the item id. | [optional]
**name** | Option<**String**> | Gets or sets the name. | [optional]
**matched_term** | Option<**String**> | Gets or sets the matched term. | [optional]
**index_number** | Option<**i32**> | Gets or sets the index number. | [optional]
**production_year** | Option<**i32**> | Gets or sets the production year. | [optional]
**parent_index_number** | Option<**i32**> | Gets or sets the parent index number. | [optional]
**primary_image_tag** | Option<**String**> | Gets or sets the image tag. | [optional]
**thumb_image_tag** | Option<**String**> | Gets or sets the thumb image tag. | [optional]
**thumb_image_item_id** | Option<**String**> | Gets or sets the thumb image item identifier. | [optional]
**backdrop_image_tag** | Option<**String**> | Gets or sets the backdrop image tag. | [optional]
**backdrop_image_item_id** | Option<**String**> | Gets or sets the backdrop image item identifier. | [optional]
**r#type** | Option<[**models::BaseItemKind**](BaseItemKind.md)> | Gets or sets the type. | [optional]
**is_folder** | Option<**bool**> | Gets or sets a value indicating whether this instance is folder. | [optional]
**run_time_ticks** | Option<**i64**> | Gets or sets the run time ticks. | [optional]
**media_type** | Option<[**models::MediaType**](MediaType.md)> | Gets or sets the type of the media. | [optional][default to Unknown]
**start_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Gets or sets the start date. | [optional]
**end_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Gets or sets the end date. | [optional]
**series** | Option<**String**> | Gets or sets the series. | [optional]
**status** | Option<**String**> | Gets or sets the status. | [optional]
**album** | Option<**String**> | Gets or sets the album. | [optional]
**album_id** | Option<**uuid::Uuid**> | Gets or sets the album id. | [optional]
**album_artist** | Option<**String**> | Gets or sets the album artist. | [optional]
**artists** | Option<**Vec<String>**> | Gets or sets the artists. | [optional]
**song_count** | Option<**i32**> | Gets or sets the song count. | [optional]
**episode_count** | Option<**i32**> | Gets or sets the episode count. | [optional]
**channel_id** | Option<**uuid::Uuid**> | Gets or sets the channel identifier. | [optional]
**channel_name** | Option<**String**> | Gets or sets the name of the channel. | [optional]
**primary_image_aspect_ratio** | Option<**f64**> | Gets or sets the primary image aspect ratio. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


