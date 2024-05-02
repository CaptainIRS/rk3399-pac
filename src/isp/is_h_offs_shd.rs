#[doc = "Register `IS_H_OFFS_SHD` reader"]
pub type R = crate::R<IsHOffsShdSpec>;
#[doc = "Field `is_h_offs_shd` reader - current horizonatl picture offset in lines\n\n"]
pub type IsHOffsShdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - current horizonatl picture offset in lines\n\n"]
    #[inline(always)]
    pub fn is_h_offs_shd(&self) -> IsHOffsShdR {
        IsHOffsShdR::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "current horizontal offset of output window (shadow register)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`is_h_offs_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsHOffsShdSpec;
impl crate::RegisterSpec for IsHOffsShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`is_h_offs_shd::R`](R) reader structure"]
impl crate::Readable for IsHOffsShdSpec {}
#[doc = "`reset()` method sets IS_H_OFFS_SHD to value 0"]
impl crate::Resettable for IsHOffsShdSpec {
    const RESET_VALUE: u32 = 0;
}
