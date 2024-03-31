#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    data: Data,
    stas: Stas,
    ctrl: Ctrl,
    dly_pu_soc: DlyPuSoc,
}
impl RegisterBlock {
    #[doc = "0x00 - This register contains the data after A/D Conversion."]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x04 - The status register of A/D Converter."]
    #[inline(always)]
    pub const fn stas(&self) -> &Stas {
        &self.stas
    }
    #[doc = "0x08 - The control register of A/D Converter."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x0c - delay between power up and start command"]
    #[inline(always)]
    pub const fn dly_pu_soc(&self) -> &DlyPuSoc {
        &self.dly_pu_soc
    }
}
#[doc = "DATA (r) register accessor: This register contains the data after A/D Conversion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "This register contains the data after A/D Conversion."]
pub mod data;
#[doc = "STAS (r) register accessor: The status register of A/D Converter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stas::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stas`]
module"]
#[doc(alias = "STAS")]
pub type Stas = crate::Reg<stas::StasSpec>;
#[doc = "The status register of A/D Converter."]
pub mod stas;
#[doc = "CTRL (rw) register accessor: The control register of A/D Converter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "The control register of A/D Converter."]
pub mod ctrl;
#[doc = "DLY_PU_SOC (rw) register accessor: delay between power up and start command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dly_pu_soc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dly_pu_soc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dly_pu_soc`]
module"]
#[doc(alias = "DLY_PU_SOC")]
pub type DlyPuSoc = crate::Reg<dly_pu_soc::DlyPuSocSpec>;
#[doc = "delay between power up and start command"]
pub mod dly_pu_soc;
