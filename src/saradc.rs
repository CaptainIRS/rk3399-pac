#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    saradc_data: SaradcData,
    saradc_stas: SaradcStas,
    saradc_ctrl: SaradcCtrl,
    saradc_dly_pu_soc: SaradcDlyPuSoc,
}
impl RegisterBlock {
    #[doc = "0x00 - This register contains the data after A/D Conversion."]
    #[inline(always)]
    pub const fn saradc_data(&self) -> &SaradcData {
        &self.saradc_data
    }
    #[doc = "0x04 - The status register of A/D Converter."]
    #[inline(always)]
    pub const fn saradc_stas(&self) -> &SaradcStas {
        &self.saradc_stas
    }
    #[doc = "0x08 - The control register of A/D Converter."]
    #[inline(always)]
    pub const fn saradc_ctrl(&self) -> &SaradcCtrl {
        &self.saradc_ctrl
    }
    #[doc = "0x0c - delay between power up and start command"]
    #[inline(always)]
    pub const fn saradc_dly_pu_soc(&self) -> &SaradcDlyPuSoc {
        &self.saradc_dly_pu_soc
    }
}
#[doc = "SARADC_DATA (r) register accessor: This register contains the data after A/D Conversion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saradc_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saradc_data`]
module"]
#[doc(alias = "SARADC_DATA")]
pub type SaradcData = crate::Reg<saradc_data::SaradcDataSpec>;
#[doc = "This register contains the data after A/D Conversion."]
pub mod saradc_data;
#[doc = "SARADC_STAS (r) register accessor: The status register of A/D Converter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saradc_stas::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saradc_stas`]
module"]
#[doc(alias = "SARADC_STAS")]
pub type SaradcStas = crate::Reg<saradc_stas::SaradcStasSpec>;
#[doc = "The status register of A/D Converter."]
pub mod saradc_stas;
#[doc = "SARADC_CTRL (rw) register accessor: The control register of A/D Converter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saradc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saradc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saradc_ctrl`]
module"]
#[doc(alias = "SARADC_CTRL")]
pub type SaradcCtrl = crate::Reg<saradc_ctrl::SaradcCtrlSpec>;
#[doc = "The control register of A/D Converter."]
pub mod saradc_ctrl;
#[doc = "SARADC_DLY_PU_SOC (rw) register accessor: delay between power up and start command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saradc_dly_pu_soc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saradc_dly_pu_soc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saradc_dly_pu_soc`]
module"]
#[doc(alias = "SARADC_DLY_PU_SOC")]
pub type SaradcDlyPuSoc = crate::Reg<saradc_dly_pu_soc::SaradcDlyPuSocSpec>;
#[doc = "delay between power up and start command"]
pub mod saradc_dly_pu_soc;
