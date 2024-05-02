#[doc = "Register `MI_SP_Y_BASE_AD_SHD` reader"]
pub type R = crate::R<MiSpYBaseAdShdSpec>;
#[doc = "Field `sp_y_base_ad` reader - Base address of self picture Y component ring buffer."]
pub type SpYBaseAdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:31 - Base address of self picture Y component ring buffer."]
    #[inline(always)]
    pub fn sp_y_base_ad(&self) -> SpYBaseAdR {
        SpYBaseAdR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
#[doc = "Base address shadow register for self picture Y \n\n\n\ncomponent ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_base_ad_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiSpYBaseAdShdSpec;
impl crate::RegisterSpec for MiSpYBaseAdShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_sp_y_base_ad_shd::R`](R) reader structure"]
impl crate::Readable for MiSpYBaseAdShdSpec {}
#[doc = "`reset()` method sets MI_SP_Y_BASE_AD_SHD to value 0"]
impl crate::Resettable for MiSpYBaseAdShdSpec {
    const RESET_VALUE: u32 = 0;
}
