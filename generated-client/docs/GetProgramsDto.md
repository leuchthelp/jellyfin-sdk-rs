# GetProgramsDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**channel_ids** | Option<**Vec<uuid::Uuid>**> | Gets or sets the channels to return guide information for. | [optional]
**user_id** | Option<**uuid::Uuid**> | Gets or sets optional. Filter by user id. | [optional]
**min_start_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Gets or sets the minimum premiere start date. | [optional]
**has_aired** | Option<**bool**> | Gets or sets filter by programs that have completed airing, or not. | [optional]
**is_airing** | Option<**bool**> | Gets or sets filter by programs that are currently airing, or not. | [optional]
**max_start_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Gets or sets the maximum premiere start date. | [optional]
**min_end_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Gets or sets the minimum premiere end date. | [optional]
**max_end_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Gets or sets the maximum premiere end date. | [optional]
**is_movie** | Option<**bool**> | Gets or sets filter for movies. | [optional]
**is_series** | Option<**bool**> | Gets or sets filter for series. | [optional]
**is_news** | Option<**bool**> | Gets or sets filter for news. | [optional]
**is_kids** | Option<**bool**> | Gets or sets filter for kids. | [optional]
**is_sports** | Option<**bool**> | Gets or sets filter for sports. | [optional]
**start_index** | Option<**i32**> | Gets or sets the record index to start at. All items with a lower index will be dropped from the results. | [optional]
**limit** | Option<**i32**> | Gets or sets the maximum number of records to return. | [optional]
**sort_by** | Option<[**Vec<models::ItemSortBy>**](ItemSortBy.md)> | Gets or sets specify one or more sort orders, comma delimited. Options: Name, StartDate. | [optional]
**sort_order** | Option<[**Vec<models::SortOrder>**](SortOrder.md)> | Gets or sets sort order. | [optional]
**genres** | Option<**Vec<String>**> | Gets or sets the genres to return guide information for. | [optional]
**genre_ids** | Option<**Vec<uuid::Uuid>**> | Gets or sets the genre ids to return guide information for. | [optional]
**enable_images** | Option<**bool**> | Gets or sets include image information in output. | [optional]
**enable_total_record_count** | Option<**bool**> | Gets or sets a value indicating whether retrieve total record count. | [optional][default to true]
**image_type_limit** | Option<**i32**> | Gets or sets the max number of images to return, per image type. | [optional]
**enable_image_types** | Option<[**Vec<models::ImageType>**](ImageType.md)> | Gets or sets the image types to include in the output. | [optional]
**enable_user_data** | Option<**bool**> | Gets or sets include user data. | [optional]
**series_timer_id** | Option<**String**> | Gets or sets filter by series timer id. | [optional]
**library_series_id** | Option<**uuid::Uuid**> | Gets or sets filter by library series id. | [optional]
**fields** | Option<[**Vec<models::ItemFields>**](ItemFields.md)> | Gets or sets specify additional fields of information to return in the output. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


