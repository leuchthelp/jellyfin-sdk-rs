# \SystemApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_configuration**](SystemApi.md#get_configuration) | **GET** /System/Configuration | Gets application configuration.
[**get_default_metadata_options**](SystemApi.md#get_default_metadata_options) | **GET** /System/Configuration/MetadataOptions/Default | Gets a default MetadataOptions object.
[**get_endpoint_info**](SystemApi.md#get_endpoint_info) | **GET** /System/Endpoint | Gets information about the request endpoint.
[**get_log_entries**](SystemApi.md#get_log_entries) | **GET** /System/ActivityLog/Entries | Gets activity log entries.
[**get_log_file**](SystemApi.md#get_log_file) | **GET** /System/Logs/Log | Gets a log file.
[**get_named_configuration**](SystemApi.md#get_named_configuration) | **GET** /System/Configuration/{key} | Gets a named configuration.
[**get_ping_system**](SystemApi.md#get_ping_system) | **GET** /System/Ping | Pings the system.
[**get_public_system_info**](SystemApi.md#get_public_system_info) | **GET** /System/Info/Public | Gets public information about the server.
[**get_server_logs**](SystemApi.md#get_server_logs) | **GET** /System/Logs | Gets a list of available server log files.
[**get_system_info**](SystemApi.md#get_system_info) | **GET** /System/Info | Gets information about the server.
[**get_system_storage**](SystemApi.md#get_system_storage) | **GET** /System/Info/Storage | Gets information about the server.
[**get_utc_time**](SystemApi.md#get_utc_time) | **GET** /GetUtcTime | Gets the current UTC time.
[**log_file**](SystemApi.md#log_file) | **POST** /ClientLog/Document | Upload a document.
[**post_ping_system**](SystemApi.md#post_ping_system) | **POST** /System/Ping | Pings the system.
[**restart_application**](SystemApi.md#restart_application) | **POST** /System/Restart | Restarts the application.
[**shutdown_application**](SystemApi.md#shutdown_application) | **POST** /System/Shutdown | Shuts down the application.
[**update_branding_configuration**](SystemApi.md#update_branding_configuration) | **POST** /System/Configuration/Branding | Updates branding configuration.
[**update_configuration**](SystemApi.md#update_configuration) | **POST** /System/Configuration | Updates application configuration.
[**update_named_configuration**](SystemApi.md#update_named_configuration) | **POST** /System/Configuration/{key} | Updates named configuration.



## get_configuration

> models::ServerConfiguration get_configuration()
Gets application configuration.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerConfiguration**](ServerConfiguration.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_metadata_options

> models::MetadataOptions get_default_metadata_options()
Gets a default MetadataOptions object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MetadataOptions**](MetadataOptions.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_endpoint_info

> models::EndPointInfo get_endpoint_info()
Gets information about the request endpoint.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::EndPointInfo**](EndPointInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_log_entries

> models::ActivityLogEntryQueryResult get_log_entries(start_index, limit, min_date, max_date, has_user_id, name, overview, short_overview, r#type, item_id, username, severity, sort_by, sort_order)
Gets activity log entries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_index** | Option<**i32**> | The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | The maximum number of records to return. |  |
**min_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The minimum date. |  |
**max_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The maximum date. |  |
**has_user_id** | Option<**bool**> | Filter log entries if it has user id, or not. |  |
**name** | Option<**String**> | Filter by name. |  |
**overview** | Option<**String**> | Filter by overview. |  |
**short_overview** | Option<**String**> | Filter by short overview. |  |
**r#type** | Option<**String**> | Filter by type. |  |
**item_id** | Option<**uuid::Uuid**> | Filter by item id. |  |
**username** | Option<**String**> | Filter by username. |  |
**severity** | Option<**models::LogLevel**> | Filter by log severity. |  |
**sort_by** | Option<[**Vec<models::ActivityLogSortBy>**](Models__ActivityLogSortBy.md)> | Specify one or more sort orders. Format: SortBy=Name,Type. |  |
**sort_order** | Option<[**Vec<models::SortOrder>**](Models__SortOrder.md)> | Sort Order.. |  |

### Return type

[**models::ActivityLogEntryQueryResult**](ActivityLogEntryQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_log_file

> std::path::PathBuf get_log_file(name)
Gets a log file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the log file to get. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_named_configuration

> std::path::PathBuf get_named_configuration(key)
Gets a named configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | Configuration key. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ping_system

> String get_ping_system()
Pings the system.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_public_system_info

> models::PublicSystemInfo get_public_system_info()
Gets public information about the server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PublicSystemInfo**](PublicSystemInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_logs

> Vec<models::LogFile> get_server_logs()
Gets a list of available server log files.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::LogFile>**](LogFile.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_info

> models::SystemInfo get_system_info()
Gets information about the server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SystemInfo**](SystemInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_storage

> models::SystemStorageDto get_system_storage()
Gets information about the server.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SystemStorageDto**](SystemStorageDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_utc_time

> models::UtcTimeResponse get_utc_time()
Gets the current UTC time.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UtcTimeResponse**](UtcTimeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## log_file

> models::ClientLogDocumentResponseDto log_file(body)
Upload a document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**models::ClientLogDocumentResponseDto**](ClientLogDocumentResponseDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: text/plain
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_ping_system

> String post_ping_system()
Pings the system.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restart_application

> restart_application()
Restarts the application.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shutdown_application

> shutdown_application()
Shuts down the application.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_branding_configuration

> update_branding_configuration(branding_options_dto)
Updates branding configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**branding_options_dto** | [**BrandingOptionsDto**](BrandingOptionsDto.md) | Branding configuration. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_configuration

> update_configuration(server_configuration)
Updates application configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_configuration** | [**ServerConfiguration**](ServerConfiguration.md) | Configuration. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_named_configuration

> update_named_configuration(key, body)
Updates named configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | Configuration key. | [required] |
**body** | Option<**serde_json::Value**> | Configuration. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

