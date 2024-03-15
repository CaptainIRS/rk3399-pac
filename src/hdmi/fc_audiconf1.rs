#[doc = "Register `FC_AUDICONF1` reader"]
pub type R = crate::R<FcAudiconf1Spec>;
#[doc = "Register `FC_AUDICONF1` writer"]
pub type W = crate::W<FcAudiconf1Spec>;
#[doc = "Field `SF` reader - Sampling frequency"]
pub type SfR = crate::FieldReader;
#[doc = "Field `SF` writer - Sampling frequency"]
pub type SfW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SS` reader - Sampling size"]
pub type SsR = crate::FieldReader;
#[doc = "Field `SS` writer - Sampling size"]
pub type SsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Sampling frequency"]
    #[inline(always)]
    pub fn sf(&self) -> SfR {
        SfR::new(self.bits & 7)
    }
    #[doc = "Bits 4:5 - Sampling size"]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sampling frequency"]
    #[inline(always)]
    #[must_use]
    pub fn sf(&mut self) -> SfW<FcAudiconf1Spec> {
        SfW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Sampling size"]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SsW<FcAudiconf1Spec> {
        SsW::new(self, 4)
    }
}
#[doc = "Sampling frequency\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audiconf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audiconf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAudiconf1Spec;
impl crate::RegisterSpec for FcAudiconf1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_audiconf1::R`](R) reader structure"]
impl crate::Readable for FcAudiconf1Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_audiconf1::W`](W) writer structure"]
impl crate::Writable for FcAudiconf1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AUDICONF1 to value 0"]
impl crate::Resettable for FcAudiconf1Spec {
    const RESET_VALUE: u8 = 0;
}
