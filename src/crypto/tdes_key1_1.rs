#[doc = "Register `TDES_KEY1_1` reader"]
pub type R = crate::R<TdesKey1_1Spec>;
#[doc = "Register `TDES_KEY1_1` writer"]
pub type W = crate::W<TdesKey1_1Spec>;
#[doc = "Field `TDES_KEY1_1` reader - Specifies TDES key1 data \\[31:0\\]"]
pub type TdesKey1_1R = crate::FieldReader<u32>;
#[doc = "Field `TDES_KEY1_1` writer - Specifies TDES key1 data \\[31:0\\]"]
pub type TdesKey1_1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies TDES key1 data \\[31:0\\]"]
    #[inline(always)]
    pub fn tdes_key1_1(&self) -> TdesKey1_1R {
        TdesKey1_1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies TDES key1 data \\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn tdes_key1_1(&mut self) -> TdesKey1_1W<TdesKey1_1Spec> {
        TdesKey1_1W::new(self, 0)
    }
}
#[doc = "TDES Key1 data 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_key1_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdes_key1_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdesKey1_1Spec;
impl crate::RegisterSpec for TdesKey1_1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdes_key1_1::R`](R) reader structure"]
impl crate::Readable for TdesKey1_1Spec {}
#[doc = "`write(|w| ..)` method takes [`tdes_key1_1::W`](W) writer structure"]
impl crate::Writable for TdesKey1_1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDES_KEY1_1 to value 0"]
impl crate::Resettable for TdesKey1_1Spec {
    const RESET_VALUE: u32 = 0;
}
