#[doc = "Register `MI_SP_CR_SIZE_SHD` reader"]
pub type R = crate::R<MiSpCrSizeShdSpec>;
#[doc = "Field `sp_cr_size` reader - Size of self picture Cr component ring buffer."]
pub type SpCrSizeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:27 - Size of self picture Cr component ring buffer."]
    #[inline(always)]
    pub fn sp_cr_size(&self) -> SpCrSizeR {
        SpCrSizeR::new((self.bits >> 3) & 0x01ff_ffff)
    }
}
#[doc = "Size shadow register of self picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cr_size_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiSpCrSizeShdSpec;
impl crate::RegisterSpec for MiSpCrSizeShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_sp_cr_size_shd::R`](R) reader structure"]
impl crate::Readable for MiSpCrSizeShdSpec {}
#[doc = "`reset()` method sets MI_SP_CR_SIZE_SHD to value 0"]
impl crate::Resettable for MiSpCrSizeShdSpec {
    const RESET_VALUE: u32 = 0;
}
