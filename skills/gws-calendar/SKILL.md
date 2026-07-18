---
name: gws-calendar
description: "Google Calendar: Manage calendars and events."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws calendar --help"
---

# calendar (v3)

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

```bash
gws calendar <resource> <method> [flags]
```

## Helper Commands

| Command | Description |
|---------|-------------|
| [`+insert`](../gws-calendar-insert/SKILL.md) | create a new event |
| [`+agenda`](../gws-calendar-agenda/SKILL.md) | Show upcoming events across all calendars |

## API Resources

### acl

  - `delete` — Deletes an access control rule.
    - Required path params: calendarId, ruleId
  - `get` — Returns an access control rule.
    - Required path params: calendarId, ruleId
    - Response type: `AclRule`
  - `insert` — Creates an access control rule.
    - Required path params: calendarId
    - Request body type: `AclRule`
    - Response type: `AclRule`
  - `list` — Returns the rules in the access control list for the calendar.
    - Required path params: calendarId
    - Response type: `Acl`
  - `patch` — Updates an access control rule. This method supports patch semantics.
    - Required path params: calendarId, ruleId
    - Request body type: `AclRule`
    - Response type: `AclRule`
  - `update` — Updates an access control rule.
    - Required path params: calendarId, ruleId
    - Request body type: `AclRule`
    - Response type: `AclRule`
  - `watch` — Watch for changes to ACL resources.
    - Required path params: calendarId
    - Request body type: `Channel`
    - Response type: `Channel`

### calendarList

  - `delete` — Removes a calendar from the user's calendar list.
    - Required path params: calendarId
  - `get` — Returns a calendar from the user's calendar list.
    - Required path params: calendarId
    - Response type: `CalendarListEntry`
  - `insert` — Inserts an existing calendar into the user's calendar list.
    - Request body type: `CalendarListEntry`
    - Response type: `CalendarListEntry`
  - `list` — Returns the calendars on the user's calendar list.
    - Response type: `CalendarList`
  - `patch` — Updates an existing calendar on the user's calendar list. This method supports patch semantics.
    - Required path params: calendarId
    - Request body type: `CalendarListEntry`
    - Response type: `CalendarListEntry`
  - `update` — Updates an existing calendar on the user's calendar list.
    - Required path params: calendarId
    - Request body type: `CalendarListEntry`
    - Response type: `CalendarListEntry`
  - `watch` — Watch for changes to CalendarList resources.
    - Request body type: `Channel`
    - Response type: `Channel`

### calendars

  - `clear` — Clears a primary calendar. This operation deletes all events associated with the primary calendar of an account.
    - Required path params: calendarId
  - `delete` — Deletes a secondary calendar. Use calendars.clear for clearing all events on primary calendars.
    - Required path params: calendarId
  - `get` — Returns metadata for a calendar.
    - Required path params: calendarId
    - Response type: `Calendar`
  - `insert` — Creates a secondary calendar.
The authenticated user for the request is made the data owner of the new calendar.

Note: We recommend to authenticate as the intended data owner of the calendar. You can use domain-wide delegation of authority to allow applications to act on behalf of a specific user. Don't use a service account for authentication. If you use a service account for authentication, the service account is the data owner, which can lead to unexpected behavior.
    - Request body type: `Calendar`
    - Response type: `Calendar`
  - `patch` — Updates metadata for a calendar. This method supports patch semantics.
    - Required path params: calendarId
    - Request body type: `Calendar`
    - Response type: `Calendar`
  - `transferOwnership` — Transfers a secondary calendar between users within a Google Workspace organization. Requires user authentication with Manage Calendars administrator privilege, and one of the following authorization scopes: 
- https://www.googleapis.com/auth/calendar 
- https://www.googleapis.com/auth/calendar.calendars In the request, set useAdminAccess to true. The secondary calendar must be active to be transferred. Transferring disabled or deleted calendars isn't supported.
    - Required path params: calendarId
    - Required query params: newDataOwner, useAdminAccess
  - `update` — Updates metadata for a calendar.
    - Required path params: calendarId
    - Request body type: `Calendar`
    - Response type: `Calendar`

