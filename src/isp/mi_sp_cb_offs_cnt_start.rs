#[doc = "Register `MI_SP_CB_OFFS_CNT_START` reader"]
pub type R = crate::R<MiSpCbOffsCntStartSpec>;
#[doc = "Field `sp_cb_offs_cnt_start` reader - Offset counter value which points to the start address\n\nof the previously processed picture (self picture Cb\n\ncomponent). Updated at frame end.\n\nNote: Soft reset will reset the contents to reset value."]
pub type SpCbOffsCntStartR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:27 - Offset counter value which points to the start address\n\nof the previously processed picture (self picture Cb\n\ncomponent). Updated at frame end.\n\nNote: Soft reset will reset the contents to reset value."]
    #[inline(always)]
    pub fn sp_cb_offs_cnt_start(&self) -> SpCbOffsCntStartR {
        SpCbOffsCntStartR::new((self.bits >> 3) & 0x01ff_ffff)
    }
}
#[doc = "Offset counter start value for self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cb_offs_cnt_start::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiSpCbOffsCntStartSpec;
impl crate::RegisterSpec for MiSpCbOffsCntStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_sp_cb_offs_cnt_start::R`](R) reader structure"]
impl crate::Readable for MiSpCbOffsCntStartSpec {}
#[doc = "`reset()` method sets MI_SP_CB_OFFS_CNT_START to value 0"]
impl crate::Resettable for MiSpCbOffsCntStartSpec {
    const RESET_VALUE: u32 = 0;
}
