#[doc = "Register `MI_SP_CB_OFFS_CNT_INIT` reader"]
pub type R = crate::R<MiSpCbOffsCntInitSpec>;
#[doc = "Register `MI_SP_CB_OFFS_CNT_INIT` writer"]
pub type W = crate::W<MiSpCbOffsCntInitSpec>;
#[doc = "Field `sp_cb_offs_cnt_init` reader - Offset counter init value of self picture Cb component\n\nring buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update.\n\nNote: Set control bit init_base_en before updating so\n\nthat a forced or automatic update can take effect."]
pub type SpCbOffsCntInitR = crate::FieldReader<u32>;
#[doc = "Field `sp_cb_offs_cnt_init` writer - Offset counter init value of self picture Cb component\n\nring buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update.\n\nNote: Set control bit init_base_en before updating so\n\nthat a forced or automatic update can take effect."]
pub type SpCbOffsCntInitW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 3:27 - Offset counter init value of self picture Cb component\n\nring buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update.\n\nNote: Set control bit init_base_en before updating so\n\nthat a forced or automatic update can take effect."]
    #[inline(always)]
    pub fn sp_cb_offs_cnt_init(&self) -> SpCbOffsCntInitR {
        SpCbOffsCntInitR::new((self.bits >> 3) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:27 - Offset counter init value of self picture Cb component\n\nring buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update.\n\nNote: Set control bit init_base_en before updating so\n\nthat a forced or automatic update can take effect."]
    #[inline(always)]
    #[must_use]
    pub fn sp_cb_offs_cnt_init(&mut self) -> SpCbOffsCntInitW<MiSpCbOffsCntInitSpec> {
        SpCbOffsCntInitW::new(self, 3)
    }
}
#[doc = "Offset counter init value for self picture Cb component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_sp_cb_offs_cnt_init::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_sp_cb_offs_cnt_init::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiSpCbOffsCntInitSpec;
impl crate::RegisterSpec for MiSpCbOffsCntInitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_sp_cb_offs_cnt_init::R`](R) reader structure"]
impl crate::Readable for MiSpCbOffsCntInitSpec {}
#[doc = "`write(|w| ..)` method takes [`mi_sp_cb_offs_cnt_init::W`](W) writer structure"]
impl crate::Writable for MiSpCbOffsCntInitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_SP_CB_OFFS_CNT_INIT to value 0"]
impl crate::Resettable for MiSpCbOffsCntInitSpec {
    const RESET_VALUE: u32 = 0;
}
