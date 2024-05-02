#[doc = "Register `IS_V_SIZE_SHD` reader"]
pub type R = crate::R<IsVSizeShdSpec>;
#[doc = "Field `isp_v_size_shd` reader - vertical picture size in lines\n\n"]
pub type IspVSizeShdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - vertical picture size in lines\n\n"]
    #[inline(always)]
    pub fn isp_v_size_shd(&self) -> IspVSizeShdR {
        IspVSizeShdR::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "current output vertical picture size (shadow register)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_v_size_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsVSizeShdSpec;
impl crate::RegisterSpec for IsVSizeShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`is_v_size_shd::R`](R) reader structure"]
impl crate::Readable for IsVSizeShdSpec {}
#[doc = "`reset()` method sets IS_V_SIZE_SHD to value 0"]
impl crate::Resettable for IsVSizeShdSpec {
    const RESET_VALUE: u32 = 0;
}
