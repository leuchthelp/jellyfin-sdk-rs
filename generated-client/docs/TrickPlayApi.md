# \TrickPlayApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_trickplay_hls_playlist**](TrickPlayApi.md#get_trickplay_hls_playlist) | **GET** /Videos/{itemId}/Trickplay/{width}/tiles.m3u8 | Gets an image tiles playlist for trickplay.
[**get_trickplay_tile_image**](TrickPlayApi.md#get_trickplay_tile_image) | **GET** /Videos/{itemId}/Trickplay/{width}/{index}.jpg | Gets a trickplay tile image.



## get_trickplay_hls_playlist

> std::path::PathBuf get_trickplay_hls_playlist(item_id, width, media_source_id)
Gets an image tiles playlist for trickplay.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**width** | **i32** | The width of a single tile. | [required] |
**media_source_id** | Option<**uuid::Uuid**> | The media version id, if using an alternate version. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-mpegURL, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trickplay_tile_image

> std::path::PathBuf get_trickplay_tile_image(item_id, width, index, media_source_id)
Gets a trickplay tile image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**width** | **i32** | The width of a single tile. | [required] |
**index** | **i32** | The index of the desired tile. | [required] |
**media_source_id** | Option<**uuid::Uuid**> | The media version id, if using an alternate version. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

