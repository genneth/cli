---
name: gws-people
description: "Google People: Manage contacts and profiles."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws people --help"
---

# people (v1)

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

```bash
gws people <resource> <method> [flags]
```

## API Resources

### contactGroups

  - `batchGet` — Get a list of contact groups owned by the authenticated user by specifying a list of contact group resource names.
    - Response type: `BatchGetContactGroupsResponse`
  - `create` — Create a new contact group owned by the authenticated user. Created contact group names must be unique to the users contact groups. Attempting to create a group with a duplicate name will return a HTTP 409 error. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.
    - Request body type: `CreateContactGroupRequest`
    - Response type: `ContactGroup`
  - `delete` — Delete an existing contact group owned by the authenticated user by specifying a contact group resource name. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.
    - Required path params: resourceName
    - Response type: `Empty`
  - `get` — Get a specific contact group owned by the authenticated user by specifying a contact group resource name.
    - Required path params: resourceName
    - Response type: `ContactGroup`
  - `list` — List all contact groups owned by the authenticated user. Members of the contact groups are not populated.
    - Response type: `ListContactGroupsResponse`
  - `update` — Update the name of an existing contact group owned by the authenticated user. Updated contact group names must be unique to the users contact groups. Attempting to create a group with a duplicate name will return a HTTP 409 error. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.
    - Required path params: resourceName
    - Request body type: `UpdateContactGroupRequest`
    - Response type: `ContactGroup`

### contactGroups.members

  - `modify` — Modify the members of a contact group owned by the authenticated user. The only system contact groups that can have members added are `contactGroups/myContacts` and `contactGroups/starred`. Other system contact groups are deprecated and can only have contacts removed.
    - Required path params: resourceName
    - Request body type: `ModifyContactGroupMembersRequest`
    - Response type: `ModifyContactGroupMembersResponse`

