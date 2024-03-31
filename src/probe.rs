#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    id_core_id: IdCoreId,
    id_revision_id: IdRevisionId,
    main_ctl: MainCtl,
    cfg_ctl: CfgCtl,
    _reserved4: [u8; 0x14],
    stat_period: StatPeriod,
    stat_go: StatGo,
    _reserved6: [u8; 0x010c],
    counters_0_src: Counters0Src,
    counters_0_val: Counters0Val,
    _reserved8: [u8; 0x0c],
    counters_1_src: Counters1Src,
    counters_1_val: Counters1Val,
    _reserved10: [u8; 0x0c],
    counters_2_src: Counters2Src,
    counters_2_val: Counters2Val,
    _reserved12: [u8; 0x0c],
    counters_3_src: Counters3Src,
    counters_3_val: Counters3Val,
}
impl RegisterBlock {
    #[doc = "0x00 - Core ID register"]
    #[inline(always)]
    pub const fn id_core_id(&self) -> &IdCoreId {
        &self.id_core_id
    }
    #[doc = "0x04 - Revision ID register"]
    #[inline(always)]
    pub const fn id_revision_id(&self) -> &IdRevisionId {
        &self.id_revision_id
    }
    #[doc = "0x08 - Register MainCtl contains probe global control bits."]
    #[inline(always)]
    pub const fn main_ctl(&self) -> &MainCtl {
        &self.main_ctl
    }
    #[doc = "0x0c - Register CfgCtl contains global enable and active bits."]
    #[inline(always)]
    pub const fn cfg_ctl(&self) -> &CfgCtl {
        &self.cfg_ctl
    }
    #[doc = "0x24 - Statistics Period"]
    #[inline(always)]
    pub const fn stat_period(&self) -> &StatPeriod {
        &self.stat_period
    }
    #[doc = "0x28 - Statistics start to dump"]
    #[inline(always)]
    pub const fn stat_go(&self) -> &StatGo {
        &self.stat_go
    }
    #[doc = "0x138 - Register CntSrc indicates the event source."]
    #[inline(always)]
    pub const fn counters_0_src(&self) -> &Counters0Src {
        &self.counters_0_src
    }
    #[doc = "0x13c - Registers Counters_M_Val contain the statistics counter values."]
    #[inline(always)]
    pub const fn counters_0_val(&self) -> &Counters0Val {
        &self.counters_0_val
    }
    #[doc = "0x14c - Register CntSrc indicates the event source."]
    #[inline(always)]
    pub const fn counters_1_src(&self) -> &Counters1Src {
        &self.counters_1_src
    }
    #[doc = "0x150 - Registers Counters_M_Val contain the statistics counter values."]
    #[inline(always)]
    pub const fn counters_1_val(&self) -> &Counters1Val {
        &self.counters_1_val
    }
    #[doc = "0x160 - Register CntSrc indicates the event source."]
    #[inline(always)]
    pub const fn counters_2_src(&self) -> &Counters2Src {
        &self.counters_2_src
    }
    #[doc = "0x164 - Registers Counters_M_Val contain the statistics counter values."]
    #[inline(always)]
    pub const fn counters_2_val(&self) -> &Counters2Val {
        &self.counters_2_val
    }
    #[doc = "0x174 - Register CntSrc indicates the event source."]
    #[inline(always)]
    pub const fn counters_3_src(&self) -> &Counters3Src {
        &self.counters_3_src
    }
    #[doc = "0x178 - Registers Counters_M_Val contain the statistics counter values."]
    #[inline(always)]
    pub const fn counters_3_val(&self) -> &Counters3Val {
        &self.counters_3_val
    }
}
#[doc = "Id_CoreId (r) register accessor: Core ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_core_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_core_id`]
module"]
#[doc(alias = "Id_CoreId")]
pub type IdCoreId = crate::Reg<id_core_id::IdCoreIdSpec>;
#[doc = "Core ID register"]
pub mod id_core_id;
#[doc = "Id_RevisionId (r) register accessor: Revision ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_revision_id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_revision_id`]
module"]
#[doc(alias = "Id_RevisionId")]
pub type IdRevisionId = crate::Reg<id_revision_id::IdRevisionIdSpec>;
#[doc = "Revision ID register"]
pub mod id_revision_id;
#[doc = "MainCtl (rw) register accessor: Register MainCtl contains probe global control bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`main_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`main_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@main_ctl`]
module"]
pub type MainCtl = crate::Reg<main_ctl::MainCtlSpec>;
#[doc = "Register MainCtl contains probe global control bits."]
pub mod main_ctl;
#[doc = "CfgCtl (rw) register accessor: Register CfgCtl contains global enable and active bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_ctl`]
module"]
pub type CfgCtl = crate::Reg<cfg_ctl::CfgCtlSpec>;
#[doc = "Register CfgCtl contains global enable and active bits."]
pub mod cfg_ctl;
#[doc = "StatPeriod (rw) register accessor: Statistics Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat_period`]
module"]
pub type StatPeriod = crate::Reg<stat_period::StatPeriodSpec>;
#[doc = "Statistics Period"]
pub mod stat_period;
#[doc = "StatGo (rw) register accessor: Statistics start to dump\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat_go::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat_go::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat_go`]
module"]
pub type StatGo = crate::Reg<stat_go::StatGoSpec>;
#[doc = "Statistics start to dump"]
pub mod stat_go;
#[doc = "Counters_0_Src (rw) register accessor: Register CntSrc indicates the event source.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`counters_0_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`counters_0_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counters_0_src`]
module"]
#[doc(alias = "Counters_0_Src")]
pub type Counters0Src = crate::Reg<counters_0_src::Counters0SrcSpec>;
#[doc = "Register CntSrc indicates the event source."]
pub mod counters_0_src;
#[doc = "Counters_0_Val (r) register accessor: Registers Counters_M_Val contain the statistics counter values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`counters_0_val::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counters_0_val`]
module"]
#[doc(alias = "Counters_0_Val")]
pub type Counters0Val = crate::Reg<counters_0_val::Counters0ValSpec>;
#[doc = "Registers Counters_M_Val contain the statistics counter values."]
pub mod counters_0_val;
#[doc = "Counters_1_Src (rw) register accessor: Register CntSrc indicates the event source.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`counters_1_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`counters_1_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counters_1_src`]
module"]
#[doc(alias = "Counters_1_Src")]
pub type Counters1Src = crate::Reg<counters_1_src::Counters1SrcSpec>;
#[doc = "Register CntSrc indicates the event source."]
pub mod counters_1_src;
#[doc = "Counters_1_Val (r) register accessor: Registers Counters_M_Val contain the statistics counter values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`counters_1_val::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counters_1_val`]
module"]
#[doc(alias = "Counters_1_Val")]
pub type Counters1Val = crate::Reg<counters_1_val::Counters1ValSpec>;
#[doc = "Registers Counters_M_Val contain the statistics counter values."]
pub mod counters_1_val;
#[doc = "Counters_2_Src (rw) register accessor: Register CntSrc indicates the event source.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`counters_2_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`counters_2_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counters_2_src`]
module"]
#[doc(alias = "Counters_2_Src")]
pub type Counters2Src = crate::Reg<counters_2_src::Counters2SrcSpec>;
#[doc = "Register CntSrc indicates the event source."]
pub mod counters_2_src;
#[doc = "Counters_2_Val (r) register accessor: Registers Counters_M_Val contain the statistics counter values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`counters_2_val::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counters_2_val`]
module"]
#[doc(alias = "Counters_2_Val")]
pub type Counters2Val = crate::Reg<counters_2_val::Counters2ValSpec>;
#[doc = "Registers Counters_M_Val contain the statistics counter values."]
pub mod counters_2_val;
#[doc = "Counters_3_Src (rw) register accessor: Register CntSrc indicates the event source.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`counters_3_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`counters_3_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counters_3_src`]
module"]
#[doc(alias = "Counters_3_Src")]
pub type Counters3Src = crate::Reg<counters_3_src::Counters3SrcSpec>;
#[doc = "Register CntSrc indicates the event source."]
pub mod counters_3_src;
#[doc = "Counters_3_Val (r) register accessor: Registers Counters_M_Val contain the statistics counter values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`counters_3_val::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counters_3_val`]
module"]
#[doc(alias = "Counters_3_Val")]
pub type Counters3Val = crate::Reg<counters_3_val::Counters3ValSpec>;
#[doc = "Registers Counters_M_Val contain the statistics counter values."]
pub mod counters_3_val;
