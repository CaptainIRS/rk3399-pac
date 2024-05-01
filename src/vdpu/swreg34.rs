#[doc = "Register `SWREG34` reader"]
pub type R = crate::R<Swreg34Spec>;
#[doc = "Register `SWREG34` writer"]
pub type W = crate::W<Swreg34Spec>;
#[doc = "Field `SW_PP_INW` reader - the picture width of PP input with in macro blocks which can be\n\ncropped from a bigger picture when in the condtion of external\n\nmode"]
pub type SwPpInwR = crate::FieldReader<u16>;
#[doc = "Field `SW_PP_INW` writer - the picture width of PP input with in macro blocks which can be\n\ncropped from a bigger picture when in the condtion of external\n\nmode"]
pub type SwPpInwW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SW_PP_INW_EXT` reader - PP input extended width\n\nin order to support jpeg"]
pub type SwPpInwExtR = crate::FieldReader;
#[doc = "Field `SW_PP_INW_EXT` writer - PP input extended width\n\nin order to support jpeg"]
pub type SwPpInwExtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_ORG_INW_EXT` reader - the orginal width of pp input pic in MBS"]
pub type SwOrgInwExtR = crate::FieldReader<u16>;
#[doc = "Field `SW_ORG_INW_EXT` writer - the orginal width of pp input pic in MBS"]
pub type SwOrgInwExtW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SW_PP_INH` reader - the picture height of PP input with in macro blocks which can be\n\ncropped from a bigger picture when in the condtion of external\n\nmode"]
pub type SwPpInhR = crate::FieldReader;
#[doc = "Field `SW_PP_INH` writer - the picture height of PP input with in macro blocks which can be\n\ncropped from a bigger picture when in the condtion of external\n\nmode"]
pub type SwPpInhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_PP_INH_EXT` reader - PP input extended height\n\nin order to support jpeg"]
pub type SwPpInhExtR = crate::FieldReader;
#[doc = "Field `SW_PP_INH_EXT` writer - PP input extended height\n\nin order to support jpeg"]
pub type SwPpInhExtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:8 - the picture width of PP input with in macro blocks which can be\n\ncropped from a bigger picture when in the condtion of external\n\nmode"]
    #[inline(always)]
    pub fn sw_pp_inw(&self) -> SwPpInwR {
        SwPpInwR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:11 - PP input extended width\n\nin order to support jpeg"]
    #[inline(always)]
    pub fn sw_pp_inw_ext(&self) -> SwPpInwExtR {
        SwPpInwExtR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:20 - the orginal width of pp input pic in MBS"]
    #[inline(always)]
    pub fn sw_org_inw_ext(&self) -> SwOrgInwExtR {
        SwOrgInwExtR::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bits 21:28 - the picture height of PP input with in macro blocks which can be\n\ncropped from a bigger picture when in the condtion of external\n\nmode"]
    #[inline(always)]
    pub fn sw_pp_inh(&self) -> SwPpInhR {
        SwPpInhR::new(((self.bits >> 21) & 0xff) as u8)
    }
    #[doc = "Bits 29:31 - PP input extended height\n\nin order to support jpeg"]
    #[inline(always)]
    pub fn sw_pp_inh_ext(&self) -> SwPpInhExtR {
        SwPpInhExtR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - the picture width of PP input with in macro blocks which can be\n\ncropped from a bigger picture when in the condtion of external\n\nmode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_inw(&mut self) -> SwPpInwW<Swreg34Spec> {
        SwPpInwW::new(self, 0)
    }
    #[doc = "Bits 9:11 - PP input extended width\n\nin order to support jpeg"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_inw_ext(&mut self) -> SwPpInwExtW<Swreg34Spec> {
        SwPpInwExtW::new(self, 9)
    }
    #[doc = "Bits 12:20 - the orginal width of pp input pic in MBS"]
    #[inline(always)]
    #[must_use]
    pub fn sw_org_inw_ext(&mut self) -> SwOrgInwExtW<Swreg34Spec> {
        SwOrgInwExtW::new(self, 12)
    }
    #[doc = "Bits 21:28 - the picture height of PP input with in macro blocks which can be\n\ncropped from a bigger picture when in the condtion of external\n\nmode"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_inh(&mut self) -> SwPpInhW<Swreg34Spec> {
        SwPpInhW::new(self, 21)
    }
    #[doc = "Bits 29:31 - PP input extended height\n\nin order to support jpeg"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pp_inh_ext(&mut self) -> SwPpInhExtW<Swreg34Spec> {
        SwPpInhExtW::new(self, 29)
    }
}
#[doc = "PP input pic size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg34::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg34::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg34Spec;
impl crate::RegisterSpec for Swreg34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg34::R`](R) reader structure"]
impl crate::Readable for Swreg34Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg34::W`](W) writer structure"]
impl crate::Writable for Swreg34Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG34 to value 0"]
impl crate::Resettable for Swreg34Spec {
    const RESET_VALUE: u32 = 0;
}
