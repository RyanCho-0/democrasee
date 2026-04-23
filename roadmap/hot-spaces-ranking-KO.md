# Hot Spaces Ranking

**Status**: Ready for design (Stage 2)
**Slug**: `hot-spaces-ranking`
**Primary use case**: 홈 첫 화면에서 Hot Spaces를 빠르고 신뢰 가능한 전역 랭킹으로 보여주기
**English**: `roadmap/hot-spaces-ranking.md`

## Problem

Ratel 홈의 Hot Spaces는 첫 화면 경험치 치고 너무 느립니다. 실제 사용에서는 홈에 진입할 때 Hot Spaces 때문에 대략 **1-2초**를 기다리게 됩니다.

문제는 단순 레이턴시만이 아닙니다.

- 현재 방식은 전체 공개 스페이스가 아니라 제한된 fetch window 안에서만 재정렬합니다.
- 공개 스페이스 수가 늘어나면 진짜로 뜨거운 스페이스가 첫 페이지에서 빠질 수 있습니다.
- 요청 시점 정렬 때문에 페이지네이션이 불안정해질 수 있습니다.
- 카드에 보이는 action count를 요청마다 비싸게 계산해서 홈 진입 속도가 데이터 양에 따라 계속 느려집니다.

유저 입장에서는 첫 화면이 굼뜨고, "Hot"이라는 라벨 자체가 믿기지 않게 됩니다.

## Goal

Hot Spaces를 **빠르고, 신뢰 가능하며, 전역적으로 정렬된** 홈 디스커버리 표면으로 만든다. 첫 로드는 즉각적으로 느껴져야 하고, 랭킹은 전체 eligible public space를 기준으로 해야 하며, 최근 참여/액션 변화는 충분히 빨리 반영되어야 한다.

## Non-goals

- 이번 로드맵에서 홈 캐러셀의 비주얼 레이아웃, 애니메이션, 카드 스타일을 다시 디자인하지 않는다.
- 스페이스 내부 랭킹 위젯이나 인센티브 풀 랭킹은 다루지 않는다.
- Phase 1에서는 랭킹 공식을 유저에게 설명하는 UI를 추가하지 않는다.
- Phase 1에서는 시즌 초기화, 운영자 수동 pin, 수동 큐레이션 도구를 추가하지 않는다.
- 실제 사용 데이터가 필요하다고 입증되기 전까지, 시간 감쇠(decay)는 Phase 1 요구사항으로 넣지 않는다.

## User stories

- 홈 방문자로서, 첫 화면이 멈춘 것처럼 느껴지지 않도록 Hot Spaces가 빨리 로드되길 원한다.
- 디스커버리 표면을 보는 유저로서, 스페이스 수가 많아져도 정말 뜨거운 public space가 보여야 랭킹을 신뢰할 수 있다.
- 스페이스 관리자로서, 우리 스페이스에 새 참여자나 새 action이 생기면 Hot Spaces에도 곧 반영되어 실제 모멘텀이 보이길 원한다.
- Hot Spaces를 페이지네이션하며 보는 유저로서, 카드가 중복되거나 빠지지 않는 안정적인 순서를 원한다.

## Functional requirements

### FR-1: 전역 랭킹 eligibility

1. 시스템은 현재 **Published + Public** 상태이며 홈 디스커버리에 eligible한 전체 스페이스 집합을 기준으로 Hot Spaces를 정렬해야 한다.
2. Hot Spaces 첫 페이지는 고정 fetch window 안의 상위권이 아니라, 현재 랭킹 모델 기준 실제 상위 eligible 스페이스를 반환해야 한다.
3. Published + Public이 아닌 스페이스는 Hot Spaces에 노출되면 안 된다.
4. 랭킹 슬롯을 차지했어야 할 스페이스가 eligibility를 잃은 경우, 시스템은 다음 eligible 스페이스로 그 슬롯을 채워 페이지 길이가 조용히 줄어들지 않게 해야 한다.

### FR-2: 홈 로드 성능

5. Hot Spaces 첫 페이지는 **Constraints**에 정의된 성능 예산을 대표적인 운영 데이터셋에서 만족해야 한다.
6. Published + Public 스페이스 수가 증가해도 Hot Spaces 첫 페이지 레이턴시는 본질적으로 안정적으로 유지되어야 하며, eligible space 수에 비례해 선형 증가하면 안 된다.
7. 응답은 기존 홈 UI가 필요로 하는 카드 데이터, 즉 title, description, participant count, action counts, heat state, rank를 같은 성능 예산 안에서 제공해야 한다.

### FR-3: 랭킹 freshness

8. 스페이스의 상대적 hotness를 바꾸는 participant join은 **60초 이내**에 Hot Spaces에 반영되어야 한다.
9. 스페이스의 상대적 hotness 또는 action count를 바꾸는 action 생성은 **60초 이내**에 Hot Spaces에 반영되어야 한다.
10. 스페이스의 상대적 hotness 또는 action count를 바꾸는 action 삭제는 **60초 이내**에 Hot Spaces에 반영되어야 한다.

