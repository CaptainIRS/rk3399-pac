#[doc = "Register `MI_MP_Y_BASE_AD_SHD` reader"]
pub type R = crate::R<MiMpYBaseAdShdSpec>;
#[doc = "Field `mp_y_base_ad` reader - Base address of main picture Y component ring buffer,\n\nJPEG ring buffer or raw data ring buffer."]
pub type MpYBaseAdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:31 - Base address of main picture Y component ring buffer,\n\nJPEG ring buffer or raw data ring buffer."]
    #[inline(always)]
    pub fn mp_y_base_ad(&self) -> MpYBaseAdR {
        MpYBaseAdR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
#[doc = "Base address shadow register for main picture Y \n\n\n\ncomponent, JPEG or raw data ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_y_base_ad_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiMpYBaseAdShdSpec;
impl crate::RegisterSpec for MiMpYBaseAdShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_mp_y_base_ad_shd::R`](R) reader structure"]
impl crate::Readable for MiMpYBaseAdShdSpec {}
#[doc = "`reset()` method sets MI_MP_Y_BASE_AD_SHD to value 0"]
impl crate::Resettable for MiMpYBaseAdShdSpec {
    const RESET_VALUE: u32 = 0;
}
