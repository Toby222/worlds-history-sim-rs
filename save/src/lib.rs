use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Save {
    #[serde(rename = "World")]
    world: World,
}

#[derive(Serialize, Deserialize)]
pub struct World {
    #[serde(rename = "EventsTriggered")]
    events_triggered: String,

    #[serde(rename = "EventsEvaluated")]
    events_evaluated: String,

    #[serde(rename = "ModPaths")]
    mod_paths: ExistingDiscoveryIds,

    #[serde(rename = "LayerSettings")]
    layer_settings: String,

    #[serde(rename = "EventsToHappen")]
    events_to_happen: EventsToHappen,

    #[serde(rename = "TerrainCellAlterationList")]
    terrain_cell_alteration_list: TerrainCellAlterationList,

    #[serde(rename = "CulturalPreferenceInfoList")]
    cultural_preference_info_list: CulturalPreferenceInfoList,

    #[serde(rename = "CulturalActivityInfoList")]
    cultural_activity_info_list: CulturalActivityInfoList,

    #[serde(rename = "CulturalSkillInfoList")]
    cultural_skill_info_list: CulturalSkillInfoList,

    #[serde(rename = "CulturalKnowledgeInfoList")]
    cultural_knowledge_info_list: CulturalKnowledgeInfoList,

    #[serde(rename = "ExistingDiscoveryIds")]
    existing_discovery_ids: ExistingDiscoveryIds,

    #[serde(rename = "CellGroups")]
    cell_groups: CellGroups,

    #[serde(rename = "MemorableAgents")]
    memorable_agents: String,

    #[serde(rename = "FactionInfos")]
    faction_infos: FactionInfos,

    #[serde(rename = "PolityInfos")]
    polity_infos: PolityInfos,

    #[serde(rename = "RegionInfos")]
    region_infos: RegionInfos,

    #[serde(rename = "Languages")]
    languages: Languages,

    #[serde(rename = "EventMessageIds")]
    event_message_ids: EventMessageIdsClass,

    #[serde(rename = "_xmlns:xsd")]
    xmlns_xsd: String,

    #[serde(rename = "_xmlns:xsi")]
    xmlns_xsi: String,

    #[serde(rename = "_Width")]
    width: String,

    #[serde(rename = "_Height")]
    height: String,

    #[serde(rename = "_Seed")]
    seed: String,

    #[serde(rename = "_CurrentDate")]
    current_date: String,

    #[serde(rename = "_MaxTimeToSkip")]
    max_time_to_skip: String,

    #[serde(rename = "_CellGroupCount")]
    cell_group_count: String,

    #[serde(rename = "_MemorableAgentCount")]
    memorable_agent_count: String,

    #[serde(rename = "_FactionCount")]
    faction_count: String,

    #[serde(rename = "_PolityCount")]
    polity_count: String,

    #[serde(rename = "_RegionCount")]
    region_count: String,

    #[serde(rename = "_LanguageCount")]
    language_count: String,

    #[serde(rename = "_TerrainCellAlterationListCount")]
    terrain_cell_alteration_list_count: String,

    #[serde(rename = "_AltitudeScale")]
    altitude_scale: String,

    #[serde(rename = "_SeaLevelOffset")]
    sea_level_offset: String,

    #[serde(rename = "_RiverStrength")]
    river_strength: String,

    #[serde(rename = "_RainfallOffset")]
    rainfall_offset: String,

    #[serde(rename = "_TemperatureOffset")]
    temperature_offset: String,

    #[serde(rename = "_SerializedEventCount")]
    serialized_event_count: String,
}

#[derive(Serialize, Deserialize)]
pub struct CellGroups {
    #[serde(rename = "CellGroup")]
    cell_group: Vec<CellGroup>,
}

#[derive(Serialize, Deserialize)]
pub struct CellGroup {
    #[serde(rename = "SeaMigrationRoute")]
    sea_migration_route: Option<SeaMigrationRoute>,

    #[serde(rename = "Flags")]
    flags: Flags,

    #[serde(rename = "Culture")]
    culture: CellGroupCulture,

    #[serde(rename = "Properties")]
    properties: PropertiesUnion,

    #[serde(rename = "FactionCoreIds")]
    faction_core_ids: String,

    #[serde(rename = "PolityProminences")]
    polity_prominences: PolityProminencesUnion,

