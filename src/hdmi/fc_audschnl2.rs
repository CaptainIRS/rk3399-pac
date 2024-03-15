#[doc = "Register `FC_AUDSCHNL2` reader"]
pub type R = crate::R<FcAudschnl2Spec>;
#[doc = "Register `FC_AUDSCHNL2` writer"]
pub type W = crate::W<FcAudschnl2Spec>;
#[doc = "Field `OIEC_SOURCENUMBER` reader - Source number"]
pub type OiecSourcenumberR = crate::FieldReader;
#[doc = "Field `OIEC_SOURCENUMBER` writer - Source number"]
pub type OiecSourcenumberW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OIEC_PCMAUDIOMODE` reader - PCM audio mode"]
pub type OiecPcmaudiomodeR = crate::FieldReader;
#[doc = "Field `OIEC_PCMAUDIOMODE` writer - PCM audio mode"]
pub type OiecPcmaudiomodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Source number"]
    #[inline(always)]
    pub fn oiec_sourcenumber(&self) -> OiecSourcenumberR {
        OiecSourcenumberR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - PCM audio mode"]
    #[inline(always)]
    pub fn oiec_pcmaudiomode(&self) -> OiecPcmaudiomodeR {
        OiecPcmaudiomodeR::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:3 - Source number"]
    #[inline(always)]
    #[must_use]
    pub fn oiec_sourcenumber(&mut self) -> OiecSourcenumberW<FcAudschnl2Spec> {
        OiecSourcenumberW::new(self, 0)
    }
    #[doc = "Bits 4:6 - PCM audio mode"]
    #[inline(always)]
    #[must_use]
    pub fn oiec_pcmaudiomode(&mut self) -> OiecPcmaudiomodeW<FcAudschnl2Spec> {
        OiecPcmaudiomodeW::new(self, 4)
    }
}
#[doc = "Source number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audschnl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audschnl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAudschnl2Spec;
impl crate::RegisterSpec for FcAudschnl2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_audschnl2::R`](R) reader structure"]
impl crate::Readable for FcAudschnl2Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_audschnl2::W`](W) writer structure"]
impl crate::Writable for FcAudschnl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AUDSCHNL2 to value 0"]
impl crate::Resettable for FcAudschnl2Spec {
    const RESET_VALUE: u8 = 0;
}
