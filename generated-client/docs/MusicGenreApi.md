# \MusicGenreApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_music_genre**](MusicGenreApi.md#get_music_genre) | **GET** /MusicGenres/{genreName} | Gets a music genre, by name.



## get_music_genre

> models::BaseItemDto get_music_genre(genre_name, user_id)
Gets a music genre, by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**genre_name** | **String** | The genre name. | [required] |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |

### Return type

[**models::BaseItemDto**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

