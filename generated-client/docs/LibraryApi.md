# \LibraryApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_item**](LibraryApi.md#delete_item) | **DELETE** /Items/{itemId} | Deletes an item from the library and filesystem.
[**delete_items**](LibraryApi.md#delete_items) | **DELETE** /Items | Deletes items from the library and filesystem.
[**get_ancestors**](LibraryApi.md#get_ancestors) | **GET** /Items/{itemId}/Ancestors | Gets all parents of an item.
[**get_download**](LibraryApi.md#get_download) | **GET** /Items/{itemId}/Download | Downloads item media.
[**get_file**](LibraryApi.md#get_file) | **GET** /Items/{itemId}/File | Get the original file of an item.
[**get_intros**](LibraryApi.md#get_intros) | **GET** /Items/{itemId}/Intros | Gets intros to play before the main media item plays.
[**get_item**](LibraryApi.md#get_item) | **GET** /Items/{itemId} | Gets an item from a user's library.
[**get_item_collections**](LibraryApi.md#get_item_collections) | **GET** /Items/{itemId}/Collections | Gets the collections that include the specified item.
[**get_item_counts**](LibraryApi.md#get_item_counts) | **GET** /Items/Counts | Get item counts.
[**get_items**](LibraryApi.md#get_items) | **GET** /Items | Gets items based on a query.
[**get_latest_media**](LibraryApi.md#get_latest_media) | **GET** /Items/Latest | Gets latest media.
[**get_library_options_info**](LibraryApi.md#get_library_options_info) | **GET** /Libraries/AvailableOptions | Gets the library options info.
[**get_local_trailers**](LibraryApi.md#get_local_trailers) | **GET** /Items/{itemId}/LocalTrailers | Gets local trailers for an item.
[**get_media_folders**](LibraryApi.md#get_media_folders) | **GET** /Library/MediaFolders | Gets all user media folders.
[**get_physical_paths**](LibraryApi.md#get_physical_paths) | **GET** /Library/PhysicalPaths | Gets a list of physical paths from virtual folders.
[**get_resume_items**](LibraryApi.md#get_resume_items) | **GET** /UserItems/Resume | Gets items based on a query.
[**get_root_folder**](LibraryApi.md#get_root_folder) | **GET** /Items/Root | Gets the root folder from a user's library.
[**get_similar_albums**](LibraryApi.md#get_similar_albums) | **GET** /Albums/{itemId}/Similar | Gets similar items.
[**get_similar_artists**](LibraryApi.md#get_similar_artists) | **GET** /Artists/{itemId}/Similar | Gets similar items.
[**get_similar_items**](LibraryApi.md#get_similar_items) | **GET** /Items/{itemId}/Similar | Gets similar items.
[**get_similar_movies**](LibraryApi.md#get_similar_movies) | **GET** /Movies/{itemId}/Similar | Gets similar items.
[**get_similar_shows**](LibraryApi.md#get_similar_shows) | **GET** /Shows/{itemId}/Similar | Gets similar items.
[**get_similar_trailers**](LibraryApi.md#get_similar_trailers) | **GET** /Trailers/{itemId}/Similar | Gets similar items.
[**get_special_features**](LibraryApi.md#get_special_features) | **GET** /Items/{itemId}/SpecialFeatures | Gets special features for an item.
[**get_theme_media**](LibraryApi.md#get_theme_media) | **GET** /Items/{itemId}/ThemeMedia | Get theme songs and videos for an item.
[**get_theme_songs**](LibraryApi.md#get_theme_songs) | **GET** /Items/{itemId}/ThemeSongs | Get theme songs for an item.
[**get_theme_videos**](LibraryApi.md#get_theme_videos) | **GET** /Items/{itemId}/ThemeVideos | Get theme videos for an item.
[**post_added_movies**](LibraryApi.md#post_added_movies) | **POST** /Library/Movies/Added | Reports that new movies have been added by an external source.
[**post_added_series**](LibraryApi.md#post_added_series) | **POST** /Library/Series/Added | Reports that new episodes of a series have been added by an external source.
[**post_updated_media**](LibraryApi.md#post_updated_media) | **POST** /Library/Media/Updated | Reports that new movies have been added by an external source.
[**post_updated_movies**](LibraryApi.md#post_updated_movies) | **POST** /Library/Movies/Updated | Reports that new movies have been added by an external source.
[**post_updated_series**](LibraryApi.md#post_updated_series) | **POST** /Library/Series/Updated | Reports that new episodes of a series have been added by an external source.
[**refresh_item**](LibraryApi.md#refresh_item) | **POST** /Items/{itemId}/Refresh | Refreshes metadata for an item.
[**refresh_library**](LibraryApi.md#refresh_library) | **POST** /Library/Refresh | Starts a library scan.



## delete_item

> delete_item(item_id)
Deletes an item from the library and filesystem.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_items

> delete_items(ids)
Deletes items from the library and filesystem.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | The item ids. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ancestors

> Vec<models::BaseItemDto> get_ancestors(item_id, user_id)
Gets all parents of an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |

### Return type

[**Vec<models::BaseItemDto>**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_download

> std::path::PathBuf get_download(item_id)
Downloads item media.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: video/*, audio/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file

> std::path::PathBuf get_file(item_id)
Get the original file of an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: video/*, audio/*, application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_intros

> models::BaseItemDtoQueryResult get_intros(item_id, user_id)
Gets intros to play before the main media item plays.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | User id. |  |

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item

> models::BaseItemDto get_item(item_id, user_id)
Gets an item from a user's library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | User id. |  |

### Return type

[**models::BaseItemDto**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item_collections

> models::BaseItemDtoQueryResult get_item_collections(item_id, user_id, start_index, limit, fields)
Gets the collections that include the specified item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |
**start_index** | Option<**i32**> | Optional. The index of the first record in the output. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_item_counts

> models::ItemCounts get_item_counts(user_id, is_favorite)
Get item counts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**uuid::Uuid**> | Optional. Get counts from a specific user's library. |  |
**is_favorite** | Option<**bool**> | Optional. Get counts of favorite items. |  |

### Return type

[**models::ItemCounts**](ItemCounts.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_items

> models::BaseItemDtoQueryResult get_items(user_id, max_official_rating, has_theme_song, has_theme_video, has_subtitles, has_special_feature, has_trailer, adjacent_to, index_number, parent_index_number, has_parental_rating, is_hd, is4_k, location_types, exclude_location_types, is_missing, is_unaired, min_community_rating, min_critic_rating, min_premiere_date, min_date_last_saved, min_date_last_saved_for_user, max_premiere_date, has_overview, has_imdb_id, has_tmdb_id, has_tvdb_id, is_movie, is_series, is_news, is_kids, is_sports, exclude_item_ids, start_index, limit, recursive, search_term, sort_order, parent_id, fields, exclude_item_types, include_item_types, filters, is_favorite, media_types, image_types, sort_by, is_played, genres, official_ratings, tags, years, enable_user_data, image_type_limit, enable_image_types, person, person_ids, person_types, studios, artists, exclude_artist_ids, artist_ids, album_artist_ids, contributing_artist_ids, albums, album_ids, ids, video_types, min_official_rating, is_locked, is_place_holder, has_official_rating, collapse_box_set_items, min_width, min_height, max_width, max_height, is3_d, series_status, name_starts_with_or_greater, name_starts_with, name_less_than, studio_ids, genre_ids, audio_languages, subtitle_languages, enable_total_record_count, enable_images)
Gets items based on a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**uuid::Uuid**> | The user id supplied as query parameter; this is required when not using an API key. |  |
**max_official_rating** | Option<**String**> | Optional filter by maximum official rating (PG, PG-13, TV-MA, etc). |  |
**has_theme_song** | Option<**bool**> | Optional filter by items with theme songs. |  |
**has_theme_video** | Option<**bool**> | Optional filter by items with theme videos. |  |
**has_subtitles** | Option<**bool**> | Optional filter by items with subtitles. |  |
**has_special_feature** | Option<**bool**> | Optional filter by items with special features. |  |
**has_trailer** | Option<**bool**> | Optional filter by items with trailers. |  |
**adjacent_to** | Option<**uuid::Uuid**> | Optional. Return items that are siblings of a supplied item. |  |
**index_number** | Option<**i32**> | Optional filter by index number. |  |
**parent_index_number** | Option<**i32**> | Optional filter by parent index number. |  |
**has_parental_rating** | Option<**bool**> | Optional filter by items that have or do not have a parental rating. |  |
**is_hd** | Option<**bool**> | Optional filter by items that are HD or not. |  |
**is4_k** | Option<**bool**> | Optional filter by items that are 4K or not. |  |
**location_types** | Option<[**Vec<models::LocationType>**](Models__LocationType.md)> | Optional. If specified, results will be filtered based on LocationType. This allows multiple, comma delimited. |  |
**exclude_location_types** | Option<[**Vec<models::LocationType>**](Models__LocationType.md)> | Optional. If specified, results will be filtered based on the LocationType. This allows multiple, comma delimited. |  |
**is_missing** | Option<**bool**> | Optional filter by items that are missing episodes or not. |  |
**is_unaired** | Option<**bool**> | Optional filter by items that are unaired episodes or not. |  |
**min_community_rating** | Option<**f64**> | Optional filter by minimum community rating. |  |
**min_critic_rating** | Option<**f64**> | Optional filter by minimum critic rating. |  |
**min_premiere_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Optional. The minimum premiere date. Format = ISO. |  |
**min_date_last_saved** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Optional. The minimum last saved date. Format = ISO. |  |
**min_date_last_saved_for_user** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Optional. The minimum last saved date for the current user. Format = ISO. |  |
**max_premiere_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Optional. The maximum premiere date. Format = ISO. |  |
**has_overview** | Option<**bool**> | Optional filter by items that have an overview or not. |  |
**has_imdb_id** | Option<**bool**> | Optional filter by items that have an IMDb id or not. |  |
**has_tmdb_id** | Option<**bool**> | Optional filter by items that have a TMDb id or not. |  |
**has_tvdb_id** | Option<**bool**> | Optional filter by items that have a TVDb id or not. |  |
**is_movie** | Option<**bool**> | Optional filter for live tv movies. |  |
**is_series** | Option<**bool**> | Optional filter for live tv series. |  |
**is_news** | Option<**bool**> | Optional filter for live tv news. |  |
**is_kids** | Option<**bool**> | Optional filter for live tv kids. |  |
**is_sports** | Option<**bool**> | Optional filter for live tv sports. |  |
**exclude_item_ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Optional. If specified, results will be filtered by excluding item ids. This allows multiple, comma delimited. |  |
**start_index** | Option<**i32**> | Optional. The record index to start at. All items with a lower index will be dropped from the results. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**recursive** | Option<**bool**> | When searching within folders, this determines whether or not the search will be recursive. true/false. |  |
**search_term** | Option<**String**> | Optional. Filter based on a search term. |  |
**sort_order** | Option<[**Vec<models::SortOrder>**](Models__SortOrder.md)> | Sort Order - Ascending, Descending. |  |
**parent_id** | Option<**uuid::Uuid**> | Specify this to localize the search to a specific item or folder. Omit to use the root. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines. |  |
**exclude_item_types** | Option<[**Vec<models::BaseItemKind>**](Models__BaseItemKind.md)> | Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited. |  |
**include_item_types** | Option<[**Vec<models::BaseItemKind>**](Models__BaseItemKind.md)> | Optional. If specified, results will be filtered based on the item type. This allows multiple, comma delimited. |  |
**filters** | Option<[**Vec<models::ItemFilter>**](Models__ItemFilter.md)> | Optional. Specify additional filters to apply. This allows multiple, comma delimited. Options: IsFolder, IsNotFolder, IsUnplayed, IsPlayed, IsFavorite, IsResumable, Likes, Dislikes. |  |
**is_favorite** | Option<**bool**> | Optional filter by items that are marked as favorite, or not. |  |
**media_types** | Option<[**Vec<models::MediaType>**](Models__MediaType.md)> | Optional filter by MediaType. Allows multiple, comma delimited. |  |
**image_types** | Option<[**Vec<models::ImageType>**](Models__ImageType.md)> | Optional. If specified, results will be filtered based on those containing image types. This allows multiple, comma delimited. |  |
**sort_by** | Option<[**Vec<models::ItemSortBy>**](Models__ItemSortBy.md)> | Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime. |  |
**is_played** | Option<**bool**> | Optional filter by items that are played, or not. |  |
**genres** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on genre. This allows multiple, pipe delimited. |  |
**official_ratings** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on OfficialRating. This allows multiple, pipe delimited. |  |
**tags** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on tag. This allows multiple, pipe delimited. |  |
**years** | Option<[**Vec<i32>**](I32.md)> | Optional. If specified, results will be filtered based on production year. This allows multiple, comma delimited. |  |
**enable_user_data** | Option<**bool**> | Optional, include user data. |  |
**image_type_limit** | Option<**i32**> | Optional, the max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<models::ImageType>**](Models__ImageType.md)> | Optional. The image types to include in the output. |  |
**person** | Option<**String**> | Optional. If specified, results will be filtered to include only those containing the specified person. |  |
**person_ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Optional. If specified, results will be filtered to include only those containing the specified person id. |  |
**person_types** | Option<[**Vec<String>**](String.md)> | Optional. If specified, along with Person, results will be filtered to include only those containing the specified person and PersonType. Allows multiple, comma-delimited. |  |
**studios** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on studio. This allows multiple, pipe delimited. |  |
**artists** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on artists. This allows multiple, pipe delimited. |  |
**exclude_artist_ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Optional. If specified, results will be filtered based on artist id. This allows multiple, pipe delimited. |  |
**artist_ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Optional. If specified, results will be filtered to include only those containing the specified artist id. |  |
**album_artist_ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Optional. If specified, results will be filtered to include only those containing the specified album artist id. |  |
**contributing_artist_ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Optional. If specified, results will be filtered to include only those containing the specified contributing artist id. |  |
**albums** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on album. This allows multiple, pipe delimited. |  |
**album_ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Optional. If specified, results will be filtered based on album id. This allows multiple, pipe delimited. |  |
**ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Optional. If specific items are needed, specify a list of item id's to retrieve. This allows multiple, comma delimited. |  |
**video_types** | Option<[**Vec<models::VideoType>**](Models__VideoType.md)> | Optional filter by VideoType (videofile, dvd, bluray, iso). Allows multiple, comma delimited. |  |
**min_official_rating** | Option<**String**> | Optional filter by minimum official rating (PG, PG-13, TV-MA, etc). |  |
**is_locked** | Option<**bool**> | Optional filter by items that are locked. |  |
**is_place_holder** | Option<**bool**> | Optional filter by items that are placeholders. |  |
**has_official_rating** | Option<**bool**> | Optional filter by items that have official ratings. |  |
**collapse_box_set_items** | Option<**bool**> | Whether or not to hide items behind their boxsets. |  |
**min_width** | Option<**i32**> | Optional. Filter by the minimum width of the item. |  |
**min_height** | Option<**i32**> | Optional. Filter by the minimum height of the item. |  |
**max_width** | Option<**i32**> | Optional. Filter by the maximum width of the item. |  |
**max_height** | Option<**i32**> | Optional. Filter by the maximum height of the item. |  |
**is3_d** | Option<**bool**> | Optional filter by items that are 3D, or not. |  |
**series_status** | Option<[**Vec<models::SeriesStatus>**](Models__SeriesStatus.md)> | Optional filter by Series Status. Allows multiple, comma delimited. |  |
**name_starts_with_or_greater** | Option<**String**> | Optional filter by items whose name is sorted equally or greater than a given input string. |  |
**name_starts_with** | Option<**String**> | Optional filter by items whose name is sorted equally than a given input string. |  |
**name_less_than** | Option<**String**> | Optional filter by items whose name is equally or lesser than a given input string. |  |
**studio_ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Optional. If specified, results will be filtered based on studio id. This allows multiple, pipe delimited. |  |
**genre_ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Optional. If specified, results will be filtered based on genre id. This allows multiple, pipe delimited. |  |
**audio_languages** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on audio language. This allows multiple, comma delimited values. |  |
**subtitle_languages** | Option<[**Vec<String>**](String.md)> | Optional. If specified, results will be filtered based on subtitle language. This allows multiple, comma delimited values. |  |
**enable_total_record_count** | Option<**bool**> | Optional. Enable the total record count. |  |[default to true]
**enable_images** | Option<**bool**> | Optional, include image information in output. |  |[default to true]

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_latest_media

> Vec<models::BaseItemDto> get_latest_media(user_id, parent_id, fields, include_item_types, is_played, enable_images, image_type_limit, enable_image_types, enable_user_data, limit, group_items)
Gets latest media.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**uuid::Uuid**> | User id. |  |
**parent_id** | Option<**uuid::Uuid**> | Specify this to localize the search to a specific item or folder. Omit to use the root. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. |  |
**include_item_types** | Option<[**Vec<models::BaseItemKind>**](Models__BaseItemKind.md)> | Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited. |  |
**is_played** | Option<**bool**> | Filter by items that are played, or not. |  |
**enable_images** | Option<**bool**> | Optional. include image information in output. |  |
**image_type_limit** | Option<**i32**> | Optional. the max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<models::ImageType>**](Models__ImageType.md)> | Optional. The image types to include in the output. |  |
**enable_user_data** | Option<**bool**> | Optional. include user data. |  |
**limit** | Option<**i32**> | Return item limit. |  |[default to 20]
**group_items** | Option<**bool**> | Whether or not to group items into a parent container. |  |[default to true]

### Return type

[**Vec<models::BaseItemDto>**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_library_options_info

> models::LibraryOptionsResultDto get_library_options_info(library_content_type, is_new_library)
Gets the library options info.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_content_type** | Option<**models::CollectionType**> | Library content type. |  |
**is_new_library** | Option<**bool**> | Whether this is a new library. |  |[default to false]

### Return type

[**models::LibraryOptionsResultDto**](LibraryOptionsResultDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_local_trailers

> Vec<models::BaseItemDto> get_local_trailers(item_id, user_id)
Gets local trailers for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | User id. |  |

### Return type

[**Vec<models::BaseItemDto>**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_media_folders

> models::BaseItemDtoQueryResult get_media_folders(is_hidden)
Gets all user media folders.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**is_hidden** | Option<**bool**> | Optional. Filter by folders that are marked hidden, or not. |  |

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_physical_paths

> Vec<String> get_physical_paths()
Gets a list of physical paths from virtual folders.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resume_items

> models::BaseItemDtoQueryResult get_resume_items(user_id, start_index, limit, search_term, parent_id, fields, media_types, enable_user_data, image_type_limit, enable_image_types, exclude_item_types, include_item_types, enable_total_record_count, enable_images, exclude_active_sessions)
Gets items based on a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**uuid::Uuid**> | The user id. |  |
**start_index** | Option<**i32**> | The start index. |  |
**limit** | Option<**i32**> | The item limit. |  |
**search_term** | Option<**String**> | The search term. |  |
**parent_id** | Option<**uuid::Uuid**> | Specify this to localize the search to a specific item or folder. Omit to use the root. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines. |  |
**media_types** | Option<[**Vec<models::MediaType>**](Models__MediaType.md)> | Optional. Filter by MediaType. Allows multiple, comma delimited. |  |
**enable_user_data** | Option<**bool**> | Optional. Include user data. |  |
**image_type_limit** | Option<**i32**> | Optional. The max number of images to return, per image type. |  |
**enable_image_types** | Option<[**Vec<models::ImageType>**](Models__ImageType.md)> | Optional. The image types to include in the output. |  |
**exclude_item_types** | Option<[**Vec<models::BaseItemKind>**](Models__BaseItemKind.md)> | Optional. If specified, results will be filtered based on item type. This allows multiple, comma delimited. |  |
**include_item_types** | Option<[**Vec<models::BaseItemKind>**](Models__BaseItemKind.md)> | Optional. If specified, results will be filtered based on the item type. This allows multiple, comma delimited. |  |
**enable_total_record_count** | Option<**bool**> | Optional. Enable the total record count. |  |[default to true]
**enable_images** | Option<**bool**> | Optional. Include image information in output. |  |[default to true]
**exclude_active_sessions** | Option<**bool**> | Optional. Whether to exclude the currently active sessions. |  |[default to false]

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_root_folder

> models::BaseItemDto get_root_folder(user_id)
Gets the root folder from a user's library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**uuid::Uuid**> | User id. |  |

### Return type

[**models::BaseItemDto**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_similar_albums

> models::BaseItemDtoQueryResult get_similar_albums(item_id, exclude_artist_ids, user_id, limit, fields)
Gets similar items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**exclude_artist_ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Exclude artist ids. |  |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls. |  |

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_similar_artists

> models::BaseItemDtoQueryResult get_similar_artists(item_id, exclude_artist_ids, user_id, limit, fields)
Gets similar items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**exclude_artist_ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Exclude artist ids. |  |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls. |  |

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_similar_items

> models::BaseItemDtoQueryResult get_similar_items(item_id, exclude_artist_ids, user_id, limit, fields)
Gets similar items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**exclude_artist_ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Exclude artist ids. |  |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls. |  |

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_similar_movies

> models::BaseItemDtoQueryResult get_similar_movies(item_id, exclude_artist_ids, user_id, limit, fields)
Gets similar items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**exclude_artist_ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Exclude artist ids. |  |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls. |  |

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_similar_shows

> models::BaseItemDtoQueryResult get_similar_shows(item_id, exclude_artist_ids, user_id, limit, fields)
Gets similar items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**exclude_artist_ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Exclude artist ids. |  |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls. |  |

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_similar_trailers

> models::BaseItemDtoQueryResult get_similar_trailers(item_id, exclude_artist_ids, user_id, limit, fields)
Gets similar items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**exclude_artist_ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> | Exclude artist ids. |  |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |
**limit** | Option<**i32**> | Optional. The maximum number of records to return. |  |
**fields** | Option<[**Vec<models::ItemFields>**](Models__ItemFields.md)> | Optional. Specify additional fields of information to return in the output. This allows multiple, comma delimited. Options: Budget, Chapters, DateCreated, Genres, HomePageUrl, IndexOptions, MediaStreams, Overview, ParentId, Path, People, ProviderIds, PrimaryImageAspectRatio, Revenue, SortName, Studios, Taglines, TrailerUrls. |  |

### Return type

[**models::BaseItemDtoQueryResult**](BaseItemDtoQueryResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_special_features

> Vec<models::BaseItemDto> get_special_features(item_id, user_id)
Gets special features for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | User id. |  |

### Return type

[**Vec<models::BaseItemDto>**](BaseItemDto.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_theme_media

> models::AllThemeMediaResult get_theme_media(item_id, user_id, inherit_from_parent, sort_by, sort_order)
Get theme songs and videos for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |
**inherit_from_parent** | Option<**bool**> | Optional. Determines whether or not parent items should be searched for theme media. |  |[default to false]
**sort_by** | Option<[**Vec<models::ItemSortBy>**](Models__ItemSortBy.md)> | Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime. |  |
**sort_order** | Option<[**Vec<models::SortOrder>**](Models__SortOrder.md)> | Optional. Sort Order - Ascending, Descending. |  |

### Return type

[**models::AllThemeMediaResult**](AllThemeMediaResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_theme_songs

> models::ThemeMediaResult get_theme_songs(item_id, user_id, inherit_from_parent, sort_by, sort_order)
Get theme songs for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |
**inherit_from_parent** | Option<**bool**> | Optional. Determines whether or not parent items should be searched for theme media. |  |[default to false]
**sort_by** | Option<[**Vec<models::ItemSortBy>**](Models__ItemSortBy.md)> | Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime. |  |
**sort_order** | Option<[**Vec<models::SortOrder>**](Models__SortOrder.md)> | Optional. Sort Order - Ascending, Descending. |  |

### Return type

[**models::ThemeMediaResult**](ThemeMediaResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_theme_videos

> models::ThemeMediaResult get_theme_videos(item_id, user_id, inherit_from_parent, sort_by, sort_order)
Get theme videos for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | The item id. | [required] |
**user_id** | Option<**uuid::Uuid**> | Optional. Filter by user id, and attach user data. |  |
**inherit_from_parent** | Option<**bool**> | Optional. Determines whether or not parent items should be searched for theme media. |  |[default to false]
**sort_by** | Option<[**Vec<models::ItemSortBy>**](Models__ItemSortBy.md)> | Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime. |  |
**sort_order** | Option<[**Vec<models::SortOrder>**](Models__SortOrder.md)> | Optional. Sort Order - Ascending, Descending. |  |

### Return type

[**models::ThemeMediaResult**](ThemeMediaResult.md)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_added_movies

> post_added_movies(tmdb_id, imdb_id)
Reports that new movies have been added by an external source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tmdb_id** | Option<**String**> | The tmdbId. |  |
**imdb_id** | Option<**String**> | The imdbId. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_added_series

> post_added_series(tvdb_id)
Reports that new episodes of a series have been added by an external source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tvdb_id** | Option<**String**> | The tvdbId. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_updated_media

> post_updated_media(media_update_info_dto)
Reports that new movies have been added by an external source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**media_update_info_dto** | [**MediaUpdateInfoDto**](MediaUpdateInfoDto.md) | The update paths. | [required] |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_updated_movies

> post_updated_movies(tmdb_id, imdb_id)
Reports that new movies have been added by an external source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tmdb_id** | Option<**String**> | The tmdbId. |  |
**imdb_id** | Option<**String**> | The imdbId. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_updated_series

> post_updated_series(tvdb_id)
Reports that new episodes of a series have been added by an external source.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tvdb_id** | Option<**String**> | The tvdbId. |  |

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_item

> refresh_item(item_id, metadata_refresh_mode, image_refresh_mode, replace_all_metadata, replace_all_images, regenerate_trickplay)
Refreshes metadata for an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** | Item id. | [required] |
**metadata_refresh_mode** | Option<**models::MetadataRefreshMode**> | (Optional) Specifies the metadata refresh mode. |  |[default to None]
**image_refresh_mode** | Option<**models::MetadataRefreshMode**> | (Optional) Specifies the image refresh mode. |  |[default to None]
**replace_all_metadata** | Option<**bool**> | (Optional) Determines if metadata should be replaced. Only applicable if mode is FullRefresh. |  |[default to false]
**replace_all_images** | Option<**bool**> | (Optional) Determines if images should be replaced. Only applicable if mode is FullRefresh. |  |[default to false]
**regenerate_trickplay** | Option<**bool**> | (Optional) Determines if trickplay images should be replaced. Only applicable if mode is FullRefresh. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; profile=CamelCase, application/json; profile=PascalCase, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_library

> refresh_library()
Starts a library scan.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[CustomAuthentication](../README.md#CustomAuthentication)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

