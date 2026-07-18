---
name: gws-meet
description: "Manage Google Meet conferences."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws meet --help"
---

# meet (v2)

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

```bash
gws meet <resource> <method> [flags]
```

## API Resources

### conferenceRecords

  - `get` — Gets a conference record by conference ID.
    - Required path params: name
    - Response type: `ConferenceRecord`
  - `list` — Lists the conference records. By default, ordered by start time and in descending order.
    - Response type: `ListConferenceRecordsResponse`

### conferenceRecords.participants

  - `get` — Gets a participant by participant ID.
    - Required path params: name
    - Response type: `Participant`
  - `list` — Lists the participants in a conference record. By default, ordered by join time and in descending order. This API supports `fields` as standard parameters like every other API. However, when the `fields` request parameter is omitted, this API defaults to `'participants/*, next_page_token'`.
    - Required path params: parent
    - Response type: `ListParticipantsResponse`

### conferenceRecords.participants.participantSessions

  - `get` — Gets a participant session by participant session ID.
    - Required path params: name
    - Response type: `ParticipantSession`
  - `list` — Lists the participant sessions of a participant in a conference record. By default, ordered by join time and in descending order. This API supports `fields` as standard parameters like every other API. However, when the `fields` request parameter is omitted this API defaults to `'participantsessions/*, next_page_token'`.
    - Required path params: parent
    - Response type: `ListParticipantSessionsResponse`

### conferenceRecords.recordings

  - `get` — Gets a recording by recording ID.
    - Required path params: name
    - Response type: `Recording`
  - `list` — Lists the recording resources from the conference record. By default, ordered by start time and in ascending order.
    - Required path params: parent
    - Response type: `ListRecordingsResponse`

### conferenceRecords.smartNotes

  - `get` — Gets smart notes by smart note ID.
    - Required path params: name
    - Response type: `SmartNote`
  - `list` — Lists the set of smart notes from the conference record. By default, ordered by start time and in ascending order.
    - Required path params: parent
    - Response type: `ListSmartNotesResponse`

### conferenceRecords.transcripts

  - `get` — Gets a transcript by transcript ID.
    - Required path params: name
    - Response type: `Transcript`
  - `list` — Lists the set of transcripts from the conference record. By default, ordered by start time and in ascending order.
    - Required path params: parent
    - Response type: `ListTranscriptsResponse`

### conferenceRecords.transcripts.entries

  - `get` — Gets a `TranscriptEntry` resource by entry ID. Note: The transcript entries returned by the Google Meet API might not match the transcription found in the Google Docs transcript file. This can occur when 1) we have interleaved speakers within milliseconds, or 2) the Google Docs transcript file is modified after generation.
    - Required path params: name
    - Response type: `TranscriptEntry`
  - `list` — Lists the structured transcript entries per transcript. By default, ordered by start time and in ascending order. Note: The transcript entries returned by the Google Meet API might not match the transcription found in the Google Docs transcript file. This can occur when 1) we have interleaved speakers within milliseconds, or 2) the Google Docs transcript file is modified after generation.
    - Required path params: parent
    - Response type: `ListTranscriptEntriesResponse`