    #[serde(rename = "LastPopulationMigration")]
    last_population_migration: LastPopulationMigration,

    #[serde(rename = "_Id")]
    id: String,

    #[serde(rename = "_MT")]
    mt: String,

    #[serde(rename = "_PEP")]
    pep: String,

    #[serde(rename = "_EP")]
    ep: String,

    #[serde(rename = "_P")]
    p: String,

    #[serde(rename = "_LUD")]
    lud: String,

    #[serde(rename = "_NUD")]
    nud: String,

    #[serde(rename = "_UESD")]
    uesd: String,

    #[serde(rename = "_OP")]
    op: String,

    #[serde(rename = "_Lo")]
    lo: String,

    #[serde(rename = "_La")]
    la: String,

    #[serde(rename = "_STF")]
    stf: String,

    #[serde(rename = "_TPP")]
    tpp: String,

    #[serde(rename = "_MEv")]
    m_ev: String,

    #[serde(rename = "_MD")]
    md: String,

    #[serde(rename = "_MSD")]
    msd: String,

    #[serde(rename = "_MLo")]
    m_lo: String,

    #[serde(rename = "_MLa")]
    m_la: String,

    #[serde(rename = "_MED")]
    med: String,

    #[serde(rename = "_MET")]
    met: String,

    #[serde(rename = "_MPPer")]
    mp_per: String,

    #[serde(rename = "_TFEv")]
    tf_ev: String,

    #[serde(rename = "_TFD")]
    tfd: String,

    #[serde(rename = "_ArM")]
    ar_m: String,

    #[serde(rename = "_AcM")]
    ac_m: String,

    #[serde(rename = "_NvM")]
    nv_m: String,

