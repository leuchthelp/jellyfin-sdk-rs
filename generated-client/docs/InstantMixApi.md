# \InstantMixApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_instant_mix_from_album**](InstantMixApi.md#get_instant_mix_from_album) | **GET** /Albums/{itemId}/InstantMix | Creates an instant playlist based on a given album.
[**get_instant_mix_from_artists**](InstantMixApi.md#get_instant_mix_from_artists) | **GET** /Artists/{itemId}/InstantMix | Creates an instant playlist based on a given artist.
[**get_instant_mix_from_item**](InstantMixApi.md#get_instant_mix_from_item) | **GET** /Items/{itemId}/InstantMix | Creates an instant playlist based on a given item.
[**get_instant_mix_from_music_genre_by_id**](InstantMixApi.md#get_instant_mix_from_music_genre_by_id) | **GET** /MusicGenres/InstantMix | Creates an instant playlist based on a given genre.
[**get_instant_mix_from_music_genre_by_name**](InstantMixApi.md#get_instant_mix_from_music_genre_by_name) | **GET** /MusicGenres/{name}/InstantMix | Creates an instant playlist based on a given genre.
[**get_instant_mix_from_playlist**](InstantMixApi.md#get_instant_mix_from_playlist) | **GET** /Playlists/{itemId}/InstantMix | Creates an instant playlist based on a given playlist.
[**get_instant_mix_from_song**](InstantMixApi.md#get_instant_mix_from_song) | **GET** /Songs/{itemId}/InstantMix | Creates an instant playlist based on a given song.



## get_instant_mix_from_album

> models::BaseItemDtoQueryResult get_instant_mix_from_album(item_id, user_id, limit, fields, enable_images, enable_user_data, image_type_limit, enable_image_types)
Creates an instant playlist based on a given album.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**enable_images** | Option<**bool**> | Optional. Include image information in output. |  |
**enable_user_data** | Option<**bool**> | Optional. Include user data. |  |
**image_type_limit** | Option<**i32**> | Optional. The max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<models::ImageType>**](Models__ImageType.md)> | Optional. The image types to include in the output. |  |

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instant_mix_from_artists

> models::BaseItemDtoQueryResult get_instant_mix_from_artists(item_id, user_id, limit, fields, enable_images, enable_user_data, image_type_limit, enable_image_types)
Creates an instant playlist based on a given artist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**enable_images** | Option<**bool**> | Optional. Include image information in output. |  |
**enable_user_data** | Option<**bool**> | Optional. Include user data. |  |
**image_type_limit** | Option<**i32**> | Optional. The max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<models::ImageType>**](Models__ImageType.md)> | Optional. The image types to include in the output. |  |

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instant_mix_from_item

> models::BaseItemDtoQueryResult get_instant_mix_from_item(item_id, user_id, limit, fields, enable_images, enable_user_data, image_type_limit, enable_image_types)
Creates an instant playlist based on a given item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**enable_images** | Option<**bool**> | Optional. Include image information in output. |  |
**enable_user_data** | Option<**bool**> | Optional. Include user data. |  |
**image_type_limit** | Option<**i32**> | Optional. The max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<models::ImageType>**](Models__ImageType.md)> | Optional. The image types to include in the output. |  |

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instant_mix_from_music_genre_by_id

> models::BaseItemDtoQueryResult get_instant_mix_from_music_genre_by_id(id, user_id, limit, fields, enable_images, enable_user_data, image_type_limit, enable_image_types)
Creates an instant playlist based on a given genre.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | The item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**enable_images** | Option<**bool**> | Optional. Include image information in output. |  |
**enable_user_data** | Option<**bool**> | Optional. Include user data. |  |
**image_type_limit** | Option<**i32**> | Optional. The max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<models::ImageType>**](Models__ImageType.md)> | Optional. The image types to include in the output. |  |

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instant_mix_from_music_genre_by_name

> models::BaseItemDtoQueryResult get_instant_mix_from_music_genre_by_name(name, user_id, limit, fields, enable_images, enable_user_data, image_type_limit, enable_image_types)
Creates an instant playlist based on a given genre.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The genre name. | [required] |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**enable_images** | Option<**bool**> | Optional. Include image information in output. |  |
**enable_user_data** | Option<**bool**> | Optional. Include user data. |  |
**image_type_limit** | Option<**i32**> | Optional. The max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<models::ImageType>**](Models__ImageType.md)> | Optional. The image types to include in the output. |  |

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instant_mix_from_playlist

> models::BaseItemDtoQueryResult get_instant_mix_from_playlist(item_id, user_id, limit, fields, enable_images, enable_user_data, image_type_limit, enable_image_types)
Creates an instant playlist based on a given playlist.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**enable_images** | Option<**bool**> | Optional. Include image information in output. |  |
**enable_user_data** | Option<**bool**> | Optional. Include user data. |  |
**image_type_limit** | Option<**i32**> | Optional. The max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<models::ImageType>**](Models__ImageType.md)> | Optional. The image types to include in the output. |  |

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instant_mix_from_song

> models::BaseItemDtoQueryResult get_instant_mix_from_song(item_id, user_id, limit, fields, enable_images, enable_user_data, image_type_limit, enable_image_types)
Creates an instant playlist based on a given song.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**enable_images** | Option<**bool**> | Optional. Include image information in output. |  |
**enable_user_data** | Option<**bool**> | Optional. Include user data. |  |
**image_type_limit** | Option<**i32**> | Optional. The max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<models::ImageType>**](Models__ImageType.md)> | Optional. The image types to include in the output. |  |

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

