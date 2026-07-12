# \ChannelApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_channel_features**](ChannelApi.md#get_all_channel_features) | **GET** /Channels/Features | Get all channel features.
[**get_channel_features**](ChannelApi.md#get_channel_features) | **GET** /Channels/{channelId}/Features | Get channel features.
[**get_channel_items**](ChannelApi.md#get_channel_items) | **GET** /Channels/{channelId}/Items | Get channel items.
[**get_channels**](ChannelApi.md#get_channels) | **GET** /Channels | Gets available channels.
[**get_latest_channel_items**](ChannelApi.md#get_latest_channel_items) | **GET** /Channels/Items/Latest | Gets latest channel items.



## get_all_channel_features

> Vec<models::ChannelFeatures> get_all_channel_features()
Get all channel features.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ChannelFeatures>**](ChannelFeatures.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel_features

> models::ChannelFeatures get_channel_features(channel_id)
Get channel features.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **uuid::Uuid** | Channel id. | [required] |

### Return type

[**models::ChannelFeatures**](ChannelFeatures.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel_items

> models::BaseItemDtoQueryResult get_channel_items(channel_id, folder_id, user_id, start_index, limit, sort_order, filters, sort_by, fields)
Get channel items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **uuid::Uuid** | Channel Id. | [required] |
**folder_id** | Option<**uuid::Uuid**> | Optional. Folder Id. |  |
**user_id** | Option<**uuid::Uuid**> | Optional. User Id. |  |
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**sort_order** | Option<[**Vec<models::SortOrder>**](Models__SortOrder.md)> | Optional. Sort Order - Ascending,Descending. |  |
**filters** | Option<[**Vec<models::ItemFilter>**](Models__ItemFilter.md)> | Optional. Specify additional filters to apply. |  |
**sort_by** | Option<[**Vec<models::ItemSortBy>**](Models__ItemSortBy.md)> | Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channels

> models::BaseItemDtoQueryResult get_channels(user_id, start_index, limit, supports_latest_items, supports_media_deletion, is_favorite)
Gets available channels.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**uuid::Uuid**> | User Id to filter by. Use System.Guid.Empty to not filter by user. |  |
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**supports_latest_items** | Option<**bool**> | Optional. Filter by channels that support getting latest items. |  |
**supports_media_deletion** | Option<**bool**> | Optional. Filter by channels that support media deletion. |  |
**is_favorite** | Option<**bool**> | Optional. Filter by channels that are favorite. |  |

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_latest_channel_items

> models::BaseItemDtoQueryResult get_latest_channel_items(user_id, start_index, limit, filters, fields, channel_ids)
Gets latest channel items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**uuid::Uuid**> | Optional. User Id. |  |
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**filters** | Option<[**Vec<models::ItemFilter>**](Models__ItemFilter.md)> | Optional. Specify additional filters to apply. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**channel_ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Optional. Specify one or more channel id's, comma delimited. |  |

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

