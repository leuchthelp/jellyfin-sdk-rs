# \PluginApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_package_installation**](PluginApi.md#cancel_package_installation) | **DELETE** /Packages/Installing/{packageId} | Cancels a package installation.
[**disable_plugin**](PluginApi.md#disable_plugin) | **POST** /Plugins/{pluginId}/{version}/Disable | Disable a plugin.
[**enable_plugin**](PluginApi.md#enable_plugin) | **POST** /Plugins/{pluginId}/{version}/Enable | Enables a disabled plugin.
[**get_configuration_pages**](PluginApi.md#get_configuration_pages) | **GET** /web/ConfigurationPages | Gets the configuration pages.
[**get_dashboard_configuration_page**](PluginApi.md#get_dashboard_configuration_page) | **GET** /web/ConfigurationPage | Gets a dashboard configuration page.
[**get_package_info**](PluginApi.md#get_package_info) | **GET** /Packages/{name} | Gets a package by name or assembly GUID.
[**get_packages**](PluginApi.md#get_packages) | **GET** /Packages | Gets available packages.
[**get_plugin_configuration**](PluginApi.md#get_plugin_configuration) | **GET** /Plugins/{pluginId}/Configuration | Gets plugin configuration.
[**get_plugin_image**](PluginApi.md#get_plugin_image) | **GET** /Plugins/{pluginId}/{version}/Image | Gets a plugin's image.
[**get_plugin_manifest**](PluginApi.md#get_plugin_manifest) | **POST** /Plugins/{pluginId}/Manifest | Gets a plugin's manifest.
[**get_plugins**](PluginApi.md#get_plugins) | **GET** /Plugins | Gets a list of currently installed plugins.
[**get_repositories**](PluginApi.md#get_repositories) | **GET** /Repositories | Gets all package repositories.
[**install_package**](PluginApi.md#install_package) | **POST** /Packages/Installed/{name} | Installs a package.
[**set_repositories**](PluginApi.md#set_repositories) | **POST** /Repositories | Sets the enabled and existing package repositories.
[**uninstall_plugin**](PluginApi.md#uninstall_plugin) | **DELETE** /Plugins/{pluginId} | Uninstalls a plugin.
[**uninstall_plugin_by_version**](PluginApi.md#uninstall_plugin_by_version) | **DELETE** /Plugins/{pluginId}/{version} | Uninstalls a plugin by version.
[**update_plugin_configuration**](PluginApi.md#update_plugin_configuration) | **POST** /Plugins/{pluginId}/Configuration | Updates plugin configuration.



## cancel_package_installation

> cancel_package_installation(package_id)
Cancels a package installation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_id** | **uuid::Uuid** | Installation Id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_plugin

> disable_plugin(plugin_id, version)
Disable a plugin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **uuid::Uuid** | Plugin id. | [required] |
**version** | **String** | Plugin version. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_plugin

> enable_plugin(plugin_id, version)
Enables a disabled plugin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **uuid::Uuid** | Plugin id. | [required] |
**version** | **String** | Plugin version. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_configuration_pages

> Vec<models::ConfigurationPageInfo> get_configuration_pages(enable_in_main_menu)
Gets the configuration pages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enable_in_main_menu** | Option<**bool**> | Whether to enable in the main menu. |  |

### Return type

[**Vec<models::ConfigurationPageInfo>**](ConfigurationPageInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dashboard_configuration_page

> std::path::PathBuf get_dashboard_configuration_page(name)
Gets a dashboard configuration page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the page. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html, application/x-javascript, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_package_info

> models::PackageInfo get_package_info(name, assembly_guid)
Gets a package by name or assembly GUID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the package. | [required] |
**assembly_guid** | Option<**uuid::Uuid**> | The GUID of the associated assembly. |  |

### Return type

[**models::PackageInfo**](PackageInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_packages

> Vec<models::PackageInfo> get_packages()
Gets available packages.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::PackageInfo>**](PackageInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plugin_configuration

> serde_json::Value get_plugin_configuration(plugin_id)
Gets plugin configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **uuid::Uuid** | Plugin id. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plugin_image

> std::path::PathBuf get_plugin_image(plugin_id, version)
Gets a plugin's image.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **uuid::Uuid** | Plugin id. | [required] |
**version** | **String** | Plugin version. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plugin_manifest

> get_plugin_manifest(plugin_id)
Gets a plugin's manifest.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **uuid::Uuid** | Plugin id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plugins

> Vec<models::PluginInfo> get_plugins()
Gets a list of currently installed plugins.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::PluginInfo>**](PluginInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repositories

> Vec<models::RepositoryInfo> get_repositories()
Gets all package repositories.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::RepositoryInfo>**](RepositoryInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## install_package

> install_package(name, assembly_guid, version, repository_url)
Installs a package.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Package name. | [required] |
**assembly_guid** | Option<**uuid::Uuid**> | GUID of the associated assembly. |  |
**version** | Option<**String**> | Optional version. Defaults to latest version. |  |
**repository_url** | Option<**String**> | Optional. Specify the repository to install from. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_repositories

> set_repositories(repository_info)
Sets the enabled and existing package repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repository_info** | [**Vec<models::RepositoryInfo>**](RepositoryInfo.md) | The list of package repositories. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uninstall_plugin

> uninstall_plugin(plugin_id)
Uninstalls a plugin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **uuid::Uuid** | Plugin id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uninstall_plugin_by_version

> uninstall_plugin_by_version(plugin_id, version)
Uninstalls a plugin by version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **uuid::Uuid** | Plugin id. | [required] |
**version** | **String** | Plugin version. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_plugin_configuration

> update_plugin_configuration(plugin_id)
Updates plugin configuration.

Accepts plugin configuration as JSON body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_id** | **uuid::Uuid** | Plugin id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

