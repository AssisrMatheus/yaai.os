---
description: How a form validates with one shared schema, submits through the trusted side, maps errors back to fields and behaves for keyboard, touch and password managers.
tags: [best-practices, ui, forms]
---

# Forms

Holds for any UI framework. Validation at the trusted boundary is in [[best-practices/data-boundaries.rules]]; labels and
announcements in [[best-practices/accessibility.rules]].

## Validation

### BP-FORM-001: Validate with one schema on both sides

One schema per form is used by the form and by the trusted side that handles it; validation rules live in the schema, not in control attributes.
Why: both sides make the same accept or reject decision, and control-level rules never reach the trusted side.
Check: review

### BP-FORM-002: Keep the schema importable by the trusted side

The schema sits beside the form in a module with no client-only marker; a schema used by several forms moves to a shared module.
Why: the trusted side imports the same file.
Check: review

### BP-FORM-003: Translate schema messages

Schema error messages are message keys or come from a translation factory, never literal text.
Why: errors must read in the user's language ([[best-practices/i18n.rules]]).
Check: review

### BP-FORM-004: Re-validate on the trusted side

The handler parses its input with the same schema before calling the service; when the form validated with the identical schema, a trusted-side field failure returns one form-level message.
Why: the client can be bypassed, and such a failure means a tampered request.
Check: review

### BP-FORM-005: Let the schema be the only validator

Disable native validation on the form, put no `required` or pattern attributes on controls, and mark required fields with `aria-required`.
Why: two validators disagree on messages and timing.
Check: review

### BP-FORM-006: Validate on touch or submit

Validate a field when it is first left or on submit, not on every keystroke; always set default values.
Why: early errors interrupt typing, and defaults make dirty-checking and reset work.
Check: review

## Controls

### BP-FORM-007: Use a native form

Wrap fields in a form element so Enter submits; a multi-line field submits with Cmd or Ctrl plus Enter.
Why: keyboard users expect submission to work without the mouse.
Check: review

### BP-FORM-008: Leave editing to the platform

Use uncontrolled inputs where the platform handles editing; never combine a registered input with a manually set value.
Why: half-controlled inputs lose keystrokes and cursor position.
Check: review

### BP-FORM-009: Keep custom inputs library-neutral

A custom input exposes plain value, change, blur and error props and is adapted to the form library at the use site.
Why: the input works in any form and any test.
Check: review

### BP-FORM-010: Use the right input type and autocomplete

Choose the input type matching the data (email, tel, url, number, search, password) and set `autocomplete` tokens on personal-data fields; turn off autocomplete and spellcheck where they do not apply.
Why: the right keyboard and autofill appear, and noise does not.
Check: review

### BP-FORM-011: Keep paste and password managers working

Never block paste or strip what a password manager fills in.
Why: both are how people enter credentials safely.
Check: review

### BP-FORM-012: Size text inputs at 16 CSS px or more

Text inputs use at least 16 CSS pixels.
Why: smaller text makes mobile browsers zoom on focus.
Check: review

### BP-FORM-013: Make the whole label clickable

Clicking a label focuses or toggles its control, and a checkbox row has no dead zone between box and text.
Why: small targets are missed.
Check: review

### BP-FORM-014: Overlay input decorations

Icons and prefixes sit over the input with padding and focus the input when clicked; they are not separate siblings.
Why: the whole field behaves as one target.
Check: review

### BP-FORM-015: Autofocus only in dialogs and not on touch

Focus the first field when a dialog opens, but never autofocus on touch devices.
Why: autofocus on touch opens the keyboard unasked.
Check: review

### BP-FORM-016: Prefill from context

Prefill fields with what is already known (the signed-in user, the item being edited).
Why: retyping known data is friction and a source of typos.
Check: review

## Submission and errors

### BP-FORM-017: Return stable codes and field paths

A failed submission returns a stable code, a message key and field paths; field errors go to their fields, everything else to a form-level message.
Why: the form renders errors in place and in the user's language.
Check: review

### BP-FORM-018: Associate errors with their fields

Each field error is linked to its control (invalid state and description), and form-level failures are announced through an alert or live region.
Why: screen-reader users hear what went wrong and where.
Check: review

### BP-FORM-019: Say how to fix the input

Error text says how to correct the input, not only that it is wrong.
Why: a user who knows the fix completes the form.
Check: review

### BP-FORM-020: Keep values after a failure

A failed submission keeps everything the user entered.
Why: retyping after an error is how forms get abandoned.
Check: review

### BP-FORM-021: Prevent duplicate submission

Disable submission while a request is pending and restore the controls when it completes.
Why: double clicks send duplicate requests.
Check: review

### BP-FORM-022: Confirm or allow undoing destructive actions

A destructive action asks for confirmation in a dialog, or can be undone.
Why: irreversible mistakes are the costliest kind.
Check: review

### BP-FORM-023: Upload files outside the form request

Upload files directly to storage and submit references, never file bodies through the form handler.
Why: handler request bodies have size limits and tie up the trusted side.
Check: review

### BP-FORM-024: Colocate a form with its schema and handler

The form, its schema and a handler used only by it sit in one folder; a handler shared by several features goes to that domain's module.
Why: the three change together and are reviewed together.
Check: review

### BP-FORM-025: Submit through the resource's single transport

A form submits through the same transport the rest of the app uses for that resource.
Why: one transport keeps one cache and one error model.
Check: review

## Covers

- `apps/`, `packages/` (forms, schemas and submit handlers)
