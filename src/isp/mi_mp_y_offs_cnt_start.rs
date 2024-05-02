#[doc = "Register `MI_MP_Y_OFFS_CNT_START` reader"]
pub type R = crate::R<MiMpYOffsCntStartSpec>;
#[doc = "Field `mp_y_offs_cnt_start` reader - Offset counter value which points to the start address\n\nof the previously processed picture (main picture Y\n\ncomponent, JPEG or raw data). Updated at frame end.\n\nNote: A soft reset resets the contents to the reset\n\nvalue."]
pub type MpYOffsCntStartR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:28 - Offset counter value which points to the start address\n\nof the previously processed picture (main picture Y\n\ncomponent, JPEG or raw data). Updated at frame end.\n\nNote: A soft reset resets the contents to the reset\n\nvalue."]
    #[inline(always)]
    pub fn mp_y_offs_cnt_start(&self) -> MpYOffsCntStartR {
        MpYOffsCntStartR::new((self.bits >> 3) & 0x03ff_ffff)
    }
}
#[doc = "Offset counter start value for main picture Y, JPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_y_offs_cnt_start::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiMpYOffsCntStartSpec;
impl crate::RegisterSpec for MiMpYOffsCntStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_mp_y_offs_cnt_start::R`](R) reader structure"]
impl crate::Readable for MiMpYOffsCntStartSpec {}
#[doc = "`reset()` method sets MI_MP_Y_OFFS_CNT_START to value 0"]
impl crate::Resettable for MiMpYOffsCntStartSpec {
    const RESET_VALUE: u32 = 0;
}
