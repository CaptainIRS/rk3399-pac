#[doc = "Register `MI_MP_Y_IRQ_OFFS_INIT` reader"]
pub type R = crate::R<MiMpYIrqOffsInitSpec>;
#[doc = "Register `MI_MP_Y_IRQ_OFFS_INIT` writer"]
pub type W = crate::W<MiMpYIrqOffsInitSpec>;
#[doc = "Field `mp_y_irq_offs_init` reader - Reaching this programmed value by the current offset\n\ncounter for addressing main picture Y component, JPEG or\n\nraw data leads to generation of fill level interrupt fill_mp_y.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a\n\nforced software update or an automatic config update."]
pub type MpYIrqOffsInitR = crate::FieldReader<u32>;
#[doc = "Field `mp_y_irq_offs_init` writer - Reaching this programmed value by the current offset\n\ncounter for addressing main picture Y component, JPEG or\n\nraw data leads to generation of fill level interrupt fill_mp_y.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a\n\nforced software update or an automatic config update."]
pub type MpYIrqOffsInitW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 3:28 - Reaching this programmed value by the current offset\n\ncounter for addressing main picture Y component, JPEG or\n\nraw data leads to generation of fill level interrupt fill_mp_y.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a\n\nforced software update or an automatic config update."]
    #[inline(always)]
    pub fn mp_y_irq_offs_init(&self) -> MpYIrqOffsInitR {
        MpYIrqOffsInitR::new((self.bits >> 3) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:28 - Reaching this programmed value by the current offset\n\ncounter for addressing main picture Y component, JPEG or\n\nraw data leads to generation of fill level interrupt fill_mp_y.\n\nProgrammed value becomes effective (visible in\n\ncorresponding shadow register) after a soft reset, a\n\nforced software update or an automatic config update."]
    #[inline(always)]
    #[must_use]
    pub fn mp_y_irq_offs_init(&mut self) -> MpYIrqOffsInitW<MiMpYIrqOffsInitSpec> {
        MpYIrqOffsInitW::new(self, 3)
    }
}
#[doc = "Fill level interrupt offset value for main picture Y, JPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\nregister description for details. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_y_irq_offs_init::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_mp_y_irq_offs_init::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiMpYIrqOffsInitSpec;
impl crate::RegisterSpec for MiMpYIrqOffsInitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_mp_y_irq_offs_init::R`](R) reader structure"]
impl crate::Readable for MiMpYIrqOffsInitSpec {}
#[doc = "`write(|w| ..)` method takes [`mi_mp_y_irq_offs_init::W`](W) writer structure"]
impl crate::Writable for MiMpYIrqOffsInitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_MP_Y_IRQ_OFFS_INIT to value 0"]
impl crate::Resettable for MiMpYIrqOffsInitSpec {
    const RESET_VALUE: u32 = 0;
}