### otherContacts

  - `copyOtherContactToMyContactsGroup` — Copies an "Other contact" to a new contact in the user's "myContacts" group Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.
    - Required path params: resourceName
    - Request body type: `CopyOtherContactToMyContactsGroupRequest`
    - Response type: `Person`
  - `list` — List all "Other contacts", that is contacts that are not in a contact group. "Other contacts" are typically auto created contacts from interactions. Sync tokens expire 7 days after the full sync. A request with an expired sync token will get an error with an [google.rpc.ErrorInfo](https://cloud.google.com/apis/design/errors#error_info) with reason "EXPIRED_SYNC_TOKEN". In the case of such an error clients should make a full sync request without a `sync_token`.
    - Response type: `ListOtherContactsResponse`
  - `search` — Provides a list of contacts in the authenticated user's other contacts that matches the search query. The query matches on a contact's `names`, `emailAddresses`, and `phoneNumbers` fields that are from the OTHER_CONTACT source. **IMPORTANT**: Before searching, clients should send a warmup request with an empty query to update the cache. See https://developers.google.com/people/v1/other-contacts#search_the_users_other_contacts
    - Response type: `SearchResponse`

### people

  - `batchCreateContacts` — Create a batch of new contacts and return the PersonResponses for the newly Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.
    - Request body type: `BatchCreateContactsRequest`
    - Response type: `BatchCreateContactsResponse`
  - `batchUpdateContacts` — Update a batch of contacts and return a map of resource names to PersonResponses for the updated contacts. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.
    - Request body type: `BatchUpdateContactsRequest`
    - Response type: `BatchUpdateContactsResponse`
  - `createContact` — Create a new contact and return the person resource for that contact. The request returns a 400 error if more than one field is specified on a field that is a singleton for contact sources: * biographies * birthdays * genders * names Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.
    - Request body type: `Person`
    - Response type: `Person`
  - `deleteContactPhoto` — Delete a contact's photo. Mutate requests for the same user should be done sequentially to avoid // lock contention.
    - Required path params: resourceName
    - Response type: `DeleteContactPhotoResponse`
  - `get` — Provides information about a person by specifying a resource name. Use `people/me` to indicate the authenticated user. The request returns a 400 error if 'personFields' is not specified.
    - Required path params: resourceName
    - Response type: `Person`
  - `getBatchGet` — Provides information about a list of specific people by specifying a list of requested resource names. Use `people/me` to indicate the authenticated user. The request returns a 400 error if 'personFields' is not specified.
    - Response type: `GetPeopleResponse`
  - `listDirectoryPeople` — Provides a list of domain profiles and domain contacts in the authenticated user's domain directory. When the `sync_token` is specified, resources deleted since the last sync will be returned as a person with `PersonMetadata.deleted` set to true. When the `page_token` or `sync_token` is specified, all other request parameters must match the first call. Writes may have a propagation delay of several minutes for sync requests. Incremental syncs are not intended for read-after-write use cases.
    - Response type: `ListDirectoryPeopleResponse`
  - `searchContacts` — Provides a list of contacts in the authenticated user's grouped contacts that matches the search query. The query matches on a contact's `names`, `nickNames`, `emailAddresses`, `phoneNumbers`, and `organizations` fields that are from the CONTACT source. **IMPORTANT**: Before searching, clients should send a warmup request with an empty query to update the cache. See https://developers.google.com/people/v1/contacts#search_the_users_contacts
    - Response type: `SearchResponse`
  - `searchDirectoryPeople` — Provides a list of domain profiles and domain contacts in the authenticated user's domain directory that match the search query.
    - Response type: `SearchDirectoryPeopleResponse`
  - `updateContact` — Update contact data for an existing contact person. Any non-contact data will not be modified. Any non-contact data in the person to update will be ignored. All fields specified in the `update_mask` will be replaced. The server returns a 400 error if `person.metadata.sources` is not specified for the contact to be updated or if there is no contact source.
    - Required path params: resourceName
    - Request body type: `Person`
    - Response type: `Person`
  - `updateContactPhoto` — Update a contact's photo. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.
    - Required path params: resourceName
    - Request body type: `UpdateContactPhotoRequest`
    - Response type: `UpdateContactPhotoResponse`

### people.connections

  - `list` — Provides a list of the authenticated user's contacts. Sync tokens expire 7 days after the full sync. A request with an expired sync token will get an error with an [google.rpc.ErrorInfo](https://cloud.google.com/apis/design/errors#error_info) with reason "EXPIRED_SYNC_TOKEN". In the case of such an error clients should make a full sync request without a `sync_token`. The first page of a full sync request has an additional quota. If the quota is exceeded, a 429 error will be returned.
    - Required path params: resourceName
    - Response type: `ListConnectionsResponse`

## Common Schemas

### BatchCreateContactsRequest

*Description: A request to create a batch of contacts.*

| Field | Type | Description |
|---|---|---|
| `contacts` | array of `ContactToCreate` | Required. The contact to create. Allows up to 200 contacts in a single request. |
| `readMask` | string (format: google-fieldmask) | Required. A field mask to restrict which fields on each person are returned in the response. Multiple fields can be specified by separating them with commas. |
| `sources` | array of string | Optional. A mask of what source types to return in the post mutate read. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set. |

### BatchCreateContactsResponse

*Description: If not successful, returns BatchCreateContactsErrorDetails which contains a list of errors for each invalid contact. The response to a request to create a batch of contacts.*

| Field | Type | Description |
|---|---|---|
| `createdPeople` | array of `PersonResponse` | The contacts that were created, unless the request `read_mask` is empty. |

### BatchGetContactGroupsResponse

*Description: The response to a batch get contact groups request.*

| Field | Type | Description |
|---|---|---|
| `responses` | array of `ContactGroupResponse` | The list of responses for each requested contact group resource. |

### BatchUpdateContactsRequest

*Description: A request to update a batch of contacts.*

| Field | Type | Description |
|---|---|---|
| `contacts` | object | Required. A map of resource names to the person data to be updated. Allows up to 200 contacts in a single request. |
| `readMask` | string (format: google-fieldmask) | Required. A field mask to restrict which fields on each person are returned. Multiple fields can be specified by separating them with commas. |
| `sources` | array of string | Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set. |
| `updateMask` | string (format: google-fieldmask) | Required. A field mask to restrict which fields on the person are updated. Multiple fields can be specified by separating them with commas. |

### BatchUpdateContactsResponse

*Description: If not successful, returns BatchUpdateContactsErrorDetails, a list of errors corresponding to each contact. The response to a request to update a batch of contacts.*

| Field | Type | Description |
|---|---|---|
| `updateResult` | object | A map of resource names to the contacts that were updated, unless the request `read_mask` is empty. |

### ContactGroup

*Description: A contact group.*

| Field | Type | Description |
|---|---|---|
| `clientData` | array of `GroupClientData` | The group's client data. |
| `etag` | string | The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the resource. Used for web cache validation. |
| `formattedName` | string | Output only. The name translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale for system groups names. Group names set by the owner are the same as name. |
| `groupType` | string | Output only. The contact group type. |
| `memberCount` | integer (format: int32) | Output only. The total number of contacts in the group irrespective of max members in specified in the request. |
| `memberResourceNames` | array of string | Output only. The list of contact person resource names that are members of the contact group. |
| `metadata` | `ContactGroupMetadata` | Output only. Metadata about the contact group. |
| `name` | string | The contact group name set by the group owner or a system provided name for system groups. |
| `resourceName` | string | The resource name for the contact group, assigned by the server. An ASCII string, in the form of `contactGroups/{contact_group_id}`. |

### CopyOtherContactToMyContactsGroupRequest

*Description: A request to copy an "Other contact" to my contacts group.*

| Field | Type | Description |
|---|---|---|
| `copyMask` | string (format: google-fieldmask) | Required. A field mask to restrict which fields are copied into the new contact. Valid values are: * emailAddresses * names * phoneNumbers |
| `readMask` | string (format: google-fieldmask) | Optional. A field mask to restrict which fields on the person are returned. Multiple fields can be specified by separating them with commas. |
| `sources` | array of string | Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set. |

### CreateContactGroupRequest

*Description: A request to create a new contact group.*

| Field | Type | Description |
|---|---|---|
| `contactGroup` | `ContactGroup` | Required. The contact group to create. |
| `readGroupFields` | string (format: google-fieldmask) | Optional. A field mask to restrict which fields on the group are returned. Defaults to `metadata`, `groupType`, and `name` if not set or set to empty. |

### DeleteContactPhotoResponse

*Description: The response for deleting a contact's photo.*

| Field | Type | Description |
|---|---|---|
| `person` | `Person` | The updated person, if person_fields is set in the DeleteContactPhotoRequest; otherwise this will be unset. |

### Empty

*Description: A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }*

