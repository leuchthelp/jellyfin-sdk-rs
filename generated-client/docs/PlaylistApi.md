# \PlaylistApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_item_to_playlist**](PlaylistApi.md#add_item_to_playlist) | **POST** /Playlists/{playlistId}/Items | Adds items to a playlist.
[**create_playlist**](PlaylistApi.md#create_playlist) | **POST** /Playlists | Creates a new playlist.
[**get_playlist**](PlaylistApi.md#get_playlist) | **GET** /Playlists/{playlistId} | Get a playlist.
[**get_playlist_items**](PlaylistApi.md#get_playlist_items) | **GET** /Playlists/{playlistId}/Items | Gets the original items of a playlist.
[**get_playlist_user**](PlaylistApi.md#get_playlist_user) | **GET** /Playlists/{playlistId}/Users/{userId} | Get a playlist user.
[**get_playlist_users**](PlaylistApi.md#get_playlist_users) | **GET** /Playlists/{playlistId}/Users | Get a playlist's users.
[**move_item**](PlaylistApi.md#move_item) | **POST** /Playlists/{playlistId}/Items/{itemId}/Move/{newIndex} | Moves a playlist item.
[**remove_item_from_playlist**](PlaylistApi.md#remove_item_from_playlist) | **DELETE** /Playlists/{playlistId}/Items | Removes items from a playlist.
[**remove_user_from_playlist**](PlaylistApi.md#remove_user_from_playlist) | **DELETE** /Playlists/{playlistId}/Users/{userId} | Remove a user from a playlist's users.
[**update_playlist**](PlaylistApi.md#update_playlist) | **POST** /Playlists/{playlistId} | Updates a playlist.
[**update_playlist_user**](PlaylistApi.md#update_playlist_user) | **POST** /Playlists/{playlistId}/Users/{userId} | Modify a user of a playlist's users.



## add_item_to_playlist

> add_item_to_playlist(playlist_id, ids, position, user_id)
Adds items to a playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**playlist_id** | **uuid::Uuid** | The playlist id. | [required] |
**ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Item id, comma delimited. |  |
**position** | Option<**i32**> | Optional. 0-based index where to place the items or at the end if `null`. |  |
**user_id** | Option<**uuid::Uuid**> | The userId. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_playlist

> models::PlaylistCreationResult create_playlist(name, ids, user_id, media_type, create_playlist_dto)
Creates a new playlist.

For backwards compatibility parameters can be sent via Query or Body, with Query having higher precedence. Query parameters are obsolete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | The playlist name. |  |
**ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | The item ids. |  |
**user_id** | Option<**uuid::Uuid**> | The user id. |  |
**media_type** | Option<**models::MediaType**> | The media type. |  |
**create_playlist_dto** | Option<[**CreatePlaylistDto**](CreatePlaylistDto.md)> | The create playlist payload. |  |

### Return type

[**models::PlaylistCreationResult**](PlaylistCreationResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_playlist

> models::PlaylistDto get_playlist(playlist_id)
Get a playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**playlist_id** | **uuid::Uuid** | The playlist id. | [required] |

### Return type

[**models::PlaylistDto**](PlaylistDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_playlist_items

> models::BaseItemDtoQueryResult get_playlist_items(playlist_id, user_id, start_index, limit, fields, enable_images, enable_user_data, image_type_limit, enable_image_types)
Gets the original items of a playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**playlist_id** | **uuid::Uuid** | The playlist id. | [required] |
**user_id** | Option<**uuid::Uuid**> | User id. |  |
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**enable_images** | Option<**bool**> | Optional. Include image information in output. |  |
**enable_user_data** | Option<**bool**> | Optional. Include user data. |  |
**image_type_limit** | Option<**i32**> | Optional. The max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<models::ImageType>**](Models__ImageType.md)> | Optional. The image types to include in the output. |  |

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_playlist_user

> models::PlaylistUserPermissions get_playlist_user(playlist_id, user_id)
Get a playlist user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**playlist_id** | **uuid::Uuid** | The playlist id. | [required] |
**user_id** | **uuid::Uuid** | The user id. | [required] |

### Return type

[**models::PlaylistUserPermissions**](PlaylistUserPermissions.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_playlist_users

> Vec<models::PlaylistUserPermissions> get_playlist_users(playlist_id)
Get a playlist's users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**playlist_id** | **uuid::Uuid** | The playlist id. | [required] |

### Return type

[**Vec<models::PlaylistUserPermissions>**](PlaylistUserPermissions.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_item

> move_item(playlist_id, item_id, new_index)
Moves a playlist item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**playlist_id** | **String** | The playlist id. | [required] |
**item_id** | **String** | The item id. | [required] |
**new_index** | **i32** | The new index. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_item_from_playlist

> remove_item_from_playlist(playlist_id, entry_ids)
Removes items from a playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**playlist_id** | **String** | The playlist id. | [required] |
**entry_ids** | Option<[**Vec<String>**](String.md)> | The item ids, comma delimited. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_from_playlist

> remove_user_from_playlist(playlist_id, user_id)
Remove a user from a playlist's users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**playlist_id** | **uuid::Uuid** | The playlist id. | [required] |
**user_id** | **uuid::Uuid** | The user id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_playlist

> update_playlist(playlist_id, update_playlist_dto)
Updates a playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**playlist_id** | **uuid::Uuid** | The playlist id. | [required] |
**update_playlist_dto** | [**UpdatePlaylistDto**](UpdatePlaylistDto.md) | The Jellyfin.Api.Models.PlaylistDtos.UpdatePlaylistDto id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_playlist_user

> update_playlist_user(playlist_id, user_id, update_playlist_user_dto)
Modify a user of a playlist's users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**playlist_id** | **uuid::Uuid** | The playlist id. | [required] |
**user_id** | **uuid::Uuid** | The user id. | [required] |
**update_playlist_user_dto** | [**UpdatePlaylistUserDto**](UpdatePlaylistUserDto.md) | The Jellyfin.Api.Models.PlaylistDtos.UpdatePlaylistUserDto. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