### spaces

  - `create` — Creates a space.
    - Request body type: `Space`
    - Response type: `Space`
  - `endActiveConference` — Ends an active conference (if there's one). For an example, see [End active conference](https://developers.google.com/workspace/meet/api/guides/meeting-spaces#end-active-conference).
    - Required path params: name
    - Request body type: `EndActiveConferenceRequest`
    - Response type: `Empty`
  - `get` — Gets details about a meeting space. For an example, see [Get a meeting space](https://developers.google.com/workspace/meet/api/guides/meeting-spaces#get-meeting-space).
    - Required path params: name
    - Response type: `Space`
  - `patch` — Updates details about a meeting space. For an example, see [Update a meeting space](https://developers.google.com/workspace/meet/api/guides/meeting-spaces#update-meeting-space).
    - Required path params: name
    - Request body type: `Space`
    - Response type: `Space`

## Common Schemas

### ConferenceRecord

*Description: Single instance of a meeting held in a space.*

| Field | Type | Description |
|---|---|---|
| `endTime` | string (format: google-datetime) | Output only. Timestamp when the conference ended. Set for past conferences. Unset if the conference is ongoing. |
| `expireTime` | string (format: google-datetime) | Output only. Server enforced expiration time for when this conference record resource is deleted. The resource is deleted 30 days after the conference ends. |
| `name` | string | Identifier. Resource name of the conference record. Format: `conferenceRecords/{conference_record}` where `{conference_record}` is a unique ID for each instance of a call within a space. |
| `space` | string | Output only. The space where the conference was held. |
| `startTime` | string (format: google-datetime) | Output only. Timestamp when the conference started. Always set. |

### Empty

*Description: A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }*

*(No fields)*

### EndActiveConferenceRequest

*Description: Request to end an ongoing conference of a space.*

*(No fields)*

### ListConferenceRecordsResponse

*Description: Response of ListConferenceRecords method.*

| Field | Type | Description |
|---|---|---|
| `conferenceRecords` | array of `ConferenceRecord` | List of conferences in one page. |
| `nextPageToken` | string | Token to be circulated back for further List call if current List does NOT include all the Conferences. Unset if all conferences have been returned. |

### ListParticipantSessionsResponse

*Description: Response of ListParticipants method.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | Token to be circulated back for further List call if current List doesn't include all the participants. Unset if all participants are returned. |
| `participantSessions` | array of `ParticipantSession` | List of participants in one page. |

### ListParticipantsResponse

*Description: Response of ListParticipants method.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | Token to be circulated back for further List call if current List doesn't include all the participants. Unset if all participants are returned. |
| `participants` | array of `Participant` | List of participants in one page. |
| `totalSize` | integer (format: int32) | Total, exact number of `participants`. By default, this field isn't included in the response. |

### ListRecordingsResponse

*Description: Response for ListRecordings method.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | Token to be circulated back for further List call if current List doesn't include all the recordings. Unset if all recordings are returned. |
| `recordings` | array of `Recording` | List of recordings in one page. |

### ListSmartNotesResponse

*Description: Response for ListSmartNotes method.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | Token to be circulated back for further List call if current List doesn't include all the smart notes. Unset if all smart notes are returned. |
| `smartNotes` | array of `SmartNote` | List of smart notes in one page. |

### ListTranscriptEntriesResponse

*Description: Response for ListTranscriptEntries method.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | Token to be circulated back for further List call if current List doesn't include all the transcript entries. Unset if all entries are returned. |
| `transcriptEntries` | array of `TranscriptEntry` | List of TranscriptEntries in one page. |

### ListTranscriptsResponse

*Description: Response for ListTranscripts method.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | Token to be circulated back for further List call if current List doesn't include all the transcripts. Unset if all transcripts are returned. |
| `transcripts` | array of `Transcript` | List of transcripts in one page. |

### Participant

*Description: User who attended or is attending a conference.*

| Field | Type | Description |
|---|---|---|
| `anonymousUser` | `AnonymousUser` | Anonymous user. |
| `earliestStartTime` | string (format: google-datetime) | Output only. Time when the participant first joined the meeting. |
| `latestEndTime` | string (format: google-datetime) | Output only. Time when the participant left the meeting for the last time. This can be null if it's an active meeting. |
| `name` | string | Output only. Resource name of the participant. Format: `conferenceRecords/{conference_record}/participants/{participant}` |
| `phoneUser` | `PhoneUser` | User calling from their phone. |
| `signedinUser` | `SignedinUser` | Signed-in user. |

### ParticipantSession

*Description: Refers to each unique join or leave session when a user joins a conference from a device. Note that any time a user joins the conference a new unique ID is assigned.*

| Field | Type | Description |
|---|---|---|
| `endTime` | string (format: google-datetime) | Output only. Timestamp when the user session ends. Unset if the user session hasn’t ended. |
| `name` | string | Identifier. Session id. |
| `startTime` | string (format: google-datetime) | Output only. Timestamp when the user session starts. |

### Recording

*Description: Metadata about a recording created during a conference.*

| Field | Type | Description |
|---|---|---|
| `driveDestination` | `DriveDestination` | Output only. Recording is saved to Google Drive as an MP4 file. The `drive_destination` includes the Drive `fileId` that can be used to download the file using the `files.get` method of the Drive API. |
| `endTime` | string (format: google-datetime) | Output only. Timestamp when the recording ended. |
| `name` | string | Output only. Resource name of the recording. |
| `startTime` | string (format: google-datetime) | Output only. Timestamp when the recording started. |
| `state` | string | Output only. Current state. |

### SmartNote

*Description: Metadata for a smart note generated from a conference. It refers to the notes generated from Take Notes with Gemini during the conference.*

| Field | Type | Description |
|---|---|---|
| `docsDestination` | `DocsDestination` | Output only. The Google Doc destination where the smart notes are saved. |
| `endTime` | string (format: google-datetime) | Output only. Timestamp when the smart notes stopped. |
| `name` | string | Output only. Identifier. Resource name of the smart notes. |
| `startTime` | string (format: google-datetime) | Output only. Timestamp when the smart notes started. |
| `state` | string | Output only. Current state. |

### Space

*Description: Virtual place where conferences are held. Only one active conference can be held in one space at any given time.*

| Field | Type | Description |
|---|---|---|
| `activeConference` | `ActiveConference` | Active conference, if it exists. |
| `config` | `SpaceConfig` | Configuration pertaining to the meeting space. |
| `gatewaySipAccess` | array of `GatewaySipAccess` | Output only. The SIP-based access methods that can be used to join the conference. Can be empty. |
| `meetingCode` | string | Output only. Type friendly unique string used to join the meeting. Format: `[a-z]+-[a-z]+-[a-z]+`. For example, `abc-mnop-xyz`. The maximum length is 128 characters. |
| `meetingUri` | string | Output only. URI used to join meetings consisting of `https://meet.google.com/` followed by the `meeting_code`. For example, `https://meet.google.com/abc-mnop-xyz`. |
| `name` | string | Immutable. Resource name of the space. Format: `spaces/{space}`. `{space}` is the resource identifier for the space. It's a unique, server-generated ID and is case sensitive. |
| `phoneAccess` | array of `PhoneAccess` | Output only. All regional phone access methods for this meeting space. Can be empty. |

### Transcript

*Description: Metadata for a transcript generated from a conference. It refers to the ASR (Automatic Speech Recognition) result of user's speech during the conference.*

| Field | Type | Description |
|---|---|---|
| `docsDestination` | `DocsDestination` | Output only. Where the Google Docs transcript is saved. |
| `endTime` | string (format: google-datetime) | Output only. Timestamp when the transcript stopped. |
| `name` | string | Output only. Resource name of the transcript. |
| `startTime` | string (format: google-datetime) | Output only. Timestamp when the transcript started. |
| `state` | string | Output only. Current state. |

### TranscriptEntry

*Description: Single entry for one user’s speech during a transcript session.*

| Field | Type | Description |
|---|---|---|
| `endTime` | string (format: google-datetime) | Output only. Timestamp when the transcript entry ended. |
| `languageCode` | string | Output only. Language of spoken text, such as "en-US". IETF BCP 47 syntax (https://tools.ietf.org/html/bcp47) |
| `name` | string | Output only. Resource name of the entry. Format: "conferenceRecords/{conference_record}/transcripts/{transcript}/entries/{entry}" |
| `participant` | string | Output only. Refers to the participant who speaks. |
| `startTime` | string (format: google-datetime) | Output only. Timestamp when the transcript entry started. |
| `text` | string | Output only. The transcribed text of the participant's voice, at maximum 10K words. Note that the limit is subject to change. |

## Discovering Commands

Before calling any API method, inspect it:

```bash
# Browse resources and methods
gws meet --help

# Inspect a method's required params, types, and defaults
gws schema meet.<resource>.<method>
```

Use `gws schema` output to build your `--params` and `--json` flags.

