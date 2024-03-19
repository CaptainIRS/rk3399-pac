#[doc = "Register `FC_AUDSCHNL1` reader"]
pub type R = crate::R<FcAudschnl1Spec>;
#[doc = "Register `FC_AUDSCHNL1` writer"]
pub type W = crate::W<FcAudschnl1Spec>;
#[doc = "Field `OIEC_CATEGORYCODE` reader - Category code"]
pub type OiecCategorycodeR = crate::FieldReader;
#[doc = "Field `OIEC_CATEGORYCODE` writer - Category code"]
pub type OiecCategorycodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Category code"]
    #[inline(always)]
    pub fn oiec_categorycode(&self) -> OiecCategorycodeR {
        OiecCategorycodeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Category code"]
    #[inline(always)]
    #[must_use]
    pub fn oiec_categorycode(&mut self) -> OiecCategorycodeW<FcAudschnl1Spec> {
        OiecCategorycodeW::new(self, 0)
    }
}
#[doc = "Frame Composer Audio Sample Channel Status Configuration Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audschnl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audschnl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAudschnl1Spec;
impl crate::RegisterSpec for FcAudschnl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_audschnl1::R`](R) reader structure"]
impl crate::Readable for FcAudschnl1Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_audschnl1::W`](W) writer structure"]
impl crate::Writable for FcAudschnl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AUDSCHNL1 to value 0"]
impl crate::Resettable for FcAudschnl1Spec {
    const RESET_VALUE: u8 = 0;
}
