#[doc = "Register `MIPI_RIS` reader"]
pub type R = crate::R<MipiRisSpec>;
#[doc = "Field `RIS_ERR_SOT` reader - PPI interface sot error occured, one bit for each lane"]
pub type RisErrSotR = crate::FieldReader;
#[doc = "Field `RIS_ERR_SOT_SYNC` reader - PPI interface sot sync error occured, one bit for each\n\nlane"]
pub type RisErrSotSyncR = crate::FieldReader;
#[doc = "Field `RIS_ERR_EOT_SYNC` reader - PPI interface eot sync error occured, one bit for each\n\nlane"]
pub type RisErrEotSyncR = crate::FieldReader;
#[doc = "Field `RIS_ERR_CONTROL` reader - PPI interface control error occured, one bit for each\n\nlane"]
pub type RisErrControlR = crate::FieldReader;
#[doc = "Field `RIS_ERR_PROTOCOL` reader - packet start detected within current packet"]
pub type RisErrProtocolR = crate::BitReader;
#[doc = "Field `RIS_ERR_ECC2` reader - 2-bit ecc error occurred"]
pub type RisErrEcc2R = crate::BitReader;
#[doc = "Field `RIS_ERR_ECC1` reader - 1-bit ecc error occurred"]
pub type RisErrEcc1R = crate::BitReader;
#[doc = "Field `RIS_ERR_CS` reader - checksum error occurred"]
pub type RisErrCsR = crate::BitReader;
#[doc = "Field `RIS_FRAME_END` reader - frame end send to output interface"]
pub type RisFrameEndR = crate::BitReader;
#[doc = "Field `RIS_ADD_DATA_OVFLW` reader - additional data fifo overflow occurred"]
pub type RisAddDataOvflwR = crate::BitReader;
#[doc = "Field `RIS_GEN_SHORT_PACK` reader - generic short packet was received (only available in\n\nversion 2 of MIPI interface)\n\n\n\nWhen this interrupt is cleared, all the bits of the MIPI_GEN_SHORT_DT status register are cleared as well;\n\nSetting of this interrupt via MIPI_ISR register will set all\n\nthe bits of the MIPI_GEN_SHORT_DT register."]
pub type RisGenShortPackR = crate::BitReader;
impl R {
    #[doc = "Bits 4:7 - PPI interface sot error occured, one bit for each lane"]
    #[inline(always)]
    pub fn ris_err_sot(&self) -> RisErrSotR {
        RisErrSotR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PPI interface sot sync error occured, one bit for each\n\nlane"]
    #[inline(always)]
    pub fn ris_err_sot_sync(&self) -> RisErrSotSyncR {
        RisErrSotSyncR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PPI interface eot sync error occured, one bit for each\n\nlane"]
    #[inline(always)]
    pub fn ris_err_eot_sync(&self) -> RisErrEotSyncR {
        RisErrEotSyncR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PPI interface control error occured, one bit for each\n\nlane"]
    #[inline(always)]
    pub fn ris_err_control(&self) -> RisErrControlR {
        RisErrControlR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - packet start detected within current packet"]
    #[inline(always)]
    pub fn ris_err_protocol(&self) -> RisErrProtocolR {
        RisErrProtocolR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 2-bit ecc error occurred"]
    #[inline(always)]
    pub fn ris_err_ecc2(&self) -> RisErrEcc2R {
        RisErrEcc2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1-bit ecc error occurred"]
    #[inline(always)]
    pub fn ris_err_ecc1(&self) -> RisErrEcc1R {
        RisErrEcc1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - checksum error occurred"]
    #[inline(always)]
    pub fn ris_err_cs(&self) -> RisErrCsR {
        RisErrCsR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - frame end send to output interface"]
    #[inline(always)]
    pub fn ris_frame_end(&self) -> RisFrameEndR {
        RisFrameEndR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - additional data fifo overflow occurred"]
    #[inline(always)]
    pub fn ris_add_data_ovflw(&self) -> RisAddDataOvflwR {
        RisAddDataOvflwR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - generic short packet was received (only available in\n\nversion 2 of MIPI interface)\n\n\n\nWhen this interrupt is cleared, all the bits of the MIPI_GEN_SHORT_DT status register are cleared as well;\n\nSetting of this interrupt via MIPI_ISR register will set all\n\nthe bits of the MIPI_GEN_SHORT_DT register."]
    #[inline(always)]
    pub fn ris_gen_short_pack(&self) -> RisGenShortPackR {
        RisGenShortPackR::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiRisSpec;
impl crate::RegisterSpec for MipiRisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_ris::R`](R) reader structure"]
impl crate::Readable for MipiRisSpec {}
#[doc = "`reset()` method sets MIPI_RIS to value 0"]
impl crate::Resettable for MipiRisSpec {
    const RESET_VALUE: u32 = 0;
}
