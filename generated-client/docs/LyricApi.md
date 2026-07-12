# \LyricApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lyrics**](LyricApi.md#delete_lyrics) | **DELETE** /Audio/{itemId}/Lyrics | Deletes an external lyric file.
[**download_remote_lyrics**](LyricApi.md#download_remote_lyrics) | **POST** /Audio/{itemId}/RemoteSearch/Lyrics/{lyricId} | Downloads a remote lyric.
[**get_lyrics**](LyricApi.md#get_lyrics) | **GET** /Audio/{itemId}/Lyrics | Gets an item's lyrics.
[**get_remote_lyrics**](LyricApi.md#get_remote_lyrics) | **GET** /Providers/Lyrics/{lyricId} | Gets the remote lyrics.
[**search_remote_lyrics**](LyricApi.md#search_remote_lyrics) | **GET** /Audio/{itemId}/RemoteSearch/Lyrics | Search remote lyrics.
[**upload_lyrics**](LyricApi.md#upload_lyrics) | **POST** /Audio/{itemId}/Lyrics | Upload an external lyric file.



## delete_lyrics

> delete_lyrics(item_id)
Deletes an external lyric file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_remote_lyrics

> models::LyricDto download_remote_lyrics(item_id, lyric_id)
Downloads a remote lyric.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**lyric_id** | **String** | The lyric id. | [required] |

### Return type

[**models::LyricDto**](LyricDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lyrics

> models::LyricDto get_lyrics(item_id)
Gets an item's lyrics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item id. | [required] |

### Return type

[**models::LyricDto**](LyricDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_remote_lyrics

> models::LyricDto get_remote_lyrics(lyric_id)
Gets the remote lyrics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lyric_id** | **String** | The remote provider item id. | [required] |

### Return type

[**models::LyricDto**](LyricDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_remote_lyrics

> Vec<models::RemoteLyricInfoDto> search_remote_lyrics(item_id)
Search remote lyrics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |

### Return type

[**Vec<models::RemoteLyricInfoDto>**](RemoteLyricInfoDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_lyrics

> models::LyricDto upload_lyrics(item_id, file_name, body)
Upload an external lyric file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item the lyric belongs to. | [required] |
**file_name** | **String** | Name of the file being uploaded. | [required] |
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**models::LyricDto**](LyricDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

