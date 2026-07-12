# \ItemLookupApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apply_search_criteria**](ItemLookupApi.md#apply_search_criteria) | **POST** /Items/RemoteSearch/Apply/{itemId} | Applies search criteria to an item and refreshes metadata.
[**get_book_remote_search_results**](ItemLookupApi.md#get_book_remote_search_results) | **POST** /Items/RemoteSearch/Book | Get book remote search.
[**get_box_set_remote_search_results**](ItemLookupApi.md#get_box_set_remote_search_results) | **POST** /Items/RemoteSearch/BoxSet | Get box set remote search.
[**get_external_id_infos**](ItemLookupApi.md#get_external_id_infos) | **GET** /Items/{itemId}/ExternalIdInfos | Get the item's external id info.
[**get_movie_remote_search_results**](ItemLookupApi.md#get_movie_remote_search_results) | **POST** /Items/RemoteSearch/Movie | Get movie remote search.
[**get_music_album_remote_search_results**](ItemLookupApi.md#get_music_album_remote_search_results) | **POST** /Items/RemoteSearch/MusicAlbum | Get music album remote search.
[**get_music_artist_remote_search_results**](ItemLookupApi.md#get_music_artist_remote_search_results) | **POST** /Items/RemoteSearch/MusicArtist | Get music artist remote search.
[**get_music_video_remote_search_results**](ItemLookupApi.md#get_music_video_remote_search_results) | **POST** /Items/RemoteSearch/MusicVideo | Get music video remote search.
[**get_person_remote_search_results**](ItemLookupApi.md#get_person_remote_search_results) | **POST** /Items/RemoteSearch/Person | Get person remote search.
[**get_series_remote_search_results**](ItemLookupApi.md#get_series_remote_search_results) | **POST** /Items/RemoteSearch/Series | Get series remote search.
[**get_trailer_remote_search_results**](ItemLookupApi.md#get_trailer_remote_search_results) | **POST** /Items/RemoteSearch/Trailer | Get trailer remote search.



## apply_search_criteria

> apply_search_criteria(item_id, remote_search_result, replace_all_images)
Applies search criteria to an item and refreshes metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item id. | [required] |
**remote_search_result** | [**RemoteSearchResult**](RemoteSearchResult.md) | The remote search result. | [required] |
**replace_all_images** | Option<**bool**> | Optional. Whether or not to replace all images. Default: True. |  |[default to true]

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_book_remote_search_results

> Vec<models::RemoteSearchResult> get_book_remote_search_results(book_info_remote_search_query)
Get book remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**book_info_remote_search_query** | [**BookInfoRemoteSearchQuery**](BookInfoRemoteSearchQuery.md) | Remote search query. | [required] |

### Return type

[**Vec<models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_box_set_remote_search_results

> Vec<models::RemoteSearchResult> get_box_set_remote_search_results(box_set_info_remote_search_query)
Get box set remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**box_set_info_remote_search_query** | [**BoxSetInfoRemoteSearchQuery**](BoxSetInfoRemoteSearchQuery.md) | Remote search query. | [required] |

### Return type

[**Vec<models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_external_id_infos

> Vec<models::ExternalIdInfo> get_external_id_infos(item_id)
Get the item's external id info.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item id. | [required] |

### Return type

[**Vec<models::ExternalIdInfo>**](ExternalIdInfo.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_remote_search_results

> Vec<models::RemoteSearchResult> get_movie_remote_search_results(movie_info_remote_search_query)
Get movie remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**movie_info_remote_search_query** | [**MovieInfoRemoteSearchQuery**](MovieInfoRemoteSearchQuery.md) | Remote search query. | [required] |

### Return type

[**Vec<models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_music_album_remote_search_results

> Vec<models::RemoteSearchResult> get_music_album_remote_search_results(album_info_remote_search_query)
Get music album remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_info_remote_search_query** | [**AlbumInfoRemoteSearchQuery**](AlbumInfoRemoteSearchQuery.md) | Remote search query. | [required] |

### Return type

[**Vec<models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_music_artist_remote_search_results

> Vec<models::RemoteSearchResult> get_music_artist_remote_search_results(artist_info_remote_search_query)
Get music artist remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**artist_info_remote_search_query** | [**ArtistInfoRemoteSearchQuery**](ArtistInfoRemoteSearchQuery.md) | Remote search query. | [required] |

### Return type

[**Vec<models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_music_video_remote_search_results

> Vec<models::RemoteSearchResult> get_music_video_remote_search_results(music_video_info_remote_search_query)
Get music video remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**music_video_info_remote_search_query** | [**MusicVideoInfoRemoteSearchQuery**](MusicVideoInfoRemoteSearchQuery.md) | Remote search query. | [required] |

### Return type

[**Vec<models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_person_remote_search_results

> Vec<models::RemoteSearchResult> get_person_remote_search_results(person_lookup_info_remote_search_query)
Get person remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**person_lookup_info_remote_search_query** | [**PersonLookupInfoRemoteSearchQuery**](PersonLookupInfoRemoteSearchQuery.md) | Remote search query. | [required] |

### Return type

[**Vec<models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_remote_search_results

> Vec<models::RemoteSearchResult> get_series_remote_search_results(series_info_remote_search_query)
Get series remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_info_remote_search_query** | [**SeriesInfoRemoteSearchQuery**](SeriesInfoRemoteSearchQuery.md) | Remote search query. | [required] |

### Return type

[**Vec<models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trailer_remote_search_results

> Vec<models::RemoteSearchResult> get_trailer_remote_search_results(trailer_info_remote_search_query)
Get trailer remote search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trailer_info_remote_search_query** | [**TrailerInfoRemoteSearchQuery**](TrailerInfoRemoteSearchQuery.md) | Remote search query. | [required] |

### Return type

[**Vec<models::RemoteSearchResult>**](RemoteSearchResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

