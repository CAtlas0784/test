---
name: hsr-challenge-protocol
description: Complete specification and protocol guide for Forgotten Hall, Memory of Chaos (MoC), Pure Fiction, and Apocalyptic Shadow unlocks, star bitmasks, active stage mappings, and packet handlers.
---

# HSR Challenge & Endgame Protocol Guide

## 1. Star Bitmask & Completion Flags
In `Challenge` protobuf messages:
- `star` is a **3-bit bitfield mask**, NOT an integer count:
  - Bit 0 (`1 = 0b001`): First objective star
  - Bit 1 (`2 = 0b010`): Second objective star
  - Bit 2 (`4 = 0b100`): Third objective star
  - **Full 3 Stars**: Must set `star = 7` (`1 | 2 | 4 = 0b111`).
- `taken_reward`: Set to 7 (`1 | 2 | 4`) so all 3 star rewards are marked claimed.
- `record_id`: Set to 1 (or non-zero). Indicates a valid cleared battle record exists.
- `hgpkmhfpmbj`: Must be `false`! In IL2CPP client, this maps to `set_IsFirstOpen(bool)`. Setting it to `true` marks the stage as "Newly opened and never completed", which blocks the client's `_IsFinishPreChallenge()` check and locks all subsequent floors (e.g. floors 2–15 locked)!
- `score_id` / `score_two`:
  - **Forgotten Hall & MoC (`id < 20000`)**: Cycle-based countdowns (`ChallengeCountDown: 20`), NOT score-based. Must be `0`! Injecting score points into MoC causes cycle target evaluation mismatch.
  - **Pure Fiction (`20000 <= id < 30000`)**: Set `score_id = 80000` and `score_two = 80000` (exceeds the 60,000 threshold for 3 stars).
  - **Apocalyptic Shadow (`id >= 30000`)**: Set `score_id = 4000` and `score_two = 4000` (total 8,000 exceeds the 6,600 threshold for 3 stars).

## 2. Active Stage IDs & Group Mapping (v4.5.52 Client)
If a stage ID is missing from `challenge_list`, the client has no data for that floor, evaluates it as 0 stars, and locks all subsequent floors:
- **Forgotten Hall Story**:
  - Jarilo-VI: Group 100 -> Stages `1..=15` (15 stages, 45/45 ⭐)
  - Luofu: Group 900 -> Stages `21..=26` (6 stages, 18/18 ⭐)
  - Groups 101..119 -> Stages `101..110`, `201..210`, ..., `1901..1910`
- **Memory of Chaos (MoC)**:
  - Groups 1001..1008: 10 floors each (`2001..2010`, `2101..2110`, ..., `2701..2710`)
  - Groups 1009..1034: 12 floors each (`2801..2812`, ..., `5301..5312`)
  - **Group 1035 ("Survival of the Fittest" - Active in 4.5.52!)**: 12 floors -> Stages `5501..=5512` (36/36 ⭐)
  - Group 1036: 12 floors -> Stages `5401..=5412`
- **Pure Fiction (PF)**:
  - Groups 2001..2025: 4 stages each (`20011..20014`, ..., `20251..20254`)
  - **Group 2026 (Active in 4.5.52!)**: 4 stages -> Stages `20261..=20264` (12/12 ⭐)
  - Group 2027: 4 stages -> Stages `20271..=20274`
- **Apocalyptic Shadow (AS)**:
  - Groups 3001..3020: 4 stages each (`30011..30014`, ..., `30201..30204`)
  - **Group 3021 (Active in 4.5.52!)**: 4 stages -> Stages `30211..=30214` (12/12 ⭐)
- **Total Valid Stages**: 819 stages across all game modes.

## 3. Packet 1711 Varint Decoding & Statistics Injection (MoC & PF Unlocks)
When selecting an endgame group in the client:
- The client sends `GetChallengeGroupStatisticsCsReq` (`CmdId: 1711`) containing `group_id` at tag 12 (`0x60`).
- **CRITICAL**: Because group IDs exceed 127 (e.g. `1035 -> [0x8B, 0x08]`, `2026 -> [0xEA, 0x0F]`), extracting `payload[1] as u32` truncates the varint. Must decode `group_id` via `prost::encoding::decode_varint`.
- In `GetChallengeGroupStatisticsScRsp` (`CmdId: 1736`):
  - Tag 9: `group_id` (varint)
  - Tag 15: `retcode = 0` (varint)
  - **Pure Fiction (`2000..3000`)**: Inject Tag 8 `challenge_story` (`ChallengeStoryStatistics`) with `score_id = 80000`, `level = 4`, `record_id = 1`. This clears the "No Data" state and enables full 12/12 display.
  - **Memory of Chaos (`1000..2000`)**: Inject Tag 2 `challenge_default` (`ChallengeStatistics`) with `round_count = 20`, `level = 12`, `record_id = 1`.
  - **Apocalyptic Shadow (`>= 3000`)**: Inject Tag 11 `challenge_boss` (`ChallengeBossStatistics`) with `score_id = 8000`, `level = 4`, `record_id = 1`.

