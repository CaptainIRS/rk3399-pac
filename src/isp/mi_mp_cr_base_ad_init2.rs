#[doc = "Register `MI_MP_CR_BASE_AD_INIT2` reader"]
pub type R = crate::R<MiMpCrBaseAdInit2Spec>;
#[doc = "Register `MI_MP_CR_BASE_AD_INIT2` writer"]
pub type W = crate::W<MiMpCrBaseAdInit2Spec>;
#[doc = "Field `mp_cr_base_ad_init2` reader - 2nd ping pong Base address of main picture Cr\n\ncomponent buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update.\n\nNote: Set control bit init_base_en before updating so\n\nthat a forced or automatic update can take effect."]
pub type MpCrBaseAdInit2R = crate::FieldReader<u32>;
#[doc = "Field `mp_cr_base_ad_init2` writer - 2nd ping pong Base address of main picture Cr\n\ncomponent buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update.\n\nNote: Set control bit init_base_en before updating so\n\nthat a forced or automatic update can take effect."]
pub type MpCrBaseAdInit2W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 3:31 - 2nd ping pong Base address of main picture Cr\n\ncomponent buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update.\n\nNote: Set control bit init_base_en before updating so\n\nthat a forced or automatic update can take effect."]
    #[inline(always)]
    pub fn mp_cr_base_ad_init2(&self) -> MpCrBaseAdInit2R {
        MpCrBaseAdInit2R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - 2nd ping pong Base address of main picture Cr\n\ncomponent buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update.\n\nNote: Set control bit init_base_en before updating so\n\nthat a forced or automatic update can take effect."]
    #[inline(always)]
    #[must_use]
    pub fn mp_cr_base_ad_init2(&mut self) -> MpCrBaseAdInit2W<MiMpCrBaseAdInit2Spec> {
        MpCrBaseAdInit2W::new(self, 3)
    }
}
#[doc = "Base address 2 (pingpong) for main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cr_base_ad_init2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_mp_cr_base_ad_init2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiMpCrBaseAdInit2Spec;
impl crate::RegisterSpec for MiMpCrBaseAdInit2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_mp_cr_base_ad_init2::R`](R) reader structure"]
impl crate::Readable for MiMpCrBaseAdInit2Spec {}
#[doc = "`write(|w| ..)` method takes [`mi_mp_cr_base_ad_init2::W`](W) writer structure"]
impl crate::Writable for MiMpCrBaseAdInit2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_MP_CR_BASE_AD_INIT2 to value 0"]
impl crate::Resettable for MiMpCrBaseAdInit2Spec {
    const RESET_VALUE: u32 = 0;
}
