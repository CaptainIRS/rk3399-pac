#[doc = "Register `IS_V_OFFS_SHD` reader"]
pub type R = crate::R<IsVOffsShdSpec>;
#[doc = "Field `is_v_offs_shd` reader - current vertical picture offset in lines\n\n"]
pub type IsVOffsShdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - current vertical picture offset in lines\n\n"]
    #[inline(always)]
    pub fn is_v_offs_shd(&self) -> IsVOffsShdR {
        IsVOffsShdR::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "current vertical offset of output window (shadow register)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_v_offs_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsVOffsShdSpec;
impl crate::RegisterSpec for IsVOffsShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`is_v_offs_shd::R`](R) reader structure"]
impl crate::Readable for IsVOffsShdSpec {}
#[doc = "`reset()` method sets IS_V_OFFS_SHD to value 0"]
impl crate::Resettable for IsVOffsShdSpec {
    const RESET_VALUE: u32 = 0;
}
