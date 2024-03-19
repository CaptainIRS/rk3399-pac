#[doc = "Register `FC_AUDSCHNL8` reader"]
pub type R = crate::R<FcAudschnl8Spec>;
#[doc = "Register `FC_AUDSCHNL8` writer"]
pub type W = crate::W<FcAudschnl8Spec>;
#[doc = "Field `OIEC_WORDLENGTH` reader - Word length configuration"]
pub type OiecWordlengthR = crate::FieldReader;
#[doc = "Field `OIEC_WORDLENGTH` writer - Word length configuration"]
pub type OiecWordlengthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OIEC_ORIGSAMPFREQ` reader - Original sampling frequency"]
pub type OiecOrigsampfreqR = crate::FieldReader;
#[doc = "Field `OIEC_ORIGSAMPFREQ` writer - Original sampling frequency"]
pub type OiecOrigsampfreqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Word length configuration"]
    #[inline(always)]
    pub fn oiec_wordlength(&self) -> OiecWordlengthR {
        OiecWordlengthR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Original sampling frequency"]
    #[inline(always)]
    pub fn oiec_origsampfreq(&self) -> OiecOrigsampfreqR {
        OiecOrigsampfreqR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Word length configuration"]
    #[inline(always)]
    #[must_use]
    pub fn oiec_wordlength(&mut self) -> OiecWordlengthW<FcAudschnl8Spec> {
        OiecWordlengthW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Original sampling frequency"]
    #[inline(always)]
    #[must_use]
    pub fn oiec_origsampfreq(&mut self) -> OiecOrigsampfreqW<FcAudschnl8Spec> {
        OiecOrigsampfreqW::new(self, 4)
    }
}
#[doc = "Frame Composer Audio Sample Channel Status Configuration Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audschnl8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audschnl8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAudschnl8Spec;
impl crate::RegisterSpec for FcAudschnl8Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_audschnl8::R`](R) reader structure"]
impl crate::Readable for FcAudschnl8Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_audschnl8::W`](W) writer structure"]
impl crate::Writable for FcAudschnl8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AUDSCHNL8 to value 0"]
impl crate::Resettable for FcAudschnl8Spec {
    const RESET_VALUE: u8 = 0;
}
