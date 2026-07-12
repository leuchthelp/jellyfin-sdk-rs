# \PersonApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_person**](PersonApi.md#get_person) | **GET** /Persons/{name} | Get person by name.
[**get_persons**](PersonApi.md#get_persons) | **GET** /Persons | Gets all persons.



## get_person

> models::BaseItemDto get_person(name, user_id)
Get person by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Person name. | [required] |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |

### Return type

[**models::BaseItemDto**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_persons

> models::BaseItemDtoQueryResult get_persons(start_index, limit, search_term, name_starts_with, name_less_than, name_starts_with_or_greater, fields, filters, is_favorite, enable_user_data, image_type_limit, enable_image_types, exclude_person_types, person_types, parent_id, appears_in_item_id, user_id, enable_images)
Gets all persons.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_index** | Option<**i32**> | Optional. All items with a lower index will be dropped from the response. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**search_term** | Option<**String**> | The search term. |  |
**name_starts_with** | Option<**String**> | Optional. Filter by items whose name starts with the given input string. |  |
**name_less_than** | Option<**String**> | Optional. Filter by items whose name will appear before this value when sorted alphabetically. |  |
**name_starts_with_or_greater** | Option<**String**> | Optional. Filter by items whose name will appear after this value when sorted alphabetically. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**filters** | Option<[**Vec<models::ItemFilter>**](Models__ItemFilter.md)> | Optional. Specify additional filters to apply. |  |
**is_favorite** | Option<**bool**> | Optional filter by items that are marked as favorite, or not. userId is required. |  |
**enable_user_data** | Option<**bool**> | Optional, include user data. |  |
**image_type_limit** | Option<**i32**> | Optional, the max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<models::ImageType>**](Models__ImageType.md)> | Optional. The image types to include in the output. |  |
**exclude_person_types** | Option<[**Vec<String>**](String.md)> | Optional. If specified results will be filtered to exclude those containing the specified PersonType. Allows multiple, comma-delimited. |  |
**person_types** | Option<[**Vec<String>**](String.md)> | Optional. If specified results will be filtered to include only those containing the specified PersonType. Allows multiple, comma-delimited. |  |
**parent_id** | Option<**uuid::Uuid**> | Optional. Specify this to localize the search to a specific library. Omit to use the root. |  |
**appears_in_item_id** | Option<**uuid::Uuid**> | Optional. If specified, person results will be filtered on items related to said persons. |  |
**user_id** | Option<**uuid::Uuid**> | User id. |  |
**enable_images** | Option<**bool**> | Optional, include image information in output. |  |[default to true]

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

