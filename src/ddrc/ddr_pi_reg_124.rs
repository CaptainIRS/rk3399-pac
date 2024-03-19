#[doc = "Register `DDR_PI_REG_124` reader"]
pub type R = crate::R<DdrPiReg124Spec>;
#[doc = "Register `DDR_PI_REG_124` writer"]
pub type W = crate::W<DdrPiReg124Spec>;
#[doc = "Field `PI_WDQLVL_INTERVAL` reader - Indicates write DQ train interval counter program value."]
pub type PiWdqlvlIntervalR = crate::FieldReader<u16>;
#[doc = "Field `PI_WDQLVL_INTERVAL` writer - Indicates write DQ train interval counter program value."]
pub type PiWdqlvlIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_WDQLVL_EN` reader - Indicates if write DQ leveling is enabled. Bit1 represents the\n\nsupport when non-initialization. Bit0 represents the support when\n\ninitialization."]
pub type PiWdqlvlEnR = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_EN` writer - Indicates if write DQ leveling is enabled. Bit1 represents the\n\nsupport when non-initialization. Bit0 represents the support when\n\ninitialization."]
pub type PiWdqlvlEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_WDQLVL_ON_SREF_EXIT` reader - Issues a write DQ training command on self-refresh exit."]
pub type PiWdqlvlOnSrefExitR = crate::BitReader;
#[doc = "Field `PI_WDQLVL_ON_SREF_EXIT` writer - Issues a write DQ training command on self-refresh exit."]
pub type PiWdqlvlOnSrefExitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Indicates write DQ train interval counter program value."]
    #[inline(always)]
    pub fn pi_wdqlvl_interval(&self) -> PiWdqlvlIntervalR {
        PiWdqlvlIntervalR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Indicates if write DQ leveling is enabled. Bit1 represents the\n\nsupport when non-initialization. Bit0 represents the support when\n\ninitialization."]
    #[inline(always)]
    pub fn pi_wdqlvl_en(&self) -> PiWdqlvlEnR {
        PiWdqlvlEnR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Issues a write DQ training command on self-refresh exit."]
    #[inline(always)]
    pub fn pi_wdqlvl_on_sref_exit(&self) -> PiWdqlvlOnSrefExitR {
        PiWdqlvlOnSrefExitR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicates write DQ train interval counter program value."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_interval(&mut self) -> PiWdqlvlIntervalW<DdrPiReg124Spec> {
        PiWdqlvlIntervalW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Indicates if write DQ leveling is enabled. Bit1 represents the\n\nsupport when non-initialization. Bit0 represents the support when\n\ninitialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_en(&mut self) -> PiWdqlvlEnW<DdrPiReg124Spec> {
        PiWdqlvlEnW::new(self, 16)
    }
    #[doc = "Bit 24 - Issues a write DQ training command on self-refresh exit."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_on_sref_exit(&mut self) -> PiWdqlvlOnSrefExitW<DdrPiReg124Spec> {
        PiWdqlvlOnSrefExitW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 124\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_124::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_124::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg124Spec;
impl crate::RegisterSpec for DdrPiReg124Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_124::R`](R) reader structure"]
impl crate::Readable for DdrPiReg124Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_124::W`](W) writer structure"]
impl crate::Writable for DdrPiReg124Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_124 to value 0"]
impl crate::Resettable for DdrPiReg124Spec {
    const RESET_VALUE: u32 = 0;
}
