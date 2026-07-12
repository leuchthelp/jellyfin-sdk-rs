# \SuggestionApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_suggestions**](SuggestionApi.md#get_suggestions) | **GET** /Items/Suggestions | Gets suggestions.



## get_suggestions

> models::BaseItemDtoQueryResult get_suggestions(user_id, media_type, r#type, start_index, limit, enable_total_record_count)
Gets suggestions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**uuid::Uuid**> | The user id. |  |
**media_type** | Option<[**Vec<models::MediaType>**](Models__MediaType.md)> | The media types. |  |
**r#type** | Option<[**Vec<models::BaseItemKind>**](Models__BaseItemKind.md)> | The type. |  |
**start_index** | Option<**i32**> | Optional. The start index. |  |
**limit** | Option<**i32**> | Optional. The limit. |  |
**enable_total_record_count** | Option<**bool**> | Whether to enable the total record count. |  |[default to false]

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