## 4. Floor Unlocks & Max Level Mapping
- `ChallengeHistoryMaxLevel`:
  - MoC: `level: 12`, `reward_display_type: 101212`.
  - Pure Fiction: `level: 4`, `reward_display_type: 101404`.
  - Apocalyptic Shadow: `level: 4`, `reward_display_type: 101913`.
  - Jarilo-VI Story: `level: 15`, `reward_display_type: 101015`.
  - Luofu Story: `level: 6`, `reward_display_type: 101021`.

## 5. Required Packet Handlers
- `1711` (`GetChallengeGroupStatisticsCsReq`): Responds with `1736` (`GetChallengeGroupStatisticsScRsp`).
- `8981` (`GetChallengeTierceDataCsReq`): Responds with `8980` containing full `challenge_info_list` (Tag 10) and `retcode: 0` (Tag 12: `0x60, 0x00`).
- `8978` (`GetChallengeTierceControllerCsReq`): Responds with `8971` (`retcode: 0` Tag 7: `0x38, 0x00`).
- `8909` (`GetChallengePeakDataCsReq`): Responds with `8923` (`retcode: 0` Tag 12: `0x60, 0x00`).
- `8924` (`GetCurChallengePeakCsReq`): Responds with `8920` (`retcode: 0` Tag 2: `0x10, 0x00`).
- `1713` (`GetCurChallengeCsReq`): Responds with `1771` (`retcode: 0` Tag 13: `0x68, 0x00`).
- `1738` / `1739` (`TakeChallengeRewardCsReq`): Responds with `1748` / `1704` (`retcode: 0`).
- `2966` (`GetFriendRecommendLineupCsReq`): Responds with `2955` (`retcode: 0` Tag 6: `0x30, 0x00`).
- `2916` (`GetFriendRecommendLineupDetailCsReq`): Responds with `2906` (`retcode: 0` Tag 3: `0x18, 0x00`).
- `2919` (`GetFriendBattleRecordDetailCsReq`): Responds with `2928` (`retcode: 0` Tag 9: `0x48, 0x00`).
- `740` (`GetLineupAvatarDataCsReq`): Responds with `733` (`retcode: 0` Tag 11: `0x58, 0x00`).
- `2625` (`GetActivityScheduleConfigCsReq`): Responds with `2616` injecting `activity_id: 21001..21030` and `panel_id: 21001` to clear Tab 4 padlock, plus `1001..1050`, `2001..2050`, `3001..3050`.

## 6. Challenge Battle Entry Protocol ("Start Challenge" Lineup Flow)
When the player clicks "Start Challenge" on the lineup UI:
1. **Packet 1793 (`StartChallengeCsReq`)**:
   - For standard 1-node and 2-node challenge stages (Forgotten Hall, MoC, Pure Fiction, Apocalyptic Shadow).
   - Tag 12: `challenge_id` (varint).
   - Tag 3: `first_avatars` (repeated varint or packed length-delimited).
   - Tag 6: `second_avatars` (repeated varint or packed length-delimited).
   - Tag 8: `stage_index` (0 for Node 1, 1 for Node 2).
   - Tag 4: `buff_id` (varint).
   - **Response 1758 (`StartChallengeScRsp`)**:
     - Tag 3: `retcode: 0`
     - Tag 2: `lineup_list`
     - Tag 10: `cur_challenge`
     - Tag 14: `scene` (`SceneInfo` built for `MapEntranceID` via `load_challenge_scene`)
2. **Packet 8988 (`StartChallengeTierceCsReq`)**:
   - For 3-node Tierce/Starward challenge stages.
   - Tag 5: `challenge_id` (varint).
   - Tag 1: `stage_index` (0, 1, or 2).
   - Tag 8: `stage_info_list` (submessages containing stage buffs and avatar list).
   - **Response 8974 (`StartChallengeTierceScRsp`)**:
     - Tag 4: `retcode: 0`
     - Tag 7: `scene` (`SceneInfo`)
     - Tag 12: `challenge_tierce_info`
3. **Lineup & Exit Packets**:
   - `8979` (`SetChallengeTierceLineupCsReq`) -> Responds with `8995` (`retcode: 0`).
   - `1788` (`LeaveChallengeCsReq`) -> Responds with `1781` (`retcode: 0`).
   - `8991` (`LeaveChallengeTierceCsReq`) -> Responds with `8982` (`retcode: 0`).
   - `1713` (`GetCurChallengeCsReq`) -> Responds with `1771` (`retcode: 0`).
   - `188` (`GetCurBattleInfoCsReq`) -> Responds with `181` (`retcode: 0`).

## 7. Arena Scene & Battle Flow
- `load_challenge_scene`:
  - Arena `entry_id` from `ChallengeMazeConfig.json` (e.g. `3000101` for Jarilo, `3000301` for Luofu, `3014002` for MoC/Tierce).
  - Canisters/props generated in Group 1.
  - Monster entity spawned: `entity_id: 30001`, `monster_id` (e.g. `3014022`), `event_id` (Stage ID).
  - Player party spawned with active character standing in front of boss.
- Player's `player.battle_config`:
  - `custom_battle_lineup` set to chosen avatar map.
  - `monsters` populated with stage monster waves from `StageConfig.json`.
  - `blessings` populated with stage buff `BattleBuffJson`.
- Attacking the boss entity triggers `battle.rs::create_battle_info`, launching directly into the combat arena with full turn-based mechanics!

