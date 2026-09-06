use super::*;
use proto::*;
use std::collections::{BTreeMap, HashMap};
use std::sync::{LazyLock, Mutex};
use serde::Deserialize;
use prost::Message;
use common::structs::{AvatarJson, BattleType, BattleBuffJson, Monster};

#[derive(Deserialize, Clone, Debug, Default)]
pub struct ChallengeStageData {
    pub entrance: u32,
    pub entrance2: u32,
    pub group1: u32,
    pub group2: u32,
    pub monster1: u32,
    pub monster2: u32,
    pub event1: u32,
    pub event2: u32,
    pub buff: u32,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct ChallengeTierceStageData {
    pub entrance: u32,
    pub group: u32,
    pub monster: u32,
    pub event: u32,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct StageBattleData {
    pub level: u32,
    pub monsters: Vec<Vec<u32>>,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct ChallengeConfigData {
    pub challenges: HashMap<u32, ChallengeStageData>,
    pub tierce: HashMap<u32, ChallengeTierceStageData>,
    pub stages: HashMap<u32, StageBattleData>,
}

pub static CHALLENGE_DATA: LazyLock<ChallengeConfigData> = LazyLock::new(|| {
    let path = "challenge_data.json";
    if let Ok(content) = std::fs::read_to_string(path) {
        serde_json::from_str(&content).unwrap_or_else(|e| {
            tracing::error!("Failed to parse challenge_data.json: {e}");
            ChallengeConfigData::default()
        })
    } else {
        tracing::error!("challenge_data.json not found!");
        ChallengeConfigData::default()
    }
});

pub static TIERCE_LINEUPS: LazyLock<Mutex<HashMap<u32, Vec<Vec<u32>>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));


pub const ALL_CHALLENGE_STAGES: [u32; 819] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    21, 22, 23, 24, 25, 26, 101, 102, 103, 104, 105, 106, 107, 108, 109,
    110, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 301, 302, 303, 304,
    305, 306, 307, 308, 309, 310, 401, 402, 403, 404, 405, 406, 407, 408, 409,
    410, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 601, 602, 603, 604,
    605, 606, 607, 608, 609, 610, 701, 702, 703, 704, 705, 706, 707, 708, 709,
    710, 801, 802, 803, 804, 805, 806, 807, 808, 809, 810, 901, 902, 903, 904,
    905, 906, 907, 908, 909, 910, 1001, 1002, 1003, 1004, 1005, 1006, 1007, 1008, 1009,
    1010, 1101, 1102, 1103, 1104, 1105, 1106, 1107, 1108, 1109, 1110, 1201, 1202, 1203, 1204,
    1205, 1206, 1207, 1208, 1209, 1210, 1301, 1302, 1303, 1304, 1305, 1306, 1307, 1308, 1309,
    1310, 1401, 1402, 1403, 1404, 1405, 1406, 1407, 1408, 1409, 1410, 1501, 1502, 1503, 1504,
    1505, 1506, 1507, 1508, 1509, 1510, 1601, 1602, 1603, 1604, 1605, 1606, 1607, 1608, 1609,
    1610, 1701, 1702, 1703, 1704, 1705, 1706, 1707, 1708, 1709, 1710, 1801, 1802, 1803, 1804,
    1805, 1806, 1807, 1808, 1809, 1810, 1901, 1902, 1903, 1904, 1905, 1906, 1907, 1908, 1909,
    1910, 2001, 2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2101, 2102, 2103, 2104,
    2105, 2106, 2107, 2108, 2109, 2110, 2201, 2202, 2203, 2204, 2205, 2206, 2207, 2208, 2209,
    2210, 2301, 2302, 2303, 2304, 2305, 2306, 2307, 2308, 2309, 2310, 2401, 2402, 2403, 2404,
    2405, 2406, 2407, 2408, 2409, 2410, 2501, 2502, 2503, 2504, 2505, 2506, 2507, 2508, 2509,
    2510, 2601, 2602, 2603, 2604, 2605, 2606, 2607, 2608, 2609, 2610, 2701, 2702, 2703, 2704,
    2705, 2706, 2707, 2708, 2709, 2710, 2801, 2802, 2803, 2804, 2805, 2806, 2807, 2808, 2809,
    2810, 2811, 2812, 2901, 2902, 2903, 2904, 2905, 2906, 2907, 2908, 2909, 2910, 2911, 2912,
    3001, 3002, 3003, 3004, 3005, 3006, 3007, 3008, 3009, 3010, 3011, 3012, 3101, 3102, 3103,
    3104, 3105, 3106, 3107, 3108, 3109, 3110, 3111, 3112, 3201, 3202, 3203, 3204, 3205, 3206,
    3207, 3208, 3209, 3210, 3211, 3212, 3301, 3302, 3303, 3304, 3305, 3306, 3307, 3308, 3309,
    3310, 3311, 3312, 3401, 3402, 3403, 3404, 3405, 3406, 3407, 3408, 3409, 3410, 3411, 3412,
    3501, 3502, 3503, 3504, 3505, 3506, 3507, 3508, 3509, 3510, 3511, 3512, 3601, 3602, 3603,
    3604, 3605, 3606, 3607, 3608, 3609, 3610, 3611, 3612, 3701, 3702, 3703, 3704, 3705, 3706,
    3707, 3708, 3709, 3710, 3711, 3712, 3801, 3802, 3803, 3804, 3805, 3806, 3807, 3808, 3809,
    3810, 3811, 3812, 3901, 3902, 3903, 3904, 3905, 3906, 3907, 3908, 3909, 3910, 3911, 3912,
    4001, 4002, 4003, 4004, 4005, 4006, 4007, 4008, 4009, 4010, 4011, 4012, 4101, 4102, 4103,
    4104, 4105, 4106, 4107, 4108, 4109, 4110, 4111, 4112, 4201, 4202, 4203, 4204, 4205, 4206,
    4207, 4208, 4209, 4210, 4211, 4212, 4301, 4302, 4303, 4304, 4305, 4306, 4307, 4308, 4309,
    4310, 4311, 4312, 4401, 4402, 4403, 4404, 4405, 4406, 4407, 4408, 4409, 4410, 4411, 4412,
    4501, 4502, 4503, 4504, 4505, 4506, 4507, 4508, 4509, 4510, 4511, 4512, 4601, 4602, 4603,
    4604, 4605, 4606, 4607, 4608, 4609, 4610, 4611, 4612, 4701, 4702, 4703, 4704, 4705, 4706,
    4707, 4708, 4709, 4710, 4711, 4712, 4801, 4802, 4803, 4804, 4805, 4806, 4807, 4808, 4809,
    4810, 4811, 4812, 4901, 4902, 4903, 4904, 4905, 4906, 4907, 4908, 4909, 4910, 4911, 4912,
    5001, 5002, 5003, 5004, 5005, 5006, 5007, 5008, 5009, 5010, 5011, 5012, 5101, 5102, 5103,
    5104, 5105, 5106, 5107, 5108, 5109, 5110, 5111, 5112, 5201, 5202, 5203, 5204, 5205, 5206,
    5207, 5208, 5209, 5210, 5211, 5212, 5301, 5302, 5303, 5304, 5305, 5306, 5307, 5308, 5309,
    5310, 5311, 5312, 5401, 5402, 5403, 5404, 5405, 5406, 5407, 5408, 5409, 5410, 5411, 5412,
    5501, 5502, 5503, 5504, 5505, 5506, 5507, 5508, 5509, 5510, 5511, 5512, 20011, 20012, 20013,
    20014, 20021, 20022, 20023, 20024, 20031, 20032, 20033, 20034, 20041, 20042, 20043, 20044, 20051, 20052,
    20053, 20054, 20061, 20062, 20063, 20064, 20071, 20072, 20073, 20074, 20081, 20082, 20083, 20084, 20091,
    20092, 20093, 20094, 20101, 20102, 20103, 20104, 20111, 20112, 20113, 20114, 20121, 20122, 20123, 20124,
    20131, 20132, 20133, 20134, 20141, 20142, 20143, 20144, 20151, 20152, 20153, 20154, 20161, 20162, 20163,
    20164, 20171, 20172, 20173, 20174, 20181, 20182, 20183, 20184, 20191, 20192, 20193, 20194, 20201, 20202,
    20203, 20204, 20211, 20212, 20213, 20214, 20221, 20222, 20223, 20224, 20231, 20232, 20233, 20234, 20241,
    20242, 20243, 20244, 20251, 20252, 20253, 20254, 20261, 20262, 20263, 20264, 20271, 20272, 20273, 20274,
    30011, 30012, 30013, 30014, 30021, 30022, 30023, 30024, 30031, 30032, 30033, 30034, 30041, 30042, 30043,
    30044, 30051, 30052, 30053, 30054, 30061, 30062, 30063, 30064, 30071, 30072, 30073, 30074, 30081, 30082,
    30083, 30084, 30091, 30092, 30093, 30094, 30101, 30102, 30103, 30104, 30111, 30112, 30113, 30114, 30121,
    30122, 30123, 30124, 30131, 30132, 30133, 30134, 30141, 30142, 30143, 30144, 30151, 30152, 30153, 30154,
    30161, 30162, 30163, 30164, 30171, 30172, 30173, 30174, 30181, 30182, 30183, 30184, 30191, 30192, 30193,
    30194, 30201, 30202, 30203, 30204, 30211, 30212, 30213, 30214,
];