*(No fields)*

### GetPeopleResponse

*Description: The response to a get request for a list of people by resource name.*

| Field | Type | Description |
|---|---|---|
| `responses` | array of `PersonResponse` | The response for each requested resource name. |

### ListConnectionsResponse

*Description: The response to a request for the authenticated user's connections.*

| Field | Type | Description |
|---|---|---|
| `connections` | array of `Person` | The list of people that the requestor is connected to. |
| `nextPageToken` | string | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `nextSyncToken` | string | A token, which can be sent as `sync_token` to retrieve changes since the last request. Request must set `request_sync_token` to return the sync token. |
| `totalItems` | integer (format: int32) | The total number of items in the list without pagination. |
| `totalPeople` | integer (format: int32) | **DEPRECATED** (Please use totalItems) The total number of people in the list without pagination. |

### ListContactGroupsResponse

*Description: The response to a list contact groups request.*

| Field | Type | Description |
|---|---|---|
| `contactGroups` | array of `ContactGroup` | The list of contact groups. Members of the contact groups are not populated. |
| `nextPageToken` | string | The token that can be used to retrieve the next page of results. |
| `nextSyncToken` | string | The token that can be used to retrieve changes since the last request. |
| `totalItems` | integer (format: int32) | The total number of items in the list without pagination. |

### ListDirectoryPeopleResponse

*Description: The response to a request for the authenticated user's domain directory.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `nextSyncToken` | string | A token, which can be sent as `sync_token` to retrieve changes since the last request. Request must set `request_sync_token` to return the sync token. |
| `people` | array of `Person` | The list of people in the domain directory. |

### ListOtherContactsResponse

*Description: The response to a request for the authenticated user's "Other contacts".*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `nextSyncToken` | string | A token, which can be sent as `sync_token` to retrieve changes since the last request. Request must set `request_sync_token` to return the sync token. |
| `otherContacts` | array of `Person` | The list of "Other contacts" returned as Person resources. "Other contacts" support a limited subset of fields. See ListOtherContactsRequest.request_mask for more detailed information. |
| `totalSize` | integer (format: int32) | The total number of other contacts in the list without pagination. |

### ModifyContactGroupMembersRequest

*Description: A request to modify an existing contact group's members. Contacts can be removed from any group but they can only be added to a user group or "myContacts" or "starred" system groups.*

| Field | Type | Description |
|---|---|---|
| `resourceNamesToAdd` | array of string | Optional. The resource names of the contact people to add in the form of `people/{person_id}`. |
| `resourceNamesToRemove` | array of string | Optional. The resource names of the contact people to remove in the form of `people/{person_id}`. |

### ModifyContactGroupMembersResponse

*Description: The response to a modify contact group members request.*

| Field | Type | Description |
|---|---|---|
| `canNotRemoveLastContactGroupResourceNames` | array of string | The contact people resource names that cannot be removed from their last contact group. |
| `notFoundResourceNames` | array of string | The contact people resource names that were not found. |

### Person

*Description: Information about a person merged from various data sources such as the authenticated user's contacts and profile data. Most fields can have multiple items.*

