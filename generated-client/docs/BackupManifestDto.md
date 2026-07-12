# BackupManifestDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**server_version** | **String** | Gets or sets the jellyfin version this backup was created with. | 
**backup_engine_version** | **String** | Gets or sets the backup engine version this backup was created with. | 
**date_created** | **chrono::DateTime<chrono::FixedOffset>** | Gets or sets the date this backup was created with. | 
**path** | **String** | Gets or sets the path to the backup on the system. | 
**options** | [**models::BackupOptionsDto**](BackupOptionsDto.md) | Gets or sets the contents of the backup archive. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