fn get_challenge_groups() -> Vec<u32> {
    let mut groups = Vec::new();
    for g in 1..=120 {
        groups.push(g);
    }
    groups.push(900);
    for g in 1001..=1036 {
        groups.push(g);
    }
    for g in 2001..=2027 {
        groups.push(g);
    }
    for g in 3001..=3021 {
        groups.push(g);
    }
    for base in [1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000, 20000, 21000, 30000, 40000, 50000, 60000] {
        for offset in 1..=50 {
            groups.push(base + offset);
        }
    }
    groups.sort();
    groups.dedup();
    groups
}

fn get_challenge_list() -> Vec<Challenge> {
    let mut list = Vec::new();

    let make_challenge = |id: u32| Challenge {
        challenge_id: id,
        star: 7, // 7 = (1 << 0) | (1 << 1) | (1 << 2) -> Bitmask for all 3 stars!
        taken_reward: 7, // 7 = (1 << 0) | (1 << 1) | (1 << 2) -> All 3 star rewards claimed
        record_id: 1, // Non-zero record confirms stage cleared
        hgpkmhfpmbj: false, // NOT first open (already cleared, allows unlocking next stages)
        score_two: if (20000..30000).contains(&id) {
            80000
        } else if id >= 30000 {
            4000
        } else {
            0
        },
        score_id: if (20000..30000).contains(&id) {
            80000
        } else if id >= 30000 {
            4000
        } else {
            0
        },
        ..Default::default()
    };

    // All 790 Stages across all modes (Jarilo 1-15, Luofu 1-6, MoC 1-12, Pure Fiction 1-4, Apocalyptic Shadow 1-4)
    for &id in &ALL_CHALLENGE_STAGES {
        list.push(make_challenge(id));
    }

    list
}

