# \UserApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user_by_name**](UserApi.md#create_user_by_name) | **POST** /Users/New | Creates a user.
[**delete_user**](UserApi.md#delete_user) | **DELETE** /Users/{userId} | Deletes a user.
[**get_current_user**](UserApi.md#get_current_user) | **GET** /Users/Me | Gets the user based on auth token.
[**get_public_users**](UserApi.md#get_public_users) | **GET** /Users/Public | Gets a list of publicly visible users for display on a login screen.
[**get_user_by_id**](UserApi.md#get_user_by_id) | **GET** /Users/{userId} | Gets a user by Id.
[**get_users**](UserApi.md#get_users) | **GET** /Users | Gets a list of users.
[**update_user**](UserApi.md#update_user) | **POST** /Users | Updates a user.
[**update_user_configuration**](UserApi.md#update_user_configuration) | **POST** /Users/Configuration | Updates a user configuration.
[**update_user_password**](UserApi.md#update_user_password) | **POST** /Users/Password | Updates a user's password.
[**update_user_policy**](UserApi.md#update_user_policy) | **POST** /Users/{userId}/Policy | Updates a user policy.



## create_user_by_name

> models::UserDto create_user_by_name(create_user_by_name)
Creates a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_user_by_name** | [**CreateUserByName**](CreateUserByName.md) | The create user by name request body. | [required] |

### Return type

[**models::UserDto**](UserDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> delete_user(user_id)
Deletes a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | The user id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_user

> models::UserDto get_current_user()
Gets the user based on auth token.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserDto**](UserDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_public_users

> Vec<models::UserDto> get_public_users()
Gets a list of publicly visible users for display on a login screen.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::UserDto>**](UserDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_by_id

> models::UserDto get_user_by_id(user_id)
Gets a user by Id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | The user id. | [required] |

### Return type

[**models::UserDto**](UserDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users

> Vec<models::UserDto> get_users(is_hidden, is_disabled)
Gets a list of users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**is_hidden** | Option<**bool**> | Optional filter by IsHidden=true or false. |  |
**is_disabled** | Option<**bool**> | Optional filter by IsDisabled=true or false. |  |

### Return type

[**Vec<models::UserDto>**](UserDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> update_user(user_dto, user_id)
Updates a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_dto** | [**UserDto**](UserDto.md) | The updated user model. | [required] |
**user_id** | Option<**uuid::Uuid**> | The user id. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_configuration

> update_user_configuration(user_configuration, user_id)
Updates a user configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_configuration** | [**UserConfiguration**](UserConfiguration.md) | The new user configuration. | [required] |
**user_id** | Option<**uuid::Uuid**> | The user id. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_password

> update_user_password(update_user_password, user_id)
Updates a user's password.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_user_password** | [**UpdateUserPassword**](UpdateUserPassword.md) | The M:Jellyfin.Api.Controllers.UserController.UpdateUserPassword(System.Nullable{System.Guid},Jellyfin.Api.Models.UserDtos.UpdateUserPassword) request. | [required] |
**user_id** | Option<**uuid::Uuid**> | The user id. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_policy

> update_user_policy(user_id, user_policy)
Updates a user policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | The user id. | [required] |
**user_policy** | [**UserPolicy**](UserPolicy.md) | The new user policy. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

