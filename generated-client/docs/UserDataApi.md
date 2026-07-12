# \UserDataApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_user_item_rating**](UserDataApi.md#delete_user_item_rating) | **DELETE** /UserItems/{itemId}/Rating | Deletes a user's saved personal rating for an item.
[**get_item_user_data**](UserDataApi.md#get_item_user_data) | **GET** /UserItems/{itemId}/UserData | Get Item User Data.
[**mark_favorite_item**](UserDataApi.md#mark_favorite_item) | **POST** /UserFavoriteItems/{itemId} | Marks an item as a favorite.
[**mark_played_item**](UserDataApi.md#mark_played_item) | **POST** /UserPlayedItems/{itemId} | Marks an item as played for user.
[**mark_unplayed_item**](UserDataApi.md#mark_unplayed_item) | **DELETE** /UserPlayedItems/{itemId} | Marks an item as unplayed for user.
[**unmark_favorite_item**](UserDataApi.md#unmark_favorite_item) | **DELETE** /UserFavoriteItems/{itemId} | Unmarks item as a favorite.
[**update_item_user_data**](UserDataApi.md#update_item_user_data) | **POST** /UserItems/{itemId}/UserData | Update Item User Data.
[**update_user_item_rating**](UserDataApi.md#update_user_item_rating) | **POST** /UserItems/{itemId}/Rating | Updates a user's rating for an item.



## delete_user_item_rating

> models::UserItemDataDto delete_user_item_rating(item_id, user_id)
Deletes a user's saved personal rating for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | User id. |  |

### Return type

[**models::UserItemDataDto**](UserItemDataDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item_user_data

> models::UserItemDataDto get_item_user_data(item_id, user_id)
Get Item User Data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | The user id. |  |

### Return type

[**models::UserItemDataDto**](UserItemDataDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_favorite_item

> models::UserItemDataDto mark_favorite_item(item_id, user_id)
Marks an item as a favorite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | User id. |  |

### Return type

[**models::UserItemDataDto**](UserItemDataDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_played_item

> models::UserItemDataDto mark_played_item(item_id, user_id, date_played)
Marks an item as played for user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | User id. |  |
**date_played** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Optional. The date the item was played. |  |

### Return type

[**models::UserItemDataDto**](UserItemDataDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_unplayed_item

> models::UserItemDataDto mark_unplayed_item(item_id, user_id)
Marks an item as unplayed for user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | User id. |  |

### Return type

[**models::UserItemDataDto**](UserItemDataDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unmark_favorite_item

> models::UserItemDataDto unmark_favorite_item(item_id, user_id)
Unmarks item as a favorite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | User id. |  |

### Return type

[**models::UserItemDataDto**](UserItemDataDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_item_user_data

> models::UserItemDataDto update_item_user_data(item_id, update_user_item_data_dto, user_id)
Update Item User Data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**update_user_item_data_dto** | [**UpdateUserItemDataDto**](UpdateUserItemDataDto.md) | New user data object. | [required] |
**user_id** | Option<**uuid::Uuid**> | The user id. |  |

### Return type

[**models::UserItemDataDto**](UserItemDataDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_item_rating

> models::UserItemDataDto update_user_item_rating(item_id, user_id, likes)
Updates a user's rating for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | User id. |  |
**likes** | Option<**bool**> | Whether this M:Jellyfin.Api.Controllers.UserLibraryController.UpdateUserItemRating(System.Nullable{System.Guid},System.Guid,System.Nullable{System.Boolean}) is likes. |  |

### Return type

[**models::UserItemDataDto**](UserItemDataDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

