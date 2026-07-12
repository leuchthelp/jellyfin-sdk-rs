# \AuthenticationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**authenticate_user_by_name**](AuthenticationApi.md#authenticate_user_by_name) | **POST** /Users/AuthenticateByName | Authenticates a user by name.
[**authenticate_with_quick_connect**](AuthenticationApi.md#authenticate_with_quick_connect) | **POST** /Users/AuthenticateWithQuickConnect | Authenticates a user with quick connect.
[**authorize_quick_connect**](AuthenticationApi.md#authorize_quick_connect) | **POST** /QuickConnect/Authorize | Authorizes a pending quick connect request.
[**create_key**](AuthenticationApi.md#create_key) | **POST** /Auth/Keys | Create a new api key.
[**forgot_password**](AuthenticationApi.md#forgot_password) | **POST** /Users/ForgotPassword | Initiates the forgot password process for a local user.
[**forgot_password_pin**](AuthenticationApi.md#forgot_password_pin) | **POST** /Users/ForgotPassword/Pin | Redeems a forgot password pin.
[**get_auth_providers**](AuthenticationApi.md#get_auth_providers) | **GET** /Auth/Providers | Get all auth providers.
[**get_keys**](AuthenticationApi.md#get_keys) | **GET** /Auth/Keys | Get all keys.
[**get_password_reset_providers**](AuthenticationApi.md#get_password_reset_providers) | **GET** /Auth/PasswordResetProviders | Get all password reset providers.
[**get_quick_connect_enabled**](AuthenticationApi.md#get_quick_connect_enabled) | **GET** /QuickConnect/Enabled | Gets the current quick connect state.
[**get_quick_connect_state**](AuthenticationApi.md#get_quick_connect_state) | **GET** /QuickConnect/Connect | Attempts to retrieve authentication information.
[**initiate_quick_connect**](AuthenticationApi.md#initiate_quick_connect) | **POST** /QuickConnect/Initiate | Initiate a new quick connect request.
[**revoke_key**](AuthenticationApi.md#revoke_key) | **DELETE** /Auth/Keys/{key} | Remove an api key.



## authenticate_user_by_name

> models::AuthenticationResult authenticate_user_by_name(authenticate_user_by_name)
Authenticates a user by name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authenticate_user_by_name** | [**AuthenticateUserByName**](AuthenticateUserByName.md) | The M:Jellyfin.Api.Controllers.UserController.AuthenticateUserByName(Jellyfin.Api.Models.UserDtos.AuthenticateUserByName) request. | [required] |

### Return type

[**models::AuthenticationResult**](AuthenticationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## authenticate_with_quick_connect

> models::AuthenticationResult authenticate_with_quick_connect(quick_connect_dto)
Authenticates a user with quick connect.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quick_connect_dto** | [**QuickConnectDto**](QuickConnectDto.md) | The Jellyfin.Api.Models.UserDtos.QuickConnectDto request. | [required] |

### Return type

[**models::AuthenticationResult**](AuthenticationResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## authorize_quick_connect

> bool authorize_quick_connect(code, user_id)
Authorizes a pending quick connect request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Quick connect code to authorize. | [required] |
**user_id** | Option<**uuid::Uuid**> | The user the authorize. Access to the requested user is required. |  |

### Return type

**bool**

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_key

> create_key(app)
Create a new api key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app** | **String** | Name of the app using the authentication key. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forgot_password

> models::ForgotPasswordResult forgot_password(forgot_password_dto)
Initiates the forgot password process for a local user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**forgot_password_dto** | [**ForgotPasswordDto**](ForgotPasswordDto.md) | The forgot password request containing the entered username. | [required] |

### Return type

[**models::ForgotPasswordResult**](ForgotPasswordResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## forgot_password_pin

> models::PinRedeemResult forgot_password_pin(forgot_password_pin_dto)
Redeems a forgot password pin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**forgot_password_pin_dto** | [**ForgotPasswordPinDto**](ForgotPasswordPinDto.md) | The forgot password pin request containing the entered pin. | [required] |

### Return type

[**models::PinRedeemResult**](PinRedeemResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_auth_providers

> Vec<models::NameIdPair> get_auth_providers()
Get all auth providers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::NameIdPair>**](NameIdPair.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_keys

> models::AuthenticationInfoQueryResult get_keys()
Get all keys.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AuthenticationInfoQueryResult**](AuthenticationInfoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_password_reset_providers

> Vec<models::NameIdPair> get_password_reset_providers()
Get all password reset providers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::NameIdPair>**](NameIdPair.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quick_connect_enabled

> bool get_quick_connect_enabled()
Gets the current quick connect state.

### Parameters

This endpoint does not need any parameter.

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quick_connect_state

> models::QuickConnectResult get_quick_connect_state(secret)
Attempts to retrieve authentication information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret** | **String** | Secret previously returned from the Initiate endpoint. | [required] |

### Return type

[**models::QuickConnectResult**](QuickConnectResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## initiate_quick_connect

> models::QuickConnectResult initiate_quick_connect()
Initiate a new quick connect request.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::QuickConnectResult**](QuickConnectResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_key

> revoke_key(key)
Remove an api key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The access token to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

