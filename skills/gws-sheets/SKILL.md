---
name: gws-sheets
description: "Google Sheets: Read and write spreadsheets."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws sheets --help"
---

# sheets (v4)

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

```bash
gws sheets <resource> <method> [flags]
```

## Helper Commands

| Command | Description |
|---------|-------------|
| [`+append`](../gws-sheets-append/SKILL.md) | Append a row to a spreadsheet |
| [`+read`](../gws-sheets-read/SKILL.md) | Read values from a spreadsheet |

## API Resources

### spreadsheets

  - `batchUpdate` — Applies one or more updates to the spreadsheet. Each request is validated before being applied. If any request is not valid then the entire request will fail and nothing will be applied. Some requests have replies to give you some information about how they are applied. The replies will mirror the requests. For example, if you applied 4 updates and the 3rd one had a reply, then the response will have 2 empty replies, the actual reply, and another empty reply, in that order.
    - Required path params: spreadsheetId
    - Request body type: `BatchUpdateSpreadsheetRequest`
    - Response type: `BatchUpdateSpreadsheetResponse`
  - `create` — Creates a spreadsheet, returning the newly created spreadsheet.
    - Request body type: `Spreadsheet`
    - Response type: `Spreadsheet`
  - `get` — Returns the spreadsheet at the given ID. The caller must specify the spreadsheet ID. By default, data within grids is not returned. You can include grid data in one of 2 ways: * Specify a [field mask](https://developers.google.com/workspace/sheets/api/guides/field-masks) listing your desired fields using the `fields` URL parameter in HTTP * Set the includeGridData URL parameter to true.
    - Required path params: spreadsheetId
    - Response type: `Spreadsheet`
  - `getByDataFilter` — Returns the spreadsheet at the given ID. The caller must specify the spreadsheet ID. For more information, see [Read, write, and search metadata](https://developers.google.com/workspace/sheets/api/guides/metadata). This method differs from GetSpreadsheet in that it allows selecting which subsets of spreadsheet data to return by specifying a dataFilters parameter. Multiple DataFilters can be specified.
    - Required path params: spreadsheetId
    - Request body type: `GetSpreadsheetByDataFilterRequest`
    - Response type: `Spreadsheet`

### spreadsheets.developerMetadata

  - `get` — Returns the developer metadata with the specified ID. The caller must specify the spreadsheet ID and the developer metadata's unique metadataId. For more information, see [Read, write, and search metadata](https://developers.google.com/workspace/sheets/api/guides/metadata).
    - Required path params: metadataId, spreadsheetId
    - Response type: `DeveloperMetadata`
  - `search` — Returns all developer metadata matching the specified DataFilter. For more information, see [Read, write, and search metadata](https://developers.google.com/workspace/sheets/api/guides/metadata). If the provided DataFilter represents a DeveloperMetadataLookup object, this will return all DeveloperMetadata entries selected by it. If the DataFilter represents a location in a spreadsheet, this will return all developer metadata associated with locations intersecting that region.
    - Required path params: spreadsheetId
    - Request body type: `SearchDeveloperMetadataRequest`
    - Response type: `SearchDeveloperMetadataResponse`

### spreadsheets.sheets

  - `copyTo` — Copies a single sheet from a spreadsheet to another spreadsheet. Returns the properties of the newly created sheet.
    - Required path params: sheetId, spreadsheetId
    - Request body type: `CopySheetToAnotherSpreadsheetRequest`
    - Response type: `SheetProperties`

### spreadsheets.values

  - `append` — Appends values to a spreadsheet. The input range is used to search for existing data and find a "table" within that range. Values will be appended to the next row of the table, starting with the first column of the table. See the [guide](https://developers.google.com/workspace/sheets/api/guides/values#appending_values) and [sample code](https://developers.google.com/workspace/sheets/api/samples/writing#append_values) for specific details of how tables are detected and data is appended.
    - Required path params: range, spreadsheetId
    - Request body type: `ValueRange`
    - Response type: `AppendValuesResponse`
  - `batchClear` — Clears one or more ranges of values from a spreadsheet. The caller must specify the spreadsheet ID and one or more ranges. Only values are cleared -- all other properties of the cell (such as formatting and data validation) are kept.
    - Required path params: spreadsheetId
    - Request body type: `BatchClearValuesRequest`
    - Response type: `BatchClearValuesResponse`
  - `batchClearByDataFilter` — Clears one or more ranges of values from a spreadsheet. For more information, see [Read, write, and search metadata](https://developers.google.com/workspace/sheets/api/guides/metadata). The caller must specify the spreadsheet ID and one or more DataFilters. Ranges matching any of the specified data filters will be cleared. Only values are cleared -- all other properties of the cell (such as formatting, data validation, etc.) are kept.
    - Required path params: spreadsheetId
    - Request body type: `BatchClearValuesByDataFilterRequest`
    - Response type: `BatchClearValuesByDataFilterResponse`
  - `batchGet` — Returns one or more ranges of values from a spreadsheet. The caller must specify the spreadsheet ID and one or more ranges.
    - Required path params: spreadsheetId
    - Response type: `BatchGetValuesResponse`
  - `batchGetByDataFilter` — Returns one or more ranges of values that match the specified data filters. For more information, see [Read, write, and search metadata](https://developers.google.com/workspace/sheets/api/guides/metadata). The caller must specify the spreadsheet ID and one or more DataFilters. Ranges that match any of the data filters in the request will be returned.
    - Required path params: spreadsheetId
    - Request body type: `BatchGetValuesByDataFilterRequest`
    - Response type: `BatchGetValuesByDataFilterResponse`
  - `batchUpdate` — Sets values in one or more ranges of a spreadsheet. The caller must specify the spreadsheet ID, a valueInputOption, and one or more ValueRanges.
    - Required path params: spreadsheetId
    - Request body type: `BatchUpdateValuesRequest`
    - Response type: `BatchUpdateValuesResponse`
  - `batchUpdateByDataFilter` — Sets values in one or more ranges of a spreadsheet. For more information, see [Read, write, and search metadata](https://developers.google.com/workspace/sheets/api/guides/metadata). The caller must specify the spreadsheet ID, a valueInputOption, and one or more DataFilterValueRanges.
    - Required path params: spreadsheetId
    - Request body type: `BatchUpdateValuesByDataFilterRequest`
    - Response type: `BatchUpdateValuesByDataFilterResponse`
  - `clear` — Clears values from a spreadsheet. The caller must specify the spreadsheet ID and range. Only values are cleared -- all other properties of the cell (such as formatting, data validation, etc..) are kept.
    - Required path params: range, spreadsheetId
    - Request body type: `ClearValuesRequest`
    - Response type: `ClearValuesResponse`
  - `get` — Returns a range of values from a spreadsheet. The caller must specify the spreadsheet ID and a range.
    - Required path params: range, spreadsheetId
    - Response type: `ValueRange`
  - `update` — Sets values in a range of a spreadsheet. The caller must specify the spreadsheet ID, range, and a valueInputOption.
    - Required path params: range, spreadsheetId
    - Request body type: `ValueRange`
    - Response type: `UpdateValuesResponse`

## Common Schemas

### AppendValuesResponse

*Description: The response when updating a range of values in a spreadsheet.*

| Field | Type | Description |
|---|---|---|
| `spreadsheetId` | string | The spreadsheet the updates were applied to. |
| `tableRange` | string | The range (in A1 notation) of the table that values are being appended to (before the values were appended). Empty if no table was found. |
| `updates` | `UpdateValuesResponse` | Information about the updates that were applied. |

### BatchClearValuesByDataFilterRequest

*Description: The request for clearing more than one range selected by a DataFilter in a spreadsheet.*

| Field | Type | Description |
|---|---|---|
| `dataFilters` | array of `DataFilter` | The DataFilters used to determine which ranges to clear. |

### BatchClearValuesByDataFilterResponse

*Description: The response when clearing a range of values selected with DataFilters in a spreadsheet.*

| Field | Type | Description |
|---|---|---|
| `clearedRanges` | array of string | The ranges that were cleared, in [A1 notation](https://developers.google.com/workspace/sheets/api/guides/concepts#cell). |
| `spreadsheetId` | string | The spreadsheet the updates were applied to. |

### BatchClearValuesRequest

*Description: The request for clearing more than one range of values in a spreadsheet.*

| Field | Type | Description |
|---|---|---|
| `ranges` | array of string | The ranges to clear, in [A1 notation or R1C1 notation](https://developers.google.com/workspace/sheets/api/guides/concepts#cell). |

### BatchClearValuesResponse

*Description: The response when clearing a range of values in a spreadsheet.*

| Field | Type | Description |
|---|---|---|
| `clearedRanges` | array of string | The ranges that were cleared, in A1 notation. |
| `spreadsheetId` | string | The spreadsheet the updates were applied to. |

### BatchGetValuesByDataFilterRequest

*Description: The request for retrieving a range of values in a spreadsheet selected by a set of DataFilters.*

| Field | Type | Description |
|---|---|---|
| `dataFilters` | array of `DataFilter` | The data filters used to match the ranges of values to retrieve. Ranges that match any of the specified data filters are included in the response. |
| `dateTimeRenderOption` | string | How dates, times, and durations should be represented in the output. This is ignored if value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER. |
| `majorDimension` | string | The major dimension that results should use. |
| `valueRenderOption` | string | How values should be represented in the output. The default render option is FORMATTED_VALUE. |

### BatchGetValuesByDataFilterResponse

*Description: The response when retrieving more than one range of values in a spreadsheet selected by DataFilters.*

| Field | Type | Description |
|---|---|---|
| `spreadsheetId` | string | The ID of the spreadsheet the data was retrieved from. |
| `valueRanges` | array of `MatchedValueRange` | The requested values with the list of data filters that matched them. |

### BatchGetValuesResponse

*Description: The response when retrieving more than one range of values in a spreadsheet.*

| Field | Type | Description |
|---|---|---|
| `spreadsheetId` | string | The ID of the spreadsheet the data was retrieved from. |
| `valueRanges` | array of `ValueRange` | The requested values. The order of the ValueRanges is the same as the order of the requested ranges. |

### BatchUpdateSpreadsheetRequest

*Description: The request for updating any aspect of a spreadsheet.*

| Field | Type | Description |
|---|---|---|
| `includeSpreadsheetInResponse` | boolean | Determines if the update response should include the spreadsheet resource. |
| `requests` | array of `Request` | A list of updates to apply to the spreadsheet. Requests will be applied in the order they are specified. If any request is not valid, no requests will be applied. |
| `responseIncludeGridData` | boolean | True if grid data should be returned. Meaningful only if include_spreadsheet_in_response is 'true'. This parameter is ignored if a field mask was set in the request. |
| `responseRanges` | array of string | Limits the ranges included in the response spreadsheet. Meaningful only if include_spreadsheet_in_response is 'true'. |

### BatchUpdateSpreadsheetResponse

*Description: The reply for batch updating a spreadsheet.*

| Field | Type | Description |
|---|---|---|
| `replies` | array of `Response` | The reply of the updates. This maps 1:1 with the updates, although replies to some requests may be empty. |
| `spreadsheetId` | string | The spreadsheet the updates were applied to. |
| `updatedSpreadsheet` | `Spreadsheet` | The spreadsheet after updates were applied. This is only set if BatchUpdateSpreadsheetRequest.include_spreadsheet_in_response is `true`. |

### BatchUpdateValuesByDataFilterRequest

*Description: The request for updating more than one range of values in a spreadsheet.*

| Field | Type | Description |
|---|---|---|
| `data` | array of `DataFilterValueRange` | The new values to apply to the spreadsheet. If more than one range is matched by the specified DataFilter the specified values are applied to all of those ranges. |
| `includeValuesInResponse` | boolean | Determines if the update response should include the values of the cells that were updated. By default, responses do not include the updated values. |
| `responseDateTimeRenderOption` | string | Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER. |
| `responseValueRenderOption` | string | Determines how values in the response should be rendered. The default render option is FORMATTED_VALUE. |
| `valueInputOption` | string | How the input data should be interpreted. |

### BatchUpdateValuesByDataFilterResponse

*Description: The response when updating a range of values in a spreadsheet.*

| Field | Type | Description |
|---|---|---|
| `responses` | array of `UpdateValuesByDataFilterResponse` | The response for each range updated. |
| `spreadsheetId` | string | The spreadsheet the updates were applied to. |
| `totalUpdatedCells` | integer (format: int32) | The total number of cells updated. |
| `totalUpdatedColumns` | integer (format: int32) | The total number of columns where at least one cell in the column was updated. |
| `totalUpdatedRows` | integer (format: int32) | The total number of rows where at least one cell in the row was updated. |
| `totalUpdatedSheets` | integer (format: int32) | The total number of sheets where at least one cell in the sheet was updated. |

### BatchUpdateValuesRequest

*Description: The request for updating more than one range of values in a spreadsheet.*

| Field | Type | Description |
|---|---|---|
| `data` | array of `ValueRange` | The new values to apply to the spreadsheet. |
| `includeValuesInResponse` | boolean | Determines if the update response should include the values of the cells that were updated. By default, responses do not include the updated values. |
| `responseDateTimeRenderOption` | string | Determines how dates, times, and durations in the response should be rendered. This is ignored if response_value_render_option is FORMATTED_VALUE. The default dateTime render option is SERIAL_NUMBER. |
| `responseValueRenderOption` | string | Determines how values in the response should be rendered. The default render option is FORMATTED_VALUE. |
| `valueInputOption` | string | How the input data should be interpreted. |

### BatchUpdateValuesResponse

*Description: The response when updating a range of values in a spreadsheet.*

| Field | Type | Description |
|---|---|---|
| `responses` | array of `UpdateValuesResponse` | One UpdateValuesResponse per requested range, in the same order as the requests appeared. |
| `spreadsheetId` | string | The spreadsheet the updates were applied to. |
| `totalUpdatedCells` | integer (format: int32) | The total number of cells updated. |
| `totalUpdatedColumns` | integer (format: int32) | The total number of columns where at least one cell in the column was updated. |
| `totalUpdatedRows` | integer (format: int32) | The total number of rows where at least one cell in the row was updated. |
| `totalUpdatedSheets` | integer (format: int32) | The total number of sheets where at least one cell in the sheet was updated. |

### ClearValuesRequest

*Description: The request for clearing a range of values in a spreadsheet.*

*(No fields)*

### ClearValuesResponse

*Description: The response when clearing a range of values in a spreadsheet.*

| Field | Type | Description |
|---|---|---|
| `clearedRange` | string | The range (in A1 notation) that was cleared. |
| `spreadsheetId` | string | The spreadsheet the updates were applied to. |

### CopySheetToAnotherSpreadsheetRequest

*Description: The request to copy a sheet across spreadsheets.*

| Field | Type | Description |
|---|---|---|
| `destinationSpreadsheetId` | string | The ID of the spreadsheet to copy the sheet to. |

### DeveloperMetadata

*Description: Developer metadata associated with a location or object in a spreadsheet. For more information, see [Read, write, and search metadata](https://developers.google.com/workspace/sheets/api/guides/metadata).*

| Field | Type | Description |
|---|---|---|
| `location` | `DeveloperMetadataLocation` | The location where the metadata is associated. |
| `metadataId` | integer (format: int32) | The spreadsheet-scoped unique ID that identifies the metadata. IDs may be specified when metadata is created, otherwise one will be randomly generated and assigned. Must be positive. |
| `metadataKey` | string | The metadata key. There may be multiple metadata in a spreadsheet with the same key. Developer metadata must always have a key specified. |
| `metadataValue` | string | Data associated with the metadata's key. |
| `visibility` | string | The metadata visibility. Developer metadata must always have visibility specified. |

### GetSpreadsheetByDataFilterRequest

*Description: The request for retrieving a Spreadsheet.*

| Field | Type | Description |
|---|---|---|
| `dataFilters` | array of `DataFilter` | The DataFilters used to select which ranges to retrieve from the spreadsheet. |
| `excludeTablesInBandedRanges` | boolean | True if tables should be excluded in the banded ranges. False if not set. |
| `includeGridData` | boolean | True if grid data should be returned. This parameter is ignored if a field mask was set in the request. |

### SearchDeveloperMetadataRequest

*Description: A request to retrieve all developer metadata matching the set of specified criteria.*

| Field | Type | Description |
|---|---|---|
| `dataFilters` | array of `DataFilter` | The data filters describing the criteria used to determine which DeveloperMetadata entries to return. DeveloperMetadata matching any of the specified filters are included in the response. |

### SearchDeveloperMetadataResponse

*Description: A reply to a developer metadata search request.*

| Field | Type | Description |
|---|---|---|
| `matchedDeveloperMetadata` | array of `MatchedDeveloperMetadata` | The metadata matching the criteria of the search request. |

### SheetProperties

*Description: Properties of a sheet.*

| Field | Type | Description |
|---|---|---|
| `dataSourceSheetProperties` | `DataSourceSheetProperties` | Output only. If present, the field contains DATA_SOURCE sheet specific properties. |
| `gridProperties` | `GridProperties` | Additional properties of the sheet if this sheet is a grid. |
| `hidden` | boolean | True if the sheet is hidden in the UI, false if it's visible. |
| `index` | integer (format: int32) | The index of the sheet within the spreadsheet. When adding or updating sheet properties, if this field is excluded then the sheet is added or moved to the end of the sheet list. |
| `rightToLeft` | boolean | True if the sheet is an RTL sheet instead of an LTR sheet. |
| `sheetId` | integer (format: int32) | The ID of the sheet. Must be non-negative. This field cannot be changed once set. |
| `sheetType` | string | The type of sheet. Defaults to GRID. This field cannot be changed once set. |
| `tabColor` | `Color` | The color of the tab in the UI. Deprecated: Use tab_color_style. |
| `tabColorStyle` | `ColorStyle` | The color of the tab in the UI. If tab_color is also set, this field takes precedence. |
| `title` | string | The name of the sheet. |

### Spreadsheet

*Description: Resource that represents a spreadsheet.*

| Field | Type | Description |
|---|---|---|
| `dataSourceSchedules` | array of `DataSourceRefreshSchedule` | Output only. A list of data source refresh schedules. |
| `dataSources` | array of `DataSource` | A list of external data sources connected with the spreadsheet. |
| `developerMetadata` | array of `DeveloperMetadata` | The developer metadata associated with a spreadsheet. |
| `namedRanges` | array of `NamedRange` | The named ranges defined in a spreadsheet. |
| `properties` | `SpreadsheetProperties` | Overall properties of a spreadsheet. |
| `sheets` | array of `Sheet` | The sheets that are part of a spreadsheet. |
| `spreadsheetId` | string | The ID of the spreadsheet. This field is read-only. |
| `spreadsheetUrl` | string | The url of the spreadsheet. This field is read-only. |

### UpdateValuesResponse

*Description: The response when updating a range of values in a spreadsheet.*

| Field | Type | Description |
|---|---|---|
| `spreadsheetId` | string | The spreadsheet the updates were applied to. |
| `updatedCells` | integer (format: int32) | The number of cells updated. |
| `updatedColumns` | integer (format: int32) | The number of columns where at least one cell in the column was updated. |
| `updatedData` | `ValueRange` | The values of the cells after updates were applied. This is only included if the request's `includeValuesInResponse` field was `true`. |
| `updatedRange` | string | The range (in A1 notation) that updates were applied to. |
| `updatedRows` | integer (format: int32) | The number of rows where at least one cell in the row was updated. |

### ValueRange

*Description: Data within a range of the spreadsheet.*

| Field | Type | Description |
|---|---|---|
| `majorDimension` | string | The major dimension of the values. |
| `range` | string | The range the values cover, in [A1 notation](https://developers.google.com/workspace/sheets/api/guides/concepts#cell). |
| `values` | array of array | The data that was read or to be written. This is an array of arrays, the outer array representing all the data and each inner array representing a major dimension. |

## Discovering Commands

Before calling any API method, inspect it:

```bash
# Browse resources and methods
gws sheets --help

# Inspect a method's required params, types, and defaults
gws schema sheets.<resource>.<method>
```

Use `gws schema` output to build your `--params` and `--json` flags.

