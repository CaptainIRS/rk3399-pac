#[doc = "Register `MI_SP_CB_BASE_AD_SHD` reader"]
pub type R = crate::R<MiSpCbBaseAdShdSpec>;
#[doc = "Field `sp_cb_base_ad` reader - Base address of self picture Cb component ring buffer."]
pub type SpCbBaseAdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:31 - Base address of self picture Cb component ring buffer."]
    #[inline(always)]
    pub fn sp_cb_base_ad(&self) -> SpCbBaseAdR {
        SpCbBaseAdR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
#[doc = "Base address shadow register for self picture Cb \n\n\n\ncomponent ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cb_base_ad_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiSpCbBaseAdShdSpec;
impl crate::RegisterSpec for MiSpCbBaseAdShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_sp_cb_base_ad_shd::R`](R) reader structure"]
impl crate::Readable for MiSpCbBaseAdShdSpec {}
#[doc = "`reset()` method sets MI_SP_CB_BASE_AD_SHD to value 0"]
impl crate::Resettable for MiSpCbBaseAdShdSpec {
    const RESET_VALUE: u32 = 0;
}
