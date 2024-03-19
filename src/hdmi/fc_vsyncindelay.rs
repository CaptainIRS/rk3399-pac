#[doc = "Register `FC_VSYNCINDELAY` reader"]
pub type R = crate::R<FcVsyncindelaySpec>;
#[doc = "Register `FC_VSYNCINDELAY` writer"]
pub type W = crate::W<FcVsyncindelaySpec>;
#[doc = "Field `V_IN_DELAY` reader - Input video Vsync active edge delay. Integer\n\nnumber of Hsync pulses from \"de\" non active edge\n\nof the last \"de\" valid period. \\[0...255\\]."]
pub type VInDelayR = crate::FieldReader;
#[doc = "Field `V_IN_DELAY` writer - Input video Vsync active edge delay. Integer\n\nnumber of Hsync pulses from \"de\" non active edge\n\nof the last \"de\" valid period. \\[0...255\\]."]
pub type VInDelayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Input video Vsync active edge delay. Integer\n\nnumber of Hsync pulses from \"de\" non active edge\n\nof the last \"de\" valid period. \\[0...255\\]."]
    #[inline(always)]
    pub fn v_in_delay(&self) -> VInDelayR {
        VInDelayR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input video Vsync active edge delay. Integer\n\nnumber of Hsync pulses from \"de\" non active edge\n\nof the last \"de\" valid period. \\[0...255\\]."]
    #[inline(always)]
    #[must_use]
    pub fn v_in_delay(&mut self) -> VInDelayW<FcVsyncindelaySpec> {
        VInDelayW::new(self, 0)
    }
}
#[doc = "Frame Composer Input Video VSync Front Porch Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_vsyncindelay::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_vsyncindelay::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcVsyncindelaySpec;
impl crate::RegisterSpec for FcVsyncindelaySpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_vsyncindelay::R`](R) reader structure"]
impl crate::Readable for FcVsyncindelaySpec {}
#[doc = "`write(|w| ..)` method takes [`fc_vsyncindelay::W`](W) writer structure"]
impl crate::Writable for FcVsyncindelaySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_VSYNCINDELAY to value 0"]
impl crate::Resettable for FcVsyncindelaySpec {
    const RESET_VALUE: u8 = 0;
}
