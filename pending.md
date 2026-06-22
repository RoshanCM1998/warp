# Pending: Progressive streaming for the code review panel

## Status
The **speed fix is done** (this commit): Head-mode diff loading now runs per-file
diffs concurrently (bounded at 8) instead of one-at-a-time — see
`app/src/code_review/diff_state/local.rs`
(`diff_state_against_head`, `diff_state_against_head_split`, `MAX_CONCURRENT_FILE_LOADS`).
That removes the multi-second "panel keeps loading" spin on larger diffs.

This file tracks the **follow-up** we scoped out: *progressive streaming* — render the
file list immediately and let each file's diff appear as it finishes loading, instead of
waiting for the whole batch to complete.

## Agreed UX
- Rows render immediately (the header +/- stats already load instantly via `git diff --numstat`).
- Every file row is visible up-front and fills in as each file's diff+content finishes
  loading ("keep all files open — they just look empty until loaded").
- First file is prioritized to load first.
- A file the user manually collapsed stays collapsed (honor the existing `file_expanded` map).

## Why deferred
The parallel-load speed fix already kills the actual complaint (seconds of spinning).
Streaming is extra polish and a bigger, riskier change.

## Why it's not trivial (findings from investigation)
- The existing per-file streaming queue `SyncQueue::new_streaming`
  (`crates/warp_core/src/sync_queue.rs:371`) processes tasks **sequentially**
  (`while let Some(task) = receiver.next().await`). Reusing it gives streaming but
  **not** parallelism — so streaming needs a **custom parallel-stream loader**, not the queue.
- The view's lightweight per-file update path `update_from_single_file_diff_result`
  (`(Some, Some)` branch, `app/src/code_review/code_review_view.rs:2528`) only refreshes
  `file_diff` (hunks) — it does **not** rebuild the editor or apply `content_at_head`.
  A skeleton row with no editor would never be populated by that path; filling it needs the
  rebuild branch (`shift_remove` + `build_view_state_for_file_diffs`), like the
  `status_changed` branch does.

## Implementation sketch
1. **Model (`local.rs`)** — for Head + staging-split: spawn a producer future that
   (a) emits a skeleton (file list + per-file numstat counts, empty hunks), then
   (b) loads each file bounded-parallel and pipes each result over an `async_channel`;
   consume via `ctx.spawn_stream_local`, emitting `DiffStateModelEvent::SingleFileUpdated`
   per file. Load the first file first.
2. **Suppression** — the initial load sets `invalidate_all_pending` via
   `queue_full_invalidation` (`local.rs:470`), which drops `SingleFileUpdated`
   (`local.rs:236`). Clear it (or route initial-load emissions around it) after the skeleton
   is emitted, while keeping watcher-driven invalidation + merge-base flush correct.
3. **View (`code_review_view.rs`)** — render skeleton rows (no editor) without flashing the
   "No changes" empty state; in `update_from_single_file_diff_result`, when a row has no
   editor yet (skeleton), rebuild it via `build_view_state_for_file_diffs` instead of the
   lightweight update. Keep `viewported_list_state` stable across per-file fills.
4. **Scope** — Head + staging-split first; branch-compare modes
   (`diff_state_against_base_branch` / `diff_state_against_specific_branch`) follow the same
   pattern later.

## Risks to watch
- Manual-collapse surviving a late-arriving load event.
- List-state / scroll jumping as rows grow from empty placeholder to real content
  (cheap mitigation: reserve placeholder height from the per-file numstat line count).
- Don't regress file-watcher invalidation, staging, remote-repo serving, or the
  `load_duration` telemetry.
