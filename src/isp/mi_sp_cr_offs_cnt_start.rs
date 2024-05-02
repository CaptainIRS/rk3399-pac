#[doc = "Register `MI_SP_CR_OFFS_CNT_START` reader"]
pub type R = crate::R<MiSpCrOffsCntStartSpec>;
#[doc = "Field `sp_cr_offs_cnt_start` reader - Offset counter value which points to the start address\n\nof the previously processed picture (self picture Cr\n\ncomponent).\n\nUpdated at frame end."]
pub type SpCrOffsCntStartR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:27 - Offset counter value which points to the start address\n\nof the previously processed picture (self picture Cr\n\ncomponent).\n\nUpdated at frame end."]
    #[inline(always)]
    pub fn sp_cr_offs_cnt_start(&self) -> SpCrOffsCntStartR {
        SpCrOffsCntStartR::new((self.bits >> 3) & 0x01ff_ffff)
    }
}
#[doc = "Offset counter start value for self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cr_offs_cnt_start::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiSpCrOffsCntStartSpec;
impl crate::RegisterSpec for MiSpCrOffsCntStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_sp_cr_offs_cnt_start::R`](R) reader structure"]
impl crate::Readable for MiSpCrOffsCntStartSpec {}
#[doc = "`reset()` method sets MI_SP_CR_OFFS_CNT_START to value 0"]
impl crate::Resettable for MiSpCrOffsCntStartSpec {
    const RESET_VALUE: u32 = 0;
}
