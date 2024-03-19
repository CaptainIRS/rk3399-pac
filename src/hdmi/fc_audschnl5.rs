#[doc = "Register `FC_AUDSCHNL5` reader"]
pub type R = crate::R<FcAudschnl5Spec>;
#[doc = "Register `FC_AUDSCHNL5` writer"]
pub type W = crate::W<FcAudschnl5Spec>;
#[doc = "Field `OIEC_CHANNELNUMCL0` reader - Channel number for first left sample"]
pub type OiecChannelnumcl0R = crate::FieldReader;
#[doc = "Field `OIEC_CHANNELNUMCL0` writer - Channel number for first left sample"]
pub type OiecChannelnumcl0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OIEC_CHANNELNUMCL1` reader - Channel number for second left sample"]
pub type OiecChannelnumcl1R = crate::FieldReader;
#[doc = "Field `OIEC_CHANNELNUMCL1` writer - Channel number for second left sample"]
pub type OiecChannelnumcl1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Channel number for first left sample"]
    #[inline(always)]
    pub fn oiec_channelnumcl0(&self) -> OiecChannelnumcl0R {
        OiecChannelnumcl0R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Channel number for second left sample"]
    #[inline(always)]
    pub fn oiec_channelnumcl1(&self) -> OiecChannelnumcl1R {
        OiecChannelnumcl1R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel number for first left sample"]
    #[inline(always)]
    #[must_use]
    pub fn oiec_channelnumcl0(&mut self) -> OiecChannelnumcl0W<FcAudschnl5Spec> {
        OiecChannelnumcl0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Channel number for second left sample"]
    #[inline(always)]
    #[must_use]
    pub fn oiec_channelnumcl1(&mut self) -> OiecChannelnumcl1W<FcAudschnl5Spec> {
        OiecChannelnumcl1W::new(self, 4)
    }
}
#[doc = "Frame Composer Audio Sample Channel Status Configuration Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audschnl5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audschnl5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAudschnl5Spec;
impl crate::RegisterSpec for FcAudschnl5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_audschnl5::R`](R) reader structure"]
impl crate::Readable for FcAudschnl5Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_audschnl5::W`](W) writer structure"]
impl crate::Writable for FcAudschnl5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AUDSCHNL5 to value 0"]
impl crate::Resettable for FcAudschnl5Spec {
    const RESET_VALUE: u8 = 0;
}
