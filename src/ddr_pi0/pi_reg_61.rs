#[doc = "Register `PI_REG_61` reader"]
pub type R = crate::R<PiReg61Spec>;
#[doc = "Register `PI_REG_61` writer"]
pub type W = crate::W<PiReg61Spec>;
#[doc = "Field `PI_WRLVL_PERIODIC` reader - Enables the use of the dfi_lvl_periodic signal during write leveling. Set to 1 to enable."]
pub type PiWrlvlPeriodicR = crate::BitReader;
#[doc = "Field `PI_WRLVL_PERIODIC` writer - Enables the use of the dfi_lvl_periodic signal during write leveling. Set to 1 to enable."]
pub type PiWrlvlPeriodicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WRLVL_ON_SREF_EXIT` reader - Enables automatic write leveling on a self-refresh exit. Set to 1 to enable."]
pub type PiWrlvlOnSrefExitR = crate::BitReader;
#[doc = "Field `PI_WRLVL_ON_SREF_EXIT` writer - Enables automatic write leveling on a self-refresh exit. Set to 1 to enable."]
pub type PiWrlvlOnSrefExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WRLVL_RESP_MASK` reader - Indicates mask for the dfi_wrlvl_resp signal during write leveling."]
pub type PiWrlvlRespMaskR = crate::FieldReader;
#[doc = "Field `PI_WRLVL_RESP_MASK` writer - Indicates mask for the dfi_wrlvl_resp signal during write leveling."]
pub type PiWrlvlRespMaskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_WRLVL_ROTATE` reader - Enables rotational chip select for interval write leveling. Set to 1 for rotating chip select."]
pub type PiWrlvlRotateR = crate::BitReader;
#[doc = "Field `PI_WRLVL_ROTATE` writer - Enables rotational chip select for interval write leveling. Set to 1 for rotating chip select."]
pub type PiWrlvlRotateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables the use of the dfi_lvl_periodic signal during write leveling. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_wrlvl_periodic(&self) -> PiWrlvlPeriodicR {
        PiWrlvlPeriodicR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enables automatic write leveling on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_wrlvl_on_sref_exit(&self) -> PiWrlvlOnSrefExitR {
        PiWrlvlOnSrefExitR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Indicates mask for the dfi_wrlvl_resp signal during write leveling."]
    #[inline(always)]
    pub fn pi_wrlvl_resp_mask(&self) -> PiWrlvlRespMaskR {
        PiWrlvlRespMaskR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Enables rotational chip select for interval write leveling. Set to 1 for rotating chip select."]
    #[inline(always)]
    pub fn pi_wrlvl_rotate(&self) -> PiWrlvlRotateR {
        PiWrlvlRotateR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the use of the dfi_lvl_periodic signal during write leveling. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_periodic(&mut self) -> PiWrlvlPeriodicW<PiReg61Spec> {
        PiWrlvlPeriodicW::new(self, 0)
    }
    #[doc = "Bit 8 - Enables automatic write leveling on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_on_sref_exit(&mut self) -> PiWrlvlOnSrefExitW<PiReg61Spec> {
        PiWrlvlOnSrefExitW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Indicates mask for the dfi_wrlvl_resp signal during write leveling."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_resp_mask(&mut self) -> PiWrlvlRespMaskW<PiReg61Spec> {
        PiWrlvlRespMaskW::new(self, 16)
    }
    #[doc = "Bit 24 - Enables rotational chip select for interval write leveling. Set to 1 for rotating chip select."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_rotate(&mut self) -> PiWrlvlRotateW<PiReg61Spec> {
        PiWrlvlRotateW::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 61\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_61::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_61::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg61Spec;
impl crate::RegisterSpec for PiReg61Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_61::R`](R) reader structure"]
impl crate::Readable for PiReg61Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_61::W`](W) writer structure"]
impl crate::Writable for PiReg61Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_61 to value 0"]
impl crate::Resettable for PiReg61Spec {
    const RESET_VALUE: u32 = 0;
}
