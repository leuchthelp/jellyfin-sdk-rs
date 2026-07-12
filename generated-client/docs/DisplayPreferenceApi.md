# \DisplayPreferenceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_display_preferences**](DisplayPreferenceApi.md#get_display_preferences) | **GET** /DisplayPreferences/{displayPreferencesId} | Get Display Preferences.
[**update_display_preferences**](DisplayPreferenceApi.md#update_display_preferences) | **POST** /DisplayPreferences/{displayPreferencesId} | Update Display Preferences.



## get_display_preferences

> models::DisplayPreferencesDto get_display_preferences(display_preferences_id, client, user_id)
Get Display Preferences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**display_preferences_id** | **String** | Display preferences id. | [required] |
**client** | **String** | Client. | [required] |
**user_id** | Option<**uuid::Uuid**> | User id. |  |

### Return type

[**models::DisplayPreferencesDto**](DisplayPreferencesDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_display_preferences

> update_display_preferences(display_preferences_id, client, display_preferences_dto, user_id)
Update Display Preferences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**display_preferences_id** | **String** | Display preferences id. | [required] |
**client** | **String** | Client. | [required] |
**display_preferences_dto** | [**DisplayPreferencesDto**](DisplayPreferencesDto.md) | New Display Preferences object. | [required] |
**user_id** | Option<**uuid::Uuid**> | User Id. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

