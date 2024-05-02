#[doc = "Register `MI_SP_Y_BASE_AD_INIT2` reader"]
pub type R = crate::R<MiSpYBaseAdInit2Spec>;
#[doc = "Register `MI_SP_Y_BASE_AD_INIT2` writer"]
pub type W = crate::W<MiSpYBaseAdInit2Spec>;
#[doc = "Field `sp_y_base_ad_init2` reader - 2nd ping pong base address of self picture Y\n\ncomponent buffer. Programmed value becomes effective\n\n(visible in corresponding shadow register) after a soft\n\nreset, a forced software update or an automatic config\n\nupdate.\n\nnote: Set control bit init_base_en before updating so\n\nthat a forced or automatic update can take effect."]
pub type SpYBaseAdInit2R = crate::FieldReader<u32>;
#[doc = "Field `sp_y_base_ad_init2` writer - 2nd ping pong base address of self picture Y\n\ncomponent buffer. Programmed value becomes effective\n\n(visible in corresponding shadow register) after a soft\n\nreset, a forced software update or an automatic config\n\nupdate.\n\nnote: Set control bit init_base_en before updating so\n\nthat a forced or automatic update can take effect."]
pub type SpYBaseAdInit2W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 3:31 - 2nd ping pong base address of self picture Y\n\ncomponent buffer. Programmed value becomes effective\n\n(visible in corresponding shadow register) after a soft\n\nreset, a forced software update or an automatic config\n\nupdate.\n\nnote: Set control bit init_base_en before updating so\n\nthat a forced or automatic update can take effect."]
    #[inline(always)]
    pub fn sp_y_base_ad_init2(&self) -> SpYBaseAdInit2R {
        SpYBaseAdInit2R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - 2nd ping pong base address of self picture Y\n\ncomponent buffer. Programmed value becomes effective\n\n(visible in corresponding shadow register) after a soft\n\nreset, a forced software update or an automatic config\n\nupdate.\n\nnote: Set control bit init_base_en before updating so\n\nthat a forced or automatic update can take effect."]
    #[inline(always)]
    #[must_use]
    pub fn sp_y_base_ad_init2(&mut self) -> SpYBaseAdInit2W<MiSpYBaseAdInit2Spec> {
        SpYBaseAdInit2W::new(self, 3)
    }
}
#[doc = "Base address 2 (ping pong) for self picture Y component\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_base_ad_init2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_y_base_ad_init2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiSpYBaseAdInit2Spec;
impl crate::RegisterSpec for MiSpYBaseAdInit2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_sp_y_base_ad_init2::R`](R) reader structure"]
impl crate::Readable for MiSpYBaseAdInit2Spec {}
#[doc = "`write(|w| ..)` method takes [`mi_sp_y_base_ad_init2::W`](W) writer structure"]
impl crate::Writable for MiSpYBaseAdInit2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_SP_Y_BASE_AD_INIT2 to value 0"]
impl crate::Resettable for MiSpYBaseAdInit2Spec {
    const RESET_VALUE: u32 = 0;
}
