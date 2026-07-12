# \YearApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_year**](YearApi.md#get_year) | **GET** /Years/{year} | Gets a year.
[**get_years**](YearApi.md#get_years) | **GET** /Years | Get years.



## get_year

> models::BaseItemDto get_year(year, user_id)
Gets a year.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | The year. | [required] |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |

### Return type

[**models::BaseItemDto**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_years

> models::BaseItemDtoQueryResult get_years(start_index, limit, sort_order, parent_id, fields, exclude_item_types, include_item_types, media_types, sort_by, enable_user_data, image_type_limit, enable_image_types, user_id, recursive, enable_images)
Get years.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_index** | Option<**i32**> | Skips over a given number of items within the results. Use for paging. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**sort_order** | Option<[**Vec<models::SortOrder>**](Models__SortOrder.md)> | Sort Order - Ascending,Descending. |  |
**parent_id** | Option<**uuid::Uuid**> | Specify this to localize the search to a specific item or folder. Omit to use the root. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**exclude_item_types** | Option<[**Vec<models::BaseItemKind>**](Models__BaseItemKind.md)> | Optional. If specified, results will be excluded based on item type. This allows multiple, comma delimited. |  |
**include_item_types** | Option<[**Vec<models::BaseItemKind>**](Models__BaseItemKind.md)> | Optional. If specified, results will be included based on item type. This allows multiple, comma delimited. |  |
**media_types** | Option<[**Vec<models::MediaType>**](Models__MediaType.md)> | Optional. Filter by MediaType. Allows multiple, comma delimited. |  |
**sort_by** | Option<[**Vec<models::ItemSortBy>**](Models__ItemSortBy.md)> | Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime. |  |
**enable_user_data** | Option<**bool**> | Optional. Include user data. |  |
**image_type_limit** | Option<**i32**> | Optional. The max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<models::ImageType>**](Models__ImageType.md)> | Optional. The image types to include in the output. |  |
**user_id** | Option<**uuid::Uuid**> | User Id. |  |
**recursive** | Option<**bool**> | Search recursively. |  |[default to true]
**enable_images** | Option<**bool**> | Optional. Include image information in output. |  |[default to true]

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

