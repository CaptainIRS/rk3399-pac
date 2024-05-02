#[doc = "Register `BLS_D_FIXED` reader"]
pub type R = crate::R<BlsDFixedSpec>;
#[doc = "Register `BLS_D_FIXED` writer"]
pub type W = crate::W<BlsDFixedSpec>;
#[doc = "Field `BLS_D_FIXED` reader - Fixed black level for D pixels - signed\n\ntwo's complement, value range from -4096 to +4095\n\n"]
pub type BlsDFixedR = crate::FieldReader<u16>;
#[doc = "Field `BLS_D_FIXED` writer - Fixed black level for D pixels - signed\n\ntwo's complement, value range from -4096 to +4095\n\n"]
pub type BlsDFixedW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Fixed black level for D pixels - signed\n\ntwo's complement, value range from -4096 to +4095\n\n"]
    #[inline(always)]
    pub fn bls_d_fixed(&self) -> BlsDFixedR {
        BlsDFixedR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Fixed black level for D pixels - signed\n\ntwo's complement, value range from -4096 to +4095\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn bls_d_fixed(&mut self) -> BlsDFixedW<BlsDFixedSpec> {
        BlsDFixedW::new(self, 0)
    }
}
#[doc = "fixed black level D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_d_fixed::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_d_fixed::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlsDFixedSpec;
impl crate::RegisterSpec for BlsDFixedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bls_d_fixed::R`](R) reader structure"]
impl crate::Readable for BlsDFixedSpec {}
#[doc = "`write(|w| ..)` method takes [`bls_d_fixed::W`](W) writer structure"]
impl crate::Writable for BlsDFixedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLS_D_FIXED to value 0"]
impl crate::Resettable for BlsDFixedSpec {
    const RESET_VALUE: u32 = 0;
}
