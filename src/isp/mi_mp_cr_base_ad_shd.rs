#[doc = "Register `MI_MP_CR_BASE_AD_SHD` reader"]
pub type R = crate::R<MiMpCrBaseAdShdSpec>;
#[doc = "Field `mp_cr_base_ad` reader - Base address of main picture Cr component ring\n\nbuffer."]
pub type MpCrBaseAdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:31 - Base address of main picture Cr component ring\n\nbuffer."]
    #[inline(always)]
    pub fn mp_cr_base_ad(&self) -> MpCrBaseAdR {
        MpCrBaseAdR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
#[doc = "Base address shadow register for main picture Cr \n\n\n\ncomponent ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cr_base_ad_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiMpCrBaseAdShdSpec;
impl crate::RegisterSpec for MiMpCrBaseAdShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_mp_cr_base_ad_shd::R`](R) reader structure"]
impl crate::Readable for MiMpCrBaseAdShdSpec {}
#[doc = "`reset()` method sets MI_MP_CR_BASE_AD_SHD to value 0"]
impl crate::Resettable for MiMpCrBaseAdShdSpec {
    const RESET_VALUE: u32 = 0;
}
