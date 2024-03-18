#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpio_swporta_dr: GpioSwportaDr,
    gpio_swporta_ddr: GpioSwportaDdr,
    _reserved2: [u8; 0x28],
    gpio_inten: GpioInten,
    gpio_intmask: GpioIntmask,
    gpio_inttype_level: GpioInttypeLevel,
    gpio_int_polarity: GpioIntPolarity,
    gpio_int_status: GpioIntStatus,
    gpio_int_rawstatus: GpioIntRawstatus,
    gpio_debounce: GpioDebounce,
    gpio_porta_eoi: GpioPortaEoi,
    gpio_ext_porta: GpioExtPorta,
    _reserved11: [u8; 0x0c],
    gpio_ls_sync: GpioLsSync,
}
impl RegisterBlock {
    #[doc = "0x00 - Port A data register"]
    #[inline(always)]
    pub const fn gpio_swporta_dr(&self) -> &GpioSwportaDr {
        &self.gpio_swporta_dr
    }
    #[doc = "0x04 - Port A data direction register"]
    #[inline(always)]
    pub const fn gpio_swporta_ddr(&self) -> &GpioSwportaDdr {
        &self.gpio_swporta_ddr
    }
    #[doc = "0x30 - Interrupt enable register"]
    #[inline(always)]
    pub const fn gpio_inten(&self) -> &GpioInten {
        &self.gpio_inten
    }
    #[doc = "0x34 - Interrupt mask register"]
    #[inline(always)]
    pub const fn gpio_intmask(&self) -> &GpioIntmask {
        &self.gpio_intmask
    }
    #[doc = "0x38 - Interrupt level register"]
    #[inline(always)]
    pub const fn gpio_inttype_level(&self) -> &GpioInttypeLevel {
        &self.gpio_inttype_level
    }
    #[doc = "0x3c - Interrupt polarity register"]
    #[inline(always)]
    pub const fn gpio_int_polarity(&self) -> &GpioIntPolarity {
        &self.gpio_int_polarity
    }
    #[doc = "0x40 - Interrupt status of port A"]
    #[inline(always)]
    pub const fn gpio_int_status(&self) -> &GpioIntStatus {
        &self.gpio_int_status
    }
    #[doc = "0x44 - Raw Interrupt status of port A"]
    #[inline(always)]
    pub const fn gpio_int_rawstatus(&self) -> &GpioIntRawstatus {
        &self.gpio_int_rawstatus
    }
    #[doc = "0x48 - Debounce enable register"]
    #[inline(always)]
    pub const fn gpio_debounce(&self) -> &GpioDebounce {
        &self.gpio_debounce
    }
    #[doc = "0x4c - Port A clear interrupt register"]
    #[inline(always)]
    pub const fn gpio_porta_eoi(&self) -> &GpioPortaEoi {
        &self.gpio_porta_eoi
    }
    #[doc = "0x50 - Port A external port register"]
    #[inline(always)]
    pub const fn gpio_ext_porta(&self) -> &GpioExtPorta {
        &self.gpio_ext_porta
    }
    #[doc = "0x60 - Level_sensitive synchronization enable register"]
    #[inline(always)]
    pub const fn gpio_ls_sync(&self) -> &GpioLsSync {
        &self.gpio_ls_sync
    }
}
#[doc = "GPIO_SWPORTA_DR (rw) register accessor: Port A data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_swporta_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_swporta_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_swporta_dr`]
module"]
#[doc(alias = "GPIO_SWPORTA_DR")]
pub type GpioSwportaDr = crate::Reg<gpio_swporta_dr::GpioSwportaDrSpec>;
#[doc = "Port A data register"]
pub mod gpio_swporta_dr;
#[doc = "GPIO_SWPORTA_DDR (rw) register accessor: Port A data direction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_swporta_ddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_swporta_ddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_swporta_ddr`]
module"]
#[doc(alias = "GPIO_SWPORTA_DDR")]
pub type GpioSwportaDdr = crate::Reg<gpio_swporta_ddr::GpioSwportaDdrSpec>;
#[doc = "Port A data direction register"]
pub mod gpio_swporta_ddr;
#[doc = "GPIO_INTEN (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_inten`]
module"]
#[doc(alias = "GPIO_INTEN")]
pub type GpioInten = crate::Reg<gpio_inten::GpioIntenSpec>;
#[doc = "Interrupt enable register"]
pub mod gpio_inten;
#[doc = "GPIO_INTMASK (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_intmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_intmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_intmask`]
module"]
#[doc(alias = "GPIO_INTMASK")]
pub type GpioIntmask = crate::Reg<gpio_intmask::GpioIntmaskSpec>;
#[doc = "Interrupt mask register"]
pub mod gpio_intmask;
#[doc = "GPIO_INTTYPE_LEVEL (rw) register accessor: Interrupt level register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_inttype_level::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_inttype_level::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_inttype_level`]
module"]
#[doc(alias = "GPIO_INTTYPE_LEVEL")]
pub type GpioInttypeLevel = crate::Reg<gpio_inttype_level::GpioInttypeLevelSpec>;
#[doc = "Interrupt level register"]
pub mod gpio_inttype_level;
#[doc = "GPIO_INT_POLARITY (rw) register accessor: Interrupt polarity register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_int_polarity::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_int_polarity::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_int_polarity`]
module"]
#[doc(alias = "GPIO_INT_POLARITY")]
pub type GpioIntPolarity = crate::Reg<gpio_int_polarity::GpioIntPolaritySpec>;
#[doc = "Interrupt polarity register"]
pub mod gpio_int_polarity;
#[doc = "GPIO_INT_STATUS (r) register accessor: Interrupt status of port A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_int_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_int_status`]
module"]
#[doc(alias = "GPIO_INT_STATUS")]
pub type GpioIntStatus = crate::Reg<gpio_int_status::GpioIntStatusSpec>;
#[doc = "Interrupt status of port A"]
pub mod gpio_int_status;
#[doc = "GPIO_INT_RAWSTATUS (r) register accessor: Raw Interrupt status of port A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_int_rawstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_int_rawstatus`]
module"]
#[doc(alias = "GPIO_INT_RAWSTATUS")]
pub type GpioIntRawstatus = crate::Reg<gpio_int_rawstatus::GpioIntRawstatusSpec>;
#[doc = "Raw Interrupt status of port A"]
pub mod gpio_int_rawstatus;
#[doc = "GPIO_DEBOUNCE (rw) register accessor: Debounce enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_debounce::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_debounce::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_debounce`]
module"]
#[doc(alias = "GPIO_DEBOUNCE")]
pub type GpioDebounce = crate::Reg<gpio_debounce::GpioDebounceSpec>;
#[doc = "Debounce enable register"]
pub mod gpio_debounce;
#[doc = "GPIO_PORTA_EOI (w) register accessor: Port A clear interrupt register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_porta_eoi::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_porta_eoi`]
module"]
#[doc(alias = "GPIO_PORTA_EOI")]
pub type GpioPortaEoi = crate::Reg<gpio_porta_eoi::GpioPortaEoiSpec>;
#[doc = "Port A clear interrupt register"]
pub mod gpio_porta_eoi;
#[doc = "GPIO_EXT_PORTA (r) register accessor: Port A external port register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_ext_porta::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ext_porta`]
module"]
#[doc(alias = "GPIO_EXT_PORTA")]
pub type GpioExtPorta = crate::Reg<gpio_ext_porta::GpioExtPortaSpec>;
#[doc = "Port A external port register"]
pub mod gpio_ext_porta;
#[doc = "GPIO_LS_SYNC (rw) register accessor: Level_sensitive synchronization enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_ls_sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_ls_sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_ls_sync`]
module"]
#[doc(alias = "GPIO_LS_SYNC")]
pub type GpioLsSync = crate::Reg<gpio_ls_sync::GpioLsSyncSpec>;
#[doc = "Level_sensitive synchronization enable register"]
pub mod gpio_ls_sync;
