# TrickplayOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enable_hw_acceleration** | Option<**bool**> | Gets or sets a value indicating whether or not to use HW acceleration. | [optional]
**enable_hw_encoding** | Option<**bool**> | Gets or sets a value indicating whether or not to use HW accelerated MJPEG encoding. | [optional]
**enable_key_frame_only_extraction** | Option<**bool**> | Gets or sets a value indicating whether to only extract key frames. Significantly faster, but is not compatible with all decoders and/or video files. | [optional]
**scan_behavior** | Option<[**models::TrickplayScanBehavior**](TrickplayScanBehavior.md)> | Gets or sets the behavior used by trickplay provider on library scan/update. | [optional]
**process_priority** | Option<[**models::ProcessPriorityClass**](ProcessPriorityClass.md)> | Gets or sets the process priority for the ffmpeg process. | [optional]
**interval** | Option<**i32**> | Gets or sets the interval, in ms, between each new trickplay image. | [optional]
**width_resolutions** | Option<**Vec<i32>**> | Gets or sets the target width resolutions, in px, to generates preview images for. | [optional]
**tile_width** | Option<**i32**> | Gets or sets number of tile images to allow in X dimension. | [optional]
**tile_height** | Option<**i32**> | Gets or sets number of tile images to allow in Y dimension. | [optional]
**qscale** | Option<**i32**> | Gets or sets the ffmpeg output quality level. | [optional]
**jpeg_quality** | Option<**i32**> | Gets or sets the jpeg quality to use for image tiles. | [optional]
**process_threads** | Option<**i32**> | Gets or sets the number of threads to be used by ffmpeg. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


