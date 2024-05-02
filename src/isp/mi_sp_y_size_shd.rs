#[doc = "Register `MI_SP_Y_SIZE_SHD` reader"]
pub type R = crate::R<MiSpYSizeShdSpec>;
#[doc = "Field `sp_y_size` reader - Size of self picture Y component ring buffer."]
pub type SpYSizeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:28 - Size of self picture Y component ring buffer."]
    #[inline(always)]
    pub fn sp_y_size(&self) -> SpYSizeR {
        SpYSizeR::new((self.bits >> 3) & 0x03ff_ffff)
    }
}
#[doc = "Size shadow register of self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_size_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiSpYSizeShdSpec;
impl crate::RegisterSpec for MiSpYSizeShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_sp_y_size_shd::R`](R) reader structure"]
impl crate::Readable for MiSpYSizeShdSpec {}
#[doc = "`reset()` method sets MI_SP_Y_SIZE_SHD to value 0"]
impl crate::Resettable for MiSpYSizeShdSpec {
    const RESET_VALUE: u32 = 0;
}
