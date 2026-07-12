# NetworkConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**base_url** | Option<**String**> | Gets or sets a value used to specify the URL prefix that your Jellyfin instance can be accessed at. | [optional]
**enable_https** | Option<**bool**> | Gets or sets a value indicating whether to use HTTPS. | [optional]
**require_https** | Option<**bool**> | Gets or sets a value indicating whether the server should force connections over HTTPS. | [optional]
**certificate_path** | Option<**String**> | Gets or sets the filesystem path of an X.509 certificate to use for SSL. | [optional]
**certificate_password** | Option<**String**> | Gets or sets the password required to access the X.509 certificate data in the file specified by MediaBrowser.Common.Net.NetworkConfiguration.CertificatePath. | [optional]
**internal_http_port** | Option<**i32**> | Gets or sets the internal HTTP server port. | [optional]
**internal_https_port** | Option<**i32**> | Gets or sets the internal HTTPS server port. | [optional]
**public_http_port** | Option<**i32**> | Gets or sets the public HTTP port. | [optional]
**public_https_port** | Option<**i32**> | Gets or sets the public HTTPS port. | [optional]
**auto_discovery** | Option<**bool**> | Gets or sets a value indicating whether Autodiscovery is enabled. | [optional]
**enable_upn_p** | Option<**bool**> | Gets or sets a value indicating whether to enable automatic port forwarding. | [optional]
**enable_ipv4** | Option<**bool**> | Gets or sets a value indicating whether IPv6 is enabled. | [optional]
**enable_ipv6** | Option<**bool**> | Gets or sets a value indicating whether IPv6 is enabled. | [optional]
**enable_remote_access** | Option<**bool**> | Gets or sets a value indicating whether access from outside of the LAN is permitted. | [optional]
**local_network_subnets** | Option<**Vec<String>**> | Gets or sets the subnets that are deemed to make up the LAN. | [optional]
**local_network_addresses** | Option<**Vec<String>**> | Gets or sets the interface addresses which Jellyfin will bind to. If empty, all interfaces will be used. | [optional]
**known_proxies** | Option<**Vec<String>**> | Gets or sets the known proxies. | [optional]
**ignore_virtual_interfaces** | Option<**bool**> | Gets or sets a value indicating whether address names that match MediaBrowser.Common.Net.NetworkConfiguration.VirtualInterfaceNames should be ignored for the purposes of binding. | [optional]
**virtual_interface_names** | Option<**Vec<String>**> | Gets or sets a value indicating the interface name prefixes that should be ignored. The list can be comma separated and values are case-insensitive. <seealso cref=\"P:MediaBrowser.Common.Net.NetworkConfiguration.IgnoreVirtualInterfaces\" />. | [optional]
**enable_published_server_uri_by_request** | Option<**bool**> | Gets or sets a value indicating whether the published server uri is based on information in HTTP requests. | [optional]
**published_server_uri_by_subnet** | Option<**Vec<String>**> | Gets or sets the PublishedServerUriBySubnet Gets or sets PublishedServerUri to advertise for specific subnets. | [optional]
**remote_ip_filter** | Option<**Vec<String>**> | Gets or sets the filter for remote IP connectivity. Used in conjunction with <seealso cref=\"P:MediaBrowser.Common.Net.NetworkConfiguration.IsRemoteIPFilterBlacklist\" />. | [optional]
**is_remote_ip_filter_blacklist** | Option<**bool**> | Gets or sets a value indicating whether <seealso cref=\"P:MediaBrowser.Common.Net.NetworkConfiguration.RemoteIPFilter\" /> contains a blacklist or a whitelist. Default is a whitelist. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


