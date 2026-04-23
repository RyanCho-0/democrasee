# Hot Spaces Ranking

**Status**: Ready for design (Stage 2)
**Slug**: `hot-spaces-ranking`
**Primary use case**: Home visitors should see a fast, globally ranked Hot Spaces carousel on first load
**Korean translation**: `roadmap/hot-spaces-ranking-KO.md`

## Problem

Ratel's home Hot Spaces surface is currently too slow for a first-screen experience. In real use, entering the home screen can spend roughly **1-2 seconds** waiting for Hot Spaces.

The underlying product problem is not only latency:

- The current flow ranks within a limited fetch window instead of the full eligible public-space set.
- The ranking can miss genuinely hot spaces once the total number of public spaces grows.
- Pagination becomes unstable when the server re-sorts a fetched window at read time.
- Action counts shown on the cards are expensive to derive during the request, which makes home latency grow with the amount of data.

For users, this means the very first screen feels slow and the label "Hot" stops feeling trustworthy.

## Goal

Make Hot Spaces a **fast, credible, globally ranked** home surface: first load should feel responsive, the ranking should reflect the full set of eligible public spaces, and recent participation/action changes should appear quickly enough to feel alive.

## Non-goals

- No redesign of the home carousel's visual layout, animations, or card styling in this roadmap.
- No changes to space-internal ranking widgets or incentive-pool rankings.
- No user-facing explanation of the ranking formula in Phase 1.
- No seasonal reset, editorial pinning, or manual curation tools in Phase 1.
- No time-decay / freshness-decay requirement in Phase 1 unless real usage proves it necessary.

## User stories

- As a home visitor, I want the Hot Spaces carousel to load quickly so the first screen does not feel stalled or broken.
- As a user browsing discovery surfaces, I want the hottest public spaces to appear even when there are many spaces, so the ranking feels credible.
- As a space admin, I want new participation and new actions in my space to affect Hot Spaces soon after they happen, so the home surface reflects real momentum.
- As a user paging through Hot Spaces, I want stable ordering with no skipped or duplicated cards, so pagination feels reliable.

## Functional requirements

### FR-1: Global ranking eligibility

1. The system SHALL rank Hot Spaces across the full set of spaces that are currently **Published + Public** and eligible for home discovery.
2. The Hot Spaces first page SHALL return the actual top-ranked eligible spaces under the current ranking model, not merely the best spaces inside a fixed fetch window.
3. A space that is not Published + Public SHALL NOT appear in Hot Spaces.
4. When an ineligible space would otherwise occupy a ranked slot, the system SHALL fill that slot with the next eligible space so the page does not silently shrink.

### FR-2: Home-load performance

5. The Hot Spaces first page SHALL meet the performance budget defined in **Constraints** on a representative production-like dataset.
6. Hot Spaces read latency SHALL remain effectively stable as the total number of Published + Public spaces grows; adding more eligible spaces SHALL NOT create linear growth in first-page latency.
7. The response SHALL include the card data needed by the existing home UI, including title, description, participant count, action counts, heat state, and rank, within the same performance budget.

### FR-3: Ranking freshness

8. A participant join that changes a space's relative hotness SHALL be reflected in Hot Spaces within **60 seconds**.
9. Creating a space action that changes a space's relative hotness or action counts SHALL be reflected in Hot Spaces within **60 seconds**.
10. Deleting a space action that changes a space's relative hotness or action counts SHALL be reflected in Hot Spaces within **60 seconds**.

### FR-4: Pagination stability

11. Pagination/bookmarks for Hot Spaces SHALL preserve ranking order across pages without client-side re-sorting that can introduce duplicates, skips, or reordered cards between page loads.
12. The `rank` shown for each returned item SHALL match the order returned by the API.

### FR-5: Home-surface count consistency

13. The home screen SHALL avoid repeating the same per-space action-count bottleneck in neighboring home surfaces that display the same count data.
14. The `My Spaces` home surface SHALL use the same freshness budget for action counts as Hot Spaces.
15. After freshness propagation completes, Hot Spaces and My Spaces SHALL NOT disagree on action counts for the same space.

## Acceptance criteria

- [ ] AC-1: In a dataset with more than 50 Published + Public spaces, a legitimately top-ranked space outside the current fetch-window boundary still appears on the first Hot Spaces page.
- [ ] AC-2: On a representative production-like dataset, the first Hot Spaces page meets the latency budget in **Constraints** and no longer exhibits the current 1-2 second first-load stall.
- [ ] AC-3: Increasing the number of Published + Public spaces from a small dataset to a large dataset does not cause first-page Hot Spaces latency to grow linearly.
- [ ] AC-4: After a participant joins a space, that space's Hot Spaces rank updates within 60 seconds on the next home load.
- [ ] AC-5: After a new space action is created, the card's action counts and ranking impact are visible within 60 seconds.
- [ ] AC-6: After a space action is deleted, the card's action counts and ranking impact are visible within 60 seconds.
- [ ] AC-7: If a previously hot space becomes non-public or unpublished, it disappears from Hot Spaces within 60 seconds and the first page still returns a full page by filling the next eligible space.
- [ ] AC-8: Loading Hot Spaces page 1 and then page 2 produces no duplicates, no skipped spaces, and no client-side reshuffle artifacts.
- [ ] AC-9: The My Spaces home section shows action counts within the same freshness window without reintroducing obvious per-space loading slowdown.

## Constraints

- **Performance budget**: for a representative production-like dataset with at least hundreds of eligible public spaces, the server-side Hot Spaces first-page path should target **P50 <= 300 ms** and **P95 <= 700 ms**, excluding cold starts.
- **Freshness budget**: rank and count updates may be eventually consistent, but visible changes from joins/action create/action delete must propagate within **60 seconds**.
- **Eligibility boundary**: only **Published + Public** spaces are eligible for Hot Spaces. Private or unpublished spaces must not leak into the home discovery surface.
- **Backward compatibility**: the existing home card schema and carousel UI remain intact in Phase 1; this roadmap changes ranking behavior and performance, not the visual design.
- **Scalability**: the home read path must not rely on request-time work that scales with the total public-space population.

## Open questions

- OQ-1: Should participant growth carry more long-term weight than action creation, or should both be tuned to reward short bursts of activity more aggressively? Proposed default: tune after observing real usage; no user-facing formula in Phase 1.
- OQ-2: Should old-but-once-popular spaces decay over time? Proposed default: no decay in Phase 1; revisit after live data.
- OQ-3: Should the product surface explain *why* a space is hot? Proposed default: no explanatory UI in Phase 1.

## References

- `app/ratel/src/features/spaces/space_common/controllers/list_hot_spaces.rs`
- `app/ratel/src/features/spaces/space_common/controllers/list_my_home_spaces.rs`
- `.claude/rules/workflows/roadmap-elaboration.md`
