#[doc = "Register `MI_MP_Y_BASE_AD_INIT` reader"]
pub type R = crate::R<MiMpYBaseAdInitSpec>;
#[doc = "Register `MI_MP_Y_BASE_AD_INIT` writer"]
pub type W = crate::W<MiMpYBaseAdInitSpec>;
#[doc = "Field `mp_y_base_ad_init` reader - Base address of main picture Y component ring\n\nbuffer, JPEG ring buffer or raw data ring buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update."]
pub type MpYBaseAdInitR = crate::FieldReader<u32>;
#[doc = "Field `mp_y_base_ad_init` writer - Base address of main picture Y component ring\n\nbuffer, JPEG ring buffer or raw data ring buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update."]
pub type MpYBaseAdInitW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 3:31 - Base address of main picture Y component ring\n\nbuffer, JPEG ring buffer or raw data ring buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update."]
    #[inline(always)]
    pub fn mp_y_base_ad_init(&self) -> MpYBaseAdInitR {
        MpYBaseAdInitR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - Base address of main picture Y component ring\n\nbuffer, JPEG ring buffer or raw data ring buffer.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a forced\n\nsoftware update or an automatic config update."]
    #[inline(always)]
    #[must_use]
    pub fn mp_y_base_ad_init(&mut self) -> MpYBaseAdInitW<MiMpYBaseAdInitSpec> {
        MpYBaseAdInitW::new(self, 3)
    }
}
#[doc = "Base address for main picture Y component, JPEG or raw data\n\nNote: This register protects from non-aligned access. The bits 0 to 2 are hard wired to \n\n'000'. As a consequence any byte address that is written to the register will automatically be \n\nre-mapped to the next lower 64 bit aligned address: write(MI_MP_Y_BASE_AD_INIT, \n\naddress_value) is equivalent to write(MI_Y_BASE_AD_INIT, address_value &amp; 0xFFFFFFF8). \n\nAnyhow, in order to avoid confusion it is NOT recommended to use non-aligned address values \n\nfor access. It is also NOT recommended to actively consider the register slice for register \n\naccess in order to avoid unneccessary mask and shift operations. \n\n\n\nIn addition, if ISP provides AXI interfaces the programmed base address shall be \n\n\n\nburst aligned with respect to the burst length configured in MI_CTRL . \n\n\n\nSet control bit init_base_en before updating so that a forced or automatic update can take \n\n\n\neffect. \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_y_base_ad_init::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_mp_y_base_ad_init::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiMpYBaseAdInitSpec;
impl crate::RegisterSpec for MiMpYBaseAdInitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_mp_y_base_ad_init::R`](R) reader structure"]
impl crate::Readable for MiMpYBaseAdInitSpec {}
#[doc = "`write(|w| ..)` method takes [`mi_mp_y_base_ad_init::W`](W) writer structure"]
impl crate::Writable for MiMpYBaseAdInitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_MP_Y_BASE_AD_INIT to value 0"]
impl crate::Resettable for MiMpYBaseAdInitSpec {
    const RESET_VALUE: u32 = 0;
}
