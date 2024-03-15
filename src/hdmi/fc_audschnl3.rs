#[doc = "Register `FC_AUDSCHNL3` reader"]
pub type R = crate::R<FcAudschnl3Spec>;
#[doc = "Register `FC_AUDSCHNL3` writer"]
pub type W = crate::W<FcAudschnl3Spec>;
#[doc = "Field `OIEC_CHANNELNUMCR0` reader - Channel number for first right sample"]
pub type OiecChannelnumcr0R = crate::FieldReader;
#[doc = "Field `OIEC_CHANNELNUMCR0` writer - Channel number for first right sample"]
pub type OiecChannelnumcr0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OIEC_CHANNELNUMCR1` reader - Channel number for second right sample"]
pub type OiecChannelnumcr1R = crate::FieldReader;
#[doc = "Field `OIEC_CHANNELNUMCR1` writer - Channel number for second right sample"]
pub type OiecChannelnumcr1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Channel number for first right sample"]
    #[inline(always)]
    pub fn oiec_channelnumcr0(&self) -> OiecChannelnumcr0R {
        OiecChannelnumcr0R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Channel number for second right sample"]
    #[inline(always)]
    pub fn oiec_channelnumcr1(&self) -> OiecChannelnumcr1R {
        OiecChannelnumcr1R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel number for first right sample"]
    #[inline(always)]
    #[must_use]
    pub fn oiec_channelnumcr0(&mut self) -> OiecChannelnumcr0W<FcAudschnl3Spec> {
        OiecChannelnumcr0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Channel number for second right sample"]
    #[inline(always)]
    #[must_use]
    pub fn oiec_channelnumcr1(&mut self) -> OiecChannelnumcr1W<FcAudschnl3Spec> {
        OiecChannelnumcr1W::new(self, 4)
    }
}
#[doc = "Channel number for first right sample\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audschnl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audschnl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAudschnl3Spec;
impl crate::RegisterSpec for FcAudschnl3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_audschnl3::R`](R) reader structure"]
impl crate::Readable for FcAudschnl3Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_audschnl3::W`](W) writer structure"]
impl crate::Writable for FcAudschnl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AUDSCHNL3 to value 0"]
impl crate::Resettable for FcAudschnl3Spec {
    const RESET_VALUE: u8 = 0;
}
