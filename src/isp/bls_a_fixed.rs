#[doc = "Register `BLS_A_FIXED` reader"]
pub type R = crate::R<BlsAFixedSpec>;
#[doc = "Register `BLS_A_FIXED` writer"]
pub type W = crate::W<BlsAFixedSpec>;
#[doc = "Field `BLS_A_FIXED` reader - Fixed black level for A pixels – signed\n\ntwo's complement, value range from -4096 to +4095,\n\na positive value will be subtracted from the pixel\n\nvalues\n\n"]
pub type BlsAFixedR = crate::FieldReader<u16>;
#[doc = "Field `BLS_A_FIXED` writer - Fixed black level for A pixels – signed\n\ntwo's complement, value range from -4096 to +4095,\n\na positive value will be subtracted from the pixel\n\nvalues\n\n"]
pub type BlsAFixedW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Fixed black level for A pixels – signed\n\ntwo's complement, value range from -4096 to +4095,\n\na positive value will be subtracted from the pixel\n\nvalues\n\n"]
    #[inline(always)]
    pub fn bls_a_fixed(&self) -> BlsAFixedR {
        BlsAFixedR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Fixed black level for A pixels – signed\n\ntwo's complement, value range from -4096 to +4095,\n\na positive value will be subtracted from the pixel\n\nvalues\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn bls_a_fixed(&mut self) -> BlsAFixedW<BlsAFixedSpec> {
        BlsAFixedW::new(self, 0)
    }
}
#[doc = "fixed black level A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_a_fixed::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_a_fixed::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlsAFixedSpec;
impl crate::RegisterSpec for BlsAFixedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bls_a_fixed::R`](R) reader structure"]
impl crate::Readable for BlsAFixedSpec {}
#[doc = "`write(|w| ..)` method takes [`bls_a_fixed::W`](W) writer structure"]
impl crate::Writable for BlsAFixedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLS_A_FIXED to value 0"]
impl crate::Resettable for BlsAFixedSpec {
    const RESET_VALUE: u32 = 0;
}
