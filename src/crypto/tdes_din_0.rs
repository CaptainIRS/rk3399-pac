#[doc = "Register `TDES_DIN_0` reader"]
pub type R = crate::R<TdesDin0Spec>;
#[doc = "Register `TDES_DIN_0` writer"]
pub type W = crate::W<TdesDin0Spec>;
#[doc = "Field `TDES_DIN_0` reader - Specifies TDES Input data \\[63:32\\]."]
pub type TdesDin0R = crate::FieldReader<u32>;
#[doc = "Field `TDES_DIN_0` writer - Specifies TDES Input data \\[63:32\\]."]
pub type TdesDin0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies TDES Input data \\[63:32\\]."]
    #[inline(always)]
    pub fn tdes_din_0(&self) -> TdesDin0R {
        TdesDin0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies TDES Input data \\[63:32\\]."]
    #[inline(always)]
    #[must_use]
    pub fn tdes_din_0(&mut self) -> TdesDin0W<TdesDin0Spec> {
        TdesDin0W::new(self, 0)
    }
}
#[doc = "TDES Input Data 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_din_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdes_din_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdesDin0Spec;
impl crate::RegisterSpec for TdesDin0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdes_din_0::R`](R) reader structure"]
impl crate::Readable for TdesDin0Spec {}
#[doc = "`write(|w| ..)` method takes [`tdes_din_0::W`](W) writer structure"]
impl crate::Writable for TdesDin0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDES_DIN_0 to value 0"]
impl crate::Resettable for TdesDin0Spec {
    const RESET_VALUE: u32 = 0;
}