### FR-4: 페이지네이션 안정성

11. Hot Spaces의 pagination/bookmark는 페이지 간 랭킹 순서를 보존해야 하며, client-side 재정렬 때문에 카드 중복, 누락, 순서 뒤바뀜이 생기면 안 된다.
12. 각 아이템의 `rank` 값은 API가 반환한 순서와 일치해야 한다.

### FR-5: 홈 표면 간 count 일관성

13. 홈 화면은 같은 count 데이터를 보여주는 다른 홈 섹션에서도 동일한 per-space action-count 병목을 반복하면 안 된다.
14. `My Spaces` 홈 표면도 Hot Spaces와 동일한 freshness budget으로 action count를 보여줘야 한다.
15. freshness 전파가 끝난 뒤에는 Hot Spaces와 My Spaces가 같은 스페이스의 action count에서 서로 다르면 안 된다.

## Acceptance criteria

- [ ] AC-1: Published + Public 스페이스가 50개를 넘는 데이터셋에서도, 현재 fetch-window 경계 밖에 있던 진짜 상위권 스페이스가 Hot Spaces 첫 페이지에 나타난다.
- [ ] AC-2: 대표적인 운영 유사 데이터셋에서 Hot Spaces 첫 페이지가 **Constraints**의 레이턴시 예산을 만족하며, 현재의 1-2초 첫 로드 지연이 사라진다.
- [ ] AC-3: Published + Public 스페이스 수가 적은 경우와 많은 경우를 비교해도 Hot Spaces 첫 페이지 레이턴시가 선형적으로 증가하지 않는다.
- [ ] AC-4: participant join 이후 다음 홈 로드에서 해당 스페이스의 Hot Spaces 순위가 60초 이내에 반영된다.
- [ ] AC-5: 새 action 생성 이후 카드의 action count와 랭킹 영향이 60초 이내에 보인다.
- [ ] AC-6: action 삭제 이후 카드의 action count와 랭킹 영향이 60초 이내에 보인다.
- [ ] AC-7: 이전에는 Hot 했던 스페이스가 non-public 또는 unpublished로 바뀌면 60초 이내에 Hot Spaces에서 사라지고, 첫 페이지는 다음 eligible 스페이스로 채워져 full page를 유지한다.
- [ ] AC-8: Hot Spaces 1페이지와 2페이지를 연속으로 로드해도 중복 카드, 누락 카드, client-side 재정렬 흔적이 없다.
- [ ] AC-9: `My Spaces` 홈 섹션도 같은 freshness window 안에서 action count를 보여주며, 눈에 띄는 per-space 로딩 지연을 다시 만들지 않는다.

## Constraints

- **성능 예산**: 수백 개 이상의 eligible public space를 포함한 대표적인 운영 유사 데이터셋에서, Hot Spaces 첫 페이지 서버 응답은 cold start를 제외하고 **P50 <= 300 ms**, **P95 <= 700 ms**를 목표로 한다.
- **Freshness 예산**: 랭크와 count는 eventual consistency를 허용하되, join / action create / action delete로 인한 유저 가시 변화는 **60초 이내**에 전파되어야 한다.
- **Eligibility 경계**: **Published + Public** 스페이스만 Hot Spaces 대상이다. Private 또는 unpublished 스페이스가 홈 디스커버리 표면에 새어 나오면 안 된다.
- **하위 호환성**: 기존 홈 카드 스키마와 캐러셀 UI는 Phase 1에서 유지한다. 이번 로드맵은 비주얼 디자인이 아니라 랭킹 동작과 성능을 바꾼다.
- **확장성**: 홈 읽기 경로는 전체 public-space 개수에 비례하는 요청 시점 작업에 의존하면 안 된다.

## Open questions

- OQ-1: participant 증가를 장기 신호로 더 크게 볼지, action 생성을 단기 버스트로 더 강하게 볼지 어떻게 튜닝할까? 제안 기본값: 실제 사용 데이터를 본 뒤 조정하고, Phase 1에서는 유저-facing 공식은 노출하지 않는다.
- OQ-2: 한때 인기였지만 지금은 죽은 스페이스를 시간에 따라 감쇠시킬까? 제안 기본값: Phase 1에서는 decay 없음, 라이브 데이터 확인 후 재검토.
- OQ-3: 왜 해당 스페이스가 Hot인지 설명하는 UI가 필요할까? 제안 기본값: Phase 1에서는 설명 UI 없음.

## References

- `app/ratel/src/features/spaces/space_common/controllers/list_hot_spaces.rs`
- `app/ratel/src/features/spaces/space_common/controllers/list_my_home_spaces.rs`
- `.claude/rules/workflows/roadmap-elaboration.md`
