#[doc = "Register `CABC_GAMMA_LUT_ADDR` reader"]
pub type R = crate::R<CabcGammaLutAddrSpec>;
#[doc = "Register `CABC_GAMMA_LUT_ADDR` writer"]
pub type W = crate::W<CabcGammaLutAddrSpec>;
#[doc = "Field `GAMMA_LUT_ADDR` reader - the head of gamma lut address"]
pub type GammaLutAddrR = crate::FieldReader<u32>;
#[doc = "Field `GAMMA_LUT_ADDR` writer - the head of gamma lut address"]
pub type GammaLutAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the head of gamma lut address"]
    #[inline(always)]
    pub fn gamma_lut_addr(&self) -> GammaLutAddrR {
        GammaLutAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the head of gamma lut address"]
    #[inline(always)]
    #[must_use]
    pub fn gamma_lut_addr(&mut self) -> GammaLutAddrW<CabcGammaLutAddrSpec> {
        GammaLutAddrW::new(self, 0)
    }
}
#[doc = "CABC GAMMA lut base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cabc_gamma_lut_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cabc_gamma_lut_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CabcGammaLutAddrSpec;
impl crate::RegisterSpec for CabcGammaLutAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cabc_gamma_lut_addr::R`](R) reader structure"]
impl crate::Readable for CabcGammaLutAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`cabc_gamma_lut_addr::W`](W) writer structure"]
impl crate::Writable for CabcGammaLutAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CABC_GAMMA_LUT_ADDR to value 0"]
impl crate::Resettable for CabcGammaLutAddrSpec {
    const RESET_VALUE: u32 = 0;
}
