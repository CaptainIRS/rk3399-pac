#[doc = "Register `MI_MP_CB_OFFS_CNT_START` reader"]
pub type R = crate::R<MiMpCbOffsCntStartSpec>;
#[doc = "Field `mp_cb_offs_cnt_start` reader - Offset counter value which points to the start address\n\nof the previously processed picture (main picture Cb\n\ncomponent). Updated at frame end."]
pub type MpCbOffsCntStartR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:27 - Offset counter value which points to the start address\n\nof the previously processed picture (main picture Cb\n\ncomponent). Updated at frame end."]
    #[inline(always)]
    pub fn mp_cb_offs_cnt_start(&self) -> MpCbOffsCntStartR {
        MpCbOffsCntStartR::new((self.bits >> 3) & 0x01ff_ffff)
    }
}
#[doc = "Offset counter start value for main picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cb_offs_cnt_start::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiMpCbOffsCntStartSpec;
impl crate::RegisterSpec for MiMpCbOffsCntStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_mp_cb_offs_cnt_start::R`](R) reader structure"]
impl crate::Readable for MiMpCbOffsCntStartSpec {}
#[doc = "`reset()` method sets MI_MP_CB_OFFS_CNT_START to value 0"]
impl crate::Resettable for MiMpCbOffsCntStartSpec {
    const RESET_VALUE: u32 = 0;
}
