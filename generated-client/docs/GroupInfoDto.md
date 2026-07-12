# GroupInfoDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group_id** | Option<**uuid::Uuid**> | Gets the group identifier. | [optional]
**group_name** | Option<**String**> | Gets the group name. | [optional]
**state** | Option<[**models::GroupStateType**](GroupStateType.md)> | Gets the group state. | [optional]
**participants** | Option<**Vec<String>**> | Gets the participants. | [optional]
**last_updated_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Gets the date when this DTO has been created. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


