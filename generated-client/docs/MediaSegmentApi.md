# \MediaSegmentApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_item_segments**](MediaSegmentApi.md#get_item_segments) | **GET** /MediaSegments/{itemId} | Gets all media segments based on an itemId.



## get_item_segments

> models::MediaSegmentDtoQueryResult get_item_segments(item_id, include_segment_types)
Gets all media segments based on an itemId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The ItemId. | [required] |
**include_segment_types** | Option<[**Vec<models::MediaSegmentType>**](Models__MediaSegmentType.md)> | Optional filter of requested segment types. |  |

### Return type

[**models::MediaSegmentDtoQueryResult**](MediaSegmentDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

