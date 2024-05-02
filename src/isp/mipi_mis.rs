#[doc = "Register `MIPI_MIS` reader"]
pub type R = crate::R<MipiMisSpec>;
#[doc = "Field `MIS_ERR_SOT` reader - PPI interface sot error occured, one bit for each lane"]
pub type MisErrSotR = crate::FieldReader;
#[doc = "Field `MIS_ERR_SOT_SYNC` reader - PPI interface sot sync error occured, one bit for each\n\nlane"]
pub type MisErrSotSyncR = crate::FieldReader;
#[doc = "Field `MIS_ERR_EOT_SYNC` reader - PPI interface eot sync error occured, one bit for each\n\nlane"]
pub type MisErrEotSyncR = crate::FieldReader;
#[doc = "Field `MIS_ERR_CONTROL` reader - PPI interface control error occured, one bit for each\n\nlane"]
pub type MisErrControlR = crate::FieldReader;
#[doc = "Field `MIS_ERR_PROTOCOL` reader - packet start detected within current packet"]
pub type MisErrProtocolR = crate::BitReader;
#[doc = "Field `MIS_ERR_ECC2` reader - 2-bit ecc error occurred"]
pub type MisErrEcc2R = crate::BitReader;
#[doc = "Field `MIS_ERR_ECC1` reader - 1-bit ecc error occurred"]
pub type MisErrEcc1R = crate::BitReader;
#[doc = "Field `MIS_ERR_CS` reader - checksum error occurred\n\n"]
pub type MisErrCsR = crate::BitReader;
#[doc = "Field `MIS_FRAME_END` reader - frame end send to output interface"]
pub type MisFrameEndR = crate::BitReader;
#[doc = "Field `MIS_ADD_DATA_OVFLW` reader - additional data fifo overflow"]
pub type MisAddDataOvflwR = crate::BitReader;
#[doc = "Field `MIS_GEN_SHORT_PACK` reader - generic short packet was received (only available in\n\nversion 2 of MIPI interface)"]
pub type MisGenShortPackR = crate::BitReader;
impl R {
    #[doc = "Bits 4:7 - PPI interface sot error occured, one bit for each lane"]
    #[inline(always)]
    pub fn mis_err_sot(&self) -> MisErrSotR {
        MisErrSotR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PPI interface sot sync error occured, one bit for each\n\nlane"]
    #[inline(always)]
    pub fn mis_err_sot_sync(&self) -> MisErrSotSyncR {
        MisErrSotSyncR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PPI interface eot sync error occured, one bit for each\n\nlane"]
    #[inline(always)]
    pub fn mis_err_eot_sync(&self) -> MisErrEotSyncR {
        MisErrEotSyncR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PPI interface control error occured, one bit for each\n\nlane"]
    #[inline(always)]
    pub fn mis_err_control(&self) -> MisErrControlR {
        MisErrControlR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - packet start detected within current packet"]
    #[inline(always)]
    pub fn mis_err_protocol(&self) -> MisErrProtocolR {
        MisErrProtocolR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 2-bit ecc error occurred"]
    #[inline(always)]
    pub fn mis_err_ecc2(&self) -> MisErrEcc2R {
        MisErrEcc2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1-bit ecc error occurred"]
    #[inline(always)]
    pub fn mis_err_ecc1(&self) -> MisErrEcc1R {
        MisErrEcc1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - checksum error occurred\n\n"]
    #[inline(always)]
    pub fn mis_err_cs(&self) -> MisErrCsR {
        MisErrCsR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - frame end send to output interface"]
    #[inline(always)]
    pub fn mis_frame_end(&self) -> MisFrameEndR {
        MisFrameEndR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - additional data fifo overflow"]
    #[inline(always)]
    pub fn mis_add_data_ovflw(&self) -> MisAddDataOvflwR {
        MisAddDataOvflwR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - generic short packet was received (only available in\n\nversion 2 of MIPI interface)"]
    #[inline(always)]
    pub fn mis_gen_short_pack(&self) -> MisGenShortPackR {
        MisGenShortPackR::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiMisSpec;
impl crate::RegisterSpec for MipiMisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_mis::R`](R) reader structure"]
impl crate::Readable for MipiMisSpec {}
#[doc = "`reset()` method sets MIPI_MIS to value 0"]
impl crate::Resettable for MipiMisSpec {
    const RESET_VALUE: u32 = 0;
}
