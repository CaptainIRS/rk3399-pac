#[doc = "Register `MI_SP_CB_BASE_AD_INIT2` reader"]
pub type R = crate::R<MiSpCbBaseAdInit2Spec>;
#[doc = "Register `MI_SP_CB_BASE_AD_INIT2` writer"]
pub type W = crate::W<MiSpCbBaseAdInit2Spec>;
#[doc = "Field `sp_cb_base_ad_init2` reader - 2nd ping pong base address of self picture Cb\n\ncomponent buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a\n\nforced software update or an automatic config update.\n\nnote: Set control bit init_base_en before\n\nupdating so that a forced or automatic update can\n\ntake effect."]
pub type SpCbBaseAdInit2R = crate::FieldReader<u32>;
#[doc = "Field `sp_cb_base_ad_init2` writer - 2nd ping pong base address of self picture Cb\n\ncomponent buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a\n\nforced software update or an automatic config update.\n\nnote: Set control bit init_base_en before\n\nupdating so that a forced or automatic update can\n\ntake effect."]
pub type SpCbBaseAdInit2W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 3:31 - 2nd ping pong base address of self picture Cb\n\ncomponent buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a\n\nforced software update or an automatic config update.\n\nnote: Set control bit init_base_en before\n\nupdating so that a forced or automatic update can\n\ntake effect."]
    #[inline(always)]
    pub fn sp_cb_base_ad_init2(&self) -> SpCbBaseAdInit2R {
        SpCbBaseAdInit2R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - 2nd ping pong base address of self picture Cb\n\ncomponent buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a\n\nforced software update or an automatic config update.\n\nnote: Set control bit init_base_en before\n\nupdating so that a forced or automatic update can\n\ntake effect."]
    #[inline(always)]
    #[must_use]
    pub fn sp_cb_base_ad_init2(&mut self) -> SpCbBaseAdInit2W<MiSpCbBaseAdInit2Spec> {
        SpCbBaseAdInit2W::new(self, 3)
    }
}
#[doc = "Base address 2 (pingpong) for self picture Cb component\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cb_base_ad_init2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_cb_base_ad_init2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiSpCbBaseAdInit2Spec;
impl crate::RegisterSpec for MiSpCbBaseAdInit2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_sp_cb_base_ad_init2::R`](R) reader structure"]
impl crate::Readable for MiSpCbBaseAdInit2Spec {}
#[doc = "`write(|w| ..)` method takes [`mi_sp_cb_base_ad_init2::W`](W) writer structure"]
impl crate::Writable for MiSpCbBaseAdInit2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_SP_CB_BASE_AD_INIT2 to value 0"]
impl crate::Resettable for MiSpCbBaseAdInit2Spec {
    const RESET_VALUE: u32 = 0;
}
