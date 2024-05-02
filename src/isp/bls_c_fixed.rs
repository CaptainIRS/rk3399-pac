#[doc = "Register `BLS_C_FIXED` reader"]
pub type R = crate::R<BlsCFixedSpec>;
#[doc = "Register `BLS_C_FIXED` writer"]
pub type W = crate::W<BlsCFixedSpec>;
#[doc = "Field `BLS_C_FIXED` reader - Fixed black level for C pixels – signed\n\ntwo's complement, value range from -4096 to +4095\n\n"]
pub type BlsCFixedR = crate::FieldReader<u16>;
#[doc = "Field `BLS_C_FIXED` writer - Fixed black level for C pixels – signed\n\ntwo's complement, value range from -4096 to +4095\n\n"]
pub type BlsCFixedW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Fixed black level for C pixels – signed\n\ntwo's complement, value range from -4096 to +4095\n\n"]
    #[inline(always)]
    pub fn bls_c_fixed(&self) -> BlsCFixedR {
        BlsCFixedR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Fixed black level for C pixels – signed\n\ntwo's complement, value range from -4096 to +4095\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn bls_c_fixed(&mut self) -> BlsCFixedW<BlsCFixedSpec> {
        BlsCFixedW::new(self, 0)
    }
}
#[doc = "fixed black level C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_c_fixed::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_c_fixed::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlsCFixedSpec;
impl crate::RegisterSpec for BlsCFixedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bls_c_fixed::R`](R) reader structure"]
impl crate::Readable for BlsCFixedSpec {}
#[doc = "`write(|w| ..)` method takes [`bls_c_fixed::W`](W) writer structure"]
impl crate::Writable for BlsCFixedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLS_C_FIXED to value 0"]
impl crate::Resettable for BlsCFixedSpec {
    const RESET_VALUE: u32 = 0;
}
