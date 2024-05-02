#[doc = "Register `MI_SP_Y_BASE_AD_INIT` reader"]
pub type R = crate::R<MiSpYBaseAdInitSpec>;
#[doc = "Register `MI_SP_Y_BASE_AD_INIT` writer"]
pub type W = crate::W<MiSpYBaseAdInitSpec>;
#[doc = "Field `sp_y_base_ad_init` reader - Base address of self picture Y component ring buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update.\n\nnote: Set control bit init_base_en before\n\nupdating so that a forced or automatic update can\n\ntake effect."]
pub type SpYBaseAdInitR = crate::FieldReader<u32>;
#[doc = "Field `sp_y_base_ad_init` writer - Base address of self picture Y component ring buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update.\n\nnote: Set control bit init_base_en before\n\nupdating so that a forced or automatic update can\n\ntake effect."]
pub type SpYBaseAdInitW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 3:31 - Base address of self picture Y component ring buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update.\n\nnote: Set control bit init_base_en before\n\nupdating so that a forced or automatic update can\n\ntake effect."]
    #[inline(always)]
    pub fn sp_y_base_ad_init(&self) -> SpYBaseAdInitR {
        SpYBaseAdInitR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - Base address of self picture Y component ring buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update.\n\nnote: Set control bit init_base_en before\n\nupdating so that a forced or automatic update can\n\ntake effect."]
    #[inline(always)]
    #[must_use]
    pub fn sp_y_base_ad_init(&mut self) -> SpYBaseAdInitW<MiSpYBaseAdInitSpec> {
        SpYBaseAdInitW::new(self, 3)
    }
}
#[doc = "Base address for self picture Y component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nRefer also to MI_MP_Y_BASE_AD_INIT with respect to the burst alignment restriction for \n\nAXI. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_y_base_ad_init::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_y_base_ad_init::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiSpYBaseAdInitSpec;
impl crate::RegisterSpec for MiSpYBaseAdInitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_sp_y_base_ad_init::R`](R) reader structure"]
impl crate::Readable for MiSpYBaseAdInitSpec {}
#[doc = "`write(|w| ..)` method takes [`mi_sp_y_base_ad_init::W`](W) writer structure"]
impl crate::Writable for MiSpYBaseAdInitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_SP_Y_BASE_AD_INIT to value 0"]
impl crate::Resettable for MiSpYBaseAdInitSpec {
    const RESET_VALUE: u32 = 0;
}
