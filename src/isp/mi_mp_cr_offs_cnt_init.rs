#[doc = "Register `MI_MP_CR_OFFS_CNT_INIT` reader"]
pub type R = crate::R<MiMpCrOffsCntInitSpec>;
#[doc = "Register `MI_MP_CR_OFFS_CNT_INIT` writer"]
pub type W = crate::W<MiMpCrOffsCntInitSpec>;
#[doc = "Field `mp_cr_offs_cnt_init` reader - Offset counter init value of main picture Cr component\n\nring buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update.\n\nNote: Set control bit init_base_en before updating so\n\nthat a forced or automatic update can take effect. Check\n\nexceptional handling in skip modes."]
pub type MpCrOffsCntInitR = crate::FieldReader<u32>;
#[doc = "Field `mp_cr_offs_cnt_init` writer - Offset counter init value of main picture Cr component\n\nring buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update.\n\nNote: Set control bit init_base_en before updating so\n\nthat a forced or automatic update can take effect. Check\n\nexceptional handling in skip modes."]
pub type MpCrOffsCntInitW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 3:27 - Offset counter init value of main picture Cr component\n\nring buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update.\n\nNote: Set control bit init_base_en before updating so\n\nthat a forced or automatic update can take effect. Check\n\nexceptional handling in skip modes."]
    #[inline(always)]
    pub fn mp_cr_offs_cnt_init(&self) -> MpCrOffsCntInitR {
        MpCrOffsCntInitR::new((self.bits >> 3) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:27 - Offset counter init value of main picture Cr component\n\nring buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update.\n\nNote: Set control bit init_base_en before updating so\n\nthat a forced or automatic update can take effect. Check\n\nexceptional handling in skip modes."]
    #[inline(always)]
    #[must_use]
    pub fn mp_cr_offs_cnt_init(&mut self) -> MpCrOffsCntInitW<MiMpCrOffsCntInitSpec> {
        MpCrOffsCntInitW::new(self, 3)
    }
}
#[doc = "Offset counter init value for main picture Cr component ring buffer\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_cr_offs_cnt_init::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_mp_cr_offs_cnt_init::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiMpCrOffsCntInitSpec;
impl crate::RegisterSpec for MiMpCrOffsCntInitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_mp_cr_offs_cnt_init::R`](R) reader structure"]
impl crate::Readable for MiMpCrOffsCntInitSpec {}
#[doc = "`write(|w| ..)` method takes [`mi_mp_cr_offs_cnt_init::W`](W) writer structure"]
impl crate::Writable for MiMpCrOffsCntInitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_MP_CR_OFFS_CNT_INIT to value 0"]
impl crate::Resettable for MiMpCrOffsCntInitSpec {
    const RESET_VALUE: u32 = 0;
}