| Field | Type | Description |
|---|---|---|
| `addresses` | array of `Address` | The person's street addresses. |
| `ageRange` | string | Output only. **DEPRECATED** (Please use `person.ageRanges` instead) The person's age range. |
| `ageRanges` | array of `AgeRangeType` | Output only. The person's age ranges. |
| `biographies` | array of `Biography` | The person's biographies. This field is a singleton for contact sources. |
| `birthdays` | array of `Birthday` | The person's birthdays. This field is a singleton for contact sources. |
| `braggingRights` | array of `BraggingRights` | **DEPRECATED**: No data will be returned The person's bragging rights. |
| `calendarUrls` | array of `CalendarUrl` | The person's calendar URLs. |
| `clientData` | array of `ClientData` | The person's client data. |
| `coverPhotos` | array of `CoverPhoto` | Output only. The person's cover photos. |
| `emailAddresses` | array of `EmailAddress` | The person's email addresses. For `people.connections.list` and `otherContacts.list` the number of email addresses is limited to 100. |
| `etag` | string | The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the resource. Used for web cache validation. |
| `events` | array of `Event` | The person's events. |
| `externalIds` | array of `ExternalId` | The person's external IDs. |
| `fileAses` | array of `FileAs` | The person's file-ases. |
| `genders` | array of `Gender` | The person's genders. This field is a singleton for contact sources. |
| `imClients` | array of `ImClient` | The person's instant messaging clients. |
| `interests` | array of `Interest` | The person's interests. |
| `locales` | array of `Locale` | The person's locale preferences. |
| `locations` | array of `Location` | The person's locations. |
| `memberships` | array of `Membership` | The person's group memberships. |
| `metadata` | `PersonMetadata` | Output only. Metadata about the person. |
| `miscKeywords` | array of `MiscKeyword` | The person's miscellaneous keywords. |
| `names` | array of `Name` | The person's names. This field is a singleton for contact sources. |
| `nicknames` | array of `Nickname` | The person's nicknames. |
| `occupations` | array of `Occupation` | The person's occupations. |
| `organizations` | array of `Organization` | The person's past or current organizations. |
| `phoneNumbers` | array of `PhoneNumber` | The person's phone numbers. For `people.connections.list` and `otherContacts.list` the number of phone numbers is limited to 100. |
| `photos` | array of `Photo` | Output only. The person's photos. |
| `relations` | array of `Relation` | The person's relations. |
| `relationshipInterests` | array of `RelationshipInterest` | Output only. **DEPRECATED**: No data will be returned The person's relationship interests. |
| `relationshipStatuses` | array of `RelationshipStatus` | Output only. **DEPRECATED**: No data will be returned The person's relationship statuses. |
| `residences` | array of `Residence` | **DEPRECATED**: (Please use `person.locations` instead) The person's residences. |
| `resourceName` | string | The resource name for the person, assigned by the server. An ASCII string in the form of `people/{person_id}`. |
| `sipAddresses` | array of `SipAddress` | The person's SIP addresses. |
| `skills` | array of `Skill` | The person's skills. |
| `taglines` | array of `Tagline` | Output only. **DEPRECATED**: No data will be returned The person's taglines. |
| `urls` | array of `Url` | The person's associated URLs. |
| `userDefined` | array of `UserDefined` | The person's user defined data. |

### SearchDirectoryPeopleResponse

*Description: The response to a request for people in the authenticated user's domain directory that match the specified query.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `people` | array of `Person` | The list of people in the domain directory that match the query. |
| `totalSize` | integer (format: int32) | The total number of items in the list without pagination. |

### SearchResponse

*Description: The response to a search request for the authenticated user, given a query.*

| Field | Type | Description |
|---|---|---|
| `results` | array of `SearchResult` | The results of the request. |

### UpdateContactGroupRequest

*Description: A request to update an existing user contact group. All updated fields will be replaced.*

| Field | Type | Description |
|---|---|---|
| `contactGroup` | `ContactGroup` | Required. The contact group to update. |
| `readGroupFields` | string (format: google-fieldmask) | Optional. A field mask to restrict which fields on the group are returned. Defaults to `metadata`, `groupType`, and `name` if not set or set to empty. |
| `updateGroupFields` | string (format: google-fieldmask) | Optional. A field mask to restrict which fields on the group are updated. Multiple fields can be specified by separating them with commas. Defaults to `name` if not set or set to empty. |

### UpdateContactPhotoRequest

*Description: A request to update an existing contact's photo. All requests must have a valid photo format: JPEG or PNG.*

| Field | Type | Description |
|---|---|---|
| `personFields` | string (format: google-fieldmask) | Optional. A field mask to restrict which fields on the person are returned. Multiple fields can be specified by separating them with commas. |
| `photoBytes` | string (format: byte) | Required. Raw photo bytes |
| `sources` | array of string | Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set. |

### UpdateContactPhotoResponse

*Description: The response for updating a contact's photo.*

| Field | Type | Description |
|---|---|---|
| `person` | `Person` | The updated person, if person_fields is set in the UpdateContactPhotoRequest; otherwise this will be unset. |

## Discovering Commands

Before calling any API method, inspect it:

```bash
# Browse resources and methods
gws people --help

# Inspect a method's required params, types, and defaults
gws schema people.<resource>.<method>
```

Use `gws schema` output to build your `--params` and `--json` flags.

