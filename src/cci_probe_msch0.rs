#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    probe_id_core_id: ProbeIdCoreId,
    probe_id_revision_id: ProbeIdRevisionId,
    probe_main_ctl: ProbeMainCtl,
    probe_cfg_ctl: ProbeCfgCtl,
    _reserved4: [u8; 0x14],
    probe_stat_period: ProbeStatPeriod,
    probe_stat_go: ProbeStatGo,
    _reserved6: [u8; 0x010c],
    probe_counters_0_src: ProbeCounters0Src,
    probe_counters_0_val: ProbeCounters0Val,
    _reserved8: [u8; 0x0c],
    probe_counters_1_src: ProbeCounters1Src,
    probe_counters_1_val: ProbeCounters1Val,
    _reserved10: [u8; 0x0c],
    probe_counters_2_src: ProbeCounters2Src,
    probe_counters_2_val: ProbeCounters2Val,
    _reserved12: [u8; 0x0c],
    probe_counters_3_src: ProbeCounters3Src,
    probe_counters_3_val: ProbeCounters3Val,
}
impl RegisterBlock {
    #[doc = "0x00 - Core ID register"]
    #[inline(always)]
    pub const fn probe_id_core_id(&self) -> &ProbeIdCoreId {
        &self.probe_id_core_id
    }
    #[doc = "0x04 - Revision ID register"]
    #[inline(always)]
    pub const fn probe_id_revision_id(&self) -> &ProbeIdRevisionId {
        &self.probe_id_revision_id
    }
    #[doc = "0x08 - Register MainCtl contains probe global control bits."]
    #[inline(always)]
    pub const fn probe_main_ctl(&self) -> &ProbeMainCtl {
        &self.probe_main_ctl
    }
    #[doc = "0x0c - Register CfgCtl contains global enable and active bits."]
    #[inline(always)]
    pub const fn probe_cfg_ctl(&self) -> &ProbeCfgCtl {
        &self.probe_cfg_ctl
    }
    #[doc = "0x24 - Statistics Period"]
    #[inline(always)]
    pub const fn probe_stat_period(&self) -> &ProbeStatPeriod {
        &self.probe_stat_period
    }
    #[doc = "0x28 - Statistics start to dump"]
    #[inline(always)]
    pub const fn probe_stat_go(&self) -> &ProbeStatGo {
        &self.probe_stat_go
    }
    #[doc = "0x138 - Register CntSrc indicates the event source."]
    #[inline(always)]
    pub const fn probe_counters_0_src(&self) -> &ProbeCounters0Src {
        &self.probe_counters_0_src
    }
    #[doc = "0x13c - Registers Counters_M_Val contain the statistics counter values."]
    #[inline(always)]
    pub const fn probe_counters_0_val(&self) -> &ProbeCounters0Val {
        &self.probe_counters_0_val
    }
    #[doc = "0x14c - Register CntSrc indicates the event source."]
    #[inline(always)]
    pub const fn probe_counters_1_src(&self) -> &ProbeCounters1Src {
        &self.probe_counters_1_src
    }
    #[doc = "0x150 - Registers Counters_M_Val contain the statistics counter values."]
    #[inline(always)]
    pub const fn probe_counters_1_val(&self) -> &ProbeCounters1Val {
        &self.probe_counters_1_val
    }
    #[doc = "0x160 - Register CntSrc indicates the event source."]
    #[inline(always)]
    pub const fn probe_counters_2_src(&self) -> &ProbeCounters2Src {
        &self.probe_counters_2_src
    }
    #[doc = "0x164 - Registers Counters_M_Val contain the statistics counter values."]
    #[inline(always)]
    pub const fn probe_counters_2_val(&self) -> &ProbeCounters2Val {
        &self.probe_counters_2_val
    }
    #[doc = "0x174 - Register CntSrc indicates the event source."]
    #[inline(always)]
    pub const fn probe_counters_3_src(&self) -> &ProbeCounters3Src {
        &self.probe_counters_3_src
    }
    #[doc = "0x178 - Registers Counters_M_Val contain the statistics counter values."]
    #[inline(always)]
    pub const fn probe_counters_3_val(&self) -> &ProbeCounters3Val {
        &self.probe_counters_3_val
    }
}
#[doc = "PROBE_Id_CoreId (r) register accessor: Core ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_id_core_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probe_id_core_id`]
module"]
#[doc(alias = "PROBE_Id_CoreId")]
pub type ProbeIdCoreId = crate::Reg<probe_id_core_id::ProbeIdCoreIdSpec>;
#[doc = "Core ID register"]
pub mod probe_id_core_id;
#[doc = "PROBE_Id_RevisionId (r) register accessor: Revision ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_id_revision_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probe_id_revision_id`]
module"]
#[doc(alias = "PROBE_Id_RevisionId")]
pub type ProbeIdRevisionId = crate::Reg<probe_id_revision_id::ProbeIdRevisionIdSpec>;
#[doc = "Revision ID register"]
pub mod probe_id_revision_id;
#[doc = "PROBE_MainCtl (rw) register accessor: Register MainCtl contains probe global control bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_main_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`probe_main_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probe_main_ctl`]
module"]
#[doc(alias = "PROBE_MainCtl")]
pub type ProbeMainCtl = crate::Reg<probe_main_ctl::ProbeMainCtlSpec>;
#[doc = "Register MainCtl contains probe global control bits."]
pub mod probe_main_ctl;
#[doc = "PROBE_CfgCtl (rw) register accessor: Register CfgCtl contains global enable and active bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_cfg_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`probe_cfg_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probe_cfg_ctl`]
module"]
#[doc(alias = "PROBE_CfgCtl")]
pub type ProbeCfgCtl = crate::Reg<probe_cfg_ctl::ProbeCfgCtlSpec>;
#[doc = "Register CfgCtl contains global enable and active bits."]
pub mod probe_cfg_ctl;
#[doc = "PROBE_StatPeriod (rw) register accessor: Statistics Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_stat_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`probe_stat_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probe_stat_period`]
module"]
#[doc(alias = "PROBE_StatPeriod")]
pub type ProbeStatPeriod = crate::Reg<probe_stat_period::ProbeStatPeriodSpec>;
#[doc = "Statistics Period"]
pub mod probe_stat_period;
#[doc = "PROBE_StatGo (rw) register accessor: Statistics start to dump\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_stat_go::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`probe_stat_go::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probe_stat_go`]
module"]
#[doc(alias = "PROBE_StatGo")]
pub type ProbeStatGo = crate::Reg<probe_stat_go::ProbeStatGoSpec>;
#[doc = "Statistics start to dump"]
pub mod probe_stat_go;
#[doc = "PROBE_Counters_0_Src (rw) register accessor: Register CntSrc indicates the event source.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_counters_0_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`probe_counters_0_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probe_counters_0_src`]
module"]
#[doc(alias = "PROBE_Counters_0_Src")]
pub type ProbeCounters0Src = crate::Reg<probe_counters_0_src::ProbeCounters0SrcSpec>;
#[doc = "Register CntSrc indicates the event source."]
pub mod probe_counters_0_src;
#[doc = "PROBE_Counters_0_Val (r) register accessor: Registers Counters_M_Val contain the statistics counter values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_counters_0_val::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probe_counters_0_val`]
module"]
#[doc(alias = "PROBE_Counters_0_Val")]
pub type ProbeCounters0Val = crate::Reg<probe_counters_0_val::ProbeCounters0ValSpec>;
#[doc = "Registers Counters_M_Val contain the statistics counter values."]
pub mod probe_counters_0_val;
#[doc = "PROBE_Counters_1_Src (rw) register accessor: Register CntSrc indicates the event source.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_counters_1_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`probe_counters_1_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probe_counters_1_src`]
module"]
#[doc(alias = "PROBE_Counters_1_Src")]
pub type ProbeCounters1Src = crate::Reg<probe_counters_1_src::ProbeCounters1SrcSpec>;
#[doc = "Register CntSrc indicates the event source."]
pub mod probe_counters_1_src;
#[doc = "PROBE_Counters_1_Val (r) register accessor: Registers Counters_M_Val contain the statistics counter values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_counters_1_val::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probe_counters_1_val`]
module"]
#[doc(alias = "PROBE_Counters_1_Val")]
pub type ProbeCounters1Val = crate::Reg<probe_counters_1_val::ProbeCounters1ValSpec>;
#[doc = "Registers Counters_M_Val contain the statistics counter values."]
pub mod probe_counters_1_val;
#[doc = "PROBE_Counters_2_Src (rw) register accessor: Register CntSrc indicates the event source.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_counters_2_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`probe_counters_2_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probe_counters_2_src`]
module"]
#[doc(alias = "PROBE_Counters_2_Src")]
pub type ProbeCounters2Src = crate::Reg<probe_counters_2_src::ProbeCounters2SrcSpec>;
#[doc = "Register CntSrc indicates the event source."]
pub mod probe_counters_2_src;
#[doc = "PROBE_Counters_2_Val (r) register accessor: Registers Counters_M_Val contain the statistics counter values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_counters_2_val::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probe_counters_2_val`]
module"]
#[doc(alias = "PROBE_Counters_2_Val")]
pub type ProbeCounters2Val = crate::Reg<probe_counters_2_val::ProbeCounters2ValSpec>;
#[doc = "Registers Counters_M_Val contain the statistics counter values."]
pub mod probe_counters_2_val;
#[doc = "PROBE_Counters_3_Src (rw) register accessor: Register CntSrc indicates the event source.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_counters_3_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`probe_counters_3_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probe_counters_3_src`]
module"]
#[doc(alias = "PROBE_Counters_3_Src")]
pub type ProbeCounters3Src = crate::Reg<probe_counters_3_src::ProbeCounters3SrcSpec>;
#[doc = "Register CntSrc indicates the event source."]
pub mod probe_counters_3_src;
#[doc = "PROBE_Counters_3_Val (r) register accessor: Registers Counters_M_Val contain the statistics counter values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_counters_3_val::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@probe_counters_3_val`]
module"]
#[doc(alias = "PROBE_Counters_3_Val")]
pub type ProbeCounters3Val = crate::Reg<probe_counters_3_val::ProbeCounters3ValSpec>;
#[doc = "Registers Counters_M_Val contain the statistics counter values."]
pub mod probe_counters_3_val;