### channels

  - `stop` — Stop watching resources through this channel
    - Request body type: `Channel`

### colors

  - `get` — Returns the color definitions for calendars and events.
    - Response type: `Colors`

### events

  - `delete` — Deletes an event.
    - Required path params: calendarId, eventId
  - `get` — Returns an event based on its Google Calendar ID. To retrieve an event using its iCalendar ID, call the events.list method using the iCalUID parameter.
    - Required path params: calendarId, eventId
    - Response type: `Event`
  - `import` — Imports an event. This operation is used to add a private copy of an existing event to a calendar. Only events with an eventType of default may be imported.
Deprecated behavior: If a non-default event is imported, its type will be changed to default and any event-type-specific properties it may have will be dropped.
    - Required path params: calendarId
    - Request body type: `Event`
    - Response type: `Event`
  - `insert` — Creates an event.
    - Required path params: calendarId
    - Request body type: `Event`
    - Response type: `Event`
  - `instances` — Returns instances of the specified recurring event.
    - Required path params: calendarId, eventId
    - Response type: `Events`
  - `list` — Returns events on the specified calendar.
    - Required path params: calendarId
    - Response type: `Events`
  - `move` — Moves an event to another calendar, i.e. changes an event's organizer. Note that only default events can be moved; birthday, focusTime, fromGmail, outOfOffice and workingLocation events cannot be moved.
    - Required path params: calendarId, eventId
    - Required query params: destination
    - Response type: `Event`
  - `patch` — Updates an event. This method supports patch semantics.
    - Required path params: calendarId, eventId
    - Request body type: `Event`
    - Response type: `Event`
  - `quickAdd` — Creates an event based on a simple text string.
    - Required path params: calendarId
    - Required query params: text
    - Response type: `Event`
  - `update` — Updates an event.
    - Required path params: calendarId, eventId
    - Request body type: `Event`
    - Response type: `Event`
  - `watch` — Watch for changes to Events resources.
    - Required path params: calendarId
    - Request body type: `Channel`
    - Response type: `Channel`

### freebusy

  - `query` — Returns free/busy information for a set of calendars.
    - Request body type: `FreeBusyRequest`
    - Response type: `FreeBusyResponse`

### settings

  - `get` — Returns a single user setting.
    - Required path params: setting
    - Response type: `Setting`
  - `list` — Returns all user settings for the authenticated user.
    - Response type: `Settings`
  - `watch` — Watch for changes to Settings resources.
    - Request body type: `Channel`
    - Response type: `Channel`

## Common Schemas

### Acl

| Field | Type | Description |
|---|---|---|
| `etag` | string | ETag of the collection. |
| `items` | array of `AclRule` | List of rules on the access control list. |
| `kind` | string | Type of the collection ("calendar#acl"). |
| `nextPageToken` | string | Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided. |
| `nextSyncToken` | string | Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided. |

### AclRule

| Field | Type | Description |
|---|---|---|
| `etag` | string | ETag of the resource. |
| `id` | string | Identifier of the Access Control List (ACL) rule. See Sharing calendars. |
| `kind` | string | Type of the resource ("calendar#aclRule"). |
| `role` | string | The role assigned to the scope. Possible values are:  
- "none" - Provides no access. 
- "freeBusyReader" - Provides read access to free/busy information. |
| `scope` | object | The extent to which calendar access is granted by this ACL rule. |

### Calendar

| Field | Type | Description |
|---|---|---|
| `autoAcceptInvitations` | boolean | Whether this calendar automatically accepts invitations. Only valid for resource calendars. |
| `conferenceProperties` | `ConferenceProperties` | Conferencing properties for this calendar, for example what types of conferences are allowed. |
| `dataOwner` | string | The email of the owner of the calendar. Set only for secondary calendars. Read-only. |
| `description` | string | Description of the calendar. Optional. |
| `etag` | string | ETag of the resource. |
| `id` | string | Identifier of the calendar. To retrieve IDs call the calendarList.list() method. |
| `kind` | string | Type of the resource ("calendar#calendar"). |
| `labelProperties` | `LabelProperties` | Label properties defined on this calendar. If specified, overwrites the existing label properties. If not specified, the label properties remain unchanged. |
| `location` | string | Geographic location of the calendar as free-form text. Optional. |
| `summary` | string | Title of the calendar. |
| `timeZone` | string | The time zone of the calendar. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) Optional. |

