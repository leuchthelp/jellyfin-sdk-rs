# PluginUninstalledMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | Option<[**models::PluginInfo**](PluginInfo.md)> | This is a serializable stub class that is used by the api to provide information about installed plugins. | [optional]
**message_id** | Option<**uuid::Uuid**> | Gets or sets the message id. | [optional]
**message_type** | Option<[**models::SessionMessageType**](SessionMessageType.md)> | The different kinds of messages that are used in the WebSocket api. | [optional][readonly][default to PackageUninstalled]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


