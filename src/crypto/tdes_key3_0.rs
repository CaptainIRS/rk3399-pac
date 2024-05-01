#[doc = "Register `TDES_KEY3_0` reader"]
pub type R = crate::R<TdesKey3_0Spec>;
#[doc = "Register `TDES_KEY3_0` writer"]
pub type W = crate::W<TdesKey3_0Spec>;
#[doc = "Field `TDES_KEY3_0` reader - Specifies TDES key3 data \\[63:32\\]"]
pub type TdesKey3_0R = crate::FieldReader<u32>;
#[doc = "Field `TDES_KEY3_0` writer - Specifies TDES key3 data \\[63:32\\]"]
pub type TdesKey3_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies TDES key3 data \\[63:32\\]"]
    #[inline(always)]
    pub fn tdes_key3_0(&self) -> TdesKey3_0R {
        TdesKey3_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies TDES key3 data \\[63:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn tdes_key3_0(&mut self) -> TdesKey3_0W<TdesKey3_0Spec> {
        TdesKey3_0W::new(self, 0)
    }
}
#[doc = "TDES Key3 data 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_key3_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdes_key3_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdesKey3_0Spec;
impl crate::RegisterSpec for TdesKey3_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdes_key3_0::R`](R) reader structure"]
impl crate::Readable for TdesKey3_0Spec {}
#[doc = "`write(|w| ..)` method takes [`tdes_key3_0::W`](W) writer structure"]
impl crate::Writable for TdesKey3_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDES_KEY3_0 to value 0"]
impl crate::Resettable for TdesKey3_0Spec {
    const RESET_VALUE: u32 = 0;
}
