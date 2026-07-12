# DatabaseConfigurationOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**database_type** | **String** | Gets or Sets the type of database jellyfin should use. | 
**custom_provider_options** | Option<[**models::CustomDatabaseOptions**](CustomDatabaseOptions.md)> | Gets or sets the options required to use a custom database provider. | [optional]
**locking_behavior** | Option<[**models::DatabaseLockingBehaviorTypes**](DatabaseLockingBehaviorTypes.md)> | Gets or Sets the kind of locking behavior jellyfin should perform. Possible options are \"NoLock\", \"Pessimistic\", \"Optimistic\". Defaults to \"NoLock\". | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


