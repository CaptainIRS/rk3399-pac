#[doc = "Register `FC_AUDICONF2` reader"]
pub type R = crate::R<FcAudiconf2Spec>;
#[doc = "Register `FC_AUDICONF2` writer"]
pub type W = crate::W<FcAudiconf2Spec>;
#[doc = "Field `CA` reader - Channel allocation"]
pub type CaR = crate::FieldReader;
#[doc = "Field `CA` writer - Channel allocation"]
pub type CaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Channel allocation"]
    #[inline(always)]
    pub fn ca(&self) -> CaR {
        CaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel allocation"]
    #[inline(always)]
    #[must_use]
    pub fn ca(&mut self) -> CaW<FcAudiconf2Spec> {
        CaW::new(self, 0)
    }
}
#[doc = "Channel allocation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audiconf2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audiconf2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAudiconf2Spec;
impl crate::RegisterSpec for FcAudiconf2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_audiconf2::R`](R) reader structure"]
impl crate::Readable for FcAudiconf2Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_audiconf2::W`](W) writer structure"]
impl crate::Writable for FcAudiconf2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AUDICONF2 to value 0"]
impl crate::Resettable for FcAudiconf2Spec {
    const RESET_VALUE: u8 = 0;
}
