# DeviceProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Gets or sets the name of this device profile. User profiles must have a unique name. | [optional]
**id** | Option<**uuid::Uuid**> | Gets or sets the unique internal identifier. | [optional]
**max_streaming_bitrate** | Option<**i32**> | Gets or sets the maximum allowed bitrate for all streamed content. | [optional]
**max_static_bitrate** | Option<**i32**> | Gets or sets the maximum allowed bitrate for statically streamed content (= direct played files). | [optional]
**music_streaming_transcoding_bitrate** | Option<**i32**> | Gets or sets the maximum allowed bitrate for transcoded music streams. | [optional]
**max_static_music_bitrate** | Option<**i32**> | Gets or sets the maximum allowed bitrate for statically streamed (= direct played) music files. | [optional]
**direct_play_profiles** | Option<[**Vec<models::DirectPlayProfile>**](DirectPlayProfile.md)> | Gets or sets the direct play profiles. | [optional]
**transcoding_profiles** | Option<[**Vec<models::TranscodingProfile>**](TranscodingProfile.md)> | Gets or sets the transcoding profiles. | [optional]
**container_profiles** | Option<[**Vec<models::ContainerProfile>**](ContainerProfile.md)> | Gets or sets the container profiles. Failing to meet these optional conditions causes transcoding to occur. | [optional]
**codec_profiles** | Option<[**Vec<models::CodecProfile>**](CodecProfile.md)> | Gets or sets the codec profiles. | [optional]
**subtitle_profiles** | Option<[**Vec<models::SubtitleProfile>**](SubtitleProfile.md)> | Gets or sets the subtitle profiles. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


