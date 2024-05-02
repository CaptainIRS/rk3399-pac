#[doc = "Register `IS_H_SIZE_SHD` reader"]
pub type R = crate::R<IsHSizeShdSpec>;
#[doc = "Field `isp_h_size_shd` reader - current horizontal picture size in pixel\n\n"]
pub type IspHSizeShdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - current horizontal picture size in pixel\n\n"]
    #[inline(always)]
    pub fn isp_h_size_shd(&self) -> IspHSizeShdR {
        IspHSizeShdR::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "current output horizontal picture size (shadow register)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_h_size_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsHSizeShdSpec;
impl crate::RegisterSpec for IsHSizeShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`is_h_size_shd::R`](R) reader structure"]
impl crate::Readable for IsHSizeShdSpec {}
#[doc = "`reset()` method sets IS_H_SIZE_SHD to value 0"]
impl crate::Resettable for IsHSizeShdSpec {
    const RESET_VALUE: u32 = 0;
}
