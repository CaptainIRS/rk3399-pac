#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    swporta_dr: SwportaDr,
    swporta_ddr: SwportaDdr,
    _reserved2: [u8; 0x28],
    inten: Inten,
    intmask: Intmask,
    inttype_level: InttypeLevel,
    int_polarity: IntPolarity,
    int_status: IntStatus,
    int_rawstatus: IntRawstatus,
    debounce: Debounce,
    porta_eoi: PortaEoi,
    ext_porta: ExtPorta,
    _reserved11: [u8; 0x0c],
    ls_sync: LsSync,
}
impl RegisterBlock {
    #[doc = "0x00 - Port A data register"]
    #[inline(always)]
    pub const fn swporta_dr(&self) -> &SwportaDr {
        &self.swporta_dr
    }
    #[doc = "0x04 - Port A data direction register"]
    #[inline(always)]
    pub const fn swporta_ddr(&self) -> &SwportaDdr {
        &self.swporta_ddr
    }
    #[doc = "0x30 - Interrupt enable register"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x34 - Interrupt mask register"]
    #[inline(always)]
    pub const fn intmask(&self) -> &Intmask {
        &self.intmask
    }
    #[doc = "0x38 - Interrupt level register"]
    #[inline(always)]
    pub const fn inttype_level(&self) -> &InttypeLevel {
        &self.inttype_level
    }
    #[doc = "0x3c - Interrupt polarity register"]
    #[inline(always)]
    pub const fn int_polarity(&self) -> &IntPolarity {
        &self.int_polarity
    }
    #[doc = "0x40 - Interrupt status of port A"]
    #[inline(always)]
    pub const fn int_status(&self) -> &IntStatus {
        &self.int_status
    }
    #[doc = "0x44 - Raw Interrupt status of port A"]
    #[inline(always)]
    pub const fn int_rawstatus(&self) -> &IntRawstatus {
        &self.int_rawstatus
    }
    #[doc = "0x48 - Debounce enable register"]
    #[inline(always)]
    pub const fn debounce(&self) -> &Debounce {
        &self.debounce
    }
    #[doc = "0x4c - Port A clear interrupt register"]
    #[inline(always)]
    pub const fn porta_eoi(&self) -> &PortaEoi {
        &self.porta_eoi
    }
    #[doc = "0x50 - Port A external port register"]
    #[inline(always)]
    pub const fn ext_porta(&self) -> &ExtPorta {
        &self.ext_porta
    }
    #[doc = "0x60 - Level_sensitive synchronization enable register"]
    #[inline(always)]
    pub const fn ls_sync(&self) -> &LsSync {
        &self.ls_sync
    }
}
#[doc = "SWPORTA_DR (rw) register accessor: Port A data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swporta_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swporta_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swporta_dr`]
module"]
#[doc(alias = "SWPORTA_DR")]
pub type SwportaDr = crate::Reg<swporta_dr::SwportaDrSpec>;
#[doc = "Port A data register"]
pub mod swporta_dr;
#[doc = "SWPORTA_DDR (rw) register accessor: Port A data direction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swporta_ddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swporta_ddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swporta_ddr`]
module"]
#[doc(alias = "SWPORTA_DDR")]
pub type SwportaDdr = crate::Reg<swporta_ddr::SwportaDdrSpec>;
#[doc = "Port A data direction register"]
pub mod swporta_ddr;
#[doc = "INTEN (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Interrupt enable register"]
pub mod inten;
#[doc = "INTMASK (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmask`]
module"]
#[doc(alias = "INTMASK")]
pub type Intmask = crate::Reg<intmask::IntmaskSpec>;
#[doc = "Interrupt mask register"]
pub mod intmask;
#[doc = "INTTYPE_LEVEL (rw) register accessor: Interrupt level register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inttype_level::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inttype_level::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttype_level`]
module"]
#[doc(alias = "INTTYPE_LEVEL")]
pub type InttypeLevel = crate::Reg<inttype_level::InttypeLevelSpec>;
#[doc = "Interrupt level register"]
pub mod inttype_level;
#[doc = "INT_POLARITY (rw) register accessor: Interrupt polarity register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_polarity::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_polarity::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_polarity`]
module"]
#[doc(alias = "INT_POLARITY")]
pub type IntPolarity = crate::Reg<int_polarity::IntPolaritySpec>;
#[doc = "Interrupt polarity register"]
pub mod int_polarity;
#[doc = "INT_STATUS (r) register accessor: Interrupt status of port A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status`]
module"]
#[doc(alias = "INT_STATUS")]
pub type IntStatus = crate::Reg<int_status::IntStatusSpec>;
#[doc = "Interrupt status of port A"]
pub mod int_status;
#[doc = "INT_RAWSTATUS (r) register accessor: Raw Interrupt status of port A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_rawstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_rawstatus`]
module"]
#[doc(alias = "INT_RAWSTATUS")]
pub type IntRawstatus = crate::Reg<int_rawstatus::IntRawstatusSpec>;
#[doc = "Raw Interrupt status of port A"]
pub mod int_rawstatus;
#[doc = "DEBOUNCE (rw) register accessor: Debounce enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debounce::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debounce::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debounce`]
module"]
#[doc(alias = "DEBOUNCE")]
pub type Debounce = crate::Reg<debounce::DebounceSpec>;
#[doc = "Debounce enable register"]
pub mod debounce;
#[doc = "PORTA_EOI (w) register accessor: Port A clear interrupt register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`porta_eoi::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@porta_eoi`]
module"]
#[doc(alias = "PORTA_EOI")]
pub type PortaEoi = crate::Reg<porta_eoi::PortaEoiSpec>;
#[doc = "Port A clear interrupt register"]
pub mod porta_eoi;
#[doc = "EXT_PORTA (r) register accessor: Port A external port register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_porta::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_porta`]
module"]
#[doc(alias = "EXT_PORTA")]
pub type ExtPorta = crate::Reg<ext_porta::ExtPortaSpec>;
#[doc = "Port A external port register"]
pub mod ext_porta;
#[doc = "LS_SYNC (rw) register accessor: Level_sensitive synchronization enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ls_sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ls_sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ls_sync`]
module"]
#[doc(alias = "LS_SYNC")]
pub type LsSync = crate::Reg<ls_sync::LsSyncSpec>;
#[doc = "Level_sensitive synchronization enable register"]
pub mod ls_sync;
