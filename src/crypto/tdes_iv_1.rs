#[doc = "Register `TDES_IV_1` reader"]
pub type R = crate::R<TdesIv1Spec>;
#[doc = "Register `TDES_IV_1` writer"]
pub type W = crate::W<TdesIv1Spec>;
#[doc = "Field `TDES_IV_1` reader - Specifies TDES Initialization vector \\[31:0\\]"]
pub type TdesIv1R = crate::FieldReader<u32>;
#[doc = "Field `TDES_IV_1` writer - Specifies TDES Initialization vector \\[31:0\\]"]
pub type TdesIv1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies TDES Initialization vector \\[31:0\\]"]
    #[inline(always)]
    pub fn tdes_iv_1(&self) -> TdesIv1R {
        TdesIv1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies TDES Initialization vector \\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn tdes_iv_1(&mut self) -> TdesIv1W<TdesIv1Spec> {
        TdesIv1W::new(self, 0)
    }
}
#[doc = "TDES IV data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_iv_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdes_iv_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdesIv1Spec;
impl crate::RegisterSpec for TdesIv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdes_iv_1::R`](R) reader structure"]
impl crate::Readable for TdesIv1Spec {}
#[doc = "`write(|w| ..)` method takes [`tdes_iv_1::W`](W) writer structure"]
impl crate::Writable for TdesIv1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDES_IV_1 to value 0"]
impl crate::Resettable for TdesIv1Spec {
    const RESET_VALUE: u32 = 0;
}