### CalendarList

| Field | Type | Description |
|---|---|---|
| `etag` | string | ETag of the collection. |
| `items` | array of `CalendarListEntry` | Calendars that are present on the user's calendar list. |
| `kind` | string | Type of the collection ("calendar#calendarList"). |
| `nextPageToken` | string | Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided. |
| `nextSyncToken` | string | Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided. |

### CalendarListEntry

| Field | Type | Description |
|---|---|---|
| `accessRole` | string | The effective access role that the authenticated user has on the calendar. Read-only. Possible values are:  
- "freeBusyReader" - Provides read access to free/busy information. |
| `autoAcceptInvitations` | boolean | Whether this calendar automatically accepts invitations. Only valid for resource calendars. Read-only. |
| `backgroundColor` | string | The main color of the calendar in the hexadecimal format "#0088aa". This property supersedes the index-based colorId property. |
| `colorId` | string | The color of the calendar. This is an ID referring to an entry in the calendar section of the colors definition (see the colors endpoint). |
| `conferenceProperties` | `ConferenceProperties` | Conferencing properties for this calendar, for example what types of conferences are allowed. |
| `dataOwner` | string | The email of the owner of the calendar. Set only for secondary calendars. Read-only. |
| `defaultReminders` | array of `EventReminder` | The default reminders that the authenticated user has for this calendar. |
| `deleted` | boolean | Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False. |
| `description` | string | Description of the calendar. Optional. Read-only. |
| `etag` | string | ETag of the resource. |
| `foregroundColor` | string | The foreground color of the calendar in the hexadecimal format "#ffffff". This property supersedes the index-based colorId property. |
| `hidden` | boolean | Whether the calendar has been hidden from the list. Optional. The attribute is only returned when the calendar is hidden, in which case the value is true. |
| `id` | string | Identifier of the calendar. |
| `kind` | string | Type of the resource ("calendar#calendarListEntry"). |
| `location` | string | Geographic location of the calendar as free-form text. Optional. Read-only. |
| `notificationSettings` | object | The notifications that the authenticated user is receiving for this calendar. |
| `primary` | boolean | Whether the calendar is the primary calendar of the authenticated user. Read-only. Optional. The default is False. |
| `selected` | boolean | Whether the calendar content shows up in the calendar UI. Optional. The default is False. |
| `summary` | string | Title of the calendar. Read-only. |
| `summaryOverride` | string | The summary that the authenticated user has set for this calendar. Optional. |
| `timeZone` | string | The time zone of the calendar. Optional. Read-only. |

### Channel

| Field | Type | Description |
|---|---|---|
| `address` | string | The address where notifications are delivered for this channel. |
| `expiration` | string (format: int64) | Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional. |
| `id` | string | A UUID or similar unique string that identifies this channel. |
| `kind` | string | Identifies this as a notification channel used to watch for changes to a resource, which is "api#channel". |
| `params` | object | Additional parameters controlling delivery channel behavior. Optional. |
| `payload` | boolean | A Boolean value to indicate whether payload is wanted. Optional. |
| `resourceId` | string | An opaque ID that identifies the resource being watched on this channel. Stable across different API versions. |
| `resourceUri` | string | A version-specific identifier for the watched resource. |
| `token` | string | An arbitrary string delivered to the target address with each notification delivered over this channel. Optional. |
| `type` | string | The type of delivery mechanism used for this channel. Valid values are "web_hook" (or "webhook"). Both values refer to a channel where Http requests are used to deliver messages. |

### Colors

| Field | Type | Description |
|---|---|---|
| `calendar` | object | A global palette of calendar colors, mapping from the color ID to its definition. A calendarListEntry resource refers to one of these color IDs in its colorId field. Read-only. |
| `event` | object | A global palette of event colors, mapping from the color ID to its definition. An event resource may refer to one of these color IDs in its colorId field. Read-only. |
| `kind` | string | Type of the resource ("calendar#colors"). |
| `updated` | string (format: date-time) | Last modification time of the color palette (as a RFC3339 timestamp). Read-only. |

