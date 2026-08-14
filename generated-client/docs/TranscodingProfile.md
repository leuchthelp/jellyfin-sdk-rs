# TranscodingProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container** | Option<**String**> | Gets or sets the container. | [optional]
**r#type** | Option<[**models::DlnaProfileType**](DlnaProfileType.md)> | Gets or sets the DLNA profile type. | [optional]
**video_codec** | Option<**String**> | Gets or sets the video codec. | [optional]
**audio_codec** | Option<**String**> | Gets or sets the audio codec. | [optional]
**protocol** | Option<[**models::MediaStreamProtocol**](MediaStreamProtocol.md)> | Gets or sets the protocol. | [optional]
**estimate_content_length** | Option<**bool**> | Gets or sets a value indicating whether the content length should be estimated. | [optional][default to false]
**enable_mpegts_m2_ts_mode** | Option<**bool**> | Gets or sets a value indicating whether M2TS mode is enabled. | [optional][default to false]
**transcode_seek_info** | Option<[**models::TranscodeSeekInfo**](TranscodeSeekInfo.md)> | Gets or sets the transcoding seek info mode. | [optional][default to Auto]
**copy_timestamps** | Option<**bool**> | Gets or sets a value indicating whether timestamps should be copied. | [optional][default to false]
**context** | Option<[**models::EncodingContext**](EncodingContext.md)> | Gets or sets the encoding context. | [optional][default to Streaming]
**enable_subtitles_in_manifest** | Option<**bool**> | Gets or sets a value indicating whether subtitles are allowed in the manifest. | [optional][default to false]
**max_audio_channels** | Option<**String**> | Gets or sets the maximum audio channels. | [optional]
**min_segments** | Option<**i32**> | Gets or sets the minimum amount of segments. | [optional][default to 0]
**segment_length** | Option<**i32**> | Gets or sets the segment length. | [optional][default to 0]
**break_on_non_key_frames** | Option<**bool**> | Gets or sets a value indicating whether breaking the video stream on non-keyframes is supported. | [optional][default to false]
**conditions** | Option<[**Vec<models::ProfileCondition>**](ProfileCondition.md)> | Gets or sets the profile conditions. | [optional]
**enable_audio_vbr_encoding** | Option<**bool**> | Gets or sets a value indicating whether variable bitrate encoding is supported. | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