pub async fn on_get_challenge_cs_req(
    _session: &mut PlayerSession,
    _req: &GetChallengeCsReq,
    res: &mut GetChallengeScRsp,
) {
    res.retcode = 0;
    res.challenge_group_list = get_challenge_groups()
        .into_iter()
        .map(|group_id| ChallengeGroup {
            group_id,
            taken_stars_count_reward: 0,
        })
        .collect();
    res.kkiafpfklge = Vec::new();
    res.challenge_list = get_challenge_list();

    let mut max_levels = Vec::new();

    // Map max_level for every stage (matches himeko-nova-sr logic)
    for &id in &ALL_CHALLENGE_STAGES {
        let (level, r_type) = if id >= 30000 {
            (4, 101913) // Apocalyptic Shadow Tierce
        } else if id >= 20000 {
            (4, 101404) // Pure Fiction
        } else {
            (12, 101212) // MoC 12 floors
        };

        max_levels.push(ChallengeHistoryMaxLevel {
            level,
            hnhcfjjnjce: false,
            reward_display_type: r_type,
        });
    }

    // Include max level entries for Forgotten Hall story groups (Jarilo-VI 15, Luofu 6)
    max_levels.push(ChallengeHistoryMaxLevel {
        level: 15,
        hnhcfjjnjce: false,
        reward_display_type: 101015,
    });
    max_levels.push(ChallengeHistoryMaxLevel {
        level: 6,
        hnhcfjjnjce: false,
        reward_display_type: 101021,
    });

    res.max_level_list = max_levels;
}

pub async fn on_get_cur_challenge_cs_req(
    _session: &mut PlayerSession,
    _req: &GetCurChallengeCsReq,
    res: &mut GetCurChallengeScRsp,
) {
    res.retcode = 0;
    res.cur_challenge = None;
    res.lineup_list = Vec::new();
}

pub async fn on_get_activity_schedule_config_cs_req(
    _session: &mut PlayerSession,
    _req: &GetActivityScheduleConfigCsReq,
    res: &mut GetActivityScheduleConfigScRsp,
) {
    res.retcode = 0;

    let mut schedule_list = Vec::new();

    // Standard activity panels 1..=100
    for id in 1..=100 {
        schedule_list.push(ActivityScheduleData {
            activity_id: id,
            panel_id: id,
            begin_time: 0,
            end_time: 1924992000,
        });
    }

    // Apocalyptic Shadow Activity (21001..=21030, panel 21001, and sub-modules 2100101..=2100801)
    for act_id in 21001..=21030 {
        schedule_list.push(ActivityScheduleData {
            activity_id: act_id,
            panel_id: 21001,
            begin_time: 0,
            end_time: 1924992000,
        });
    }
    for i in 1..=8 {
        schedule_list.push(ActivityScheduleData {
            activity_id: 2100000 + i * 100 + 1, // 2100101, 2100201, ...
            panel_id: 21001,
            begin_time: 0,
            end_time: 1924992000,
        });
    }

    // Challenge and event activity ranges
    for base in [1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000, 20000, 30000, 40000, 50000, 60000] {
        for offset in 1..=50 {
            let act_id = base + offset;
            schedule_list.push(ActivityScheduleData {
                activity_id: act_id,
                panel_id: act_id,
                begin_time: 0,
                end_time: 1924992000,
            });
        }
    }

    res.schedule_data = schedule_list;
}

#[derive(Default, Debug)]
pub struct DecodedChallengeStart {
    pub challenge_id: u32,
    pub stage_index: u32,
    pub is_single_stage: bool,
    pub first_avatars: Vec<u32>,
    pub second_avatars: Vec<u32>,
    pub third_avatars: Vec<u32>,
    pub stage_info_list: Vec<Vec<u32>>,
    pub buff_id: u32,
}

fn skip_wire_field(wire_type: u32, buf: &mut &[u8]) {
    match wire_type {
        0 => {
            let _ = prost::encoding::decode_varint(buf);
        }
        1 => {
            if buf.len() >= 8 {
                *buf = &buf[8..];
            } else {
                *buf = &[];
            }
        }
        2 => {
            if let Ok(len) = prost::encoding::decode_varint(buf) {
                let len = len as usize;
                if buf.len() >= len {
                    *buf = &buf[len..];
                } else {
                    *buf = &[];
                }
            } else {
                *buf = &[];
            }
        }
        5 => {
            if buf.len() >= 4 {
                *buf = &buf[4..];
            } else {
                *buf = &[];
            }
        }
        _ => {
            *buf = &[];
        }
    }
}

