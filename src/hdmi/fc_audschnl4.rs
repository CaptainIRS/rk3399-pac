#[doc = "Register `FC_AUDSCHNL4` reader"]
pub type R = crate::R<FcAudschnl4Spec>;
#[doc = "Register `FC_AUDSCHNL4` writer"]
pub type W = crate::W<FcAudschnl4Spec>;
#[doc = "Field `OIEC_CHANNELNUMCR2` reader - Channel number for third right sample"]
pub type OiecChannelnumcr2R = crate::FieldReader;
#[doc = "Field `OIEC_CHANNELNUMCR2` writer - Channel number for third right sample"]
pub type OiecChannelnumcr2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OIEC_CHANNELNUMCR3` reader - Channel number for fourth right sample"]
pub type OiecChannelnumcr3R = crate::FieldReader;
#[doc = "Field `OIEC_CHANNELNUMCR3` writer - Channel number for fourth right sample"]
pub type OiecChannelnumcr3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Channel number for third right sample"]
    #[inline(always)]
    pub fn oiec_channelnumcr2(&self) -> OiecChannelnumcr2R {
        OiecChannelnumcr2R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Channel number for fourth right sample"]
    #[inline(always)]
    pub fn oiec_channelnumcr3(&self) -> OiecChannelnumcr3R {
        OiecChannelnumcr3R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel number for third right sample"]
    #[inline(always)]
    #[must_use]
    pub fn oiec_channelnumcr2(&mut self) -> OiecChannelnumcr2W<FcAudschnl4Spec> {
        OiecChannelnumcr2W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Channel number for fourth right sample"]
    #[inline(always)]
    #[must_use]
    pub fn oiec_channelnumcr3(&mut self) -> OiecChannelnumcr3W<FcAudschnl4Spec> {
        OiecChannelnumcr3W::new(self, 4)
    }
}
#[doc = "Frame Composer Audio Sample Channel Status Configuration Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audschnl4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audschnl4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAudschnl4Spec;
impl crate::RegisterSpec for FcAudschnl4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_audschnl4::R`](R) reader structure"]
impl crate::Readable for FcAudschnl4Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_audschnl4::W`](W) writer structure"]
impl crate::Writable for FcAudschnl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AUDSCHNL4 to value 0"]
impl crate::Resettable for FcAudschnl4Spec {
    const RESET_VALUE: u8 = 0;
}
