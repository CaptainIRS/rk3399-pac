#[doc = "Register `MI_MP_Y_OFFS_CNT_INIT` reader"]
pub type R = crate::R<MiMpYOffsCntInitSpec>;
#[doc = "Register `MI_MP_Y_OFFS_CNT_INIT` writer"]
pub type W = crate::W<MiMpYOffsCntInitSpec>;
#[doc = "Field `mp_y_offs_cnt_init` reader - Offset counter init value of main picture Y component\n\nring buffer, JPEG ring buffer or raw data ring buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update."]
pub type MpYOffsCntInitR = crate::FieldReader<u32>;
#[doc = "Field `mp_y_offs_cnt_init` writer - Offset counter init value of main picture Y component\n\nring buffer, JPEG ring buffer or raw data ring buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update."]
pub type MpYOffsCntInitW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 3:28 - Offset counter init value of main picture Y component\n\nring buffer, JPEG ring buffer or raw data ring buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update."]
    #[inline(always)]
    pub fn mp_y_offs_cnt_init(&self) -> MpYOffsCntInitR {
        MpYOffsCntInitR::new((self.bits >> 3) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:28 - Offset counter init value of main picture Y component\n\nring buffer, JPEG ring buffer or raw data ring buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update."]
    #[inline(always)]
    #[must_use]
    pub fn mp_y_offs_cnt_init(&mut self) -> MpYOffsCntInitW<MiMpYOffsCntInitSpec> {
        MpYOffsCntInitW::new(self, 3)
    }
}
#[doc = "Offset counter init value for main picture Y, JPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\nSet control bit init_base_en before updating so that a forced or automatic update can \n\n\n\ntake effect. Check exceptional handling in skip modes. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_y_offs_cnt_init::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_mp_y_offs_cnt_init::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiMpYOffsCntInitSpec;
impl crate::RegisterSpec for MiMpYOffsCntInitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_mp_y_offs_cnt_init::R`](R) reader structure"]
impl crate::Readable for MiMpYOffsCntInitSpec {}
#[doc = "`write(|w| ..)` method takes [`mi_mp_y_offs_cnt_init::W`](W) writer structure"]
impl crate::Writable for MiMpYOffsCntInitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_MP_Y_OFFS_CNT_INIT to value 0"]
impl crate::Resettable for MiMpYOffsCntInitSpec {
    const RESET_VALUE: u32 = 0;
}
