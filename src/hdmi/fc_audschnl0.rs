#[doc = "Register `FC_AUDSCHNL0` reader"]
pub type R = crate::R<FcAudschnl0Spec>;
#[doc = "Register `FC_AUDSCHNL0` writer"]
pub type W = crate::W<FcAudschnl0Spec>;
#[doc = "Field `OIEC_COPYRIGHT` reader - IEC Copyright indication"]
pub type OiecCopyrightR = crate::BitReader;
#[doc = "Field `OIEC_COPYRIGHT` writer - IEC Copyright indication"]
pub type OiecCopyrightW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIEC_CGMSA` reader - CGMS-A"]
pub type OiecCgmsaR = crate::FieldReader;
#[doc = "Field `OIEC_CGMSA` writer - CGMS-A"]
pub type OiecCgmsaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - IEC Copyright indication"]
    #[inline(always)]
    pub fn oiec_copyright(&self) -> OiecCopyrightR {
        OiecCopyrightR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - CGMS-A"]
    #[inline(always)]
    pub fn oiec_cgmsa(&self) -> OiecCgmsaR {
        OiecCgmsaR::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - IEC Copyright indication"]
    #[inline(always)]
    #[must_use]
    pub fn oiec_copyright(&mut self) -> OiecCopyrightW<FcAudschnl0Spec> {
        OiecCopyrightW::new(self, 0)
    }
    #[doc = "Bits 4:5 - CGMS-A"]
    #[inline(always)]
    #[must_use]
    pub fn oiec_cgmsa(&mut self) -> OiecCgmsaW<FcAudschnl0Spec> {
        OiecCgmsaW::new(self, 4)
    }
}
#[doc = "Frame Composer Audio Sample Channel Status Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audschnl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audschnl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAudschnl0Spec;
impl crate::RegisterSpec for FcAudschnl0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_audschnl0::R`](R) reader structure"]
impl crate::Readable for FcAudschnl0Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_audschnl0::W`](W) writer structure"]
impl crate::Writable for FcAudschnl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AUDSCHNL0 to value 0"]
impl crate::Resettable for FcAudschnl0Spec {
    const RESET_VALUE: u8 = 0;
}
