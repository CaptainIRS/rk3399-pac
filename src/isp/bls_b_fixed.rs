#[doc = "Register `BLS_B_FIXED` reader"]
pub type R = crate::R<BlsBFixedSpec>;
#[doc = "Register `BLS_B_FIXED` writer"]
pub type W = crate::W<BlsBFixedSpec>;
#[doc = "Field `BLS_B_FIXED` reader - Fixed black level for B pixels – signed\n\ntwo's complement, value range from -4096 to +4095"]
pub type BlsBFixedR = crate::FieldReader<u16>;
#[doc = "Field `BLS_B_FIXED` writer - Fixed black level for B pixels – signed\n\ntwo's complement, value range from -4096 to +4095"]
pub type BlsBFixedW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Fixed black level for B pixels – signed\n\ntwo's complement, value range from -4096 to +4095"]
    #[inline(always)]
    pub fn bls_b_fixed(&self) -> BlsBFixedR {
        BlsBFixedR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Fixed black level for B pixels – signed\n\ntwo's complement, value range from -4096 to +4095"]
    #[inline(always)]
    #[must_use]
    pub fn bls_b_fixed(&mut self) -> BlsBFixedW<BlsBFixedSpec> {
        BlsBFixedW::new(self, 0)
    }
}
#[doc = "fixed black level B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_b_fixed::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_b_fixed::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlsBFixedSpec;
impl crate::RegisterSpec for BlsBFixedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bls_b_fixed::R`](R) reader structure"]
impl crate::Readable for BlsBFixedSpec {}
#[doc = "`write(|w| ..)` method takes [`bls_b_fixed::W`](W) writer structure"]
impl crate::Writable for BlsBFixedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLS_B_FIXED to value 0"]
impl crate::Resettable for BlsBFixedSpec {
    const RESET_VALUE: u32 = 0;
}