### Event

| Field | Type | Description |
|---|---|---|
| `anyoneCanAddSelf` | boolean | Whether anyone can invite themselves to the event (deprecated). Optional. The default is False. |
| `attachments` | array of `EventAttachment` | File attachments for the event.
In order to modify attachments the supportsAttachments request parameter should be set to true.
There can be at most 25 attachments per event, |
| `attendees` | array of `EventAttendee` | The attendees of the event. See the Events with attendees guide for more information on scheduling events with other calendar users. |
| `attendeesOmitted` | boolean | Whether attendees may have been omitted from the event's representation. When retrieving an event, this may be due to a restriction specified by the maxAttendee query parameter. |
| `birthdayProperties` | `EventBirthdayProperties` | Birthday or special event data. Used if eventType is "birthday". Immutable. |
| `colorId` | string | The color of the event. This is an ID referring to an entry in the event section of the colors definition (see the  colors endpoint). Optional. |
| `conferenceData` | `ConferenceData` | The conference-related information, such as details of a Google Meet conference. To create new conference details use the createRequest field. |
| `created` | string (format: date-time) | Creation time of the event (as a RFC3339 timestamp). Read-only. |
| `creator` | object | The creator of the event. Read-only. |
| `description` | string | Description of the event. Can contain HTML. Optional. |
| `end` | `EventDateTime` | The (exclusive) end time of the event. For a recurring event, this is the end time of the first instance. |
| `endTimeUnspecified` | boolean | Whether the end time is actually unspecified. An end time is still provided for compatibility reasons, even if this attribute is set to True. The default is False. |
| `etag` | string | ETag of the resource. |
| `eventLabelId` | string | The ID of the event label assigned to the event. Optional. |
| `eventType` | string | Specific type of the event. This cannot be modified after the event is created. Possible values are:  
- "birthday" - A special all-day event with an annual recurrence. |
| `extendedProperties` | object | Extended properties of the event. |
| `focusTimeProperties` | `EventFocusTimeProperties` | Focus Time event data. Used if eventType is focusTime. |
| `gadget` | object | A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata. |
| `guestsCanInviteOthers` | boolean | Whether attendees other than the organizer can invite others to the event. Optional. The default is True. |
| `guestsCanModify` | boolean | Whether attendees other than the organizer can modify the event. Optional. The default is False. |
| `guestsCanSeeOtherGuests` | boolean | Whether attendees other than the organizer can see who the event's attendees are. Optional. The default is True. |
| `hangoutLink` | string | An absolute link to the Google Hangout associated with this event. Read-only. |
| `htmlLink` | string | An absolute link to this event in the Google Calendar Web UI. Read-only. |
| `iCalUID` | string | Event unique identifier as defined in RFC5545. |
| `id` | string | Opaque identifier of the event. When creating new single or recurring events, you can specify their IDs. |
| `kind` | string | Type of the resource ("calendar#event"). |
| `location` | string | Geographic location of the event as free-form text. Optional. |
| `locked` | boolean | Whether this is a locked event copy where no changes can be made to the main event fields "summary", "description", "location", "start", "end" or "recurrence". The default is False. Read-Only. |
| `organizer` | object | The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. |
| `originalStartTime` | `EventDateTime` | For an instance of a recurring event, this is the time at which this event would start according to the recurrence data in the recurring event identified by recurringEventId. |
| `outOfOfficeProperties` | `EventOutOfOfficeProperties` | Out of office event data. Used if eventType is outOfOffice. |
| `privateCopy` | boolean | If set to True, Event propagation is disabled. Note that it is not the same thing as Private event properties. Optional. Immutable. The default is False. |
| `recurrence` | array of string | List of RRULE, EXRULE, RDATE and EXDATE lines for a recurring event, as specified in RFC5545. |
| `recurringEventId` | string | For an instance of a recurring event, this is the id of the recurring event to which this instance belongs. Immutable. |
| `reminders` | object | Information about the event's reminders for the authenticated user. Note that changing reminders does not also change the updated property of the enclosing event. |
| `sequence` | integer (format: int32) | Sequence number as per iCalendar. |
| `source` | object | Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. |
| `start` | `EventDateTime` | The (inclusive) start time of the event. For a recurring event, this is the start time of the first instance. |
| `status` | string | Status of the event. Optional. Possible values are:  
- "confirmed" - The event is confirmed. This is the default status. 
- "tentative" - The event is tentatively confirmed. |
| `summary` | string | Title of the event. |
| `transparency` | string | Whether the event blocks time on the calendar. Optional. Possible values are:  
- "opaque" - Default value. The event does block time on the calendar. |
| `updated` | string (format: date-time) | Last modification time of the main event data (as a RFC3339 timestamp). Updating event reminders will not cause this to change. Read-only. |
| `visibility` | string | Visibility of the event. Optional. Possible values are:  
- "default" - Uses the default visibility for events on the calendar. This is the default value. |
| `workingLocationProperties` | `EventWorkingLocationProperties` | Working location event data. |

