#[doc = "Register `MI_SP_Y_OFFS_CNT_START` reader"]
pub type R = crate::R<MiSpYOffsCntStartSpec>;
#[doc = "Field `sp_y_offs_cnt_start` reader - Offset counter value which points to the start address\n\nof the previously processed picture (self picture Y\n\ncomponent).\n\nUpdated at frame end.\n\nNote: Soft reset will reset the contents to reset value.\n\n"]
pub type SpYOffsCntStartR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:28 - Offset counter value which points to the start address\n\nof the previously processed picture (self picture Y\n\ncomponent).\n\nUpdated at frame end.\n\nNote: Soft reset will reset the contents to reset value.\n\n"]
    #[inline(always)]
    pub fn sp_y_offs_cnt_start(&self) -> SpYOffsCntStartR {
        SpYOffsCntStartR::new((self.bits >> 3) & 0x03ff_ffff)
    }
}
#[doc = "Offset counter start value for self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_offs_cnt_start::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiSpYOffsCntStartSpec;
impl crate::RegisterSpec for MiSpYOffsCntStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_sp_y_offs_cnt_start::R`](R) reader structure"]
impl crate::Readable for MiSpYOffsCntStartSpec {}
#[doc = "`reset()` method sets MI_SP_Y_OFFS_CNT_START to value 0"]
impl crate::Resettable for MiSpYOffsCntStartSpec {
    const RESET_VALUE: u32 = 0;
}
