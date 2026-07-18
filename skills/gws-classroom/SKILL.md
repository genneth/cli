---
name: gws-classroom
description: "Google Classroom: Manage classes, rosters, and coursework."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws classroom --help"
---

# classroom (v1)

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

```bash
gws classroom <resource> <method> [flags]
```

## API Resources

### courses

  - `create` — Creates a course. The user specified in `ownerId` is the owner of the created course and added as a teacher. A non-admin requesting user can only create a course with themselves as the owner. Domain admins can create courses owned by any user within their domain. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to create courses or for access errors. * `NOT_FOUND` if the primary teacher is not a valid user.
    - Request body type: `Course`
    - Response type: `Course`
  - `delete` — Deletes a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to delete the requested course or for access errors. * `NOT_FOUND` if no course exists with the requested ID.
    - Required path params: id
    - Response type: `Empty`
  - `get` — Returns a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or for access errors. * `NOT_FOUND` if no course exists with the requested ID.
    - Required path params: id
    - Response type: `Course`
  - `getGradingPeriodSettings` — Returns the grading period settings in a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user isn't permitted to access the grading period settings in the requested course or for access errors. * `NOT_FOUND` if the requested course does not exist.
    - Required path params: courseId
    - Response type: `GradingPeriodSettings`
  - `list` — Returns a list of courses that the requesting user is permitted to view, restricted to those that match the request. Returned courses are ordered by creation time, with the most recently created coming first. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the query argument is malformed. * `NOT_FOUND` if any users specified in the query arguments do not exist.
    - Response type: `ListCoursesResponse`
  - `patch` — Updates one or more fields in a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to modify the requested course or for access errors. * `NOT_FOUND` if no course exists with the requested ID. * `INVALID_ARGUMENT` if invalid fields are specified in the update mask or if no update mask is supplied.
    - Required path params: id
    - Request body type: `Course`
    - Response type: `Course`
  - `update` — Updates a course. Note: Unlike other fields, `levels` is not cleared if omitted from the request. The `UpdateCourse` method only modifies `levels` if it is explicitly provided; otherwise, the existing value is preserved. Use the `PatchCourse` method to clear the `levels` field. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to modify the requested course or for access errors. * `NOT_FOUND` if no course exists with the requested ID.
    - Required path params: id
    - Request body type: `Course`
    - Response type: `Course`
  - `updateGradingPeriodSettings` — Updates grading period settings of a course. Individual grading periods can be added, removed, or modified using this method. The requesting user and course owner must be eligible to modify Grading Periods. For details, see [licensing requirements](https://developers.google.com/workspace/classroom/grading-periods/manage-grading-periods#licensing_requirements).
    - Required path params: courseId
    - Request body type: `GradingPeriodSettings`
    - Response type: `GradingPeriodSettings`

### courses.aliases

  - `create` — Creates an alias for a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to create the alias or for access errors. * `NOT_FOUND` if the course does not exist. * `ALREADY_EXISTS` if the alias already exists. * `FAILED_PRECONDITION` if the alias requested does not make sense for the requesting user or course (for example, if a user not in a domain attempts to access a domain-scoped alias).
    - Required path params: courseId
    - Request body type: `CourseAlias`
    - Response type: `CourseAlias`
  - `delete` — Deletes an alias of a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to remove the alias or for access errors. * `NOT_FOUND` if the alias does not exist. * `FAILED_PRECONDITION` if the alias requested does not make sense for the requesting user or course (for example, if a user not in a domain attempts to delete a domain-scoped alias).
    - Required path params: alias, courseId
    - Response type: `Empty`
  - `list` — Returns a list of aliases for a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the course or for access errors. * `NOT_FOUND` if the course does not exist.
    - Required path params: courseId
    - Response type: `ListCourseAliasesResponse`

### courses.announcements

  - `create` — Creates an announcement. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course, create announcements in the requested course, share a Drive attachment, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course does not exist. * `FAILED_PRECONDITION` for the following request error: * AttachmentNotVisible
    - Required path params: courseId
    - Request body type: `Announcement`
    - Response type: `Announcement`
  - `delete` — Deletes an announcement. This request must be made by the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to create the corresponding announcement item. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting developer project did not create the corresponding announcement, if the requesting user is not permitted to delete the requested course or for access errors.
    - Required path params: courseId, id
    - Response type: `Empty`
  - `get` — Returns an announcement. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or announcement, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course or announcement does not exist.
    - Required path params: courseId, id
    - Response type: `Announcement`
  - `getAddOnContext` — Gets metadata for Classroom add-ons in the context of a specific post. To maintain the integrity of its own data and permissions model, an add-on should call this to validate query parameters and the requesting user's role whenever the add-on is opened in an [iframe](https://developers.google.com/workspace/classroom/add-ons/get-started/iframes/iframes-overview). This method returns the following error codes: * `PERMISSION_DENIED` for access errors.
    - Required path params: courseId, itemId
    - Response type: `AddOnContext`
  - `list` — Returns a list of announcements that the requester is permitted to view. Course students may only view `PUBLISHED` announcements. Course teachers and domain administrators may view all announcements. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course does not exist.
    - Required path params: courseId
    - Response type: `ListAnnouncementsResponse`
  - `modifyAssignees` — Modifies assignee mode and options of an announcement. Only a teacher of the course that contains the announcement may call this method. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or course work or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course or course work does not exist.
    - Required path params: courseId, id
    - Request body type: `ModifyAnnouncementAssigneesRequest`
    - Response type: `Announcement`
  - `patch` — Updates one or more fields of an announcement. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting developer project did not create the corresponding announcement or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `FAILED_PRECONDITION` if the requested announcement has already been deleted. * `NOT_FOUND` if the requested course or announcement does not exist
    - Required path params: courseId, id
    - Request body type: `Announcement`
    - Response type: `Announcement`

### courses.announcements.addOnAttachments

  - `create` — Creates an add-on attachment under a post. Requires the add-on to have permission to create new attachments on the post. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: courseId, itemId
    - Request body type: `AddOnAttachment`
    - Response type: `AddOnAttachment`
  - `delete` — Deletes an add-on attachment. Requires the add-on to have been the original creator of the attachment. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: attachmentId, courseId, itemId
    - Response type: `Empty`
  - `get` — Returns an add-on attachment. Requires the add-on requesting the attachment to be the original creator of the attachment. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: attachmentId, courseId, itemId
    - Response type: `AddOnAttachment`
  - `list` — Returns all attachments created by an add-on under the post. Requires the add-on to have active attachments on the post or have permission to create new attachments on the post. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: courseId, itemId
    - Response type: `ListAddOnAttachmentsResponse`
  - `patch` — Updates an add-on attachment. Requires the add-on to have been the original creator of the attachment. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: attachmentId, courseId, itemId
    - Request body type: `AddOnAttachment`
    - Response type: `AddOnAttachment`

### courses.courseWork

  - `create` — Creates course work. The resulting course work (and corresponding student submissions) are associated with the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to make the request. Classroom API requests to modify course work and student submissions must be made with an OAuth client ID from the associated Developer Console project.
    - Required path params: courseId
    - Request body type: `CourseWork`
    - Response type: `CourseWork`
  - `delete` — Deletes a course work. This request must be made by the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to create the corresponding course work item. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting developer project did not create the corresponding course work, if the requesting user is not permitted to delete the requested course or for access errors.
    - Required path params: courseId, id
    - Response type: `Empty`
  - `get` — Returns course work. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or course work, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course or course work does not exist.
    - Required path params: courseId, id
    - Response type: `CourseWork`
  - `getAddOnContext` — Gets metadata for Classroom add-ons in the context of a specific post. To maintain the integrity of its own data and permissions model, an add-on should call this to validate query parameters and the requesting user's role whenever the add-on is opened in an [iframe](https://developers.google.com/workspace/classroom/add-ons/get-started/iframes/iframes-overview). This method returns the following error codes: * `PERMISSION_DENIED` for access errors.
    - Required path params: courseId, itemId
    - Response type: `AddOnContext`
  - `list` — Returns a list of course work that the requester is permitted to view. Course students may only view `PUBLISHED` course work. Course teachers and domain administrators may view all course work. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course does not exist.
    - Required path params: courseId
    - Response type: `ListCourseWorkResponse`
  - `modifyAssignees` — Modifies assignee mode and options of a coursework. Only a teacher of the course that contains the coursework may call this method. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or course work or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course or course work does not exist.
    - Required path params: courseId, id
    - Request body type: `ModifyCourseWorkAssigneesRequest`
    - Response type: `CourseWork`
  - `patch` — Updates one or more fields of a course work. See google.classroom.v1.CourseWork for details of which fields may be updated and who may change them. This request must be made by the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to create the corresponding course work item.
    - Required path params: courseId, id
    - Request body type: `CourseWork`
    - Response type: `CourseWork`
  - `updateRubric` — Updates a rubric. See google.classroom.v1.Rubric for details of which fields can be updated. Rubric update capabilities are [limited](/classroom/rubrics/limitations) once grading has started. The requesting user and course owner must have rubrics creation capabilities. For details, see [licensing requirements](https://developers.google.com/workspace/classroom/rubrics/limitations#license-requirements). This request must be made by the Google Cloud console of the [OAuth client ID](https://support.
    - Required path params: courseId, courseWorkId
    - Request body type: `Rubric`
    - Response type: `Rubric`

### courses.courseWork.addOnAttachments

  - `create` — Creates an add-on attachment under a post. Requires the add-on to have permission to create new attachments on the post. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: courseId, itemId
    - Request body type: `AddOnAttachment`
    - Response type: `AddOnAttachment`
  - `delete` — Deletes an add-on attachment. Requires the add-on to have been the original creator of the attachment. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: attachmentId, courseId, itemId
    - Response type: `Empty`
  - `get` — Returns an add-on attachment. Requires the add-on requesting the attachment to be the original creator of the attachment. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: attachmentId, courseId, itemId
    - Response type: `AddOnAttachment`
  - `list` — Returns all attachments created by an add-on under the post. Requires the add-on to have active attachments on the post or have permission to create new attachments on the post. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: courseId, itemId
    - Response type: `ListAddOnAttachmentsResponse`
  - `patch` — Updates an add-on attachment. Requires the add-on to have been the original creator of the attachment. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: attachmentId, courseId, itemId
    - Request body type: `AddOnAttachment`
    - Response type: `AddOnAttachment`

### courses.courseWork.addOnAttachments.studentSubmissions

  - `get` — Returns a student submission for an add-on attachment. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: attachmentId, courseId, itemId, submissionId
    - Response type: `AddOnAttachmentStudentSubmission`
  - `patch` — Updates data associated with an add-on attachment submission. Requires the add-on to have been the original creator of the attachment and the attachment to have a positive `max_points` value set. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: attachmentId, courseId, itemId, submissionId
    - Request body type: `AddOnAttachmentStudentSubmission`
    - Response type: `AddOnAttachmentStudentSubmission`

### courses.courseWork.rubrics

  - `create` — Creates a rubric. The requesting user and course owner must have rubrics creation capabilities. For details, see [licensing requirements](https://developers.google.com/workspace/classroom/rubrics/limitations#license-requirements). For further details, see [Rubrics structure and known limitations](/classroom/rubrics/limitations).
    - Required path params: courseId, courseWorkId
    - Request body type: `Rubric`
    - Response type: `Rubric`
  - `delete` — Deletes a rubric. The requesting user and course owner must have rubrics creation capabilities. For details, see [licensing requirements](https://developers.google.com/workspace/classroom/rubrics/limitations#license-requirements). This request must be made by the Google Cloud console of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to create the corresponding rubric.
    - Required path params: courseId, courseWorkId, id
    - Response type: `Empty`
  - `get` — Returns a rubric. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course, course work, or rubric doesn't exist or if the user doesn't have access to the corresponding course work.
    - Required path params: courseId, courseWorkId, id
    - Response type: `Rubric`
  - `list` — Returns a list of rubrics that the requester is permitted to view. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course or course work doesn't exist or if the user doesn't have access to the corresponding course work.
    - Required path params: courseId, courseWorkId
    - Response type: `ListRubricsResponse`
  - `patch` — Updates a rubric. See google.classroom.v1.Rubric for details of which fields can be updated. Rubric update capabilities are [limited](/classroom/rubrics/limitations) once grading has started. The requesting user and course owner must have rubrics creation capabilities. For details, see [licensing requirements](https://developers.google.com/workspace/classroom/rubrics/limitations#license-requirements). This request must be made by the Google Cloud console of the [OAuth client ID](https://support.
    - Required path params: courseId, courseWorkId, id
    - Request body type: `Rubric`
    - Response type: `Rubric`

### courses.courseWork.studentSubmissions

  - `get` — Returns a student submission. * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course, course work, or student submission or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course, course work, or student submission does not exist.
    - Required path params: courseId, courseWorkId, id
    - Response type: `StudentSubmission`
  - `list` — Returns a list of student submissions that the requester is permitted to view, factoring in the OAuth scopes of the request. A hyphen (`-`) may be specified as the `course_work_id` to include student submissions for multiple course work items. Course students may only view their own work. Course teachers and domain administrators may view all student submissions.
    - Required path params: courseId, courseWorkId
    - Response type: `ListStudentSubmissionsResponse`
  - `modifyAttachments` — Modifies attachments of student submission. Attachments may only be added to student submissions belonging to course work objects with a `workType` of `ASSIGNMENT`. This request must be made by the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to create the corresponding course work item.
    - Required path params: courseId, courseWorkId, id
    - Request body type: `ModifyAttachmentsRequest`
    - Response type: `StudentSubmission`
  - `patch` — Updates one or more fields of a student submission. See google.classroom.v1.StudentSubmission for details of which fields may be updated and who may change them. This request must be made by the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to create the corresponding course work item.
    - Required path params: courseId, courseWorkId, id
    - Request body type: `StudentSubmission`
    - Response type: `StudentSubmission`
  - `reclaim` — Reclaims a student submission on behalf of the student that owns it. Reclaiming a student submission transfers ownership of attached Drive files to the student and updates the submission state. Only the student that owns the requested student submission may call this method, and only for a student submission that has been turned in.
    - Required path params: courseId, courseWorkId, id
    - Request body type: `ReclaimStudentSubmissionRequest`
    - Response type: `Empty`
  - `return` — Returns a student submission. Returning a student submission transfers ownership of attached Drive files to the student and may also update the submission state. Unlike the Classroom application, returning a student submission does not set assignedGrade to the draftGrade value. Only a teacher of the course that contains the requested student submission may call this method.
    - Required path params: courseId, courseWorkId, id
    - Request body type: `ReturnStudentSubmissionRequest`
    - Response type: `Empty`
  - `turnIn` — Turns in a student submission. Turning in a student submission transfers ownership of attached Drive files to the teacher and may also update the submission state. This may only be called by the student that owns the specified student submission. This request must be made by the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to create the corresponding course work item.
    - Required path params: courseId, courseWorkId, id
    - Request body type: `TurnInStudentSubmissionRequest`
    - Response type: `Empty`

### courses.courseWorkMaterials

  - `create` — Creates a course work material. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course, create course work material in the requested course, share a Drive attachment, or for access errors. * `INVALID_ARGUMENT` if the request is malformed or if more than 20 * materials are provided. * `NOT_FOUND` if the requested course does not exist.
    - Required path params: courseId
    - Request body type: `CourseWorkMaterial`
    - Response type: `CourseWorkMaterial`
  - `delete` — Deletes a course work material. This request must be made by the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to create the corresponding course work material item. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting developer project did not create the corresponding course work material, if the requesting user is not permitted to delete the requested course or for access errors.
    - Required path params: courseId, id
    - Response type: `Empty`
  - `get` — Returns a course work material. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or course work material, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course or course work material does not exist.
    - Required path params: courseId, id
    - Response type: `CourseWorkMaterial`
  - `getAddOnContext` — Gets metadata for Classroom add-ons in the context of a specific post. To maintain the integrity of its own data and permissions model, an add-on should call this to validate query parameters and the requesting user's role whenever the add-on is opened in an [iframe](https://developers.google.com/workspace/classroom/add-ons/get-started/iframes/iframes-overview). This method returns the following error codes: * `PERMISSION_DENIED` for access errors.
    - Required path params: courseId, itemId
    - Response type: `AddOnContext`
  - `list` — Returns a list of course work material that the requester is permitted to view. Course students may only view `PUBLISHED` course work material. Course teachers and domain administrators may view all course work material. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course does not exist.
    - Required path params: courseId
    - Response type: `ListCourseWorkMaterialResponse`
  - `patch` — Updates one or more fields of a course work material. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting developer project for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `FAILED_PRECONDITION` if the requested course work material has already been deleted. * `NOT_FOUND` if the requested course or course work material does not exist
    - Required path params: courseId, id
    - Request body type: `CourseWorkMaterial`
    - Response type: `CourseWorkMaterial`

### courses.courseWorkMaterials.addOnAttachments

  - `create` — Creates an add-on attachment under a post. Requires the add-on to have permission to create new attachments on the post. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: courseId, itemId
    - Request body type: `AddOnAttachment`
    - Response type: `AddOnAttachment`
  - `delete` — Deletes an add-on attachment. Requires the add-on to have been the original creator of the attachment. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: attachmentId, courseId, itemId
    - Response type: `Empty`
  - `get` — Returns an add-on attachment. Requires the add-on requesting the attachment to be the original creator of the attachment. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: attachmentId, courseId, itemId
    - Response type: `AddOnAttachment`
  - `list` — Returns all attachments created by an add-on under the post. Requires the add-on to have active attachments on the post or have permission to create new attachments on the post. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: courseId, itemId
    - Response type: `ListAddOnAttachmentsResponse`
  - `patch` — Updates an add-on attachment. Requires the add-on to have been the original creator of the attachment. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: attachmentId, courseId, itemId
    - Request body type: `AddOnAttachment`
    - Response type: `AddOnAttachment`

### courses.posts

  - `getAddOnContext` — Gets metadata for Classroom add-ons in the context of a specific post. To maintain the integrity of its own data and permissions model, an add-on should call this to validate query parameters and the requesting user's role whenever the add-on is opened in an [iframe](https://developers.google.com/workspace/classroom/add-ons/get-started/iframes/iframes-overview). This method returns the following error codes: * `PERMISSION_DENIED` for access errors.
    - Required path params: courseId, postId
    - Response type: `AddOnContext`

### courses.posts.addOnAttachments

  - `create` — Creates an add-on attachment under a post. Requires the add-on to have permission to create new attachments on the post. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: courseId, postId
    - Request body type: `AddOnAttachment`
    - Response type: `AddOnAttachment`
  - `delete` — Deletes an add-on attachment. Requires the add-on to have been the original creator of the attachment. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: attachmentId, courseId, postId
    - Response type: `Empty`
  - `get` — Returns an add-on attachment. Requires the add-on requesting the attachment to be the original creator of the attachment. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: attachmentId, courseId, postId
    - Response type: `AddOnAttachment`
  - `list` — Returns all attachments created by an add-on under the post. Requires the add-on to have active attachments on the post or have permission to create new attachments on the post. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: courseId, postId
    - Response type: `ListAddOnAttachmentsResponse`
  - `patch` — Updates an add-on attachment. Requires the add-on to have been the original creator of the attachment. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: attachmentId, courseId, postId
    - Request body type: `AddOnAttachment`
    - Response type: `AddOnAttachment`

### courses.posts.addOnAttachments.studentSubmissions

  - `get` — Returns a student submission for an add-on attachment. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: attachmentId, courseId, postId, submissionId
    - Response type: `AddOnAttachmentStudentSubmission`
  - `patch` — Updates data associated with an add-on attachment submission. Requires the add-on to have been the original creator of the attachment and the attachment to have a positive `max_points` value set. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.
    - Required path params: attachmentId, courseId, postId, submissionId
    - Request body type: `AddOnAttachmentStudentSubmission`
    - Response type: `AddOnAttachmentStudentSubmission`

### courses.studentGroups

  - `create` — Creates a student group for a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to create the student group or for access errors. * `NOT_FOUND` if the course does not exist or the requesting user doesn't have access to the course. * `FAILED_PRECONDITION` if creating the student group would exceed the maximum number of student groups per course.
    - Required path params: courseId
    - Request body type: `StudentGroup`
    - Response type: `StudentGroup`
  - `delete` — Deletes a student group. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to delete the requested student group or for access errors. * `NOT_FOUND` if the student group does not exist or the user does not have access to the student group.
    - Required path params: courseId, id
    - Response type: `Empty`
  - `list` — Returns a list of groups in a course. This method returns the following error codes: * `NOT_FOUND` if the course does not exist.
    - Required path params: courseId
    - Response type: `ListStudentGroupsResponse`
  - `patch` — Updates one or more fields in a student group. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to modify the requested student group or for access errors. * `NOT_FOUND` if the student group does not exist or the user does not have access to the student group. * `INVALID_ARGUMENT` if invalid fields are specified in the update mask or if no update mask is supplied.
    - Required path params: courseId, id
    - Request body type: `StudentGroup`
    - Response type: `StudentGroup`

### courses.studentGroups.studentGroupMembers

  - `create` — Creates a student group member for a student group. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to create the student group or member for access errors. * `NOT_FOUND` if the student group does not exist or the user does not have access to the student group. * `ALREADY_EXISTS` if the student group member already exists. * `FAILED_PRECONDITION` if attempting to add a member to a student group that has reached its member limit.
    - Required path params: courseId, studentGroupId
    - Request body type: `StudentGroupMember`
    - Response type: `StudentGroupMember`
  - `delete` — Deletes a student group member. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to delete the requested student group member or for access errors. * `NOT_FOUND` if the student group member does not exist or the user does not have access to the student group.
    - Required path params: courseId, studentGroupId, userId
    - Response type: `Empty`
  - `list` — Returns a list of students in a group. This method returns the following error codes: * `NOT_FOUND` if the course or student group does not exist.
    - Required path params: courseId, studentGroupId
    - Response type: `ListStudentGroupMembersResponse`

### courses.students

  - `create` — Adds a user as a student of a course. Domain administrators are permitted to [directly add](https://developers.google.com/workspace/classroom/guides/manage-users) users within their domain as students to courses within their domain. Students are permitted to add themselves to a course using an enrollment code. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to create students in this course or for access errors.
    - Required path params: courseId
    - Request body type: `Student`
    - Response type: `Student`
  - `delete` — Deletes a student of a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to delete students of this course or for access errors. * `NOT_FOUND` if no student of this course has the requested ID or if the course does not exist.
    - Required path params: courseId, userId
    - Response type: `Empty`
  - `get` — Returns a student of a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to view students of this course or for access errors. * `NOT_FOUND` if no student of this course has the requested ID or if the course does not exist.
    - Required path params: courseId, userId
    - Response type: `Student`
  - `list` — Returns a list of students of this course that the requester is permitted to view. This method returns the following error codes: * `NOT_FOUND` if the course does not exist. * `PERMISSION_DENIED` for access errors.
    - Required path params: courseId
    - Response type: `ListStudentsResponse`

### courses.teachers

  - `create` — Creates a teacher of a course. Domain administrators are permitted to [directly add](https://developers.google.com/workspace/classroom/guides/manage-users) users within their domain as teachers to courses within their domain. Non-admin users should send an Invitation instead. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to create teachers in this course or for access errors. * `NOT_FOUND` if the requested course ID does not exist.
    - Required path params: courseId
    - Request body type: `Teacher`
    - Response type: `Teacher`
  - `delete` — Removes the specified teacher from the specified course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to delete teachers of this course or for access errors. * `NOT_FOUND` if no teacher of this course has the requested ID or if the course does not exist. * `FAILED_PRECONDITION` if the requested ID belongs to the primary teacher of this course.
    - Required path params: courseId, userId
    - Response type: `Empty`
  - `get` — Returns a teacher of a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to view teachers of this course or for access errors. * `NOT_FOUND` if no teacher of this course has the requested ID or if the course does not exist.
    - Required path params: courseId, userId
    - Response type: `Teacher`
  - `list` — Returns a list of teachers of this course that the requester is permitted to view. This method returns the following error codes: * `NOT_FOUND` if the course does not exist. * `PERMISSION_DENIED` for access errors.
    - Required path params: courseId
    - Response type: `ListTeachersResponse`

### courses.topics

  - `create` — Creates a topic. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course, create a topic in the requested course, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `ALREADY_EXISTS` if there exists a topic in the course with the same name. * `FAILED_PRECONDITION` for the following request error: * CourseTopicLimitReached * `NOT_FOUND` if the requested course does not exist.
    - Required path params: courseId
    - Request body type: `Topic`
    - Response type: `Topic`
  - `delete` — Deletes a topic. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not allowed to delete the requested topic or for access errors. * `FAILED_PRECONDITION` if the requested topic has already been deleted. * `NOT_FOUND` if no course or topic exists with the requested ID.
    - Required path params: courseId, id
    - Response type: `Empty`
  - `get` — Returns a topic. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or topic, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course or topic does not exist.
    - Required path params: courseId, id
    - Response type: `Topic`
  - `list` — Returns the list of topics that the requester is permitted to view. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course does not exist.
    - Required path params: courseId
    - Response type: `ListTopicResponse`
  - `patch` — Updates one or more fields of a topic. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting developer project did not create the corresponding topic or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `FAILED_PRECONDITION` if there exists a topic in the course with the same name. * `NOT_FOUND` if the requested course or topic does not exist
    - Required path params: courseId, id
    - Request body type: `Topic`
    - Response type: `Topic`

### invitations

  - `accept` — Accepts an invitation, removing it and adding the invited user to the teachers or students (as appropriate) of the specified course. Only the invited user may accept an invitation. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to accept the requested invitation or for access errors.
    - Required path params: id
    - Response type: `Empty`
  - `create` — Creates an invitation. Only one invitation for a user and course may exist at a time. Delete and re-create an invitation to make changes. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to create invitations for this course or for access errors. * `NOT_FOUND` if the course or the user does not exist. * `FAILED_PRECONDITION`: * if the requested user's account is disabled.
    - Request body type: `Invitation`
    - Response type: `Invitation`
  - `delete` — Deletes an invitation. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to delete the requested invitation or for access errors. * `NOT_FOUND` if no invitation exists with the requested ID.
    - Required path params: id
    - Response type: `Empty`
  - `get` — Returns an invitation. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to view the requested invitation or for access errors. * `NOT_FOUND` if no invitation exists with the requested ID.
    - Required path params: id
    - Response type: `Invitation`
  - `list` — Returns a list of invitations that the requesting user is permitted to view, restricted to those that match the list request. *Note:* At least one of `user_id` or `course_id` must be supplied. Both fields can be supplied. This method returns the following error codes: * `PERMISSION_DENIED` for access errors.
    - Response type: `ListInvitationsResponse`

### registrations

  - `create` — Creates a `Registration`, causing Classroom to start sending notifications from the provided `feed` to the destination provided in `cloudPubSubTopic`. Returns the created `Registration`. Currently, this will be the same as the argument, but with server-assigned fields such as `expiry_time` and `id` filled in. Note that any value specified for the `expiry_time` or `id` fields will be ignored.
    - Request body type: `Registration`
    - Response type: `Registration`
  - `delete` — Deletes a `Registration`, causing Classroom to stop sending notifications for that `Registration`.
    - Required path params: registrationId
    - Response type: `Empty`

### userProfiles

  - `get` — Returns a user profile. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access this user profile, if no profile exists with the requested ID, or for access errors.
    - Required path params: userId
    - Response type: `UserProfile`

### userProfiles.guardianInvitations

  - `create` — Creates a guardian invitation, and sends an email to the guardian asking them to confirm that they are the student's guardian. Once the guardian accepts the invitation, their `state` will change to `COMPLETED` and they will start receiving guardian notifications. A `Guardian` resource will also be created to represent the active guardian. The request object must have the `student_id` and `invited_email_address` fields set.
    - Required path params: studentId
    - Request body type: `GuardianInvitation`
    - Response type: `GuardianInvitation`
  - `get` — Returns a specific guardian invitation. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to view guardian invitations for the student identified by the `student_id`, if guardians are not enabled for the domain in question, or for other access errors. * `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot be recognized (it is not an email address, nor a `student_id` from the API, nor the literal string `me`).
    - Required path params: invitationId, studentId
    - Response type: `GuardianInvitation`
  - `list` — Returns a list of guardian invitations that the requesting user is permitted to view, filtered by the parameters provided. This method returns the following error codes: * `PERMISSION_DENIED` if a `student_id` is specified, and the requesting user is not permitted to view guardian invitations for that student, if `"-"` is specified as the `student_id` and the user is not a domain administrator, if guardians are not enabled for the domain in question, or for other access errors.
    - Required path params: studentId
    - Response type: `ListGuardianInvitationsResponse`
  - `patch` — Modifies a guardian invitation. Currently, the only valid modification is to change the `state` from `PENDING` to `COMPLETE`. This has the effect of withdrawing the invitation. This method returns the following error codes: * `PERMISSION_DENIED` if the current user does not have permission to manage guardians, if guardians are not enabled for the domain in question or for other access errors. * `FAILED_PRECONDITION` if the guardian link is not in the `PENDING` state.
    - Required path params: invitationId, studentId
    - Request body type: `GuardianInvitation`
    - Response type: `GuardianInvitation`

### userProfiles.guardians

  - `delete` — Deletes a guardian. The guardian will no longer receive guardian notifications and the guardian will no longer be accessible via the API. This method returns the following error codes: * `PERMISSION_DENIED` if no user that matches the provided `student_id` is visible to the requesting user, if the requesting user is not permitted to manage guardians for the student identified by the `student_id`, if guardians are not enabled for the domain in question, or for other access errors.
    - Required path params: guardianId, studentId
    - Response type: `Empty`
  - `get` — Returns a specific guardian. This method returns the following error codes: * `PERMISSION_DENIED` if no user that matches the provided `student_id` is visible to the requesting user, if the requesting user is not permitted to view guardian information for the student identified by the `student_id`, if guardians are not enabled for the domain in question, or for other access errors.
    - Required path params: guardianId, studentId
    - Response type: `Guardian`
  - `list` — Returns a list of guardians that the requesting user is permitted to view, restricted to those that match the request. To list guardians for any student that the requesting user may view guardians for, use the literal character `-` for the student ID.
    - Required path params: studentId
    - Response type: `ListGuardiansResponse`

## Common Schemas

### AddOnAttachment

*Description: An add-on attachment on a post.*

| Field | Type | Description |
|---|---|---|
| `copyHistory` | array of `CopyHistory` | Output only. Identifiers of attachments that were previous copies of this attachment. |
| `courseId` | string | Immutable. Identifier of the course. |
| `dueDate` | `Date` | Date, in UTC, that work on this attachment is due. This must be specified if `due_time` is specified. |
| `dueTime` | `TimeOfDay` | Time of day, in UTC, that work on this attachment is due. This must be specified if `due_date` is specified. |
| `id` | string | Immutable. Classroom-assigned identifier for this attachment, unique per post. |
| `itemId` | string | Immutable. Identifier of the `Announcement`, `CourseWork`, or `CourseWorkMaterial` under which the attachment is attached. Unique per course. |
| `maxPoints` | number (format: double) | Maximum grade for this attachment. Can only be set if `studentWorkReviewUri` is set. Set to a non-zero value to indicate that the attachment supports grade passback. |
| `postId` | string | Immutable. Deprecated, use `item_id` instead. |
| `studentViewUri` | `EmbedUri` | Required. URI to show the student view of the attachment. The URI will be opened in an iframe with the `courseId`, `itemId`, `itemType`, and `attachmentId` query parameters set. |
| `studentWorkReviewUri` | `EmbedUri` | URI for the teacher to see student work on the attachment, if applicable. |
| `teacherViewUri` | `EmbedUri` | Required. URI to show the teacher view of the attachment. The URI will be opened in an iframe with the `courseId`, `itemId`, `itemType`, and `attachmentId` query parameters set. |
| `title` | string | Required. Title of this attachment. The title must be between 1 and 1000 characters. |

### AddOnAttachmentStudentSubmission

*Description: Payload for grade update requests.*

| Field | Type | Description |
|---|---|---|
| `courseWorkSubmissionId` | string | Output only. Identifier of the course work submission under which this attachment submission was made. |
| `id` | string | Output only. Classroom-assigned identifier for this student submission. This is unique among submissions for the relevant course work and add-on attachment combination. |
| `pointsEarned` | number (format: double) | Student grade on this attachment. If unset, no grade was set. |
| `postSubmissionState` | string | Submission state of add-on attachment's parent post (i.e. assignment). |
| `userId` | string | Identifier for the student that owns this submission. Requires the user to be a teacher in the course and have permission to read student submissions. |

### AddOnContext

*Description: Attachment-relevant metadata for Classroom add-ons in the context of a specific post.*

| Field | Type | Description |
|---|---|---|
| `courseId` | string | Immutable. Identifier of the course. |
| `itemId` | string | Immutable. Identifier of the `Announcement`, `CourseWork`, or `CourseWorkMaterial` under which the attachment is attached. |
| `postId` | string | Immutable. Deprecated, use `item_id` instead. |
| `studentContext` | `StudentContext` | Add-on context corresponding to the requesting user's role as a student. Its presence implies that the requesting user is a student in the course. |
| `supportsStudentWork` | boolean | Optional. Whether the post allows the teacher to see student work and passback grades. |
| `teacherContext` | `TeacherContext` | Add-on context corresponding to the requesting user's role as a teacher. Its presence implies that the requesting user is a teacher in the course. |

### Announcement

*Description: Announcement created by a teacher for students of the course*

| Field | Type | Description |
|---|---|---|
| `alternateLink` | string | Absolute link to this announcement in the Classroom web UI. This is only populated if `state` is `PUBLISHED`. Read-only. |
| `assigneeMode` | string | Assignee mode of the announcement. If unspecified, the default value is `ALL_STUDENTS`. |
| `courseId` | string | Identifier of the course. Read-only. |
| `creationTime` | string (format: google-datetime) | Timestamp when this announcement was created. Read-only. |
| `creatorUserId` | string | Identifier for the user that created the announcement. Read-only. |
| `id` | string | Classroom-assigned identifier of this announcement, unique per course. Read-only. |
| `individualStudentsOptions` | `IndividualStudentsOptions` | Identifiers of students with access to the announcement. This field is set only if `assigneeMode` is `INDIVIDUAL_STUDENTS`. |
| `materials` | array of `Material` | Additional materials. Announcements must have no more than 20 material items. |
| `scheduledTime` | string (format: google-datetime) | Optional timestamp when this announcement is scheduled to be published. |
| `state` | string | Status of this announcement. If unspecified, the default state is `DRAFT`. |
| `text` | string | Description of this announcement. The text must be a valid UTF-8 string containing no more than 30,000 characters. |
| `updateTime` | string (format: google-datetime) | Timestamp of the most recent change to this announcement. Read-only. |

### Course

*Description: A Course in Classroom.*

| Field | Type | Description |
|---|---|---|
| `alternateLink` | string | Absolute link to this course in the Classroom web UI. Read-only. |
| `calendarId` | string | The Calendar ID for a calendar that all course members can see, to which Classroom adds events for course work and announcements in the course. |
| `courseGroupEmail` | string | The email address of a Google group containing all members of the course. This group does not accept email and can only be used for permissions. Read-only. |
| `courseMaterialSets` | array of `CourseMaterialSet` | Sets of materials that appear on the "about" page of this course. Read-only. |
| `courseState` | string | State of the course. If unspecified, the default state is `PROVISIONED`. |
| `creationTime` | string (format: google-datetime) | Creation time of the course. Specifying this field in a course update mask results in an error. Read-only. |
| `description` | string | Optional description. For example, "We'll be learning about the structure of living creatures from a combination of textbooks, guest lectures, and lab work. |
| `descriptionHeading` | string | Optional heading for the description. For example, "Welcome to 10th Grade Biology." If set, this field must be a valid UTF-8 string and no longer than 3600 characters. |
| `enrollmentCode` | string | Enrollment code to use when joining this course. Specifying this field in a course update mask results in an error. Read-only. |
| `gradebookSettings` | `GradebookSettings` | The gradebook settings that specify how a student's overall grade for the course will be calculated and who it will be displayed to. Read-only. |
| `guardiansEnabled` | boolean | Whether or not guardian notifications are enabled for this course. Read-only. |
| `id` | string | Identifier for this course assigned by Classroom. When creating a course, you may optionally set this identifier to an alias string in the request to create a corresponding alias. |
| `levels` | string | Optional. Levels for the course. Examples: "9th grade", "Middle school", "4th - 5th", "K-2", "3000". If set, this field must be a valid UTF-8 string and fewer than 1000 characters. |
| `name` | string | Name of the course. For example, "10th Grade Biology". The name is required. It must be between 1 and 750 characters and a valid UTF-8 string. |
| `ownerId` | string | The identifier of the owner of a course. When specified as a parameter of a create course request, this field is required. |
| `room` | string | Optional room location. For example, "301". If set, this field must be a valid UTF-8 string and no longer than 650 characters. |
| `section` | string | Section of the course. For example, "Period 2". If set, this field must be a valid UTF-8 string and no longer than 2800 characters. |
| `subject` | string | Optional. The subject of the course. |
| `teacherFolder` | `DriveFolder` | Information about a Drive Folder that is shared with all teachers of the course. This field will only be set for teachers of the course and domain administrators. Read-only. |
| `teacherGroupEmail` | string | The email address of a Google group containing all teachers of the course. This group does not accept email and can only be used for permissions. Read-only. |
| `updateTime` | string (format: google-datetime) | Time of the most recent update to this course. Specifying this field in a course update mask results in an error. Read-only. |

### CourseAlias

*Description: Alternative identifier for a course. An alias uniquely identifies a course. It must be unique within one of the following scopes: * domain: A domain-scoped alias is visible to all users within the alias creator's domain and can be created only by a domain admin.*

| Field | Type | Description |
|---|---|---|
| `alias` | string | Alias string. The format of the string indicates the desired alias scoping. * `d:` indicates a domain-scoped alias. Example: `d:math_101` * `p:` indicates a project-scoped alias. |

### CourseWork

*Description: Course work created by a teacher for students of the course.*

| Field | Type | Description |
|---|---|---|
| `alternateLink` | string | Absolute link to this course work in the Classroom web UI. This is only populated if `state` is `PUBLISHED`. Read-only. |
| `assigneeMode` | string | Assignee mode of the coursework. If unspecified, the default value is `ALL_STUDENTS`. |
| `assignment` | `Assignment` | Assignment details. This is populated only when `work_type` is `ASSIGNMENT`. Read-only. |
| `associatedWithDeveloper` | boolean | Whether this course work item is associated with the Developer Console project making the request. See CreateCourseWork for more details. Read-only. |
| `courseId` | string | Identifier of the course. Read-only. |
| `creationTime` | string (format: google-datetime) | Timestamp when this course work was created. Read-only. |
| `creatorUserId` | string | Identifier for the user that created the coursework. Read-only. |
| `description` | string | Optional description of this course work. If set, the description must be a valid UTF-8 string containing no more than 30,000 characters. |
| `dueDate` | `Date` | Optional date, in UTC, that submissions for this course work are due. This must be specified if `due_time` is specified. |
| `dueTime` | `TimeOfDay` | Optional time of day, in UTC, that submissions for this course work are due. This must be specified if `due_date` is specified. |
| `gradeCategory` | `GradeCategory` | The category that this coursework's grade contributes to. Present only when a category has been chosen for the coursework. May be used in calculating the overall grade. Read-only. |
| `gradingPeriodId` | string | Identifier of the grading period associated with the coursework. * At creation, if unspecified, the grading period ID will be set based on the `dueDate` (or `scheduledTime` if no `dueDate` is set). |
| `id` | string | Classroom-assigned identifier of this course work, unique per course. Read-only. |
| `individualStudentsOptions` | `IndividualStudentsOptions` | Identifiers of students with access to the coursework. This field is set only if `assigneeMode` is `INDIVIDUAL_STUDENTS`. |
| `materials` | array of `Material` | Additional materials. CourseWork must have no more than 20 material items. |
| `maxPoints` | number (format: double) | Maximum grade for this course work. If zero or unspecified, this assignment is considered ungraded. This must be a non-negative integer value. |
| `multipleChoiceQuestion` | `MultipleChoiceQuestion` | Multiple choice question details. For read operations, this field is populated only when `work_type` is `MULTIPLE_CHOICE_QUESTION`. |
| `scheduledTime` | string (format: google-datetime) | Optional timestamp when this course work is scheduled to be published. |
| `state` | string | Status of this course work. If unspecified, the default state is `DRAFT`. |
| `submissionModificationMode` | string | Setting to determine when students are allowed to modify submissions. If unspecified, the default value is `MODIFIABLE_UNTIL_TURNED_IN`. |
| `title` | string | Title of this course work. The title must be a valid UTF-8 string containing between 1 and 3000 characters. |
| `topicId` | string | Identifier for the topic that this coursework is associated with. Must match an existing topic in the course. |
| `updateTime` | string (format: google-datetime) | Timestamp of the most recent change to this course work. Read-only. |
| `workType` | string | Type of this course work. The type is set when the course work is created and cannot be changed. |

### CourseWorkMaterial

*Description: Course work material created by a teacher for students of the course*

| Field | Type | Description |
|---|---|---|
| `alternateLink` | string | Absolute link to this course work material in the Classroom web UI. This is only populated if `state` is `PUBLISHED`. Read-only. |
| `assigneeMode` | string | Assignee mode of the course work material. If unspecified, the default value is `ALL_STUDENTS`. |
| `courseId` | string | Identifier of the course. Read-only. |
| `creationTime` | string (format: google-datetime) | Timestamp when this course work material was created. Read-only. |
| `creatorUserId` | string | Identifier for the user that created the course work material. Read-only. |
| `description` | string | Optional description of this course work material. The text must be a valid UTF-8 string containing no more than 30,000 characters. |
| `id` | string | Classroom-assigned identifier of this course work material, unique per course. Read-only. |
| `individualStudentsOptions` | `IndividualStudentsOptions` | Identifiers of students with access to the course work material. This field is set only if `assigneeMode` is `INDIVIDUAL_STUDENTS`. |
| `materials` | array of `Material` | Additional materials. A course work material must have no more than 20 material items. |
| `scheduledTime` | string (format: google-datetime) | Optional timestamp when this course work material is scheduled to be published. |
| `state` | string | Status of this course work material. If unspecified, the default state is `DRAFT`. |
| `title` | string | Title of this course work material. The title must be a valid UTF-8 string containing between 1 and 3000 characters. |
| `topicId` | string | Identifier for the topic that this course work material is associated with. Must match an existing topic in the course. |
| `updateTime` | string (format: google-datetime) | Timestamp of the most recent change to this course work material. Read-only. |

### Empty

*Description: A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }*

*(No fields)*

### GradingPeriodSettings

*Description: Grading period settings that include all the individual grading periods in a course.*

| Field | Type | Description |
|---|---|---|
| `applyToExistingCoursework` | boolean | Supports toggling the application of grading periods on existing stream items. |
| `gradingPeriods` | array of `GradingPeriod` | The list of grading periods in a specific course. Grading periods must not have overlapping date ranges and must be listed in chronological order. |

### Guardian

*Description: Association between a student and a guardian of that student. The guardian may receive information about the student's course work.*

| Field | Type | Description |
|---|---|---|
| `guardianId` | string | Identifier for the guardian. |
| `guardianProfile` | `UserProfile` | User profile for the guardian. |
| `invitedEmailAddress` | string | The email address to which the initial guardian invitation was sent. This field is only visible to domain administrators. |
| `studentId` | string | Identifier for the student to whom the guardian relationship applies. |

### GuardianInvitation

*Description: An invitation to become the guardian of a specified user, sent to a specified email address.*

| Field | Type | Description |
|---|---|---|
| `creationTime` | string (format: google-datetime) | The time that this invitation was created. Read-only. |
| `invitationId` | string | Unique identifier for this invitation. Read-only. |
| `invitedEmailAddress` | string | Email address that the invitation was sent to. This field is only visible to domain administrators. |
| `state` | string | The state that this invitation is in. |
| `studentId` | string | ID of the student (in standard format) |

### Invitation

*Description: An invitation to join a course.*

| Field | Type | Description |
|---|---|---|
| `courseId` | string | Identifier of the course to invite the user to. |
| `id` | string | Identifier assigned by Classroom. Read-only. |
| `role` | string | Role to invite the user to have. Must not be `COURSE_ROLE_UNSPECIFIED`. |
| `userId` | string | Identifier of the invited user. |

### ListAddOnAttachmentsResponse

*Description: Response when listing add-on attachments.*

| Field | Type | Description |
|---|---|---|
| `addOnAttachments` | array of `AddOnAttachment` | Attachments under the given post. |
| `nextPageToken` | string | A token, which can be sent as `pageToken` to retrieve the next page. If this field is omitted, there are no subsequent pages. |

### ListAnnouncementsResponse

*Description: Response when listing course work.*

| Field | Type | Description |
|---|---|---|
| `announcements` | array of `Announcement` | Announcement items that match the request. |
| `nextPageToken` | string | Token identifying the next page of results to return. If empty, no further results are available. |

### ListCourseAliasesResponse

*Description: Response when listing course aliases.*

| Field | Type | Description |
|---|---|---|
| `aliases` | array of `CourseAlias` | The course aliases. |
| `nextPageToken` | string | Token identifying the next page of results to return. If empty, no further results are available. |

### ListCourseWorkMaterialResponse

*Description: Response when listing course work material.*

| Field | Type | Description |
|---|---|---|
| `courseWorkMaterial` | array of `CourseWorkMaterial` | Course work material items that match the request. |
| `nextPageToken` | string | Token identifying the next page of results to return. If empty, no further results are available. |

### ListCourseWorkResponse

*Description: Response when listing course work.*

| Field | Type | Description |
|---|---|---|
| `courseWork` | array of `CourseWork` | Course work items that match the request. |
| `nextPageToken` | string | Token identifying the next page of results to return. If empty, no further results are available. |

### ListCoursesResponse

*Description: Response when listing courses.*

| Field | Type | Description |
|---|---|---|
| `courses` | array of `Course` | Courses that match the list request. |
| `nextPageToken` | string | Token identifying the next page of results to return. If empty, no further results are available. |

### ListGuardianInvitationsResponse

*Description: Response when listing guardian invitations.*

| Field | Type | Description |
|---|---|---|
| `guardianInvitations` | array of `GuardianInvitation` | Guardian invitations that matched the list request. |
| `nextPageToken` | string | Token identifying the next page of results to return. If empty, no further results are available. |

### ListGuardiansResponse

*Description: Response when listing guardians.*

| Field | Type | Description |
|---|---|---|
| `guardians` | array of `Guardian` | Guardians on this page of results that met the criteria specified in the request. |
| `nextPageToken` | string | Token identifying the next page of results to return. If empty, no further results are available. |

### ListInvitationsResponse

*Description: Response when listing invitations.*

| Field | Type | Description |
|---|---|---|
| `invitations` | array of `Invitation` | Invitations that match the list request. |
| `nextPageToken` | string | Token identifying the next page of results to return. If empty, no further results are available. |

### ListRubricsResponse

*Description: Response when listing rubrics.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | Token identifying the next page of results to return. If empty, no further results are available. |
| `rubrics` | array of `Rubric` | Rubrics that match the request. |

### ListStudentGroupMembersResponse

*Description: Response when listing students in a group.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | Token identifying the next page of results to return. If empty, no further results are available. |
| `studentGroupMembers` | array of `StudentGroupMember` | The student group members. |

### ListStudentGroupsResponse

*Description: Response when listing student groups.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | Token identifying the next page of results to return. If empty, no further results are available. |
| `studentGroups` | array of `StudentGroup` | The student groups. |

### ListStudentSubmissionsResponse

*Description: Response when listing student submissions.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | Token identifying the next page of results to return. If empty, no further results are available. |
| `studentSubmissions` | array of `StudentSubmission` | Student work that matches the request. |

### ListStudentsResponse

*Description: Response when listing students.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | Token identifying the next page of results to return. If empty, no further results are available. |
| `students` | array of `Student` | Students who match the list request. |

### ListTeachersResponse

*Description: Response when listing teachers.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | Token identifying the next page of results to return. If empty, no further results are available. |
| `teachers` | array of `Teacher` | Teachers who match the list request. |

### ListTopicResponse

*Description: Response when listing topics.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | Token identifying the next page of results to return. If empty, no further results are available. |
| `topic` | array of `Topic` | Topic items that match the request. |

### ModifyAnnouncementAssigneesRequest

*Description: Request to modify assignee mode and options of an announcement.*

| Field | Type | Description |
|---|---|---|
| `assigneeMode` | string | Mode of the announcement describing whether it is accessible by all students or specified individual students. |
| `modifyIndividualStudentsOptions` | `ModifyIndividualStudentsOptions` | Set which students can view or cannot view the announcement. Must be specified only when `assigneeMode` is `INDIVIDUAL_STUDENTS`. |

### ModifyAttachmentsRequest

*Description: Request to modify the attachments of a student submission.*

| Field | Type | Description |
|---|---|---|
| `addAttachments` | array of `Attachment` | Attachments to add. A student submission may not have more than 20 attachments. Form attachments are not supported. |

### ModifyCourseWorkAssigneesRequest

*Description: Request to modify assignee mode and options of a coursework.*

| Field | Type | Description |
|---|---|---|
| `assigneeMode` | string | Mode of the coursework describing whether it will be assigned to all students or specified individual students. |
| `modifyIndividualStudentsOptions` | `ModifyIndividualStudentsOptions` | Set which students are assigned or not assigned to the coursework. Must be specified only when `assigneeMode` is `INDIVIDUAL_STUDENTS`. |

### ReclaimStudentSubmissionRequest

*Description: Request to reclaim a student submission.*

*(No fields)*

### Registration

*Description: An instruction to Classroom to send notifications from the `feed` to the provided destination.*

| Field | Type | Description |
|---|---|---|
| `cloudPubsubTopic` | `CloudPubsubTopic` | The Cloud Pub/Sub topic that notifications are to be sent to. |
| `expiryTime` | string (format: google-datetime) | The time until which the `Registration` is effective. This is a read-only field assigned by the server. |
| `feed` | `Feed` | Specification for the class of notifications that Classroom should deliver to the destination. |
| `registrationId` | string | A server-generated unique identifier for this `Registration`. Read-only. |

### ReturnStudentSubmissionRequest

*Description: Request to return a student submission.*

*(No fields)*

### Rubric

*Description: The rubric of the course work. A rubric is a scoring guide used to evaluate student work and give feedback. For further details, see [Rubrics structure and known limitations](/classroom/rubrics/limitations).*

| Field | Type | Description |
|---|---|---|
| `courseId` | string | Identifier of the course. Read-only. |
| `courseWorkId` | string | Identifier for the course work this corresponds to. Read-only. |
| `creationTime` | string (format: google-datetime) | Output only. Timestamp when this rubric was created. Read-only. |
| `criteria` | array of `Criterion` | List of criteria. Each criterion is a dimension on which performance is rated. |
| `id` | string | Classroom-assigned identifier for the rubric. This is unique among rubrics for the relevant course work. Read-only. |
| `sourceSpreadsheetId` | string | Input only. Immutable. Google Sheets ID of the spreadsheet. This spreadsheet must contain formatted rubric settings. |
| `updateTime` | string (format: google-datetime) | Output only. Timestamp of the most recent change to this rubric. Read-only. |

### Student

*Description: Student in a course.*

| Field | Type | Description |
|---|---|---|
| `courseId` | string | Identifier of the course. Read-only. |
| `profile` | `UserProfile` | Global user information for the student. Read-only. |
| `studentWorkFolder` | `DriveFolder` | Information about a Drive Folder for this student's work in this course. Only visible to the student and domain administrators. Read-only. |
| `userId` | string | Identifier of the user. |

### StudentGroup

*Description: A student group in a course.*

| Field | Type | Description |
|---|---|---|
| `courseId` | string | The identifier of the course. |
| `id` | string | The identifier of the student group. |
| `title` | string | The title of the student group. |

### StudentGroupMember

*Description: A student member in a student group.*

| Field | Type | Description |
|---|---|---|
| `courseId` | string | The identifier of the course. |
| `studentGroupId` | string | The identifier of the student group. |
| `userId` | string | Identifier of the student. |

### StudentSubmission

*Description: Student submission for course work. `StudentSubmission` items are generated when a `CourseWork` item is created. Student submissions that have never been accessed (i.e. with `state` = NEW) may not have a creation time or update time.*

| Field | Type | Description |
|---|---|---|
| `alternateLink` | string | Absolute link to the submission in the Classroom web UI. Read-only. |
| `assignedGrade` | number (format: double) | Optional grade. If unset, no grade was set. This value must be non-negative. Decimal (that is, non-integer) values are allowed, but are rounded to two decimal places. |
| `assignedRubricGrades` | object | Assigned rubric grades based on the rubric's Criteria. This map is empty if there is no rubric attached to this course work or if a rubric is attached, but no grades have been set on any Criteria. |
| `assignmentSubmission` | `AssignmentSubmission` | Submission content when course_work_type is ASSIGNMENT. Students can modify this content using ModifyAttachments. |
| `associatedWithDeveloper` | boolean | Whether this student submission is associated with the Developer Console project making the request. See CreateCourseWork for more details. Read-only. |
| `courseId` | string | Identifier of the course. Read-only. |
| `courseWorkId` | string | Identifier for the course work this corresponds to. Read-only. |
| `courseWorkType` | string | Type of course work this submission is for. Read-only. |
| `creationTime` | string (format: google-datetime) | Creation time of this submission. This may be unset if the student has not accessed this item. Read-only. |
| `draftGrade` | number (format: double) | Optional pending grade. If unset, no grade was set. This value must be non-negative. Decimal (that is, non-integer) values are allowed, but are rounded to two decimal places. |
| `draftRubricGrades` | object | Pending rubric grades based on the rubric's criteria. This map is empty if there is no rubric attached to this course work or if a rubric is attached, but no grades have been set on any criteria. |
| `id` | string | Classroom-assigned Identifier for the student submission. This is unique among submissions for the relevant course work. Read-only. |
| `late` | boolean | Whether this submission is late. Read-only. |
| `multipleChoiceSubmission` | `MultipleChoiceSubmission` | Submission content when course_work_type is MULTIPLE_CHOICE_QUESTION. |
| `shortAnswerSubmission` | `ShortAnswerSubmission` | Submission content when course_work_type is SHORT_ANSWER_QUESTION. |
| `state` | string | State of this submission. Read-only. |
| `submissionHistory` | array of `SubmissionHistory` | The history of the submission (includes state and grade histories). Read-only. |
| `updateTime` | string (format: google-datetime) | Last update time of this submission. This may be unset if the student has not accessed this item. Read-only. |
| `userId` | string | Identifier for the student that owns this submission. Read-only. |

### Teacher

*Description: Teacher of a course.*

| Field | Type | Description |
|---|---|---|
| `courseId` | string | Identifier of the course. Read-only. |
| `profile` | `UserProfile` | Global user information for the teacher. Read-only. |
| `userId` | string | Identifier of the user. |

### Topic

*Description: Topic created by a teacher for the course*

| Field | Type | Description |
|---|---|---|
| `courseId` | string | Identifier of the course. Read-only. |
| `name` | string | The name of the topic, generated by the user. Leading and trailing whitespaces, if any, are trimmed. Also, multiple consecutive whitespaces are collapsed into one inside the name. |
| `topicId` | string | Unique identifier for the topic. Read-only. |
| `updateTime` | string (format: google-datetime) | The time the topic was last updated by the system. Read-only. |

### TurnInStudentSubmissionRequest

*Description: Request to turn in a student submission.*

*(No fields)*

### UserProfile

*Description: Global information for a user.*

| Field | Type | Description |
|---|---|---|
| `emailAddress` | string | Email address of the user. Must request `https://www.googleapis.com/auth/classroom.profile.emails` scope for this field to be populated in a response body. Read-only. |
| `id` | string | Identifier of the user. Read-only. |
| `name` | `Name` | Name of the user. Read-only. |
| `permissions` | array of `GlobalPermission` | Global permissions of the user. Read-only. |
| `photoUrl` | string | URL of user's profile photo. Must request `https://www.googleapis.com/auth/classroom.profile.photos` scope for this field to be populated in a response body. Read-only. |
| `verifiedTeacher` | boolean | Represents whether a Google Workspace for Education user's domain administrator has explicitly verified them as being a teacher. |

## Discovering Commands

Before calling any API method, inspect it:

```bash
# Browse resources and methods
gws classroom --help

# Inspect a method's required params, types, and defaults
gws schema classroom.<resource>.<method>
```

Use `gws schema` output to build your `--params` and `--json` flags.