### Events

| Field | Type | Description |
|---|---|---|
| `accessRole` | string | The user's access role for this calendar. Read-only. Possible values are:  
- "none" - The user has no access. 
- "freeBusyReader" - The user has read access to free/busy information. |
| `defaultReminders` | array of `EventReminder` | The default reminders on the calendar for the authenticated user. These reminders apply to all events on this calendar that do not explicitly override them (i.e. |
| `description` | string | Description of the calendar. Read-only. |
| `etag` | string | ETag of the collection. |
| `items` | array of `Event` | List of events on the calendar. |
| `kind` | string | Type of the collection ("calendar#events"). |
| `nextPageToken` | string | Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided. |
| `nextSyncToken` | string | Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided. |
| `summary` | string | Title of the calendar. Read-only. |
| `timeZone` | string | The time zone of the calendar. Read-only. |
| `updated` | string (format: date-time) | Last modification time of the calendar (as a RFC3339 timestamp). Read-only. |

### FreeBusyRequest

| Field | Type | Description |
|---|---|---|
| `calendarExpansionMax` | integer (format: int32) | Maximal number of calendars for which FreeBusy information is to be provided. Optional. Maximum value is 50. |
| `groupExpansionMax` | integer (format: int32) | Maximal number of calendar identifiers to be provided for a single group. Optional. An error is returned for a group with more members than this value. Maximum value is 100. |
| `items` | array of `FreeBusyRequestItem` | List of calendars and/or groups to query. |
| `timeMax` | string (format: date-time) | The end of the interval for the query formatted as per RFC3339. |
| `timeMin` | string (format: date-time) | The start of the interval for the query formatted as per RFC3339. |
| `timeZone` | string | Time zone used in the response. Optional. The default is UTC. |

### FreeBusyResponse

| Field | Type | Description |
|---|---|---|
| `calendars` | object | List of free/busy information for calendars. |
| `groups` | object | Expansion of groups. |
| `kind` | string | Type of the resource ("calendar#freeBusy"). |
| `timeMax` | string (format: date-time) | The end of the interval. |
| `timeMin` | string (format: date-time) | The start of the interval. |

### Setting

| Field | Type | Description |
|---|---|---|
| `etag` | string | ETag of the resource. |
| `id` | string | The id of the user setting. |
| `kind` | string | Type of the resource ("calendar#setting"). |
| `value` | string | Value of the user setting. The format of the value depends on the ID of the setting. It must always be a UTF-8 string of length up to 1024 characters. |

### Settings

| Field | Type | Description |
|---|---|---|
| `etag` | string | Etag of the collection. |
| `items` | array of `Setting` | List of user settings. |
| `kind` | string | Type of the collection ("calendar#settings"). |
| `nextPageToken` | string | Token used to access the next page of this result. Omitted if no further results are available, in which case nextSyncToken is provided. |
| `nextSyncToken` | string | Token used at a later point in time to retrieve only the entries that have changed since this result was returned. Omitted if further results are available, in which case nextPageToken is provided. |

## Discovering Commands

Before calling any API method, inspect it:

```bash
# Browse resources and methods
gws calendar --help

# Inspect a method's required params, types, and defaults
gws schema calendar.<resource>.<method>
```

Use `gws schema` output to build your `--params` and `--json` flags.

