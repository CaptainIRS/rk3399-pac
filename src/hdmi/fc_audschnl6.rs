#[doc = "Register `FC_AUDSCHNL6` reader"]
pub type R = crate::R<FcAudschnl6Spec>;
#[doc = "Register `FC_AUDSCHNL6` writer"]
pub type W = crate::W<FcAudschnl6Spec>;
#[doc = "Field `OIEC_CHANNELNUMCL2` reader - Channel number for third left sample"]
pub type OiecChannelnumcl2R = crate::FieldReader;
#[doc = "Field `OIEC_CHANNELNUMCL2` writer - Channel number for third left sample"]
pub type OiecChannelnumcl2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OIEC_CHANNELNUMCL3` reader - Channel number for fourth left sample"]
pub type OiecChannelnumcl3R = crate::FieldReader;
#[doc = "Field `OIEC_CHANNELNUMCL3` writer - Channel number for fourth left sample"]
pub type OiecChannelnumcl3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Channel number for third left sample"]
    #[inline(always)]
    pub fn oiec_channelnumcl2(&self) -> OiecChannelnumcl2R {
        OiecChannelnumcl2R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Channel number for fourth left sample"]
    #[inline(always)]
    pub fn oiec_channelnumcl3(&self) -> OiecChannelnumcl3R {
        OiecChannelnumcl3R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel number for third left sample"]
    #[inline(always)]
    #[must_use]
    pub fn oiec_channelnumcl2(&mut self) -> OiecChannelnumcl2W<FcAudschnl6Spec> {
        OiecChannelnumcl2W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Channel number for fourth left sample"]
    #[inline(always)]
    #[must_use]
    pub fn oiec_channelnumcl3(&mut self) -> OiecChannelnumcl3W<FcAudschnl6Spec> {
        OiecChannelnumcl3W::new(self, 4)
    }
}
#[doc = "Frame Composer Audio Sample Channel Status Configuration Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audschnl6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audschnl6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAudschnl6Spec;
impl crate::RegisterSpec for FcAudschnl6Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_audschnl6::R`](R) reader structure"]
impl crate::Readable for FcAudschnl6Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_audschnl6::W`](W) writer structure"]
impl crate::Writable for FcAudschnl6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AUDSCHNL6 to value 0"]
impl crate::Resettable for FcAudschnl6Spec {
    const RESET_VALUE: u8 = 0;
}
