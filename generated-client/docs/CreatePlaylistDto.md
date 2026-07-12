# CreatePlaylistDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Gets or sets the name of the new playlist. | 
**ids** | Option<**Vec<uuid::Uuid>**> | Gets or sets item ids to add to the playlist. | [optional]
**user_id** | Option<**uuid::Uuid**> | Gets or sets the user id. | [optional]
**media_type** | Option<[**models::MediaType**](MediaType.md)> | Gets or sets the media type. | [optional]
**users** | Option<[**Vec<models::PlaylistUserPermissions>**](PlaylistUserPermissions.md)> | Gets or sets the playlist users. | [optional]
**is_public** | Option<**bool**> | Gets or sets a value indicating whether the playlist is public. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