pub fn decode_start_challenge_tierce_req(payload: &[u8]) -> DecodedChallengeStart {
    let mut req = DecodedChallengeStart::default();
    let mut buf = payload;
    let mut stages_avatars: Vec<(u32, Vec<u32>)> = Vec::new();

    while !buf.is_empty() {
        if let Ok(tag) = prost::encoding::decode_varint(&mut buf) {
            let field_number = (tag >> 3) as u32;
            let wire_type = (tag & 0x7) as u32;
            match (field_number, wire_type) {
                (1, 0) => {
                    if let Ok(v) = prost::encoding::decode_varint(&mut buf) {
                        req.stage_index = v as u32;
                    }
                }
                (3, 0) => {
                    if let Ok(v) = prost::encoding::decode_varint(&mut buf) {
                        req.challenge_id = v as u32;
                    }
                }
                (13, 0) => {
                    if let Ok(v) = prost::encoding::decode_varint(&mut buf) {
                        req.is_single_stage = v != 0;
                    }
                }
                (14, 2) => {
                    if let Ok(len) = prost::encoding::decode_varint(&mut buf) {
                        let len = len as usize;
                        if len <= buf.len() {
                            let mut sub = &buf[..len];
                            buf = &buf[len..];
                            let mut stage_buff = 0u32;
                            let mut stage_lineup = Vec::new();
                            while !sub.is_empty() {
                                if let Ok(sub_tag) = prost::encoding::decode_varint(&mut sub) {
                                    let sub_fn = (sub_tag >> 3) as u32;
                                    let sub_wt = (sub_tag & 0x7) as u32;
                                    match (sub_fn, sub_wt) {
                                        (5, 0) => {
                                            if let Ok(v) = prost::encoding::decode_varint(&mut sub) {
                                                stage_buff = v as u32;
                                            }
                                        }
                                        (9, 2) => {
                                            if let Ok(alen) = prost::encoding::decode_varint(&mut sub) {
                                                let alen = alen as usize;
                                                if alen <= sub.len() {
                                                    let mut abuf = &sub[..alen];
                                                    sub = &sub[alen..];
                                                    let mut aid = 0u32;
                                                    while !abuf.is_empty() {
                                                        if let Ok(atag) = prost::encoding::decode_varint(&mut abuf) {
                                                            let afn = (atag >> 3) as u32;
                                                            let awt = (atag & 0x7) as u32;
                                                            if afn == 9 && awt == 0 {
                                                                if let Ok(v) = prost::encoding::decode_varint(&mut abuf) {
                                                                    aid = v as u32;
                                                                }
                                                            } else {
                                                                skip_wire_field(awt, &mut abuf);
                                                            }
                                                        } else {
                                                            break;
                                                        }
                                                    }
                                                    if aid != 0 {
                                                        stage_lineup.push(aid);
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            skip_wire_field(sub_wt, &mut sub);
                                        }
                                    }
                                } else {
                                    break;
                                }
                            }
                            stages_avatars.push((stage_buff, stage_lineup));
                        }
                    }
                }
                _ => {
                    skip_wire_field(wire_type, &mut buf);
                }
            }
        } else {
            break;
        }
    }

    for (_, lineup) in &stages_avatars {
        req.stage_info_list.push(lineup.clone());
    }

    if let Some((buff, lineup)) = stages_avatars.get(0) {
        req.first_avatars = lineup.clone();
        if req.stage_index == 0 && *buff != 0 { req.buff_id = *buff; }
    }
    if let Some((buff, lineup)) = stages_avatars.get(1) {
        req.second_avatars = lineup.clone();
        if req.stage_index == 1 && *buff != 0 { req.buff_id = *buff; }
    }
    if let Some((buff, lineup)) = stages_avatars.get(2) {
        req.third_avatars = lineup.clone();
        if req.stage_index == 2 && *buff != 0 { req.buff_id = *buff; }
    }

    req
}

pub fn decode_set_challenge_tierce_lineup_req(payload: &[u8]) -> (u32, Vec<Vec<u32>>) {
    let mut challenge_id = 0u32;
    let mut stages = Vec::new();
    let mut buf = payload;

    while !buf.is_empty() {
        if let Ok(tag) = prost::encoding::decode_varint(&mut buf) {
            let field_number = (tag >> 3) as u32;
            let wire_type = (tag & 0x7) as u32;
            match (field_number, wire_type) {
                (7, 0) => {
                    if let Ok(v) = prost::encoding::decode_varint(&mut buf) {
                        challenge_id = v as u32;
                    }
                }
                (12, 2) => {
                    if let Ok(len) = prost::encoding::decode_varint(&mut buf) {
                        let len = len as usize;
                        if len <= buf.len() {
                            let mut sub = &buf[..len];
                            buf = &buf[len..];
                            let mut stage_lineup = Vec::new();
                            while !sub.is_empty() {
                                if let Ok(sub_tag) = prost::encoding::decode_varint(&mut sub) {
                                    let sub_fn = (sub_tag >> 3) as u32;
                                    let sub_wt = (sub_tag & 0x7) as u32;
                                    if sub_fn == 9 && sub_wt == 2 {
                                        if let Ok(alen) = prost::encoding::decode_varint(&mut sub) {
                                            let alen = alen as usize;
                                            if alen <= sub.len() {
                                                let mut abuf = &sub[..alen];
                                                sub = &sub[alen..];
                                                let mut aid = 0u32;
                                                while !abuf.is_empty() {
                                                    if let Ok(atag) = prost::encoding::decode_varint(&mut abuf) {
                                                        let afn = (atag >> 3) as u32;
                                                        let awt = (atag & 0x7) as u32;
                                                        if afn == 9 && awt == 0 {
                                                            if let Ok(v) = prost::encoding::decode_varint(&mut abuf) {
                                                                aid = v as u32;
                                                            }
                                                        } else {
                                                            skip_wire_field(awt, &mut abuf);
                                                        }
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                if aid != 0 {
                                                    stage_lineup.push(aid);
                                                }
                                            }
                                        }
                                    } else {
                                        skip_wire_field(sub_wt, &mut sub);
                                    }
                                } else {
                                    break;
                                }
                            }
                            stages.push(stage_lineup);
                        }
                    }
                }
                _ => {
                    skip_wire_field(wire_type, &mut buf);
                }
            }
        } else {
            break;
        }
    }

    (challenge_id, stages)
}

pub fn decode_start_challenge_req(payload: &[u8]) -> DecodedChallengeStart {
    let mut req = DecodedChallengeStart::default();
    let mut buf = payload;

    while !buf.is_empty() {
        if let Ok(tag) = prost::encoding::decode_varint(&mut buf) {
            let field_number = (tag >> 3) as u32;
            let wire_type = (tag & 0x7) as u32;
            match (field_number, wire_type) {
                (12, 0) => {
                    if let Ok(v) = prost::encoding::decode_varint(&mut buf) {
                        req.challenge_id = v as u32;
                    }
                }
                (3, 0) => {
                    if let Ok(v) = prost::encoding::decode_varint(&mut buf) {
                        req.first_avatars.push(v as u32);
                    }
                }
                (3, 2) => {
                    if let Ok(len) = prost::encoding::decode_varint(&mut buf) {
                        let len = len as usize;
                        if len <= buf.len() {
                            let mut sub = &buf[..len];
                            buf = &buf[len..];
                            while !sub.is_empty() {
                                if let Ok(v) = prost::encoding::decode_varint(&mut sub) {
                                    req.first_avatars.push(v as u32);
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
                (4, 0) => {
                    if let Ok(v) = prost::encoding::decode_varint(&mut buf) {
                        req.second_avatars.push(v as u32);
                    }
                }
                (4, 2) => {
                    if let Ok(len) = prost::encoding::decode_varint(&mut buf) {
                        let len = len as usize;
                        if len <= buf.len() {
                            let mut sub = &buf[..len];
                            buf = &buf[len..];
                            while !sub.is_empty() {
                                if let Ok(v) = prost::encoding::decode_varint(&mut sub) {
                                    req.second_avatars.push(v as u32);
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
                (6, 2) => {
                    if let Ok(len) = prost::encoding::decode_varint(&mut buf) {
                        let len = len as usize;
                        if len <= buf.len() {
                            let mut sub = &buf[..len];
                            buf = &buf[len..];
                            while !sub.is_empty() {
                                if let Ok(stag) = prost::encoding::decode_varint(&mut sub) {
                                    let sfn = (stag >> 3) as u32;
                                    let swt = (stag & 0x7) as u32;
                                    if sfn == 9 && swt == 0 {
                                        if let Ok(v) = prost::encoding::decode_varint(&mut sub) {
                                            req.first_avatars.push(v as u32);
                                        }
                                    } else {
                                        skip_wire_field(swt, &mut sub);
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
                (10, 2) => {
                    if let Ok(len) = prost::encoding::decode_varint(&mut buf) {
                        let len = len as usize;
                        if len <= buf.len() {
                            let mut sub = &buf[..len];
                            buf = &buf[len..];
                            while !sub.is_empty() {
                                if let Ok(stag) = prost::encoding::decode_varint(&mut sub) {
                                    let sfn = (stag >> 3) as u32;
                                    let swt = (stag & 0x7) as u32;
                                    if sfn == 9 && swt == 0 {
                                        if let Ok(v) = prost::encoding::decode_varint(&mut sub) {
                                            req.second_avatars.push(v as u32);
                                        }
                                    } else {
                                        skip_wire_field(swt, &mut sub);
                                    }
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
                _ => {
                    skip_wire_field(wire_type, &mut buf);
                }
            }
        } else {
            break;
        }
    }

    req
}

pub async fn send_scene_entity_move_sc_notify(
    session: &PlayerSession,
    entity_id: u32,
    entry_id: u32,
    motion: &MotionInfo,
) -> Result<()> {
    let mut body = Vec::new();
    body.push(0x20); // tag 4
    prost::encoding::encode_varint(entity_id as u64, &mut body);

    let mut motion_buf = Vec::new();
    motion.encode(&mut motion_buf)?;
    body.push(0x42); // tag 8
    prost::encoding::encode_varint(motion_buf.len() as u64, &mut body);
    body.extend_from_slice(&motion_buf);

    body.push(0x58); // tag 11
    prost::encoding::encode_varint(entry_id as u64, &mut body);

    session.send_raw(NetPacket {
        cmd_type: 1471,
        head: Vec::new(),
        body,
    }).await?;

    Ok(())
}

pub async fn handle_start_challenge_tierce(session: &mut PlayerSession, payload: &[u8]) -> Result<()> {
    let req = decode_start_challenge_tierce_req(payload);
    tracing::info!("handle_start_challenge_tierce: challenge_id={}, stage_index={}, is_single_stage={}",
        req.challenge_id, req.stage_index, req.is_single_stage);

    if !req.stage_info_list.is_empty() {
        if let Ok(mut map) = TIERCE_LINEUPS.lock() {
            map.insert(req.challenge_id, req.stage_info_list.clone());
        }
    }

    let mut chosen_avatars = match req.stage_index {
        0 => req.first_avatars.clone(),
        1 => req.second_avatars.clone(),
        2 => req.third_avatars.clone(),
        _ => Vec::new(),
    };

    if chosen_avatars.is_empty() {
        if let Ok(map) = TIERCE_LINEUPS.lock() {
            if let Some(list) = map.get(&req.challenge_id) {
                if let Some(avatars) = list.get(req.stage_index as usize) {
                    chosen_avatars = avatars.clone();
                }
            }
        }
    }

    if chosen_avatars.is_empty() {
        if let Some(json) = session.json_data.get() {
            chosen_avatars = json.lineups.values().copied().collect();
        }
    }

    let (entrance, group, monster, event, buff) = if req.stage_index == 2 {
        if let Some(t) = CHALLENGE_DATA.tierce.get(&req.challenge_id) {
            (t.entrance, t.group, t.monster, t.event, req.buff_id)
        } else if let Some(c) = CHALLENGE_DATA.challenges.get(&req.challenge_id) {
            (c.entrance2, c.group2, c.monster2, c.event2, if req.buff_id != 0 { req.buff_id } else { c.buff })
        } else {
            (3014002, 11, 5014010, 30123123, 0)
        }
    } else if req.stage_index == 1 {
        let base_id = if req.challenge_id > 1 && !CHALLENGE_DATA.challenges.contains_key(&req.challenge_id) {
            req.challenge_id - 1
        } else {
            req.challenge_id
        };
        if let Some(c) = CHALLENGE_DATA.challenges.get(&base_id) {
            (c.entrance2, c.group2, c.monster2, c.event2, if req.buff_id != 0 { req.buff_id } else { c.buff })
        } else {
            (3000301, 8, 3003015, 420494, 0)
        }
    } else {
        let base_id = if req.challenge_id > 1 && !CHALLENGE_DATA.challenges.contains_key(&req.challenge_id) {
            req.challenge_id - 1
        } else {
            req.challenge_id
        };
        if let Some(c) = CHALLENGE_DATA.challenges.get(&base_id) {
            (c.entrance, c.group1, c.monster1, c.event1, if req.buff_id != 0 { req.buff_id } else { c.buff })
        } else {
            (3000101, 2, 8013010, 30001011, 0)
        }
    };

    if let Some(json) = session.json_data.get_mut() {
        let mut custom_lineup = BTreeMap::new();
        for (i, &aid) in chosen_avatars.iter().enumerate() {
            custom_lineup.insert(i as u32, aid);
        }
        json.battle_config.custom_battle_lineup = Some(custom_lineup);
        json.battle_config.stage_id = event;

        let is_pf = (20000..30000).contains(&req.challenge_id);
        let is_as = req.challenge_id >= 30000;
        json.battle_config.cycle_count = if is_pf { 4 } else { 30 };
        json.battle_config.battle_type = if is_pf { BattleType::PF } else if is_as { BattleType::AS } else { BattleType::Default };

        if let Some(sb) = CHALLENGE_DATA.stages.get(&event) {
            json.battle_config.monsters = sb.monsters.iter().map(|wave| {
                wave.iter().map(|&mid| Monster {
                    level: sb.level,
                    monster_id: mid,
                    max_hp: 0,
                }).collect()
            }).collect();
        }

        if buff != 0 {
            json.battle_config.blessings = vec![BattleBuffJson {
                id: buff,
                level: 1,
                dynamic_key: None,
                dynamic_values: Vec::new(),
            }];
        }

        let _ = json.save_persistent().await;
    }

    let (scene_info, motion) = load_challenge_scene(session, entrance, group, monster, event, &chosen_avatars).await?;

    let mut custom_map = BTreeMap::new();
    for (i, &aid) in chosen_avatars.iter().enumerate() {
        custom_map.insert(i as u32, aid);
    }
    let lineup_info = AvatarJson::to_lineup_info(&custom_map);

    let mut tierce_info = Vec::new();
    if req.is_single_stage {
        tierce_info.extend_from_slice(&[0x08, 0x01]);
    }
    let mut lineup_buf = Vec::new();
    lineup_info.encode(&mut lineup_buf)?;
    tierce_info.push(0x3A); // tag 7
    prost::encoding::encode_varint(lineup_buf.len() as u64, &mut tierce_info);
    tierce_info.extend_from_slice(&lineup_buf);

    tierce_info.push(0x60); // tag 12
    prost::encoding::encode_varint(req.stage_index as u64, &mut tierce_info);

    tierce_info.push(0x78); // tag 15
    prost::encoding::encode_varint(req.challenge_id as u64, &mut tierce_info);

    let mut body = Vec::new();
    // tag 4: retcode = 0
    body.extend_from_slice(&[0x20, 0x00]);

    // tag 7: scene
    let mut scene_buf = Vec::new();
    scene_info.encode(&mut scene_buf)?;
    body.push(0x3A);
    prost::encoding::encode_varint(scene_buf.len() as u64, &mut body);
    body.extend_from_slice(&scene_buf);

    // tag 12: challenge_tierce_info
    body.push(0x62);
    prost::encoding::encode_varint(tierce_info.len() as u64, &mut body);
    body.extend_from_slice(&tierce_info);

    session.send_raw(NetPacket {
        cmd_type: 8974,
        head: Vec::new(),
        body,
    }).await?;

    for (i, _) in chosen_avatars.iter().enumerate() {
        let _ = send_scene_entity_move_sc_notify(session, (i as u32) + 1, entrance, &motion).await;
    }

    Ok(())
}

pub async fn handle_start_challenge(session: &mut PlayerSession, payload: &[u8]) -> Result<()> {
    let req = decode_start_challenge_req(payload);
    tracing::info!("handle_start_challenge: challenge_id={}", req.challenge_id);

    let chosen_avatars = if !req.first_avatars.is_empty() {
        req.first_avatars.clone()
    } else if let Some(json) = session.json_data.get() {
        json.lineups.values().copied().collect()
    } else {
        vec![1304, 1313, 1406, 1004]
    };

    let (entrance, group, monster, event, buff) = if let Some(c) = CHALLENGE_DATA.challenges.get(&req.challenge_id) {
        (c.entrance, c.group1, c.monster1, c.event1, if req.buff_id != 0 { req.buff_id } else { c.buff })
    } else {
        (3000101, 2, 8013010, 30001011, 0)
    };

    if let Some(json) = session.json_data.get_mut() {
        let mut custom_lineup = BTreeMap::new();
        for (i, &aid) in chosen_avatars.iter().enumerate() {
            custom_lineup.insert(i as u32, aid);
        }
        json.battle_config.custom_battle_lineup = Some(custom_lineup);
        json.battle_config.stage_id = event;

        let is_pf = (20000..30000).contains(&req.challenge_id);
        let is_as = req.challenge_id >= 30000;
        json.battle_config.cycle_count = if is_pf { 4 } else { 30 };
        json.battle_config.battle_type = if is_pf { BattleType::PF } else if is_as { BattleType::AS } else { BattleType::Default };

        if let Some(sb) = CHALLENGE_DATA.stages.get(&event) {
            json.battle_config.monsters = sb.monsters.iter().map(|wave| {
                wave.iter().map(|&mid| Monster {
                    level: sb.level,
                    monster_id: mid,
                    max_hp: 0,
                }).collect()
            }).collect();
        }

        if buff != 0 {
            json.battle_config.blessings = vec![BattleBuffJson {
                id: buff,
                level: 1,
                dynamic_key: None,
                dynamic_values: Vec::new(),
            }];
        }

        let _ = json.save_persistent().await;
    }

    let (scene_info, motion) = load_challenge_scene(session, entrance, group, monster, event, &chosen_avatars).await?;

    let mut custom_map = BTreeMap::new();
    for (i, &aid) in chosen_avatars.iter().enumerate() {
        custom_map.insert(i as u32, aid);
    }
    let lineup_info = AvatarJson::to_lineup_info(&custom_map);

    let cur_challenge = CurChallenge {
        challenge_id: req.challenge_id,
        status: 1, // CHALLENGE_DOING
        round_count: 0,
        score_id: if (20000..30000).contains(&req.challenge_id) { 40000 } else { 0 },
        score_two: 0,
        extra_lineup_type: 1,
        ..Default::default()
    };

    let mut body = Vec::new();
    // tag 3: retcode = 0
    body.extend_from_slice(&[0x18, 0x00]);

    // tag 2: lineup_list
    let mut lineup_buf = Vec::new();
    lineup_info.encode(&mut lineup_buf)?;
    body.push(0x12);
    prost::encoding::encode_varint(lineup_buf.len() as u64, &mut body);
    body.extend_from_slice(&lineup_buf);

    // tag 10: cur_challenge
    let mut chal_buf = Vec::new();
    cur_challenge.encode(&mut chal_buf)?;
    body.push(0x52);
    prost::encoding::encode_varint(chal_buf.len() as u64, &mut body);
    body.extend_from_slice(&chal_buf);

    // tag 14: scene
    let mut scene_buf = Vec::new();
    scene_info.encode(&mut scene_buf)?;
    body.push(0x72);
    prost::encoding::encode_varint(scene_buf.len() as u64, &mut body);
    body.extend_from_slice(&scene_buf);

    session.send_raw(NetPacket {
        cmd_type: 1758,
        head: Vec::new(),
        body,
    }).await?;

    for (i, _) in chosen_avatars.iter().enumerate() {
        let _ = send_scene_entity_move_sc_notify(session, (i as u32) + 1, entrance, &motion).await;
    }

    Ok(())
}

pub async fn handle_set_challenge_tierce_lineup(session: &PlayerSession, payload: &[u8]) -> Result<()> {
    let (challenge_id, stages) = decode_set_challenge_tierce_lineup_req(payload);
    if !stages.is_empty() {
        if let Ok(mut map) = TIERCE_LINEUPS.lock() {
            map.insert(challenge_id, stages);
        }
    }
    session.send_raw(NetPacket {
        cmd_type: 8995,
        head: Vec::new(),
        body: vec![0x60, 0x00], // tag 12: retcode = 0
    }).await?;
    Ok(())
}

pub async fn handle_leave_challenge(session: &mut PlayerSession) -> Result<()> {
    if let Some(json) = session.json_data.get_mut() {
        json.battle_config.custom_battle_lineup = None;
        let _ = json.save_persistent().await;
    }
    session.send_raw(NetPacket {
        cmd_type: 1781,
        head: Vec::new(),
        body: vec![0x40, 0x00], // tag 8: retcode = 0
    }).await?;
    Ok(())
}

pub async fn handle_leave_challenge_tierce(session: &mut PlayerSession) -> Result<()> {
    if let Some(json) = session.json_data.get_mut() {
        json.battle_config.custom_battle_lineup = None;
        let _ = json.save_persistent().await;
    }
    session.send_raw(NetPacket {
        cmd_type: 8982,
        head: Vec::new(),
        body: vec![0x68, 0x00], // tag 13: retcode = 0
    }).await?;
    Ok(())
}

pub fn build_get_challenge_tierce_data_sc_rsp() -> Vec<u8> {
    let tierce_stages: [(u32, &[u32], u32); 7] = [
        (5213, &[601, 602, 603, 600], 4000),
        (5313, &[601, 602, 603, 600], 4000),
        (30185, &[5001, 5002, 5003, 5000], 4000),
        (30195, &[5001, 5002, 5003, 5000], 4000),
        (30205, &[5001, 5002, 5003, 5000], 4000),
        (20245, &[4001, 4002, 4003, 4000], 40000),
        (20255, &[4001, 4002, 4003, 4000], 40000),
    ];

    let mut rsp = Vec::new();
    // Tag 12: retcode = 0
    rsp.extend_from_slice(&[0x60, 0x00]);

    for (challenge_id, targets, score) in tierce_stages {
        let mut tierce_data = Vec::new();

        // Tag 3: challenge_id
        tierce_data.push(0x18);
        prost::encoding::encode_varint(challenge_id as u64, &mut tierce_data);

        // Tag 6: is_passed = true
        tierce_data.extend_from_slice(&[0x30, 0x01]);

        // Tag 7: finished_target_list (packed varint)
        let mut targets_buf = Vec::new();
        for &t in targets {
            prost::encoding::encode_varint(t as u64, &mut targets_buf);
        }
        tierce_data.push(0x3A);
        prost::encoding::encode_varint(targets_buf.len() as u64, &mut tierce_data);
        tierce_data.extend_from_slice(&targets_buf);

        // Tag 4: result_list (ChallengeTierceStageData: 3 stages: 0, 1, 2)
        for stage_idx in 0..3u32 {
            let mut result = Vec::new();
            // Tag 3: score_id
            result.push(0x18);
            prost::encoding::encode_varint(score as u64, &mut result);
            // Tag 9: stage_index
            result.push(0x48);
            prost::encoding::encode_varint(stage_idx as u64, &mut result);
            // Tag 10: end_status = 1 (BATTLE_END_WIN)
            result.extend_from_slice(&[0x50, 0x01]);

            tierce_data.push(0x22); // Tag 4 (len-delimited)
            prost::encoding::encode_varint(result.len() as u64, &mut tierce_data);
            tierce_data.extend_from_slice(&result);
        }

        // Tag 8: stage_info_list (ChallengeTierceStageInfo: 3 stages: 0, 1, 2)
        for stage_idx in 0..3u32 {
            let mut sinfo = Vec::new();
            // Tag 14: stage_index
            sinfo.push(0x70);
            prost::encoding::encode_varint(stage_idx as u64, &mut sinfo);

            tierce_data.push(0x42); // Tag 8 (len-delimited)
            prost::encoding::encode_varint(sinfo.len() as u64, &mut tierce_data);
            tierce_data.extend_from_slice(&sinfo);
        }

        // Tag 10: challenge_info_list (repeated in GetChallengeTierceDataScRsp)
        rsp.push(0x52); // Tag 10 (len-delimited)
        prost::encoding::encode_varint(tierce_data.len() as u64, &mut rsp);
        rsp.extend_from_slice(&tierce_data);
    }

    rsp
}

pub async fn handle_get_cur_challenge(session: &PlayerSession) -> Result<()> {
    session.send_raw(NetPacket {
        cmd_type: 1771,
        head: Vec::new(),
        body: vec![0x68, 0x00], // tag 13: retcode = 0
    }).await?;
    Ok(())
}

pub async fn handle_take_challenge_reward(session: &PlayerSession, payload: &[u8]) -> Result<()> {
    let mut group_id = 0u32;
    let mut buf = payload;
    while !buf.is_empty() {
        if let Ok(tag) = prost::encoding::decode_varint(&mut buf) {
            let fn_num = tag >> 3;
            let wt = tag & 0x7;
            if fn_num == 4 && wt == 0 {
                if let Ok(v) = prost::encoding::decode_varint(&mut buf) {
                    group_id = v as u32;
                }
                break;
            } else {
                skip_wire_field(wt as u32, &mut buf);
            }
        } else {
            break;
        }
    }

    let mut body = Vec::new();
    // tag 2: group_id
    body.push(0x10);
    prost::encoding::encode_varint(group_id as u64, &mut body);
    // tag 6: retcode = 0
    body.extend_from_slice(&[0x30, 0x00]);

    session.send_raw(NetPacket {
        cmd_type: 1704,
        head: Vec::new(),
        body,
    }).await?;
    Ok(())
}

