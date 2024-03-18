#[doc = "Register `PI_REG_60` reader"]
pub type R = crate::R<PiReg60Spec>;
#[doc = "Register `PI_REG_60` writer"]
pub type W = crate::W<PiReg60Spec>;
#[doc = "Field `PI_WLMRD` reader - Indicates delay from the issuing MRS to the first write leveling strobe."]
pub type PiWlmrdR = crate::FieldReader;
#[doc = "Field `PI_WLMRD` writer - Indicates delay from the issuing MRS to the first write leveling strobe."]
pub type PiWlmrdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_WRLVL_EN` reader - Enables the PI write leveling module. Bit1 represents the support when non-initialization. Bit0 represents the support when initialization. Set to 1 to enable."]
pub type PiWrlvlEnR = crate::FieldReader;
#[doc = "Field `PI_WRLVL_EN` writer - Enables the PI write leveling module. Bit1 represents the support when non-initialization. Bit0 represents the support when initialization. Set to 1 to enable."]
pub type PiWrlvlEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_WRLVL_INTERVAL` reader - Indicates the number of long count sequences that are counted between automatic write leveling commands."]
pub type PiWrlvlIntervalR = crate::FieldReader<u16>;
#[doc = "Field `PI_WRLVL_INTERVAL` writer - Indicates the number of long count sequences that are counted between automatic write leveling commands."]
pub type PiWrlvlIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:5 - Indicates delay from the issuing MRS to the first write leveling strobe."]
    #[inline(always)]
    pub fn pi_wlmrd(&self) -> PiWlmrdR {
        PiWlmrdR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - Enables the PI write leveling module. Bit1 represents the support when non-initialization. Bit0 represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_wrlvl_en(&self) -> PiWrlvlEnR {
        PiWrlvlEnR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:31 - Indicates the number of long count sequences that are counted between automatic write leveling commands."]
    #[inline(always)]
    pub fn pi_wrlvl_interval(&self) -> PiWrlvlIntervalR {
        PiWrlvlIntervalR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - Indicates delay from the issuing MRS to the first write leveling strobe."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wlmrd(&mut self) -> PiWlmrdW<PiReg60Spec> {
        PiWlmrdW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Enables the PI write leveling module. Bit1 represents the support when non-initialization. Bit0 represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_en(&mut self) -> PiWrlvlEnW<PiReg60Spec> {
        PiWrlvlEnW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Indicates the number of long count sequences that are counted between automatic write leveling commands."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_interval(&mut self) -> PiWrlvlIntervalW<PiReg60Spec> {
        PiWrlvlIntervalW::new(self, 16)
    }
}
#[doc = "DDR PHY Independent Register 60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_60::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_60::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg60Spec;
impl crate::RegisterSpec for PiReg60Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_60::R`](R) reader structure"]
impl crate::Readable for PiReg60Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_60::W`](W) writer structure"]
impl crate::Writable for PiReg60Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_60 to value 0"]
impl crate::Resettable for PiReg60Spec {
    const RESET_VALUE: u32 = 0;
}
