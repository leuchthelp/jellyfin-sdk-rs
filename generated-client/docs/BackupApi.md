# \BackupApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_backup**](BackupApi.md#create_backup) | **POST** /Backup/Create | Creates a new Backup.
[**get_backup**](BackupApi.md#get_backup) | **GET** /Backup/Manifest | Gets the descriptor from an existing archive is present.
[**list_backups**](BackupApi.md#list_backups) | **GET** /Backup | Gets a list of all currently present backups in the backup directory.
[**start_restore_backup**](BackupApi.md#start_restore_backup) | **POST** /Backup/Restore | Restores to a backup by restarting the server and applying the backup.



## create_backup

> models::BackupManifestDto create_backup(backup_options_dto)
Creates a new Backup.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backup_options_dto** | Option<[**BackupOptionsDto**](BackupOptionsDto.md)> | The backup options. |  |

### Return type

[**models::BackupManifestDto**](BackupManifestDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_backup

> models::BackupManifestDto get_backup(path)
Gets the descriptor from an existing archive is present.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | The data to start a restore process. | [required] |

### Return type

[**models::BackupManifestDto**](BackupManifestDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_backups

> Vec<models::BackupManifestDto> list_backups()
Gets a list of all currently present backups in the backup directory.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::BackupManifestDto>**](BackupManifestDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_restore_backup

> start_restore_backup(backup_restore_request_dto)
Restores to a backup by restarting the server and applying the backup.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backup_restore_request_dto** | [**BackupRestoreRequestDto**](BackupRestoreRequestDto.md) | The data to start a restore process. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