    #[serde(rename = "_MPPId")]
    mpp_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CellGroupCulture {
    #[serde(rename = "Preferences")]
    preferences: PurplePreferences,

    #[serde(rename = "Activities")]
    activities: PurpleActivities,

    #[serde(rename = "Skills")]
    skills: PurpleSkills,

    #[serde(rename = "Knowledges")]
    knowledges: PurpleKnowledges,

    #[serde(rename = "DiscoveryIds")]
    discovery_ids: DiscoveryIds,

    #[serde(rename = "KnowledgeLimits")]
    knowledge_limits: KnowledgeLimits,

    #[serde(rename = "_LId")]
    l_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleActivities {
    #[serde(rename = "CulturalActivity")]
    cultural_activity: CulturalActivity,
}

#[derive(Serialize, Deserialize)]
pub struct Cultural {
    #[serde(rename = "_xsi:type")]
    xsi_type: Option<CulturalActivityXsiType>,

    #[serde(rename = "_Id")]
    id: CulturalActivityId,

    #[serde(rename = "_V")]
    v: String,

    #[serde(rename = "_C")]
    c: Option<String>,

    #[serde(rename = "_RO")]
    ro: Option<String>,

    #[serde(rename = "_AL")]
    al: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DiscoveryIds {
    #[serde(rename = "string")]
    string: StringUnion,
}

#[derive(Serialize, Deserialize)]
pub struct KnowledgeLimits {
    #[serde(rename = "KnowledgeLimit")]
    knowledge_limit: Vec<KnowledgeLimit>,
}

#[derive(Serialize, Deserialize)]
pub struct KnowledgeLimit {
    #[serde(rename = "_Id")]
    id: CulturalActivityId,

    #[serde(rename = "_V")]
    v: String,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleKnowledges {
    #[serde(rename = "CulturalKnowledge")]
    cultural_knowledge: Vec<Cultural>,
}

#[derive(Serialize, Deserialize)]
pub struct PurplePreferences {
    #[serde(rename = "CulturalPreference")]
    cultural_preference: Vec<Cultural>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleSkills {
    #[serde(rename = "CulturalSkill")]
    cultural_skill: Vec<Cultural>,
}

#[derive(Serialize, Deserialize)]
pub struct LastPopulationMigration {
    #[serde(rename = "_P")]
    p: String,

    #[serde(rename = "_SD")]
    sd: String,

    #[serde(rename = "_ED")]
    ed: String,

    #[serde(rename = "_SGId")]
    sg_id: String,

    #[serde(rename = "_PId")]
    p_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct PolityProminencesClass {
    #[serde(rename = "PolityProminence")]
    polity_prominence: PolityProminence,
}

#[derive(Serialize, Deserialize)]
pub struct PolityProminence {
    #[serde(rename = "_V")]
    v: String,

    #[serde(rename = "_FCT")]
    fct: String,

    #[serde(rename = "_PD")]
    pd: String,

    #[serde(rename = "_P")]
    p: String,

    #[serde(rename = "_CFId")]
    cf_id: String,

    #[serde(rename = "_PId")]
    p_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct PropertiesClass {
    #[serde(rename = "string")]
    string: String,
}

#[derive(Serialize, Deserialize)]
pub struct SeaMigrationRoute {
    #[serde(rename = "_U")]
    u: String,

    #[serde(rename = "_C")]
    c: String,

    #[serde(rename = "_CD")]
    cd: String,

    #[serde(rename = "_SLo")]
    s_lo: String,

    #[serde(rename = "_SLa")]
    s_la: String,
}

#[derive(Serialize, Deserialize)]
pub struct CulturalActivityInfoList {
    #[serde(rename = "CulturalActivityInfo")]
    cultural_activity_info: Vec<CulturalActivityInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct CulturalActivityInfo {
    #[serde(rename = "_Id")]
    id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CulturalKnowledgeInfoList {
    #[serde(rename = "CulturalKnowledgeInfo")]
    cultural_knowledge_info: Vec<CulturalActivityInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct CulturalPreferenceInfoList {
    #[serde(rename = "CulturalPreferenceInfo")]
    cultural_preference_info: Vec<CulturalActivityInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct CulturalSkillInfoList {
    #[serde(rename = "CulturalSkillInfo")]
    cultural_skill_info: Vec<CulturalActivityInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct EventMessageIdsClass {
    #[serde(rename = "long")]
    long: String,
}

#[derive(Serialize, Deserialize)]
pub struct EventsToHappen {
    #[serde(rename = "FactionModEvent")]
    faction_mod_event: Vec<FactionModEvent>,

    #[serde(rename = "CellGroupModEvent")]
    cell_group_mod_event: Vec<CellGroupModEvent>,
}

#[derive(Serialize, Deserialize)]
pub struct CellGroupModEvent {
    #[serde(rename = "GroupId")]
    group_id: CulturalActivityInfo,

    #[serde(rename = "_TId")]
    t_id: String,

    #[serde(rename = "_TD")]
    td: String,

    #[serde(rename = "_SD")]
    sd: String,

    #[serde(rename = "_Id")]
    id: String,

    #[serde(rename = "_GenId")]
    gen_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct FactionModEvent {
    #[serde(rename = "_TId")]
    t_id: String,

    #[serde(rename = "_TD")]
    td: String,

    #[serde(rename = "_SD")]
    sd: String,

    #[serde(rename = "_Id")]
    id: String,

    #[serde(rename = "_FId")]
    f_id: String,

    #[serde(rename = "_OPId")]
    op_id: String,

    #[serde(rename = "_GenId")]
    gen_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExistingDiscoveryIds {
    #[serde(rename = "string")]
    string: Vec<ExistingDiscoveryIdsString>,
}

#[derive(Serialize, Deserialize)]
pub struct FactionInfos {
    #[serde(rename = "FactionInfo")]
    faction_info: Vec<FactionInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct FactionInfo {
    #[serde(rename = "Name")]
    name: Name,

    #[serde(rename = "Faction")]
    faction: Option<Faction>,

    #[serde(rename = "_Id")]
    id: String,

    #[serde(rename = "_T")]
    t: FactionInfoT,
}

#[derive(Serialize, Deserialize)]
pub struct Faction {
    #[serde(rename = "Flags")]
    flags: Flags,

    #[serde(rename = "Culture")]
    culture: FactionCulture,

    #[serde(rename = "Relationships")]
    relationships: RelationshipsUnion,

    #[serde(rename = "EventDataList")]
    event_data_list: String,

    #[serde(rename = "LastLeader")]
    last_leader: LastLeader,

    #[serde(rename = "_xsi:type")]
    xsi_type: FactionXsiType,

    #[serde(rename = "_Inf")]
    inf: String,

    #[serde(rename = "_SP")]
    sp: String,

    #[serde(rename = "_IsDom")]
    is_dom: String,

    #[serde(rename = "_LastUpDate")]
    last_up_date: String,

    #[serde(rename = "_LeadStDate")]
    lead_st_date: String,

    #[serde(rename = "_IsCon")]
    is_con: String,

    #[serde(rename = "_PId")]
    p_id: String,

    #[serde(rename = "_CGId")]
    cg_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct FactionCulture {
    #[serde(rename = "Preferences")]
    preferences: FluffyPreferences,

    #[serde(rename = "Activities")]
    activities: FluffyActivities,

    #[serde(rename = "Skills")]
    skills: FluffySkills,

    #[serde(rename = "Knowledges")]
    knowledges: FluffyKnowledges,

    #[serde(rename = "DiscoveryIds")]
    discovery_ids: ExistingDiscoveryIds,

    #[serde(rename = "_LId")]
    l_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyActivities {
    #[serde(rename = "CulturalActivity")]
    cultural_activity: Vec<Cultural>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyKnowledges {
    #[serde(rename = "CulturalKnowledge")]
    cultural_knowledge: Vec<KnowledgeLimit>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyPreferences {
    #[serde(rename = "CulturalPreference")]
    cultural_preference: Vec<KnowledgeLimit>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffySkills {
    #[serde(rename = "CulturalSkill")]
    cultural_skill: Vec<KnowledgeLimit>,
}

#[derive(Serialize, Deserialize)]
pub struct LastLeader {
    #[serde(rename = "BirthCellPosition")]
    birth_cell_position: Position,

    #[serde(rename = "_Id")]
    id: String,

    #[serde(rename = "_Fem")]
    fem: String,

    #[serde(rename = "_Cha")]
    cha: String,

    #[serde(rename = "_Wis")]
    wis: String,

    #[serde(rename = "_SP")]
    sp: String,

    #[serde(rename = "_LId")]
    l_id: String,

    #[serde(rename = "_RId")]
    r_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "_X")]
    x: String,

    #[serde(rename = "_Y")]
    y: String,
}

#[derive(Serialize, Deserialize)]
pub struct RelationshipsClass {
    #[serde(rename = "FactionRelationship")]
    faction_relationship: FactionRelationshipUnion,
}

#[derive(Serialize, Deserialize)]
pub struct FactionRelationshipElement {
    #[serde(rename = "_Id")]
    id: String,

    #[serde(rename = "_Val")]
    val: String,
}

#[derive(Serialize, Deserialize)]
pub struct Name {
    #[serde(rename = "_Tm")]
    tm: String,

    #[serde(rename = "_LId")]
    l_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Languages {
    #[serde(rename = "Language")]
    language: Vec<Language>,
}

#[derive(Serialize, Deserialize)]
pub struct Language {
    #[serde(rename = "ArticleSyllables")]
    article_syllables: Syllables,

    #[serde(rename = "DerivativeArticleStartSyllables")]
    derivative_article_start_syllables: Syllables,

    #[serde(rename = "DerivativeArticleNextSyllables")]
    derivative_article_next_syllables: Syllables,

    #[serde(rename = "NounIndicativeSyllables")]
    noun_indicative_syllables: Syllables,

    #[serde(rename = "DerivativeNounIndicativeStartSyllables")]
    derivative_noun_indicative_start_syllables: Syllables,

    #[serde(rename = "DerivativeNounIndicativeNextSyllables")]
    derivative_noun_indicative_next_syllables: Syllables,

    #[serde(rename = "VerbIndicativeSyllables")]
    verb_indicative_syllables: Syllables,

    #[serde(rename = "DerivativeVerbIndicativeStartSyllables")]
    derivative_verb_indicative_start_syllables: Syllables,

    #[serde(rename = "DerivativeVerbIndicativeNextSyllables")]
    derivative_verb_indicative_next_syllables: Syllables,

    #[serde(rename = "AdpositionStartSyllables")]
    adposition_start_syllables: Syllables,

    #[serde(rename = "AdpositionNextSyllables")]
    adposition_next_syllables: Syllables,

    #[serde(rename = "AdjectiveStartSyllables")]
    adjective_start_syllables: Syllables,

    #[serde(rename = "AdjectiveNextSyllables")]
    adjective_next_syllables: Syllables,

    #[serde(rename = "NounStartSyllables")]
    noun_start_syllables: Syllables,

    #[serde(rename = "NounNextSyllables")]
    noun_next_syllables: Syllables,

    #[serde(rename = "VerbStartSyllables")]
    verb_start_syllables: Syllables,

    #[serde(rename = "VerbNextSyllables")]
    verb_next_syllables: Syllables,

    #[serde(rename = "Articles")]
    articles: Articles,

    #[serde(rename = "NounIndicatives")]
    noun_indicatives: NounIndicatives,

    #[serde(rename = "VerbIndicatives")]
    verb_indicatives: NounIndicatives,

    #[serde(rename = "Adpositions")]
    adpositions: AdpositionsUnion,

    #[serde(rename = "Adjectives")]
    adjectives: Adjectives,

    #[serde(rename = "Nouns")]
    nouns: Nouns,

    #[serde(rename = "Verbs")]
    verbs: String,

    #[serde(rename = "_Id")]
    id: String,

    #[serde(rename = "_AP")]
    ap: String,

    #[serde(rename = "_NIP")]
    nip: String,

    #[serde(rename = "_VIP")]
    vip: String,

    #[serde(rename = "_AAP")]
    aap: String,

    #[serde(rename = "_NIAP")]
    niap: String,

    #[serde(rename = "_VIAP")]
    viap: String,

    #[serde(rename = "_AdpAP")]
    adp_ap: String,

    #[serde(rename = "_AdjAP")]
    adj_ap: String,

    #[serde(rename = "_NAP")]
    nap: String,
}

#[derive(Serialize, Deserialize)]
pub struct Syllables {
    #[serde(rename = "_OSALC")]
    osalc: String,

    #[serde(rename = "_NSALC")]
    nsalc: String,

    #[serde(rename = "_CSALC")]
    csalc: String,
}

#[derive(Serialize, Deserialize)]
pub struct Nouns {
    #[serde(rename = "Morpheme")]
    morpheme: MorphemeUnion,
}

#[derive(Serialize, Deserialize)]
pub struct MorphemeElement {
    #[serde(rename = "_M")]
    m: String,

    #[serde(rename = "_V")]
    v: String,

    #[serde(rename = "_T")]
    t: MorphemeT,

    #[serde(rename = "_P")]
    p: String,
}

#[derive(Serialize, Deserialize)]
pub struct AdpositionsClass {
    #[serde(rename = "Morpheme")]
    morpheme: MorphemeElement,
}

#[derive(Serialize, Deserialize)]
pub struct NounIndicatives {
    #[serde(rename = "Morpheme")]
    morpheme: Vec<MorphemeElement>,
}

#[derive(Serialize, Deserialize)]
pub struct PolityInfos {
    #[serde(rename = "PolityInfo")]
    polity_info: Vec<PolityInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct PolityInfo {
    #[serde(rename = "Name")]
    name: Name,

    #[serde(rename = "Polity")]
    polity: Option<Polity>,

    #[serde(rename = "Type")]
    polity_info_type: Type,

    #[serde(rename = "_Id")]
    id: String,

    #[serde(rename = "_T")]
    t: PolityInfoT,
}

#[derive(Serialize, Deserialize)]
pub struct Polity {
    #[serde(rename = "CoreRegionIds")]
    core_region_ids: Ids,

    #[serde(rename = "ProminenceClusters")]
    prominence_clusters: ProminenceClusters,

    #[serde(rename = "Territory")]
    territory: Territory,

    #[serde(rename = "Culture")]
    culture: PolityCulture,

    #[serde(rename = "FactionIds")]
    faction_ids: Ids,

    #[serde(rename = "EventMessageIds")]
    event_message_ids: EventMessageIdsUnion,

    #[serde(rename = "EventDataList")]
    event_data_list: String,

    #[serde(rename = "_xsi:type")]
    xsi_type: Type,

    #[serde(rename = "_AC")]
    ac: String,

    #[serde(rename = "_P")]
    p: String,

    #[serde(rename = "_A")]
    a: String,

    #[serde(rename = "_CRS")]
    crs: String,

    #[serde(rename = "_NC")]
    nc: String,

    #[serde(rename = "_SP")]
    sp: String,

    #[serde(rename = "_IF")]
    polity_if: String,

    #[serde(rename = "_DFId")]
    df_id: String,

    #[serde(rename = "_CGId")]
    cg_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Ids {
    #[serde(rename = "Identifier")]
    identifier: Identifier,
}

#[derive(Serialize, Deserialize)]
pub struct PolityCulture {
    #[serde(rename = "Preferences")]
    preferences: FluffyPreferences,

    #[serde(rename = "Activities")]
    activities: FluffyActivities,

    #[serde(rename = "Skills")]
    skills: FluffySkills,

    #[serde(rename = "Knowledges")]
    knowledges: PurpleKnowledges,

    #[serde(rename = "DiscoveryIds")]
    discovery_ids: ExistingDiscoveryIds,

    #[serde(rename = "_LId")]
    l_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProminenceClusters {
    #[serde(rename = "PolityProminenceCluster")]
    polity_prominence_cluster: PolityProminenceClusterUnion,
}

#[derive(Serialize, Deserialize)]
pub struct PolityProminenceClusterElement {
    #[serde(rename = "ProminenceIds")]
    prominence_ids: Ids,

    #[serde(rename = "_Id")]
    id: String,

    #[serde(rename = "_TAC")]
    tac: String,

    #[serde(rename = "_TP")]
    tp: String,

    #[serde(rename = "_PA")]
    pa: String,

    #[serde(rename = "_NC")]
    nc: String,

    #[serde(rename = "_RId")]
    r_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Territory {
    #[serde(rename = "CellPositions")]
    cell_positions: TerritoryCellPositions,

    #[serde(rename = "EnclosedAreas")]
    enclosed_areas: EnclosedAreasUnion,

    #[serde(rename = "RegionAccesses")]
    region_accesses: RegionAccesses,
}

#[derive(Serialize, Deserialize)]
pub struct TerritoryCellPositions {
    #[serde(rename = "WorldPosition")]
    world_position: WorldPosition,
}

#[derive(Serialize, Deserialize)]
pub struct EnclosedAreasClass {
    #[serde(rename = "CellArea")]
    cell_area: CellAreaUnion,
}

#[derive(Serialize, Deserialize)]
pub struct CellAreaElement {
    #[serde(rename = "CellPositions")]
    cell_positions: TerritoryCellPositions,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleCellArea {
    #[serde(rename = "CellPositions")]
    cell_positions: PurpleCellPositions,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleCellPositions {
    #[serde(rename = "WorldPosition")]
    world_position: Position,
}

#[derive(Serialize, Deserialize)]
pub struct RegionAccesses {
    #[serde(rename = "RegionAccess")]
    region_access: RegionAccessUnion,
}

#[derive(Serialize, Deserialize)]
pub struct RegionAccessElement {
    #[serde(rename = "_C")]
    c: String,

    #[serde(rename = "_RId")]
    r_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegionInfos {
    #[serde(rename = "RegionInfo")]
    region_info: Vec<RegionInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct RegionInfo {
    #[serde(rename = "Region")]
    region: Region,

    #[serde(rename = "OriginCellPosition")]
    origin_cell_position: Position,

    #[serde(rename = "_Id")]
    id: String,

    #[serde(rename = "_R")]
    r: String,

    #[serde(rename = "_LId")]
    l_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Region {
    #[serde(rename = "CellPositions")]
    cell_positions: Option<TerritoryCellPositions>,

    #[serde(rename = "_xsi:type")]
    xsi_type: RegionXsiType,

    #[serde(rename = "SubRegionIds")]
    sub_region_ids: Option<SubRegionIds>,
}

#[derive(Serialize, Deserialize)]
pub struct TerrainCellAlterationList {
    #[serde(rename = "CellAlt")]
    cell_alt: Vec<CellAlt>,
}

#[derive(Serialize, Deserialize)]
pub struct CellAlt {
    #[serde(rename = "LayerData")]
    layer_data: LayerDataUnion,

    #[serde(rename = "_Lon")]
    lon: String,

    #[serde(rename = "_Lat")]
    lat: String,

    #[serde(rename = "_BA")]
    ba: String,

    #[serde(rename = "_BT")]
    bt: String,

    #[serde(rename = "_BR")]
    br: String,

    #[serde(rename = "_BTO")]
    bto: String,

    #[serde(rename = "_BRO")]
    bro: String,

    #[serde(rename = "_Fp")]
    fp: String,

    #[serde(rename = "_Ar")]
    ar: String,

    #[serde(rename = "_Acc")]
    acc: String,

    #[serde(rename = "_M")]
    m: String,
}

#[derive(Serialize, Deserialize)]
pub struct LayerDataClass {
    #[serde(rename = "CellLayerData")]
    cell_layer_data: CellLayerData,
}

#[derive(Serialize, Deserialize)]
pub struct CellLayerData {
    #[serde(rename = "_Id")]
    id: String,

    #[serde(rename = "_O")]
    o: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum CulturalActivity {
    Cultural(Cultural),

    CulturalArray(Vec<Cultural>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringUnion {
    Enum(FluffyString),

    EnumArray(Vec<PurpleString>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Flags {
    DiscoveryIds(DiscoveryIds),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PolityProminencesUnion {
    PolityProminencesClass(PolityProminencesClass),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertiesUnion {
    PropertiesClass(PropertiesClass),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RelationshipsUnion {
    RelationshipsClass(RelationshipsClass),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FactionRelationshipUnion {
    FactionRelationshipElement(FactionRelationshipElement),

    FactionRelationshipElementArray(Vec<FactionRelationshipElement>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Adjectives {
    Nouns(Nouns),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MorphemeUnion {
    MorphemeElement(MorphemeElement),

    MorphemeElementArray(Vec<MorphemeElement>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdpositionsUnion {
    AdpositionsClass(AdpositionsClass),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Articles {
    NounIndicatives(NounIndicatives),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Identifier {
    CulturalActivityInfo(CulturalActivityInfo),

    CulturalActivityInfoArray(Vec<CulturalActivityInfo>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventMessageIdsUnion {
    EventMessageIdsClass(EventMessageIdsClass),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PolityProminenceClusterUnion {
    PolityProminenceClusterElement(PolityProminenceClusterElement),

    PolityProminenceClusterElementArray(Vec<PolityProminenceClusterElement>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorldPosition {
    Position(Position),

    PositionArray(Vec<Position>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnclosedAreasUnion {
    EnclosedAreasClass(EnclosedAreasClass),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum CellAreaUnion {
    CellAreaElementArray(Vec<CellAreaElement>),

    PurpleCellArea(PurpleCellArea),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RegionAccessUnion {
    RegionAccessElement(RegionAccessElement),

    RegionAccessElementArray(Vec<RegionAccessElement>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubRegionIds {
    Ids(Ids),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum LayerDataUnion {
    LayerDataClass(LayerDataClass),

    String(String),
}

#[derive(Serialize, Deserialize)]
pub enum CulturalActivityId {
    #[serde(rename = "aggression")]
    Aggression,

    #[serde(rename = "agriculture")]
    Agriculture,

    #[serde(rename = "authority")]
    Authority,

    #[serde(rename = "black_fungi_survival")]
    BlackFungiSurvival,

    #[serde(rename = "cohesion")]
    Cohesion,

    #[serde(rename = "desert_survival")]
    DesertSurvival,

    #[serde(rename = "desertic_tundra_survival")]
    DeserticTundraSurvival,

    #[serde(rename = "farming")]
    Farming,

    #[serde(rename = "fishing")]
    Fishing,

    #[serde(rename = "foraging")]
    Foraging,

    #[serde(rename = "forest_survival")]
    ForestSurvival,

    #[serde(rename = "fungi_survival")]
    FungiSurvival,

    #[serde(rename = "grassland_survival")]
    GrasslandSurvival,

    #[serde(rename = "ice_sheet_survival")]
    IceSheetSurvival,

    #[serde(rename = "ice_shelf_survival")]
    IceShelfSurvival,

    #[serde(rename = "isolation")]
    Isolation,

    #[serde(rename = "rainforest_survival")]
    RainforestSurvival,

    #[serde(rename = "red_forest_survival")]
    RedForestSurvival,

    #[serde(rename = "scarlet_desert_survival")]
    ScarletDesertSurvival,

    #[serde(rename = "seafaring")]
    Seafaring,

    #[serde(rename = "shipbuilding")]
    Shipbuilding,

    #[serde(rename = "social_organization")]
    SocialOrganization,

    #[serde(rename = "taiga_survival")]
    TaigaSurvival,

    #[serde(rename = "tundra_survival")]
    TundraSurvival,
}

#[derive(Serialize, Deserialize)]
pub enum CulturalActivityXsiType {
    #[serde(rename = "BiomeSurvivalSkill")]
    BiomeSurvivalSkill,

    #[serde(rename = "CellCulturalActivity")]
    CellCulturalActivity,

    #[serde(rename = "CellCulturalPreference")]
    CellCulturalPreference,

    #[serde(rename = "PolityCulturalKnowledge")]
    PolityCulturalKnowledge,

    #[serde(rename = "SeafaringSkill")]
    SeafaringSkill,

    #[serde(rename = "ShipbuildingKnowledge")]
    ShipbuildingKnowledge,

    #[serde(rename = "SocialOrganizationKnowledge")]
    SocialOrganizationKnowledge,
}

#[derive(Serialize, Deserialize)]
pub enum PurpleString {
    #[serde(rename = "boat_making")]
    BoatMaking,

    #[serde(rename = "clan_decide_form_new_tribe_set")]
    ClanDecideFormNewTribeSet,

    #[serde(rename = "clan_decide_performing_influence_demand_set")]
    ClanDecidePerformingInfluenceDemandSet,

    #[serde(rename = "clan_decide_split_set")]
    ClanDecideSplitSet,

    #[serde(rename = "expand_toward_region_set")]
    ExpandTowardRegionSet,

    #[serde(rename = "gain_plant_cultivation_discovery_set")]
    GainPlantCultivationDiscoverySet,

    #[serde(rename = "improve_relationship_with_neighbors_clan_set")]
    ImproveRelationshipWithNeighborsClanSet,

    #[serde(rename = "keels")]
    Keels,

    #[serde(rename = "rudders")]
    Rudders,

    #[serde(rename = "sailing")]
    Sailing,

    #[serde(rename = "tribalism")]
    Tribalism,

    #[serde(rename = "tribe_invites_clan_join_set")]
    TribeInvitesClanJoinSet,
}

#[derive(Serialize, Deserialize)]
pub enum FluffyString {
    #[serde(rename = "boat_making")]
    BoatMaking,

    #[serde(rename = "expand_toward_region_set")]
    ExpandTowardRegionSet,

    #[serde(rename = "gain_sailing_discovery_set")]
    GainSailingDiscoverySet,
}

#[derive(Serialize, Deserialize)]
pub enum ExistingDiscoveryIdsString {
    #[serde(rename = "AAAAAA")]
    Aaaaaa,

    #[serde(rename = "boat_making")]
    BoatMaking,

    #[serde(rename = "keels")]
    Keels,

    #[serde(rename = "Mods\\Base")]
    ModsBase,

    #[serde(rename = "Mods\\WeirdBiomesMod")]
    ModsWeirdBiomesMod,

    #[serde(rename = "plant_cultivation")]
    PlantCultivation,

    #[serde(rename = "rudders")]
    Rudders,

    #[serde(rename = "sailing")]
    Sailing,

    #[serde(rename = "Test")]
    Test,

    #[serde(rename = "tribalism")]
    Tribalism,
}

#[derive(Serialize, Deserialize)]
pub enum FactionXsiType {
    #[serde(rename = "Clan")]
    Clan,
}

#[derive(Serialize, Deserialize)]
pub enum FactionInfoT {
    #[serde(rename = "clan")]
    Clan,
}

#[derive(Serialize, Deserialize)]
pub enum MorphemeT {
    #[serde(rename = "Adjective")]
    Adjective,

    #[serde(rename = "Adposition")]
    Adposition,

    #[serde(rename = "Article")]
    Article,

    #[serde(rename = "Indicative")]
    Indicative,

    #[serde(rename = "Noun")]
    Noun,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Tribe")]
    Tribe,
}

#[derive(Serialize, Deserialize)]
pub enum PolityInfoT {
    #[serde(rename = "tribe")]
    Tribe,
}

#[derive(Serialize, Deserialize)]
pub enum RegionXsiType {
    #[serde(rename = "CellRegion")]
    CellRegion,

    #[serde(rename = "SuperRegion")]
    SuperRegion,
}
