#[doc = "Register `TDES_DIN_1` reader"]
pub type R = crate::R<TdesDin1Spec>;
#[doc = "Register `TDES_DIN_1` writer"]
pub type W = crate::W<TdesDin1Spec>;
#[doc = "Field `TDES_DIN_1` reader - Specifies TDES Input data \\[31:0\\]."]
pub type TdesDin1R = crate::FieldReader<u32>;
#[doc = "Field `TDES_DIN_1` writer - Specifies TDES Input data \\[31:0\\]."]
pub type TdesDin1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies TDES Input data \\[31:0\\]."]
    #[inline(always)]
    pub fn tdes_din_1(&self) -> TdesDin1R {
        TdesDin1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies TDES Input data \\[31:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn tdes_din_1(&mut self) -> TdesDin1W<TdesDin1Spec> {
        TdesDin1W::new(self, 0)
    }
}
#[doc = "TDES Input Data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_din_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdes_din_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdesDin1Spec;
impl crate::RegisterSpec for TdesDin1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdes_din_1::R`](R) reader structure"]
impl crate::Readable for TdesDin1Spec {}
#[doc = "`write(|w| ..)` method takes [`tdes_din_1::W`](W) writer structure"]
impl crate::Writable for TdesDin1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDES_DIN_1 to value 0"]
impl crate::Resettable for TdesDin1Spec {
    const RESET_VALUE: u32 = 0;
}
